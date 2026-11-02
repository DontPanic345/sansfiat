//! Depletion, trails from walking and taming with no ceiling, on hand-built worlds.
mod common;

use common::{at, flat, put};
use engine::actors::Route;
use engine::clans::harvest;
use engine::data::*;
use engine::land::{build_trail, domestic, tend, wear};
use engine::stats::summary;
use engine::{EventKind, World};

const EPS: f64 = 1e-12;

/// Gives `tile`'s food `f` an unworn cap of `cap`, and fills its stock.
fn full(w: &mut World, tile: usize, f: usize, cap: f64) {
    let t = &mut w.tiles[tile];
    t.unworn[f] = cap;
    t.cap[f] = cap;
    t.stock[f] = cap;
}

/// People walking the tiles in `walked` each turn in the tests that don't run a clan.
const WALKERS: f64 = 14.0;

/// One turn of walking: `walked` is the tiles `WALKERS` people walk this turn, the rest are not.
fn walk(w: &mut World, walked: &[usize]) {
    for t in &mut w.tiles {
        t.traffic *= TRAIL.keep;
        t.used = false;
    }
    for &i in walked {
        w.tiles[i].traffic += WALKERS;
    }
    tend(w);
}

#[test]
fn a_food_taken_past_its_yield_wears_its_cap_down_never_past_max() {
    let mut w = flat();
    let t = at(0, 0);
    // Fruit regrows a fifth of its cap a turn: a yield of 2 on a cap of 10. A take of 4 is 2 over.
    full(&mut w, t, FRUIT, 10.0);
    w.tiles[t].take[FRUIT] = 4.0;
    wear(&mut w);
    let tile = &w.tiles[t];
    assert!((tile.wear[FRUIT] - DEPLETION.rate * 2.0 / 10.0).abs() < EPS);
    assert!((tile.cap[FRUIT] - 10.0 * (1.0 - tile.wear[FRUIT])).abs() < EPS);
    assert!(tile.stock[FRUIT] <= tile.cap[FRUIT] + EPS, "stock above the fallen cap is cut to it");

    let mut last = tile.wear[FRUIT];
    for _ in 0..60 {
        let cap = w.tiles[t].cap[FRUIT];
        w.tiles[t].stock[FRUIT] = cap;
        w.tiles[t].take[FRUIT] = 4.0;
        wear(&mut w);
        let tile = &w.tiles[t];
        let worn = tile.wear[FRUIT];
        assert!(worn >= last && worn <= DEPLETION.max + EPS, "wear {worn}");
        assert!((tile.cap[FRUIT] - tile.unworn[FRUIT] * (1.0 - worn)).abs() < EPS);
        assert!(tile.stock[FRUIT] <= tile.cap[FRUIT] + EPS);
        last = worn;
    }
    assert!((last - DEPLETION.max).abs() < EPS, "a food taken hard enough reaches the most wear");
    assert!(engine::land::is_depleted(&w.tiles[t]));
}

#[test]
fn resting_lowers_wear_back_to_nothing_and_the_cap_to_unworn() {
    let mut w = flat();
    let t = at(0, 0);
    full(&mut w, t, FRUIT, 10.0);
    for _ in 0..60 {
        let cap = w.tiles[t].cap[FRUIT];
        w.tiles[t].stock[FRUIT] = cap;
        w.tiles[t].take[FRUIT] = 4.0;
        wear(&mut w);
    }
    assert!((w.tiles[t].wear[FRUIT] - DEPLETION.max).abs() < EPS);

    // A take at its yield (0.2 of the worn cap) rests the food as no take does.
    let before = w.tiles[t].wear[FRUIT];
    w.tiles[t].take[FRUIT] = 0.2 * w.tiles[t].cap[FRUIT];
    wear(&mut w);
    assert!((w.tiles[t].wear[FRUIT] - (before - DEPLETION.recover)).abs() < EPS);

    for _ in 0..40 {
        w.tiles[t].take[FRUIT] = 0.0;
        wear(&mut w);
    }
    let tile = &w.tiles[t];
    assert_eq!(tile.wear[FRUIT], 0.0);
    assert_eq!(tile.cap[FRUIT], tile.unworn[FRUIT], "the cap is back to unworn");
    assert_eq!(tile.cap[FRUIT], 10.0);
}

