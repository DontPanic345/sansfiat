//! The map: smoothed noise for height, moisture and fertility inside a hex, sea at the rim,
//! inlets, inland seas and lakes, ridges with hilly flanks, deserts in the rain shadow of the
//! ridges, coast beside water, rivers to the sea and lakes with floodplains and deltas at their
//! ends, food scaled by fertility in patches, and the hidden trail costs.

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
    pub tries: Tries,
}

/// `generate` with the hidden stream's salt given: a different salt changes only the hidden
/// trail costs.
pub fn generate_with(radius: i32, rng: &mut u32, salt: u32) -> Generated {
    let hexes = Hex::ORIGIN.within(radius);
    let geo = Geo {
        radius,
        nbrs: hexes.iter().map(|h| h.neighbors().filter_map(|x| index(radius, x)).collect()).collect(),
        edge: hexes.iter().map(|h| radius - h.distance(Hex::ORIGIN)).collect(),
        hexes,
    };
    let n = geo.hexes.len();
    let open = |n: usize| (vec![Zone::Open; n], vec![false; n]);
    let mut tries = Tries::default();
    let (mut tiles, wind) = loop {
        let base = base_land(&geo, rng);
        if radius < MAP.closed_from {
            let (zone, pass) = open(n);
            break (finish(&geo, rng, &base, base.terrain.clone(), &zone, &pass), base.wind);
        }
        // Closed country on a copy of the base land, until the open land holds three sites; not
        // tried on land that even all open likely can't.
        let mut kept = None;
        if likely_sites(&geo, &base, &base.terrain, &open(n).0) >= PREGAME.players {
            for _ in 0..MAP.closed_tries {
                tries.layouts += 1;
                let mut terrain = base.terrain.clone();
                let (zone, pass) = close_country(&geo, rng, &base, &mut terrain);
                let tiles = finish(&geo, rng, &base, terrain, &zone, &pass);
                if open_sites(&tiles, radius).len() >= PREGAME.players {
                    kept = Some(tiles);
                    break;
                }
            }
        }
        if let Some(tiles) = kept {
            break (tiles, base.wind);
        }
        tries.dropped += 1;
        let (zone, pass) = open(n);
        let tiles = finish(&geo, rng, &base, base.terrain.clone(), &zone, &pass);
        if open_sites(&tiles, radius).len() >= PREGAME.players || tries.maps == MAP.map_tries {
            break (tiles, base.wind);
        }
        tries.maps += 1;
    };
    let mut hidden = *rng ^ salt;
    roll_trails(&mut tiles, &mut hidden);
    // A sealed valley's wall can always be bridged somewhere: if no wall mountain rolled a span
    // of 1, one of them (picked on the hidden stream) gets it.
    for group in zone_groups(&tiles, radius, Zone::Sealed) {
        let walls: Vec<usize> = group.into_iter().filter(|&i| tiles[i].terrain == MOUNTAIN).collect();
        if !walls.is_empty() && walls.iter().all(|&i| tiles[i].span != Some(1)) {
            let i = walls[rand_index(&mut hidden, walls.len())];
            tiles[i].span = Some(1);
            tiles[i].trail_cost = Some(MAP.mountain_cost as f64);
        }
    }
    Generated { tiles, wind, tries }
}

/// How the open-land guarantee went for a map.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tries {
    /// Closed-country layouts tried, over all the maps tried (none below `MAP.closed_from`).
    pub layouts: usize,
    /// Maps whose closed country was dropped (every layout left the open land short, or the
    /// land was too short to try one).
    pub dropped: usize,
    /// Whole maps thrown away before the one kept: even all open, they couldn't hold the sites.
    pub maps: usize,
}

impl Tries {
    /// The map kept is the one whose closed country was dropped.
    pub fn all_open(&self) -> bool {
        self.dropped > self.maps
    }

    /// The first layout on the first map didn't do.
    pub fn rerolled(&self) -> bool {
        self.layouts > 1 || self.dropped > 0
    }
}

/// The hex's tiles as neighbour lists and steps from the rim.
struct Geo {
    radius: i32,
    hexes: Vec<Hex>,
    nbrs: Vec<Vec<usize>>,
    edge: Vec<i32>,
}

