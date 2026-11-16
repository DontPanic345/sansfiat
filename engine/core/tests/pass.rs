//! A pass a scout has bridged: a mountain range cuts the map in two with one crossing. Before the
//! scout builds the trail nobody walks across (no path, no march, no line, no signpost draw); after
//! it every actor walks the pass at its trail cost: an army, its line, a clan dividing to a signpost.

mod common;

use common::led_order;
use engine::clans::new_clan;
use engine::data::*;
use engine::hex::Hex;
use engine::land::step_cost;
use engine::map::index;
use engine::stage::Stage;
use engine::units;
use engine::{dividing, homes, settle, Action, Order, World};

const RADIUS: i32 = 9;

fn tile(q: i32, r: i32) -> usize {
    index(RADIUS, Hex { q, r }).unwrap()
}

/// The pass's hidden trail cost.
const PASS: f64 = 2.0 * DAY;

/// Red at (-6, 0), Blue at (6, 0), a wall of mountains down q = 0 with one crossing at (0, 0). Red is
/// led and its store is stocked each turn by `stock`.
fn staged() -> (Stage, u32) {
    let mut s = Stage::flat(RADIUS);
    let ids = s.civs(&[tile(-6, 0), tile(6, 0)], 5.0);
    s.lead(ids[0]).unwrap();
    s.plain();
    s.world_mut().config.fight = false;
    for r in -RADIUS..=RADIUS {
        if let Some(i) = index(RADIUS, Hex { q: 0, r }) {
            if s.world().tiles[i].terrain != SEA {
                s.paint(i, MOUNTAIN, None, None);
            }
        }
    }
    s.paint(tile(0, 0), MOUNTAIN, Some(PASS), Some(1));
    (s, ids[0])
}

fn stock(s: &mut Stage, civ: u32) {
    s.world_mut().civs.get_mut(&civ).unwrap().store = [0.0, 50.0, 0.0, 0.0, 0.0];
}

/// A clan of `civ` with `people` living on `home`, which it holds.
fn clan(w: &mut World, civ: u32, home: usize, people: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let mut c = new_clan(id, home, people);
    c.civ = Some(civ);
    homes::claim_land(w, id, &[home]);
    w.clans.insert(id, c);
    id
}

#[test]
fn a_pass_the_scout_bridged_is_walked_by_an_army_its_line_and_a_clan() {
    let (mut s, red) = staged();
    let (cap, far) = (tile(-6, 0), tile(3, 0));
    let (pass, post) = (tile(0, 0), tile(2, -3));
    let rich = &mut s.world_mut().tiles[post];
    (rich.cap, rich.unworn, rich.stock) = ([6.0; 5], [6.0; 5], [6.0; 5]);
    s.world_mut().clans.clear();
    for t in &mut s.world_mut().tiles {
        t.claim = None;
    }
    let army = s.army(red, tile(-5, 0), 3.0);
    s.order(red, Order::Signpost { tile: post }).unwrap();

    // Before: the range is a wall.
    let w = s.world();
    assert_eq!(units::path_between(w, cap, tile(6, 0)), None, "no path across");
    assert!(w.check_order(red, &Order::Move { unit: army, to: far }).is_err(), "an army's march across is refused");
    let near_odds = |w: &World, seeds: std::ops::RangeInclusive<u32>| {
        seeds
            .filter(|&seed| {
                let mut v = w.clone();
                v.rng = seed;
                let people = CLAN.split_at / 2.0;
                settle::draw(&mut v, Some(red), 9999, tile(-3, 0), people, &[0.0; N], 0.0).is_some()
            })
            .count()
    };
    assert_eq!(near_odds(w, 1..=40), 0, "a signpost across the range draws nobody");

    // The led scout, given a route across, reveals the crossing and builds its trail.
    let scout = units::scout(s.world(), red).unwrap().id;
    s.act(led_order(s.world(), Order::Route { unit: scout, from: cap, to: tile(6, 0) })).unwrap();
    let mut built = None;
    for turn in 1..=120 {
        stock(&mut s, red);
        s.act(Action::EndTurn).unwrap();
        if s.trail(pass) {
            built = Some(turn);
            break;
        }
    }
    let built = built.expect("the scout builds the pass's trail");
    assert!(s.revealed(red, pass), "the crossing is revealed");
    assert!(built <= 60, "by turn 60: {built}");
    let w = s.world();
    assert_eq!(step_cost(w, tile(-1, 0), pass), Some(PASS), "the pass costs its hidden trail cost");
    assert_eq!(
        step_cost(w, pass, tile(1, 0)),
        Some(TERRAIN[PLAINS].move_cost.unwrap()),
        "and the plains beyond theirs"
    );
    assert!(units::path_between(w, cap, tile(6, 0)).is_some(), "a path across");

    // After: the led army marches across, standing on the pass, and its line runs over it.
    s.order(red, Order::Move { unit: army, to: far }).unwrap();
    let (march, _) = units::route(s.world(), army).expect("a march across");
    assert!(march.contains(&pass), "the march crosses the pass tile: {march:?}");
    let mut arrived = None;
    for turn in 1..=40 {
        stock(&mut s, red);
        s.act(Action::EndTurn).unwrap();
        let home = s.world().units[&army].home;
        if home == far {
            arrived = Some(turn);
            break;
        }
    }
    assert!(arrived.is_some_and(|t| t <= 30), "the army arrives across by turn 30: {arrived:?}");
    let (path, _, _) = units::line(s.world(), army).expect("a line runs from the far side");
    assert!(path.contains(&pass), "its line runs over the pass: {path:?}");

    // A clan dividing weighs the signpost across, and a daughter lives there.
    let mother = clan(s.world_mut(), red, tile(-3, 0), CLAN.split_at);
    let w = s.world();
    let mut went = 0;
    for seed in 1..=40 {
        let mut v = w.clone();
        v.rng = seed;
        let drawn = settle::draw(&mut v, Some(red), 9999, tile(-3, 0), CLAN.split_at / 2.0, &[0.0; N], 0.0);
        if drawn.is_some_and(|(t, _)| t == post) {
            went += 1;
        }
    }
    assert!(went > 0, "the signpost across draws a clan: {went} of 40");
    let mut seed = 1;
    let lived = loop {
        let mut v = w.clone();
        v.rng = seed;
        if dividing::split(&mut v, mother, true) {
            settle::step(&mut v);
            break v;
        }
        seed += 1;
        assert!(seed < 200, "a clan divides toward the signpost");
    };
    assert!(lived.clans.values().any(|c| c.home == post && c.civ == Some(red)), "a clan of Red lives across");
    assert!(settle::at(&lived, post).is_some(), "it founds a settlement there");
}