#[test]
fn game_yields_its_breeding_most_and_wearing_it_leaves_the_fruit() {
    let mut w = flat();
    let t = at(0, 0);
    full(&mut w, t, GAME, 10.0);
    full(&mut w, t, FRUIT, 10.0);
    // Game regrows a breeding yield of (0.35 / 4 + 0.02) × 10 = 1.075; fruit's is 2. A take of
    // 1.5 is over game's yield, though under the 3.5 a plain regrowth would allow.
    w.tiles[t].take[GAME] = 1.5;
    w.tiles[t].take[FRUIT] = 1.5;
    wear(&mut w);
    let tile = &w.tiles[t];
    let want = DEPLETION.rate * (1.5 - 1.075) / 10.0;
    assert!((tile.wear[GAME] - want).abs() < EPS);
    assert_eq!(tile.wear[FRUIT], 0.0);
    assert_eq!(tile.cap[FRUIT], 10.0);
}

#[test]
fn a_tile_with_traffic_gets_a_trail_and_one_without_loses_it() {
    let mut w = flat();
    let (home, far) = (at(0, 0), at(1, 0));
    put(&mut w, &[home, far], 3.0);
    let mut turns = 0;
    while !w.tiles[far].trail {
        walk(&mut w, &[far]);
        turns += 1;
        assert!(turns < 200, "traffic {}", w.tiles[far].traffic);
    }
    assert!(w.tiles[far].traffic >= TRAIL.on);
    assert!(!w.tiles[home].trail, "nobody walked home");

    // Unwalked, the trail goes once the traffic is under `off`, not before.
    while w.tiles[far].traffic * TRAIL.keep >= TRAIL.off {
        walk(&mut w, &[]);
        assert!(w.tiles[far].trail, "traffic {} is not under {}", w.tiles[far].traffic, TRAIL.off);
    }
    walk(&mut w, &[]);
    assert!(!w.tiles[far].trail);
}

#[test]
fn a_used_scout_or_camp_line_keeps_its_trail_with_no_other_traffic() {
    let mut w = flat();
    let t = at(1, 0);
    w.tiles[t].trail = true;
    for _ in 0..50 {
        for x in &mut w.tiles {
            x.traffic *= TRAIL.keep;
            x.used = false;
        }
        w.tiles[t].used = true;
        tend(&mut w);
    }
    assert!(w.tiles[t].traffic < TRAIL.off);
    assert!(w.tiles[t].trail, "used this turn, the trail stays");
    walk(&mut w, &[]);
    assert!(!w.tiles[t].trail, "unused, it goes");
}

/// A turn of `harvest` and `tend`.
fn turn(w: &mut World) {
    harvest(w);
    tend(w);
}

fn route(to: usize, path: &[usize]) -> Route {
    Route { to, path: path.to_vec(), cost: 0.0 }
}

#[test]
fn a_clans_home_gains_traffic_every_turn_it_walks_and_wears_a_trail() {
    let mut w = flat();
    let (o, a) = (at(0, 0), at(1, 0));
    let id = put(&mut w, &[o, a], 12.0);
    harvest(&mut w);
    assert_eq!((w.tiles[o].traffic, w.tiles[a].traffic), (12.0, 12.0), "home is on the walk");
    harvest(&mut w);
    assert_eq!(w.tiles[o].traffic, 12.0 * TRAIL.keep + 12.0);
    let mut turns = 0;
    while !w.tiles[o].trail {
        turn(&mut w);
        turns += 1;
        assert!(turns < 200);
    }
    assert!(w.tiles[a].trail && w.tiles[o].trail, "12 people on one walk reach {}", TRAIL.on);
    // A clan with nowhere to walk adds nothing.
    let lone = put(&mut w, &[at(-3, 0)], 12.0);
    harvest(&mut w);
    assert_eq!(w.tiles[at(-3, 0)].traffic, 0.0);
    let _ = (id, lone);
}

#[test]
fn a_tile_one_small_clan_walks_to_alone_stays_below_on_and_two_clans_routes_wear_one() {
    let mut w = flat();
    let (o1, o2) = (at(0, 0), at(2, 0));
    let (alone, shared) = (at(-2, 3), at(1, 0));
    let small = put(&mut w, &[o1], 6.0);
    let other = put(&mut w, &[o2], 6.0);
    w.clans.get_mut(&small).unwrap().routes = vec![route(shared, &[alone, shared])];
    w.clans.get_mut(&other).unwrap().routes = vec![route(shared, &[shared])];
    for _ in 0..120 {
        turn(&mut w);
    }
    assert!(w.tiles[alone].traffic < TRAIL.on && !w.tiles[alone].trail, "{}", w.tiles[alone].traffic);
    assert!(w.tiles[shared].trail, "two clans of 6 reach {}: {}", TRAIL.on, w.tiles[shared].traffic);
}

#[test]
fn desert_costs_two_to_enter() {
    let mut w = flat();
    let (a, d) = (at(0, 0), at(1, 0));
    w.tiles[d].terrain = DESERT;
    assert_eq!(engine::land::step_cost(&w, a, d), Some(2.0));
    assert_eq!(TERRAIN[DESERT].move_cost, Some(2.0));
}