impl Geo {
    /// The tiles exactly `k` steps from tile `c`.
    fn ring(&self, c: usize, k: i32) -> Vec<usize> {
        let at = self.hexes[c];
        at.within(k).into_iter().filter(|h| h.distance(at) == k).filter_map(|h| index(self.radius, h)).collect()
    }

    /// The tiles within `k` steps of tile `c`.
    fn disk(&self, c: usize, k: i32) -> Vec<usize> {
        self.hexes[c].within(k).into_iter().filter_map(|h| index(self.radius, h)).collect()
    }

    fn steps(&self, a: usize, b: usize) -> i32 {
        self.hexes[a].distance(self.hexes[b])
    }
}

/// The land before closed country, coast and rivers, and the noise the rest is finished from.
struct Base {
    terrain: Vec<usize>,
    m: Vec<f64>,
    h: Vec<f64>,
    /// Fertility in patches, from the third noise field.
    fertility: Vec<f64>,
    wind: usize,
}

/// Noise, the water, ridges and deserts.
fn base_land(geo: &Geo, rng: &mut u32) -> Base {
    let (radius, hexes, nbrs, edge) = (geo.radius, &geo.hexes, &geo.nbrs, &geo.edge);
    let n = hexes.len();
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

    carve_water(radius, rng, hexes, edge, nbrs, &mut terrain);
    raise_ridges(radius, rng, hexes, edge, &mut terrain);
    let wind = make_deserts(radius, rng, hexes, &m, &mut terrain);
    let fertility = y.iter().map(|&v| round1(FERTILITY.0 + (FERTILITY.1 - FERTILITY.0) * libm::pow(v, 1.5))).collect();
    Base { terrain, m, h, fertility, wind }
}

/// The tiles from finished land: coast, lakes, fertility, rivers, floodplains and deltas, food.
fn finish(geo: &Geo, rng: &mut u32, base: &Base, mut terrain: Vec<usize>, zone: &[Zone], pass: &[bool]) -> Vec<Tile> {
    let (nbrs, edge) = (&geo.nbrs, &geo.edge);
    let n = terrain.len();
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

    let mut fertility = base.fertility.clone();
    let (river, flow) = carve_rivers(rng, nbrs, &base.h, &terrain, &lake);
    let flooded = flood(nbrs, &river, &flow, &lake, pass, &mut terrain, &mut fertility);

    let mut tiles: Vec<Tile> = geo
        .hexes
        .iter()
        .enumerate()
        .map(|(i, hx)| Tile {
            q: hx.q,
            r: hx.r,
            terrain: terrain[i],
            fertility: fertility[i],
            cap: [0.0; N],
            stock: [0.0; N],
            wild_grain: 0.0,
            tame: 0,
            claim: None,
            held: 0,
            unheld: 0,
            trail: false,
            river: river[i],
            flow: flow[i],
            lake: lake[i],
            flood: flooded[i],
            zone: zone[i],
            pass: pass[i],
            trail_cost: None,
            span: None,
        })
        .collect();
    finish_caps(&mut tiles);
    tiles
}

fn is_land(terrain: usize) -> bool {
    TERRAIN[terrain].land
}

/// The largest connected group of land (ties to the group with the lowest tile).
fn mainland(geo: &Geo, terrain: &[usize]) -> Vec<bool> {
    let n = terrain.len();
    let mut seen = vec![false; n];
    let mut best: Vec<usize> = Vec::new();
    for start in 0..n {
        if seen[start] || !is_land(terrain[start]) {
            continue;
        }
        seen[start] = true;
        let (mut group, mut todo) = (vec![start], vec![start]);
        while let Some(i) = todo.pop() {
            for &j in &geo.nbrs[i] {
                if !seen[j] && is_land(terrain[j]) {
                    seen[j] = true;
                    group.push(j);
                    todo.push(j);
                }
            }
        }
        if group.len() > best.len() {
            best = group;
        }
    }
    let mut out = vec![false; n];
    for i in best {
        out[i] = true;
    }
    out
}

/// After a feature is built on `footprint`: the land reached from what is left of the mainland
/// `main` (its tiles off the footprint), or `None` if that is no longer one piece.
fn still_joined(geo: &Geo, terrain: &[usize], main: &[bool], footprint: &[usize]) -> Option<Vec<bool>> {
    let n = terrain.len();
    let left: Vec<usize> = (0..n).filter(|&i| main[i] && !footprint.contains(&i)).collect();
    let mut seen = vec![false; n];
    let mut todo = vec![*left.first()?];
    seen[todo[0]] = true;
    while let Some(i) = todo.pop() {
        for &j in &geo.nbrs[i] {
            if !seen[j] && is_land(terrain[j]) {
                seen[j] = true;
                todo.push(j);
            }
        }
    }
    left.iter().all(|&i| seen[i]).then_some(seen)
}

