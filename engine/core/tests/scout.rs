//! The scout: a civ's route search over known and possible costs, its reveals and trails, and its
//! feeding. The maps are the real pre-game's, with their land painted into the case a test needs.

use engine::data::*;
use engine::hex::{Hex, DIRS};
use engine::map::index;
use engine::stats::digest;
use engine::units::{self, UnitKind};
use engine::world::{Action, EventKind, Phase};
use engine::{Config, World};

const RADIUS: i32 = 16;

/// The real map of `seed`, run to the pick, led by its first civ.
fn lead(seed: u32) -> World {
    let mut w = World::new(Config { seed, radius: RADIUS, army: true });
    while w.phase != Phase::Choose {
        w.act(Action::EndTurn).unwrap();
    }
    let first = w.civs.keys().next().copied();
    w.act(Action::ChooseCiv { civ: first }).unwrap();
    w
}

/// Every land tile plains and bare: no rivers, trails or hidden costs, and the player's civ has
/// revealed nothing. The capital and its camp and scout are left as they are.
fn plain(w: &mut World) {
    for t in &mut w.tiles {
        if t.terrain != SEA {
            t.terrain = PLAINS;
        }
        t.river = 0;
        t.flow = None;
        t.trail = false;
        t.trail_work = 0.0;
        t.trail_cost = None;
        t.span = None;
    }
    let me = w.player.unwrap();
    w.civs.get_mut(&me).unwrap().revealed.clear();
}

/// The `n` tiles from `from` straight out in the first direction that stays on land (none of them
/// sea), in order, as `from`'s own tile is not among them.
fn open_ray(w: &World, from: usize, n: i32) -> Vec<usize> {
    let h = w.tiles[from].hex();
    for d in DIRS {
        let tiles: Vec<usize> =
            (1..=n).filter_map(|k| index(w.config.radius, Hex { q: h.q + d.0 * k, r: h.r + d.1 * k })).collect();
        if tiles.len() == n as usize && tiles.iter().all(|&i| w.tiles[i].terrain != SEA) {
            return tiles;
        }
    }
    panic!("no open ray from {from}");
}

/// The player's capital, as its hex.
fn capital(w: &World) -> (usize, Hex) {
    let c = w.civs[&w.player.unwrap()].capital;
    (c, w.tiles[c].hex())
}

/// Sets tile `i` to `terrain` with its hidden trail cost and span.
fn paint(w: &mut World, i: usize, terrain: usize, trail_cost: Option<f64>, span: Option<u8>) {
    let t = &mut w.tiles[i];
    t.terrain = terrain;
    t.trail_cost = trail_cost;
    t.span = span;
}

fn scout(w: &World) -> &units::Unit {
    units::scout(w, w.player.unwrap()).expect("the player has a scout")
}

fn has_event(w: &World, kind: EventKind) -> bool {
    w.events.iter().any(|e| e.kind == kind)
}

/// Ends turns until the scout's route is cleared; returns how many turns it took.
fn until_route_done(w: &mut World, max: u32) -> u32 {
    for turn in 1..=max {
        w.act(Action::EndTurn).unwrap();
        if scout(w).route.is_none() {
            return turn;
        }
    }
    panic!("the route is still searched after {max} turns");
}

#[test]
fn the_scout_is_made_at_the_capital_with_its_packs_from_the_store() {
    let w = lead(2);
    let me = w.player.unwrap();
    let (cap, _) = capital(&w);
    let scouts: Vec<_> = w.units.values().filter(|u| u.kind == UnitKind::Scout).collect();
    assert_eq!(scouts.len(), w.civs.len(), "one scout per civ");
    let s = units::scout(&w, me).unwrap();
    assert_eq!((s.home, s.people, s.route), (cap, SCOUT.people as f64, None));
    assert!(units::camp(&w, me).is_some_and(|c| c.id != s.id), "the scout is not the camp");
}

