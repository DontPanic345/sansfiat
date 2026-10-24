//! The map: smoothed noise for height, moisture and fertility inside a hex, sea at the rim,
//! inlets, inland seas and lakes, ridges with hilly flanks, deserts in the rain shadow of the
//! ridges, coast beside water, food scaled by fertility in patches, and the hidden trail costs.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::hex::Hex;
use crate::hex::DIRS;
use crate::market::Basket;
use crate::rng::{rand, rand_index};

/// What kind of country a tile is in. Only `Open` until closed country is built.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Zone {
    #[default]
    Open,
    Valley,
    Sealed,
    Neck,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub q: i32,
    pub r: i32,
    pub terrain: usize,
    /// Scales the terrain's food caps, `FERTILITY.0..=FERTILITY.1`.
    pub fertility: f64,
    pub cap: Basket,
    pub stock: Basket,
    /// The grain cap before taming.
    pub wild_grain: f64,
    pub tame: u32,
    /// The clan whose round the tile is on.
    pub claim: Option<u32>,
    /// Turns in a row on a round, and on none.
    pub held: u32,
    pub unheld: u32,
    pub trail: bool,
    /// River tiles upstream including this one; 0 is no river.
    pub river: u32,
    /// The tile downstream (for a mouth, the water it empties into).
    pub flow: Option<usize>,
    /// Sea that the rim's sea does not reach: a lake or an inland sea.
    pub lake: bool,
    /// A floodplain.
    pub flood: bool,
    pub zone: Zone,
    /// A gap in a ridge that a valley is entered by.
    pub pass: bool,
    /// Hidden: the trail cost a forest, hills or crossable mountain tile will have (nothing reads it yet).
    pub trail_cost: Option<f64>,
    /// Hidden: tiles a bridge over this mountain must span.
    pub span: Option<u8>,
}

impl Tile {
    pub fn hex(&self) -> Hex {
        Hex { q: self.q, r: self.r }
    }
}

/// The position of `h` in `Hex::ORIGIN.within(radius)`, the order `generate` builds tiles in.
pub fn index(radius: i32, h: Hex) -> Option<usize> {
    if h.distance(Hex::ORIGIN) > radius {
        return None;
    }
    let (q, r, rad) = (h.q as i64, h.r as i64, radius as i64);
    // Columns -rad..q come first; column c holds 2·rad + 1 − |c| tiles.
    let cols = q + rad;
    let tri = |n: i64| n * (n + 1) / 2;
    let abs_sum = if q > 0 { tri(rad) + tri(q - 1) } else { tri(rad) - tri(-q) };
    let before = cols * (2 * rad + 1) - abs_sum;
    Some((before + r - (-rad).max(-q - rad)) as usize)
}

/// Mixed into the main stream's state to seed the hidden trail costs' own stream.
const HIDDEN_SALT: u32 = 0x5bd1e995;

pub fn generate(radius: i32, rng: &mut u32) -> Vec<Tile> {
    generate_with(radius, rng, HIDDEN_SALT).tiles
}

/// A generated map and what the generator drew that the tiles don't show.
pub struct Generated {
    pub tiles: Vec<Tile>,
    /// The wind's direction, an index into `DIRS`: it blows from that side.
    pub wind: usize,
}