/// Closed country on `terrain` (the base land, before coast and rivers): sealed valleys, valleys
/// behind a pass, and peninsulas on a neck, as many as fit up to their counts, each in a few
/// random tries. Gives each tile's zone and which tiles are passes.
fn close_country(geo: &Geo, rng: &mut u32, base: &Base, terrain: &mut [usize]) -> (Vec<Zone>, Vec<bool>) {
    let n = terrain.len();
    let mut built = Closed { zone: vec![Zone::Open; n], pass: vec![false; n], blocked: vec![false; n] };
    // Taken in turns, so each kind gets its share of the room.
    let mut left = [
        (Zone::Sealed, scaled(MAP.sealed, n).max(MAP.sealed_floor)),
        (Zone::Neck, scaled(MAP.necks, n).max(MAP.necks_floor)),
        (Zone::Valley, scaled(MAP.valleys, n).max(MAP.valleys_floor)),
    ];
    while left.iter().any(|k| k.1 > 0) {
        for (kind, count) in left.iter_mut().filter(|k| k.1 > 0) {
            *count -= 1;
            for _ in 0..MAP.closed_tries_each {
                let before = (terrain.to_vec(), built.clone());
                let ok = match kind {
                    Zone::Neck => neck(geo, rng, terrain, &mut built),
                    _ => valley(geo, rng, &base.m, terrain, &mut built, *kind == Zone::Sealed),
                };
                // Never where a centre has to be: a feature that leaves the open land short of
                // sites (as near as the land shows before coast and rivers) is taken down again.
                if ok && likely_sites(geo, base, terrain, &built.zone) < PREGAME.players {
                    terrain.copy_from_slice(&before.0);
                    built = before.1;
                } else if ok {
                    break;
                }
            }
        }
    }
    (built.zone, built.pass)
}

/// Closed country built so far, and the tiles too near it for another feature.
#[derive(Clone)]
struct Closed {
    zone: Vec<Zone>,
    pass: Vec<bool>,
    blocked: Vec<bool>,
}

impl Closed {
    /// Keeps the next feature `closed_clearance` steps from `footprint`.
    fn block(&mut self, geo: &Geo, footprint: &[usize]) {
        for &i in footprint {
            for j in geo.disk(i, MAP.closed_clearance - 1) {
                self.blocked[j] = true;
            }
        }
    }
}

/// A random land tile at least `rho + 1 + closed_clearance` steps from the rim whose tiles within
/// `rho + 1` are clear of other features and pass `fits`.
fn centre(
    geo: &Geo,
    rng: &mut u32,
    terrain: &[usize],
    built: &Closed,
    rho: i32,
    fits: impl Fn(usize) -> bool,
) -> Option<usize> {
    let spots: Vec<usize> = (0..terrain.len())
        .filter(|&i| {
            is_land(terrain[i])
                && geo.edge[i] >= rho + 1 + MAP.closed_clearance
                && geo.disk(i, rho + 1).iter().all(|&j| !built.blocked[j])
                && fits(i)
        })
        .collect();
    (!spots.is_empty()).then(|| spots[rand_index(rng, spots.len())])
}

