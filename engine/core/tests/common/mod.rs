//! Hand-built worlds for the clan tests.
#![allow(dead_code)]

use engine::actors::connected;
use engine::clans::new_clan;
use engine::data::*;
use engine::hex::{Hex, DIRS};
use engine::map::{generate, index, Tile};
use engine::stage::{found_civs, open_market, Stage};
use engine::units::{self, Unit};
use engine::{Action, EventKind, Order, World};

pub const R: i32 = 6;

/// The order the player gives the civ it leads, as `act` takes it.
pub fn led_order(w: &World, order: Order) -> Action {
    Action::Order { civ: w.player.expect("a civ is led"), order }
}

/// The unit id of the camp of the civ the player leads.
pub fn led_camp(w: &World) -> u32 {
    units::camp(w, w.player.expect("a civ is led")).expect("a camp").id
}

/// The unit id of the scout of the civ the player leads.
pub fn led_scout(w: &World) -> u32 {
    units::scout(w, w.player.expect("a civ is led")).expect("a scout").id
}

/// A world of bare plains with sea at the rim: no food, no clans, no markets. Its random state
/// and next id are what the placeholder generator left, so every test's draws and ids hold; no
/// river, lake, zone or hidden cost is left on a tile.
pub fn flat() -> World {
    flat_radius(R)
}

/// The radius of `big()`: room for several centres 10 steps apart.
pub const BIG: i32 = 12;

/// As `flat`, with room: the same bare plains, radius `BIG`. Use `at_big` for its tiles.
pub fn big() -> World {
    flat_radius(BIG)
}

/// The tile at `(q, r)` in a `big()` world.
pub fn at_big(q: i32, r: i32) -> usize {
    index(BIG, Hex { q, r }).unwrap()
}

fn flat_radius(radius: i32) -> World {
    Stage::flat(radius).into_world()
}

/// As `flat`, but only `land` is land.
pub fn island(land: &[usize]) -> World {
    let mut w = flat();
    for i in 0..w.tiles.len() {
        if !land.contains(&i) {
            w.tiles[i].terrain = SEA;
        }
    }
    w
}

/// The tile at `(q, r)`.
pub fn at(q: i32, r: i32) -> usize {
    index(R, Hex { q, r }).unwrap()
}

/// The middle tile's neighbour `k` steps along `DIRS[d]`.
pub fn out(d: usize, k: i32) -> usize {
    at(DIRS[d].0 * k, DIRS[d].1 * k)
}

pub fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// Puts a clan of `pop` at `tiles[0]` holding `tiles`, and claims them.
pub fn put(w: &mut World, tiles: &[usize], pop: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let mut c = new_clan(id, tiles[0], pop);
    let mut territory = tiles.to_vec();
    territory.sort_unstable();
    c.territory = territory;
    for &t in tiles {
        w.tiles[t].claim = Some(id);
    }
    w.clans.insert(id, c);
    id
}

/// Checks every territory: sorted, holding home, connected, within the radius, and every tile's
/// claim the one territory holding it.
pub fn check_territories(w: &World) {
    let mut owner = vec![None; w.tiles.len()];
    for c in w.clans.values() {
        assert!(c.territory.windows(2).all(|p| p[0] < p[1]), "clan {}: {:?} not sorted", c.id, c.territory);
        assert!(c.territory.contains(&c.home), "clan {}: home not held", c.id);
        assert!(connected(w, c.home, &c.territory), "clan {}: {:?} not connected", c.id, c.territory);
        for &t in &c.territory {
            assert_eq!(owner[t], None, "tile {t} in two territories",);
            owner[t] = Some(c.id);
            assert!(steps(w, c.home, t) <= CLAN.territory_radius, "clan {}: tile {t} too far from home", c.id);
        }
    }
    for (t, tile) in w.tiles.iter().enumerate() {
        assert_eq!(tile.claim, owner[t], "tile {t}'s claim");
    }
}

/// Opens a market on `tile` as `form` would, founded on turn 0.
pub fn open(w: &mut World, tile: usize) -> u32 {
    open_market(w, tile)
}

/// Makes `tile` a hearth with `wild` grain.
pub fn hearth(w: &mut World, tile: usize, wild: f64) {
    let t = &mut w.tiles[tile];
    t.wild_grain = wild;
    t.tame = TENDING.tame_turns;
    t.unworn[GRAIN] = wild * TENDING.domestic;
    t.cap[GRAIN] = t.unworn[GRAIN];
}

/// Markets for three civs on the big world, 10 or more steps apart.
pub fn markets3(w: &mut World) -> [u32; 3] {
    [open(w, at_big(-5, 0)), open(w, at_big(5, 0)), open(w, at_big(0, -10))]
}

/// Refills every civ's store: founding's scouts take their packs and feeding from it.
pub fn fill_stores(w: &mut World) {
    for c in w.civs.values_mut() {
        c.store = CIV.endowment;
    }
}

/// Ends the pre-game on `centres` by hand and founds the civs.
pub fn found(w: &mut World, centres: &[u32]) {
    found_civs(w, centres);
}