/// `generate` with the hidden stream's salt given: a different salt changes only the hidden
/// trail costs.
pub fn generate_with(radius: i32, rng: &mut u32, salt: u32) -> Generated {
    let hexes = Hex::ORIGIN.within(radius);
    let n = hexes.len();
    let nbrs: Vec<Vec<usize>> =
        hexes.iter().map(|h| h.neighbors().filter_map(|x| index(radius, x)).collect()).collect();
    let edge: Vec<i32> = hexes.iter().map(|h| radius - h.distance(Hex::ORIGIN)).collect();

    // Height, moisture and fertility: noise, smoothed, then stretched to 0..1.
    let (mut h, mut m, mut y) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
    for i in 0..n {
        h[i] = rand(rng);
        m[i] = rand(rng);
        y[i] = rand(rng);
    }
    for field in [&mut h, &mut m, &mut y] {
        for _ in 0..3 {
            *field = (0..n)
                .map(|i| {
                    (2.0 * field[i] + nbrs[i].iter().map(|&j| field[j]).sum::<f64>()) / (2.0 + nbrs[i].len() as f64)
                })
                .collect();
        }
        let lo = field.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = field.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let span = if hi > lo { hi - lo } else { 1.0 };
        *field = field.iter().map(|v| (v - lo) / span).collect();
    }

    let mut terrain: Vec<usize> = (0..n)
        .map(|i| {
            let height = h[i] - if edge[i] < 2 { 0.25 } else { 0.0 };
            if edge[i] == 0 || height < 0.35 {
                SEA
            } else if height > 0.88 {
                MOUNTAIN
            } else if height > 0.72 {
                HILLS
            } else if m[i] > 0.6 {
                FOREST
            } else {
                PLAINS
            }
        })
        .collect();

    carve_water(radius, rng, &hexes, &edge, &nbrs, &mut terrain);
    raise_ridges(radius, rng, &hexes, &edge, &mut terrain);
    let wind = make_deserts(radius, rng, &hexes, &m, &mut terrain);

    let shore: Vec<usize> = (0..n)
        .filter(|&i| matches!(terrain[i], PLAINS | FOREST | DESERT) && nbrs[i].iter().any(|&j| terrain[j] == SEA))
        .collect();
    for i in shore {
        terrain[i] = COAST;
    }

    // Sea the rim's sea can't reach is a lake or an inland sea.
    let mut lake: Vec<bool> = (0..n).map(|i| terrain[i] == SEA).collect();
    let mut todo: Vec<usize> = (0..n).filter(|&i| edge[i] == 0 && terrain[i] == SEA).collect();
    for &i in &todo {
        lake[i] = false;
    }
    while let Some(i) = todo.pop() {
        for &j in &nbrs[i] {
            if lake[j] {
                lake[j] = false;
                todo.push(j);
            }
        }
    }

    // Fertility in patches (y was drawn with the terrain noise).
    let mut tiles: Vec<Tile> = hexes
        .iter()
        .enumerate()
        .map(|(i, hx)| Tile {
            q: hx.q,
            r: hx.r,
            terrain: terrain[i],
            fertility: round1(FERTILITY.0 + (FERTILITY.1 - FERTILITY.0) * libm::pow(y[i], 1.5)),
            cap: [0.0; N],
            stock: [0.0; N],
            wild_grain: 0.0,
            tame: 0,
            claim: None,
            held: 0,
            unheld: 0,
            trail: false,
            river: 0,
            flow: None,
            lake: lake[i],
            flood: false,
            zone: Zone::Open,
            pass: false,
            trail_cost: None,
            span: None,
        })
        .collect();
    finish_caps(&mut tiles);
    roll_trails(&mut tiles, *rng ^ salt);
    Generated { tiles, wind }
}

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

/// Every tile's food caps from its terrain and fertility, and its stock full. Features that
/// change a tile's food (rivers, floodplains, deltas) adjust the tile before this runs.
pub fn finish_caps(tiles: &mut [Tile]) {
    for t in tiles {
        t.cap = TERRAIN[t.terrain].food.map(|f| round1(f * t.fertility));
        t.stock = t.cap;
        t.wild_grain = t.cap[GRAIN];
    }
}

/// A wandering line of hexes from `start`, mostly holding one heading.
fn walk(rng: &mut u32, start: Hex, length: usize, wiggle: f64) -> Vec<Hex> {
    let mut dir = rand_index(rng, 6);
    let mut path = vec![start];
    for _ in 1..length {
        if rand(rng) < wiggle {
            dir = (dir + if rand(rng) < 0.5 { 5 } else { 1 }) % 6;
        }
        let last = path[path.len() - 1];
        path.push(Hex { q: last.q + DIRS[dir].0, r: last.r + DIRS[dir].1 });
    }
    path
}