/// Tries to ring a valley of radius 2 or 3 with mountain (water left as it is) at a random
/// centre: its sea and mountain become plains or forest; a valley gets one or two hills passes
/// on its wall that lead out to the mainland, a sealed valley none and food inside. Fails,
/// leaving `terrain` as it was, if the ring would cut the mainland in pieces.
fn valley(geo: &Geo, rng: &mut u32, m: &[f64], terrain: &mut [usize], built: &mut Closed, sealed: bool) -> bool {
    let rho = 2 + rand_index(rng, 2) as i32;
    let fits = |c: usize| {
        geo.disk(c, rho).iter().filter(|&&i| is_land(terrain[i])).count() >= MAP.valley_land
            && geo.ring(c, rho + 1).iter().any(|&i| terrain[i] != SEA)
    };
    let Some(c) = centre(geo, rng, terrain, built, rho, fits) else { return false };
    let (inner, wall) = (geo.disk(c, rho), geo.ring(c, rho + 1));
    let main = mainland(geo, terrain);
    let before = terrain.to_vec();
    let undo = |terrain: &mut [usize]| terrain.copy_from_slice(&before);
    for &i in &inner {
        if !is_land(terrain[i]) {
            terrain[i] = if m[i] > 0.6 { FOREST } else { PLAINS };
        }
    }
    for &i in &wall {
        if terrain[i] != SEA {
            terrain[i] = MOUNTAIN;
        }
    }
    let fed = inner.iter().filter(|&&i| geo.steps(c, i) < rho && matches!(terrain[i], PLAINS | FOREST)).count();
    let mut passes = Vec::new();
    if sealed {
        if fed < 2 {
            undo(terrain);
            return false;
        }
    } else {
        let mut gates: Vec<usize> = wall
            .iter()
            .copied()
            .filter(|&i| {
                terrain[i] == MOUNTAIN
                    && geo.nbrs[i].iter().any(|&j| geo.steps(c, j) == rho + 2 && is_land(terrain[j]) && main[j])
            })
            .collect();
        let want = 1 + rand_index(rng, 2);
        while passes.len() < want && !gates.is_empty() {
            passes.push(gates.swap_remove(rand_index(rng, gates.len())));
        }
        for &p in &passes {
            terrain[p] = HILLS;
        }
    }
    let footprint: Vec<usize> = inner.iter().chain(&wall).copied().collect();
    let reached = still_joined(geo, terrain, &main, &footprint);
    // A valley is entered from the mainland by its passes; a sealed one isn't entered at all.
    if reached.is_none_or(|seen| seen[c] == sealed) {
        undo(terrain);
        return false;
    }
    let kind = if sealed { Zone::Sealed } else { Zone::Valley };
    for &i in footprint.iter().filter(|&&i| terrain[i] != SEA) {
        built.zone[i] = kind;
    }
    for p in passes {
        built.pass[p] = true;
    }
    built.block(geo, &footprint);
    true
}

/// Tries to make a peninsula of the land within 2 or 3 of a random centre beside the water
/// (`neck_land` tiles of it: the ring round them must touch the sea, a lake or an inland sea) by cutting sea in the ring round it, leaving one or two tiles of the
/// ring that lead out to the mainland as its neck. Fails, leaving `terrain` as it was, if the cut
/// would cut the mainland in pieces or the blob's land in two.
fn neck(geo: &Geo, rng: &mut u32, terrain: &mut [usize], built: &mut Closed) -> bool {
    let n = terrain.len();
    let rho = 2;
    let fits = |c: usize| {
        let land = geo.disk(c, rho).iter().filter(|&&i| is_land(terrain[i])).count();
        (MAP.neck_land.0..=MAP.neck_land.1).contains(&land) && geo.ring(c, rho + 1).iter().any(|&i| terrain[i] == SEA)
    };
    let Some(c) = centre(geo, rng, terrain, built, rho, fits) else { return false };
    let (inner, wall) = (geo.disk(c, rho), geo.ring(c, rho + 1));
    let main = mainland(geo, terrain);
    // A neck tile joins land inside to the mainland outside.
    let joins = |i: usize| {
        is_land(terrain[i])
            && geo.nbrs[i].iter().any(|&j| geo.steps(c, j) == rho && is_land(terrain[j]))
            && geo.nbrs[i].iter().any(|&j| geo.steps(c, j) == rho + 2 && is_land(terrain[j]) && main[j])
    };
    let gates: Vec<usize> = wall.iter().copied().filter(|&i| joins(i)).collect();
    if gates.is_empty() {
        return false;
    }
    let first = gates[rand_index(rng, gates.len())];
    let mut necks = vec![first];
    if rand(rng) < 0.5 {
        let beside: Vec<usize> = wall
            .iter()
            .copied()
            .filter(|&i| {
                geo.steps(first, i) == 1
                    && is_land(terrain[i])
                    && geo.nbrs[i].iter().any(|&j| geo.steps(c, j) == rho && is_land(terrain[j]))
            })
            .collect();
        if !beside.is_empty() {
            necks.push(beside[rand_index(rng, beside.len())]);
        }
    }
    // The cut joins the water it touches into one; the map must keep a lake.
    let cut: Vec<usize> = wall.iter().copied().filter(|i| !necks.contains(i)).collect();
    let body = water_bodies(geo, terrain);
    let mut touched: Vec<usize> = cut
        .iter()
        .flat_map(|&i| std::iter::once(i).chain(geo.nbrs[i].iter().copied()))
        .filter_map(|j| body[j])
        .collect();
    touched.sort_unstable();
    touched.dedup();
    let mut oceans: Vec<usize> = (0..n).filter(|&i| geo.edge[i] == 0).filter_map(|i| body[i]).collect();
    oceans.sort_unstable();
    oceans.dedup();
    let lakes = body.iter().flatten().max().map_or(0, |&b| b + 1) - oceans.len();
    let lakes_touched = touched.iter().filter(|b| !oceans.contains(b)).count();
    let lost = if touched.len() > lakes_touched { lakes_touched } else { lakes_touched.saturating_sub(1) };
    if lakes > 0 && lost >= lakes {
        return false;
    }
    let before = terrain.to_vec();
    for &i in &cut {
        terrain[i] = SEA;
    }
    // The blob's land is one piece without its neck, and the mainland is still one piece with them.
    let inside: Vec<usize> = inner.iter().copied().filter(|&i| is_land(terrain[i])).collect();
    let mut seen = vec![inside[0]];
    let mut todo = vec![inside[0]];
    while let Some(i) = todo.pop() {
        for &j in &geo.nbrs[i] {
            if inside.contains(&j) && !seen.contains(&j) {
                seen.push(j);
                todo.push(j);
            }
        }
    }
    let blob: Vec<usize> = inside.iter().chain(&necks).copied().collect();
    let footprint: Vec<usize> = inner.iter().chain(&wall).copied().collect();
    let reached = still_joined(geo, terrain, &main, &footprint);
    if seen.len() < inside.len() || reached.is_none_or(|r| !blob.iter().all(|&i| r[i])) {
        terrain.copy_from_slice(&before);
        return false;
    }
    for &i in &blob {
        built.zone[i] = Zone::Neck;
    }
    built.block(geo, &footprint);
    true
}

