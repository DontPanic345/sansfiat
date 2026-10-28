//! Depletion, trails from walking and taming with no ceiling, on hand-built worlds.
mod common;

use common::{at, flat, put};
use engine::data::*;
use engine::land::{domestic, hold, wear};
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

/// One turn of walking: `walked` is the tiles walked this turn, the rest are not.
fn walk(w: &mut World, walked: &[usize]) {
    for t in &mut w.tiles {
        t.walked = false;
    }
    for &i in walked {
        w.tiles[i].walked = true;
    }
    hold(w);
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
fn a_tile_walked_long_enough_gets_a_trail_and_an_unwalked_one_fades() {
    let mut w = flat();
    let (home, far) = (at(0, 0), at(1, 0));
    put(&mut w, &[home, far], 3.0);
    for _ in 0..TRAIL.after - 1 {
        walk(&mut w, &[far]);
        assert!(!w.tiles[far].trail);
    }
    walk(&mut w, &[far]);
    assert!(w.tiles[far].trail, "walked {} turns in a row", TRAIL.after);
    assert!(!w.tiles[home].trail, "held and never walked, home makes no trail");

    for _ in 0..TRAIL.fade - 1 {
        walk(&mut w, &[]);
        assert!(w.tiles[far].trail, "the trail holds until it has been unwalked for {}", TRAIL.fade);
    }
    walk(&mut w, &[]);
    assert!(!w.tiles[far].trail);
    assert!(!w.tiles[home].trail);
}

#[test]
fn holding_a_tile_long_makes_no_trail() {
    let mut w = flat();
    let tiles = [at(0, 0), at(1, 0), at(0, 1)];
    put(&mut w, &tiles, 3.0);
    for _ in 0..3 * TRAIL.after {
        walk(&mut w, &[]);
    }
    assert!(tiles.iter().all(|&t| !w.tiles[t].trail));
    assert!(tiles.iter().all(|&t| w.tiles[t].held >= 3 * TRAIL.after));
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
        tile.held = TENDING.after;
    }
    for _ in 0..turns {
        hold(&mut w);
    }
    assert_eq!(w.tiles[t].tame, turns);
    assert!((w.tiles[t].cap[GRAIN] - 4.0 * TENDING.domestic).abs() < 1e-9, "a hearth's grain is domestic");

    for _ in 0..turns {
        hold(&mut w);
    }
    let tile = &w.tiles[t];
    assert_eq!(tile.tame, 2 * turns, "taming has no ceiling");
    let want = 4.0 * (TENDING.domestic + TENDING.beyond);
    assert!((tile.cap[GRAIN] - want).abs() < 1e-9, "cap {} want {want}", tile.cap[GRAIN]);
    let tamed = w.events.iter().filter(|e| e.kind == EventKind::HearthTamed).count();
    assert_eq!(tamed, 1, "a hearth is announced once");

    w.tiles[t].claim = None;
    for _ in 0..turns {
        hold(&mut w);
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