/// Inlets cut in from the coast to leave peninsulas, inland seas and lakes in the interior.
/// Counts scale with area; a feature with no room is skipped.
fn carve_water(radius: i32, rng: &mut u32, hexes: &[Hex], edge: &[i32], nbrs: &[Vec<usize>], terrain: &mut [usize]) {
    let n = terrain.len();
    let set_sea = |terrain: &mut [usize], h: Hex| {
        if let Some(i) = index(radius, h) {
            if edge[i] > 0 {
                terrain[i] = SEA;
            }
        }
    };
    let coast_sea: Vec<usize> =
        (0..n).filter(|&i| edge[i] >= 2 && terrain[i] == SEA && nbrs[i].iter().any(|&j| terrain[j] != SEA)).collect();
    if !coast_sea.is_empty() {
        for _ in 0..scaled(MAP.inlets, n) {
            let start = coast_sea[rand_index(rng, coast_sea.len())];
            let length = 6 + rand_index(rng, 10);
            for h in walk(rng, hexes[start], length, 0.2) {
                set_sea(terrain, h);
            }
        }
    }
    let inland: Vec<usize> = (0..n).filter(|&i| edge[i] >= 5).collect();
    if !inland.is_empty() {
        for _ in 0..scaled(MAP.inland_seas, n) {
            let start = inland[rand_index(rng, inland.len())];
            let length = 5 + rand_index(rng, 6);
            for h in walk(rng, hexes[start], length, 0.5) {
                for b in h.within(1) {
                    set_sea(terrain, b);
                }
            }
        }
        // Lakes: one to three tiles of land.
        let dry: Vec<usize> = inland.iter().copied().filter(|&i| terrain[i] != SEA).collect();
        if !dry.is_empty() {
            for _ in 0..scaled(MAP.lakes, n) {
                let start = dry[rand_index(rng, dry.len())];
                let length = 1 + rand_index(rng, 3);
                for h in walk(rng, hexes[start], length, 0.5) {
                    set_sea(terrain, h);
                }
            }
        }
    }
}

/// Long wandering mountain chains with hills along their flanks and the odd pass. Ridges
/// don't overwrite sea. A pass is hills where the chain would have been mountain.
fn raise_ridges(radius: i32, rng: &mut u32, hexes: &[Hex], edge: &[i32], terrain: &mut [usize]) {
    let n = terrain.len();
    let land: Vec<usize> = (0..n).filter(|&i| terrain[i] != SEA).collect();
    if land.is_empty() {
        return;
    }
    let (lo, hi) = MAP.ridge_length;
    for _ in 0..scaled(MAP.ridges, n).max(MAP.ridges_floor) {
        let start = land[rand_index(rng, land.len())];
        let length = lo + rand_index(rng, hi - lo);
        for h in walk(rng, hexes[start], length, 0.25) {
            let i = index(radius, h);
            if i.is_some_and(|i| terrain[i] == SEA) {
                continue;
            }
            let pass = rand(rng) < MAP.pass_chance;
            if let Some(i) = i.filter(|&i| edge[i] > 0) {
                terrain[i] = if pass { HILLS } else { MOUNTAIN };
            }
            for j in h.neighbors().filter_map(|x| index(radius, x)) {
                if matches!(terrain[j], PLAINS | FOREST | DESERT) && rand(rng) < MAP.flank_hills {
                    terrain[j] = HILLS;
                }
            }
        }
    }
}

/// The wind blows from one side; a plains tile with ridge upwind of it is in rain shadow.
/// The driest plains (moisture less the shadow) become desert, up to `desert_share` of the land.
fn make_deserts(radius: i32, rng: &mut u32, hexes: &[Hex], m: &[f64], terrain: &mut [usize]) -> usize {
    let wind = rand_index(rng, 6);
    let land = terrain.iter().filter(|&&t| t != SEA).count();
    let quota = (land as f64 * MAP.desert_share).floor() as usize;
    let mut plains: Vec<(f64, usize)> = (0..terrain.len())
        .filter(|&i| terrain[i] == PLAINS)
        .map(|i| (m[i] - MAP.shadow_weight * shadow(radius, hexes[i], wind, terrain) as f64, i))
        .collect();
    plains.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    for &(_, i) in plains.iter().take(quota) {
        terrain[i] = DESERT;
    }
    wind
}