#[test]
fn a_hidden_route_is_priced_by_what_the_civ_knows() {
    let mut w = lead(2);
    plain(&mut w);
    let me = w.player.unwrap();
    let (cap, _) = capital(&w);
    // Six steps out; the third is an unrevealed forest with a trail costing 1, the rest plains.
    let line = open_ray(&w, cap, 6);
    paint(&mut w, line[2], FOREST, Some(1.0), None);
    let to = line[5];
    let bare = TERRAIN[PLAINS].move_cost.unwrap();
    let forest = TERRAIN[FOREST].move_cost.unwrap();
    let plan = units::route_plan(&w, me, cap, to);
    // Known: the forest is bare; possible: it hides a trail at its cheapest hidden cost, 1.
    assert_eq!(plan.known.as_ref().unwrap().1, 5.0 * bare + forest);
    assert_eq!(plan.possible.as_ref().unwrap().1, 5.0 * bare + 1.0);
    assert_eq!(plan.candidates, vec![line[2]], "only the forest could change the answer");
    // Revealed, the same tile is its trail's cost, as known.
    w.civs.get_mut(&me).unwrap().revealed.insert(line[2]);
    let plan = units::route_plan(&w, me, cap, to);
    assert_eq!(plan.known.as_ref().unwrap().1, 5.0 * bare + 1.0);
    assert!(plan.candidates.is_empty());
}

#[test]
fn a_route_across_rough_country_is_searched_and_its_trails_built() {
    let mut w = lead(2);
    plain(&mut w);
    let (cap, _) = capital(&w);
    // Six tiles of forest and hills, each with a hidden trail.
    let line = open_ray(&w, cap, 6);
    for (k, &i) in line.iter().enumerate() {
        let terrain = if k % 2 == 0 { FOREST } else { HILLS };
        paint(&mut w, i, terrain, Some(1.0), None);
    }
    w.act(Action::SetRoute { from: cap, to: line[5] }).unwrap();
    let turns = until_route_done(&mut w, 20);
    assert!(turns <= 20, "a 6-tile route within 20 turns, took {turns}");
    let built: Vec<usize> = line.iter().copied().filter(|&i| w.tiles[i].trail).collect();
    assert_eq!(built.len(), 6, "every tile of the route has its trail when the route is found");
    // Six tiles at their hidden cost of 1 once every trail is built.
    assert!(w.events.iter().any(|e| e.kind == EventKind::ScoutRouteFound && e.cost == Some(6.0)));
    assert!(!has_event(&w, EventKind::ScoutHungry) && !has_event(&w, EventKind::ScoutStarved));
    assert_eq!(scout(&w).people, SCOUT.people as f64, "no one starved");
}

/// A ring of mountains at distance 3 around the capital, every one of span 2 but `pass`, which
/// spans 1: the capital is in a valley sealed by mountains, bridgeable at one place.
fn sealed_valley(w: &mut World, pass: Option<usize>) -> Vec<usize> {
    let (_, ch) = capital(w);
    let ring: Vec<usize> = (0..w.tiles.len()).filter(|&i| w.tiles[i].hex().distance(ch) == 3).collect();
    for &i in &ring {
        let span = if Some(i) == pass { 1 } else { 2 };
        paint(w, i, MOUNTAIN, Some(MAP.mountain_cost as f64), Some(span));
    }
    ring
}

#[test]
fn a_sealed_scout_searches_the_range_from_inside_the_valley() {
    let mut w = lead(2);
    plain(&mut w);
    let (cap, ch) = capital(&w);
    let line = open_ray(&w, cap, 6);
    let (pass, target) = (line[2], line[5]);
    sealed_valley(&mut w, Some(pass));
    w.act(Action::SetRoute { from: cap, to: target }).unwrap();
    // The scout reveals the range from where it stands, inside the ring, and only leaves it once the
    // pass is built and the walk out is open.
    for _ in 0..40 {
        w.act(Action::EndTurn).unwrap();
        let s = scout(&w);
        if !w.tiles[pass].trail {
            assert!(s.home == cap || w.tiles[s.home].hex().distance(ch) < 3, "it searched from outside the valley");
        }
        if s.route.is_none() {
            break;
        }
    }
    assert!(w.tiles[pass].trail, "the pass is built from inside");
}