/// Which connected body of sea each tile is in (`None` off the sea), numbered from the lowest tile.
fn water_bodies(geo: &Geo, terrain: &[usize]) -> Vec<Option<usize>> {
    let n = terrain.len();
    let mut body = vec![None; n];
    let mut count = 0;
    for start in 0..n {
        if body[start].is_some() || terrain[start] != SEA {
            continue;
        }
        body[start] = Some(count);
        let mut todo = vec![start];
        while let Some(i) = todo.pop() {
            for &j in &geo.nbrs[i] {
                if body[j].is_none() && terrain[j] == SEA {
                    body[j] = Some(count);
                    todo.push(j);
                }
            }
        }
        count += 1;
    }
    body
}

/// Up to `PREGAME.players` sites for a centre in the open land: tiles in the largest connected
/// group of passable land in zone `Open`, each with `site_tiles` tiles of that group with grain
/// or fruit within `site_steps` steps, of mean fertility `site_fertility` or more, taken best
/// first (most such tiles, then the lowest index), each `PREGAME.centre_spacing` steps from those
/// taken before it.
pub fn open_sites(tiles: &[Tile], radius: i32) -> Vec<usize> {
    pick_sites(
        radius,
        tiles.len(),
        |i| tiles[i].zone == Zone::Open && is_land(tiles[i].terrain),
        |i| tiles[i].cap[GRAIN] > 0.0 || tiles[i].cap[FRUIT] > 0.0,
        |i| tiles[i].fertility,
    )
}

/// How many sites `open_sites` will likely find once coast, rivers and food are added to
/// `terrain`: food is guessed from the terrain (plains and forest away from the water, hills).
fn likely_sites(geo: &Geo, base: &Base, terrain: &[usize], zone: &[Zone]) -> usize {
    let food = |i: usize| match terrain[i] {
        HILLS | MARSH => true,
        PLAINS | FOREST => geo.nbrs[i].iter().all(|&j| terrain[j] != SEA),
        _ => false,
    };
    pick_sites(geo.radius, terrain.len(), |i| zone[i] == Zone::Open && is_land(terrain[i]), food, |i| base.fertility[i])
        .len()
}