/// Mountain tiles upwind of `h` within `MAP.shadow_reach` steps along `DIRS[wind]`, at most 3.
pub fn shadow(radius: i32, h: Hex, wind: usize, terrain: &[usize]) -> usize {
    (1..=MAP.shadow_reach)
        .filter(|&k| {
            let x = Hex { q: h.q + DIRS[wind].0 * k, r: h.r + DIRS[wind].1 * k };
            index(radius, x).is_some_and(|j| terrain[j] == MOUNTAIN)
        })
        .count()
        .min(3)
}

/// A value from `pairs` in proportion to its weight.
fn weighted<T: Copy>(rng: &mut u32, pairs: &[(T, u32)]) -> T {
    let mut x = rand(rng) * pairs.iter().map(|p| p.1).sum::<u32>() as f64;
    for &(v, w) in pairs {
        x -= w as f64;
        if x < 0.0 {
            return v;
        }
    }
    pairs[pairs.len() - 1].0
}

/// Rolls the hidden trail cost of each forest and hills tile, and the cost and bridge span of
/// each mountain, from a stream of their own so the rest of the map doesn't depend on them.
fn roll_trails(tiles: &mut [Tile], seed: u32) {
    let mut sub = seed;
    for t in tiles {
        match t.terrain {
            FOREST => t.trail_cost = Some(weighted(&mut sub, &MAP.forest_cost) as f64),
            HILLS => t.trail_cost = Some(weighted(&mut sub, &MAP.hills_cost) as f64),
            MOUNTAIN => {
                if let Some(span) = weighted(&mut sub, &MAP.mountain_span) {
                    t.span = Some(span);
                    t.trail_cost = Some(MAP.mountain_cost as f64);
                }
            }
            _ => {}
        }
    }
}

/// The map's character for a tile.
pub fn glyph(t: &Tile) -> char {
    match t.terrain {
        SEA if t.lake => '=',
        SEA => '~',
        COAST => ',',
        PLAINS => '.',
        FOREST => 'f',
        HILLS => 'h',
        MOUNTAIN => '^',
        DESERT => ':',
        MARSH => '%',
        _ => '?',
    }
}

/// The map as text, a character a tile, each row of the hex half a tile to the right of the one above.
pub fn ascii(tiles: &[Tile], radius: i32) -> String {
    let mut out = String::new();
    for r in -radius..=radius {
        let (lo, hi) = ((-radius).max(-r - radius), radius.min(-r + radius));
        out += &" ".repeat((2 * lo + r + 2 * radius) as usize);
        let row: Vec<String> =
            (lo..=hi).map(|q| glyph(&tiles[index(radius, Hex { q, r }).unwrap()]).to_string()).collect();
        out += &row.join(" ");
        out.push('\n');
    }
    out
}

/// The connected groups of land tiles, each sorted, the groups in order of their lowest tile.
pub fn landmasses(tiles: &[Tile], radius: i32) -> Vec<Vec<usize>> {
    let land = |i: usize| TERRAIN[tiles[i].terrain].land;
    let mut seen = vec![false; tiles.len()];
    let mut out = Vec::new();
    for start in 0..tiles.len() {
        if seen[start] || !land(start) {
            continue;
        }
        seen[start] = true;
        let (mut group, mut todo) = (vec![start], vec![start]);
        while let Some(i) = todo.pop() {
            for j in tiles[i].hex().neighbors().filter_map(|n| index(radius, n)) {
                if !seen[j] && land(j) {
                    seen[j] = true;
                    group.push(j);
                    todo.push(j);
                }
            }
        }
        group.sort_unstable();
        out.push(group);
    }
    out
}