/// Raises the army a civ starts without (`founding.md`, The start), as its own `Raise`s
/// would: `UNIT.first` warriors at the capital with their pack from the store. The store first banks the warriors' packs (what a player banks before it raises: the
/// endowment's grain less the scout's pack can't fill them). For tests whose subject is not the start.
pub fn armed(w: &mut World, civ: u32) {
    let pack = units::pack_cap(f64::from(UNIT.first));
    w.civs.get_mut(&civ).unwrap().store[1] += pack;
    // One warrior at a time: `raise` takes at most one person from each clan, and a civ may have one.
    for _ in 0..UNIT.first {
        let into = units::camp(w, civ).map(|u| u.id);
        units::raise(w, civ, 1, Fighter::Warriors, into).expect("an army from the clans");
    }
}

/// Raises the army of every civ in the world (`armed`), as each civ's own Raises would.
pub fn armed_all(w: &mut World) {
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    for id in ids {
        armed(w, id);
    }
}

/// A big world of three civs, each with a clan of 14 on its market's tile (its scout takes its people from it,
/// and its army the rest), each with its army as `armed` raises it. Returns the
/// world, the civs and their clans.
pub fn three() -> (World, [u32; 3], [u32; 3]) {
    let mut stage = Stage::flat(BIG);
    let ids = stage.civs(&[at_big(-5, 0), at_big(5, 0), at_big(0, -10)], 14.0);
    let mut w = stage.into_world();
    for &id in &ids {
        armed(&mut w, id);
    }
    let clans = [at_big(-5, 0), at_big(5, 0), at_big(0, -10)].map(|t| w.tiles[t].claim.expect("a clan"));
    (w, [ids[0], ids[1], ids[2]], clans)
}

/// As `three`, led by the first civ.
pub fn led() -> (World, [u32; 3], [u32; 3]) {
    let (mut w, ids, clans) = three();
    w.act(Action::ChooseCiv { civ: Some(ids[0]) }).unwrap();
    w.events.clear();
    // The scouts' packs are full, as a scout refills its pack to full from the store.
    for u in w.units.values_mut().filter(|u| u.kind() == units::UnitKind::Scout) {
        u.actor.store = [units::pack_cap(u.people) / N as f64; N];
    }
    (w, ids, clans)
}

/// The unit id of `civ`'s camp.
pub fn camp_id(w: &World, civ: u32) -> u32 {
    units::camp(w, civ).expect("a camp").id
}

/// Moves `civ`'s camp onto `tile`.
pub fn put_camp(w: &mut World, civ: u32, tile: usize) -> u32 {
    let id = camp_id(w, civ);
    let u = w.units.get_mut(&id).unwrap();
    u.actor.home = tile;
    u.actor.territory = vec![tile];
    u.camp_mut().unwrap().held = vec![tile];
    id
}

pub fn camp_mut(w: &mut World, id: u32) -> &mut Unit {
    w.units.get_mut(&id).unwrap()
}

/// The sum of a basket's goods.
pub fn total(b: &[f64; N]) -> f64 {
    b.iter().sum()
}

/// How many events of `kind` the world holds.
pub fn kinds(w: &World, kind: EventKind) -> usize {
    w.events.iter().filter(|e| e.kind == kind).count()
}

pub fn end_turn(w: &mut World) {
    w.act(Action::EndTurn).unwrap();
}

/// The tiles of a generated map of `radius` for `seed`.
pub fn made(radius: i32, seed: u32) -> Vec<Tile> {
    let mut rng = seed;
    generate(radius, &mut rng)
}

/// The indices of the tiles next to tile `i` on a map of `radius`.
pub fn nbrs(tiles: &[Tile], radius: i32, i: usize) -> Vec<usize> {
    tiles[i].hex().neighbors().filter_map(|h| index(radius, h)).collect()
}

/// Makes `tile` a centre: a hearth, a clan of `people` on it, and its market with the pre-game's
/// traders. Returns the clan and the market.
pub fn centre(w: &mut World, tile: usize, people: f64) -> (u32, u32) {
    hearth(w, tile, 4.0);
    let c = put(w, &[tile], people);
    let m = open(w, tile);
    w.markets.get_mut(&m).unwrap().traders = PREGAME.centre_clans;
    let clan = w.clans.get_mut(&c).unwrap();
    clan.market = Some(m);
    clan.since = PREGAME.centre_age;
    (c, m)
}

/// The keys of the top-level object in `json`, in the order they are written (`serde_json::Value`
/// sorts them, so the text is read).
pub fn keys_in_order(json: &str) -> Vec<String> {
    let (mut keys, mut depth, mut in_str, mut escaped) = (Vec::new(), 0, false, false);
    let (mut word, mut last) = (String::new(), None::<String>);
    for c in json.chars() {
        if in_str {
            if escaped {
                escaped = false;
                word.push(c);
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_str = false;
                last = Some(std::mem::take(&mut word));
            } else {
                word.push(c);
            }
            continue;
        }
        match c {
            '"' => in_str = true,
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            ':' if depth == 1 => keys.push(last.take().expect("a key")),
            _ => {}
        }
    }
    keys
}

/// Gives `civ` the land `tiles` (its capital is added): its claim.
pub fn hold_land(w: &mut World, civ: u32, tiles: &[usize]) {
    let claim = engine::claim::check(w, civ, tiles).expect("a claim the civ may hold");
    engine::claim::set(w, civ, claim);
}

/// Removes `civ`'s scout, for a test that counts the civ's store to the last food: a scout refills
/// its pack from it.
pub fn no_scout(w: &mut World, civ: u32) {
    if let Some(id) = units::scout(w, civ).map(|u| u.id) {
        w.units.remove(&id);
    }
}