/// `open_sites` on any land: `open` tiles, which have `food` and `fertility`.
fn pick_sites(
    radius: i32,
    n: usize,
    open: impl Fn(usize) -> bool,
    food: impl Fn(usize) -> bool,
    fertility: impl Fn(usize) -> f64,
) -> Vec<usize> {
    let hexes = Hex::ORIGIN.within(radius);
    let nbrs = |i: usize| hexes[i].neighbors().filter_map(move |h| index(radius, h));
    let mut seen = vec![false; n];
    let mut best: Vec<usize> = Vec::new();
    for start in 0..n {
        if seen[start] || !open(start) {
            continue;
        }
        seen[start] = true;
        let (mut group, mut todo) = (vec![start], vec![start]);
        while let Some(i) = todo.pop() {
            for j in nbrs(i) {
                if !seen[j] && open(j) {
                    seen[j] = true;
                    group.push(j);
                    todo.push(j);
                }
            }
        }
        if group.len() > best.len() {
            best = group;
        }
    }
    let mut region = vec![false; n];
    for &i in &best {
        region[i] = true;
    }
    let mut sites: Vec<(usize, usize)> = best
        .iter()
        .filter_map(|&i| {
            let near: Vec<usize> = hexes[i]
                .within(MAP.site_steps)
                .into_iter()
                .filter_map(|h| index(radius, h))
                .filter(|&j| region[j] && food(j))
                .collect();
            let mean = near.iter().map(|&j| fertility(j)).sum::<f64>() / near.len().max(1) as f64;
            (near.len() >= MAP.site_tiles && mean >= MAP.site_fertility).then_some((near.len(), i))
        })
        .collect();
    sites.sort_by_key(|&(k, i)| (std::cmp::Reverse(k), i));
    let mut taken: Vec<usize> = Vec::new();
    for (_, i) in sites {
        if taken.len() == PREGAME.players {
            break;
        }
        if taken.iter().all(|&t| hexes[t].distance(hexes[i]) >= PREGAME.centre_spacing) {
            taken.push(i);
        }
    }
    taken
}