#[test]
fn a_sealed_valley_gets_its_pass_built_and_its_route_found() {
    let mut w = lead(2);
    plain(&mut w);
    let (cap, _) = capital(&w);
    let line = open_ray(&w, cap, 6);
    let (pass, target) = (line[2], line[5]);
    sealed_valley(&mut w, Some(pass));
    w.act(Action::SetRoute { from: cap, to: target }).unwrap();
    let turns = until_route_done(&mut w, 40);
    assert!(turns <= 40, "a mountain pass within 40 turns, took {turns}");
    assert!(w.tiles[pass].trail, "the pass is built");
    assert!(has_event(&w, EventKind::ScoutPassFound), "the pass is found");
    assert!(has_event(&w, EventKind::ScoutRouteFound));
    assert!(!has_event(&w, EventKind::ScoutNoRoute));
    assert_eq!(scout(&w).people, SCOUT.people as f64, "no one starved");
}

#[test]
fn no_route_can_exist_through_a_wall_no_mountain_of_which_can_be_bridged() {
    let mut w = lead(2);
    plain(&mut w);
    let (cap, _) = capital(&w);
    let target = open_ray(&w, cap, 6)[5];
    sealed_valley(&mut w, None);
    w.act(Action::SetRoute { from: cap, to: target }).unwrap();
    until_route_done(&mut w, 60);
    assert!(has_event(&w, EventKind::ScoutNoRoute), "the scout says no route can exist");
    assert!(!w.tiles.iter().any(|t| t.trail && t.terrain == MOUNTAIN), "no pass is built");
}

#[test]
fn a_built_trail_cuts_the_camps_and_the_clans_walks() {
    let mut w = lead(2);
    plain(&mut w);
    let (cap, _) = capital(&w);
    let line = open_ray(&w, cap, 4);
    for &i in &line {
        paint(&mut w, i, FOREST, Some(1.0), None);
    }
    let to = line[3];
    let before = units::path_between(&w, cap, to).unwrap().1;
    for &i in &line {
        w.tiles[i].trail = true;
    }
    let after = units::path_between(&w, cap, to).unwrap().1;
    assert!(after < before, "a built trail makes the camp's walk cheaper: {after} vs {before}");
    assert_eq!(after, 4.0 * 1.0, "a trail brings each forest to its hidden cost");
}

#[test]
fn a_mountain_pass_is_crossed_by_the_camp_only_once_built() {
    let mut w = lead(2);
    plain(&mut w);
    let (cap, _) = capital(&w);
    let line = open_ray(&w, cap, 6);
    let (pass, target) = (line[2], line[5]);
    let ring = sealed_valley(&mut w, Some(pass));
    assert!(units::path_between(&w, cap, target).is_none(), "sealed");
    w.tiles[pass].trail = true;
    assert!(units::path_between(&w, cap, target).is_some(), "a built pass is walked");
    assert!(ring.iter().filter(|&&i| i != pass).all(|&i| !w.tiles[i].trail));
}

#[test]
fn an_idle_scout_at_the_capital_neither_forages_nor_starves() {
    let mut w = lead(2);
    let (cap, _) = capital(&w);
    w.run(50);
    let s = scout(&w);
    assert_eq!((s.home, s.route, s.hungry), (cap, None, 0));
    assert_eq!(s.people, SCOUT.people as f64);
    assert!(!has_event(&w, EventKind::ScoutHungry) && !has_event(&w, EventKind::ScoutStarved));
    assert!(w.events.iter().all(|e| e.kind != EventKind::CampForaged || e.unit != Some(s.id)));
}

#[test]
fn a_route_order_replays_to_the_same_world() {
    let mut w = lead(2);
    let (cap, _) = capital(&w);
    let target = open_ray(&w, cap, 6)[5];
    w.act(Action::SetRoute { from: cap, to: target }).unwrap();
    w.run(15);
    w.act(Action::ClearRoute).expect("the scout is still there");
    w.run(5);
    let again = World::replay(&w.save()).unwrap();
    assert_eq!(digest(&again), digest(&w));
    assert!(w.act(Action::SetRoute { from: cap, to: cap }).is_err(), "a route needs two tiles");
}