#[test]
fn holding_a_tile_long_makes_no_trail() {
    let mut w = flat();
    let tiles = [at(0, 0), at(1, 0), at(0, 1)];
    put(&mut w, &tiles, 3.0);
    for _ in 0..60 {
        walk(&mut w, &[]);
    }
    assert!(tiles.iter().all(|&t| !w.tiles[t].trail));
    assert!(tiles.iter().all(|&t| w.tiles[t].claim_run >= 60));
}

#[test]
fn taming_keeps_raising_grain_past_a_hearth_and_falls_back_unheld() {
    let mut w = flat();
    let t = at(0, 0);
    put(&mut w, &[t], 3.0);
    let turns = TENDING.tame_turns;
    {
        let tile = &mut w.tiles[t];
        tile.wild_grain = 4.0;
        tile.unworn[GRAIN] = 4.0;
        tile.cap[GRAIN] = 4.0;
        tile.stock[GRAIN] = 4.0;
        tile.claim_run = TENDING.after;
    }
    for _ in 0..turns {
        tend(&mut w);
    }
    assert_eq!(w.tiles[t].tame, turns);
    assert!((w.tiles[t].cap[GRAIN] - 4.0 * TENDING.domestic).abs() < 1e-9, "a hearth's grain is domestic");

    for _ in 0..turns {
        tend(&mut w);
    }
    let tile = &w.tiles[t];
    assert_eq!(tile.tame, 2 * turns, "taming has no ceiling");
    let want = 4.0 * (TENDING.domestic + TENDING.beyond);
    assert!((tile.cap[GRAIN] - want).abs() < 1e-9, "cap {} want {want}", tile.cap[GRAIN]);
    let tamed = w.events.iter().filter(|e| e.kind == EventKind::HearthTamed).count();
    assert_eq!(tamed, 1, "a hearth is announced once");

    w.tiles[t].claim = None;
    for _ in 0..turns {
        tend(&mut w);
    }
    let tile = &w.tiles[t];
    assert_eq!(tile.tame, turns, "unheld, it tames back a step a turn");
    assert!((tile.cap[GRAIN] - 4.0 * TENDING.domestic).abs() < 1e-9);
    assert_eq!(domestic(0), 1.0);
    assert!((domestic(turns) - TENDING.domestic).abs() < EPS);
    assert!((domestic(2 * turns) - (TENDING.domestic + TENDING.beyond)).abs() < EPS);
    assert_eq!(w.events.iter().filter(|e| e.kind == EventKind::HearthTamed).count(), 1);
}

#[test]
fn a_depleted_tile_counts_in_the_summary() {
    let mut w = flat();
    let before = summary(&w).depleted;
    let t = at(0, 0);
    w.tiles[t].wear[GAME] = DEPLETION.depleted_at - 0.01;
    assert_eq!(summary(&w).depleted, before, "under the line it is not depleted");
    w.tiles[t].wear[GAME] = DEPLETION.depleted_at;
    assert_eq!(summary(&w).depleted, before + 1);
}

#[test]
fn a_built_trail_survives_the_turn_it_is_built_then_decays_unwalked() {
    let mut w = flat();
    let t = at(0, 0);
    w.tiles[t].terrain = FOREST;
    w.tiles[t].trail_cost = Some(1.0);
    assert!(build_trail(&mut w, t, TRAIL.work[0].1));
    walk(&mut w, &[]);
    assert!(w.tiles[t].trail, "built this turn, it survives the turn's tend");
    for _ in 0..200 {
        walk(&mut w, &[]);
        if !w.tiles[t].trail {
            break;
        }
    }
    assert!(!w.tiles[t].trail, "unwalked, it decays by traffic like any other");
}

#[test]
fn a_built_pass_is_kept_by_walking_and_no_mountain_gets_a_worn_trail() {
    let mut w = flat();
    let (pass, bare) = (at(0, 0), at(1, 0));
    for m in [pass, bare] {
        w.tiles[m].terrain = MOUNTAIN;
        w.tiles[m].span = Some(1);
        w.tiles[m].trail_cost = Some(MAP.mountain_cost as f64);
    }
    assert!(build_trail(&mut w, pass, TRAIL.work[2].1));
    for _ in 0..60 {
        walk(&mut w, &[pass, bare]);
    }
    assert!(w.tiles[pass].trail, "walked, a built pass stays");
    assert!(!w.tiles[bare].trail, "walked bare, a mountain never gets a trail");
    assert!(w.tiles[bare].traffic >= TRAIL.on);
}