/// The connected groups of tiles in `zone`, each sorted, the groups in order of their lowest tile.
pub fn zone_groups(tiles: &[Tile], radius: i32, zone: Zone) -> Vec<Vec<usize>> {
    let mut seen = vec![false; tiles.len()];
    let mut out = Vec::new();
    for start in 0..tiles.len() {
        if seen[start] || tiles[start].zone != zone {
            continue;
        }
        seen[start] = true;
        let (mut group, mut todo) = (vec![start], vec![start]);
        while let Some(i) = todo.pop() {
            for j in tiles[i].hex().neighbors().filter_map(|n| index(radius, n)) {
                if !seen[j] && tiles[j].zone == zone {
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

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

/// Every tile's food caps from its terrain, its river and its fertility, and its stock full.
/// Floodplains and deltas set the tile's terrain and fertility before this runs.
pub fn finish_caps(tiles: &mut [Tile]) {
    for t in tiles {
        let river = if t.river > 0 { FEATURES.river } else { [0.0; N] };
        t.cap = std::array::from_fn(|f| round1((TERRAIN[t.terrain].food[f] + river[f]) * t.fertility));
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

/// Steps from each tile to the nearest `water` tile over land a river may run on (any land:
/// mountains aren't); water is 0, and `None` is land walled off from all of it.
fn water_distance(nbrs: &[Vec<usize>], terrain: &[usize], water: impl Fn(usize) -> bool) -> Vec<Option<u32>> {
    let n = terrain.len();
    let mut dist = vec![None; n];
    let mut todo = std::collections::VecDeque::new();
    for i in (0..n).filter(|&i| water(i)) {
        dist[i] = Some(0);
        todo.push_back(i);
    }
    while let Some(i) = todo.pop_front() {
        let d = dist[i].unwrap() + 1;
        for &j in &nbrs[i] {
            if dist[j].is_none() && TERRAIN[terrain[j]].land {
                dist[j] = Some(d);
                todo.push_back(j);
            }
        }
    }
    dist
}

/// Runs a river from `source` down `dist`: each step goes to an unvisited neighbour nearer the
/// water, or (with chance `river_wiggle`, if there is one) sidesteps to one as near; ties to a
/// river it can join, then the lower noisy height `h`, then the lower index. It ends on reaching
/// water or another river.
/// A neighbour nearer the water is never on the path, so the river never fails.
fn run_river(
    rng: &mut u32,
    nbrs: &[Vec<usize>],
    h: &[f64],
    dist: &[Option<u32>],
    terrain: &[usize],
    flow: &mut [Option<usize>],
    source: usize,
) {
    let mut path = vec![source];
    let mut here = source;
    loop {
        let d = dist[here].expect("a river runs on land that reaches water");
        let best = |want: u32| {
            nbrs[here].iter().copied().filter(|&j| dist[j] == Some(want) && !path.contains(&j)).min_by(|&a, &b| {
                let joins = |i: usize| flow[i].is_none();
                joins(a).cmp(&joins(b)).then(h[a].total_cmp(&h[b])).then(a.cmp(&b))
            })
        };
        let next = match (best(d - 1), best(d)) {
            (Some(lower), Some(side)) => {
                if rand(rng) < MAP.river_wiggle {
                    side
                } else {
                    lower
                }
            }
            (lower, _) => lower.expect("a tile nearer the water"),
        };
        flow[here] = Some(next);
        if terrain[next] == SEA || flow[next].is_some() {
            return;
        }
        path.push(next);
        here = next;
    }
}

/// Rivers from high ground (hills, or beside a mountain) far enough from water down to the sea
/// or a lake, joining where they meet; then a lake no river feeds may drain to the ocean. Gives
/// each tile's upstream river tiles (0 off the rivers) and the tile it flows to.
fn carve_rivers(
    rng: &mut u32,
    nbrs: &[Vec<usize>],
    h: &[f64],
    terrain: &[usize],
    lake: &[bool],
) -> (Vec<u32>, Vec<Option<usize>>) {
    let n = terrain.len();
    let mut flow: Vec<Option<usize>> = vec![None; n];
    let dist = water_distance(nbrs, terrain, |i| terrain[i] == SEA);
    // On land too narrow for `river_min`, sources come from as deep as it goes less a step.
    let deepest = dist.iter().flatten().copied().max().unwrap_or(0);
    let least = MAP.river_min.min(deepest.saturating_sub(1)).max(1);
    let mut sources: Vec<usize> = (0..n)
        .filter(|&i| {
            let high = terrain[i] == HILLS || h[i] >= MAP.upland || nbrs[i].iter().any(|&j| terrain[j] == MOUNTAIN);
            high && TERRAIN[terrain[i]].land && dist[i].is_some_and(|d| d >= least)
        })
        .collect();
    for _ in 0..scaled(MAP.rivers, n).max(MAP.rivers_floor) {
        if sources.is_empty() {
            break;
        }
        let source = sources.swap_remove(rand_index(rng, sources.len()));
        // A source on or beside a river would only run alongside it.
        if flow[source].is_none() && nbrs[source].iter().all(|&j| flow[j].is_none()) {
            run_river(rng, nbrs, h, &dist, terrain, &mut flow, source);
        }
    }

    // Each lake (or inland sea) no river runs into may, at even odds, drain to the ocean from
    // the shore tile nearest it.
    let ocean = water_distance(nbrs, terrain, |i| terrain[i] == SEA && !lake[i]);
    let mut seen = vec![false; n];
    for start in 0..n {
        if !lake[start] || seen[start] {
            continue;
        }
        seen[start] = true;
        let (mut body, mut todo) = (vec![start], vec![start]);
        while let Some(i) = todo.pop() {
            for &j in &nbrs[i] {
                if lake[j] && !seen[j] {
                    seen[j] = true;
                    body.push(j);
                    todo.push(j);
                }
            }
        }
        if flow.iter().any(|f| f.is_some_and(|j| lake[j] && body.contains(&j))) || rand(rng) >= 0.5 {
            continue;
        }
        let shore = body
            .iter()
            .flat_map(|&i| nbrs[i].iter().copied())
            .filter(|&j| TERRAIN[terrain[j]].land && flow[j].is_none() && ocean[j].is_some())
            .min_by(|&a, &b| ocean[a].cmp(&ocean[b]).then(h[a].total_cmp(&h[b])).then(a.cmp(&b)));
        if let Some(source) = shore {
            run_river(rng, nbrs, h, &ocean, terrain, &mut flow, source);
        }
    }

    // Upstream counts: the rivers are trees pointing to the water, so count from the sources down.
    let mut feeders = vec![0usize; n];
    for &j in flow.iter().flatten() {
        feeders[j] += 1;
    }
    let mut river: Vec<u32> = (0..n).map(|i| flow[i].is_some() as u32).collect();
    let mut todo: Vec<usize> = (0..n).filter(|&i| flow[i].is_some() && feeders[i] == 0).collect();
    while let Some(i) = todo.pop() {
        let Some(j) = flow[i].filter(|&j| terrain[j] != SEA) else { continue };
        river[j] += river[i];
        feeders[j] -= 1;
        if feeders[j] == 0 {
            todo.push(j);
        }
    }
    (river, flow)
}

/// Floodplains and deltas. A delta is the mouth of a river of `delta_river` tiles on the ocean
/// and its land neighbours touching the ocean: marsh. A floodplain is the last `flood_reach`
/// steps of a river of `big_river` tiles to its water (sea or lake) and the land beside them,
/// marsh aside: coast and desert there become plains. Both get `flood_fertility`. A pass stays
/// hills. Gives which tiles flood.
fn flood(
    nbrs: &[Vec<usize>],
    river: &[u32],
    flow: &[Option<usize>],
    lake: &[bool],
    pass: &[bool],
    terrain: &mut [usize],
    fertility: &mut [f64],
) -> Vec<bool> {
    let n = terrain.len();
    let mut flooded = vec![false; n];
    let ocean = |terrain: &[usize], j: usize| terrain[j] == SEA && !lake[j];
    for i in 0..n {
        if river[i] < MAP.delta_river || !flow[i].is_some_and(|j| ocean(terrain, j)) {
            continue;
        }
        let delta: Vec<usize> = std::iter::once(i)
            .chain(
                nbrs[i]
                    .iter()
                    .copied()
                    .filter(|&j| TERRAIN[terrain[j]].land && nbrs[j].iter().any(|&k| ocean(terrain, k))),
            )
            .collect();
        for j in delta.into_iter().filter(|&j| !pass[j]) {
            terrain[j] = MARSH;
            flooded[j] = true;
            fertility[j] = MAP.flood_fertility;
        }
    }
    // Steps to the mouth, following the flow until the next tile is water.
    let to_mouth = |mut i: usize| {
        let mut steps = 0;
        while let Some(j) = flow[i].filter(|&j| terrain[j] != SEA) {
            i = j;
            steps += 1;
        }
        steps
    };
    let banks: Vec<usize> = (0..n)
        .filter(|&i| river[i] >= MAP.big_river && to_mouth(i) <= MAP.flood_reach)
        .flat_map(|i| std::iter::once(i).chain(nbrs[i].iter().copied()))
        .collect();
    for j in banks {
        if !TERRAIN[terrain[j]].land || terrain[j] == MARSH {
            continue;
        }
        if matches!(terrain[j], COAST | DESERT) {
            terrain[j] = PLAINS;
        }
        flooded[j] = true;
        fertility[j] = MAP.flood_fertility;
    }
    flooded
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
fn roll_trails(tiles: &mut [Tile], sub: &mut u32) {
    for t in tiles {
        match t.terrain {
            FOREST => t.trail_cost = Some(weighted(sub, &MAP.forest_cost) as f64),
            HILLS => t.trail_cost = Some(weighted(sub, &MAP.hills_cost) as f64),
            MOUNTAIN => {
                if let Some(span) = weighted(sub, &MAP.mountain_span) {
                    t.span = Some(span);
                    t.trail_cost = Some(MAP.mountain_cost as f64);
                }
            }
            _ => {}
        }
    }
}

/// The map's character for a tile. Closed country shows as its zone (walls as their terrain).
pub fn glyph(t: &Tile) -> char {
    match t.terrain {
        _ if t.pass => 'P',
        _ if TERRAIN[t.terrain].land && t.zone != Zone::Open => match t.zone {
            Zone::Valley => 'V',
            Zone::Sealed => 'S',
            _ => 'N',
        },
        SEA if t.lake => '=',
        SEA => '~',
        MARSH => '%',
        _ if t.river > 0 => 'r',
        _ if t.flood => 'z',
        COAST => ',',
        PLAINS => '.',
        FOREST => 'f',
        HILLS => 'h',
        MOUNTAIN => '^',
        DESERT => ':',
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
