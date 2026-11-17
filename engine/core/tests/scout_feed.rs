//! The scout's food on the stage: its pack, filled at a store. An errand that takes one trip to the
//! store and is done by a named turn, a rival's zone that holds it where it is, and a rival army on
//! its tile that kills it.

mod common;

use common::{at_big, led_order, led_scout, BIG};
use engine::data::*;
use engine::events::EventKind;
use engine::stage::Stage;
use engine::units::{self, Doing};
use engine::{Action, Order};

/// Two civs on bare plains twenty steps apart, led by the first, its store full.
fn plain() -> (Stage, Vec<u32>) {
    let mut s = Stage::flat(BIG);
    let ids = s.civs(&[at_big(-10, 0), at_big(10, 0)], 14.0);
    s.lead(ids[0]).unwrap();
    s.plain();
    (s, ids)
}

fn stock(s: &mut Stage, civ: u32) {
    s.world_mut().civs.get_mut(&civ).unwrap().store = [0.0, 50.0, 0.0, 0.0, 0.0, 0.0, 0.0];
}

/// A known route: forest with a cheap trail hidden on tiles 6 to 9 steps out, walled by mountains. The
/// scout starts beside it with five turns in its pack, so it goes to its store once before the trails
/// are built.
#[test]
fn an_errand_takes_one_trip_to_the_store_and_is_done_by_turn_10() {
    let (mut s, ids) = plain();
    let me = ids[0];
    for q in -4..=-1 {
        s.paint(at_big(q, 0), FOREST, Some(1.0 * DAY), None);
        for r in [-1, 1] {
            s.paint(at_big(q, r), MOUNTAIN, None, None);
            s.paint(at_big(q - r.min(0), r), MOUNTAIN, None, None);
        }
    }
    let id = led_scout(s.world());
    let u = s.world_mut().units.get_mut(&id).unwrap();
    u.actor.home = at_big(-5, 0);
    u.actor.territory = vec![at_big(-5, 0)];
    u.actor.store = [0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let (from, to) = (at_big(-10, 0), at_big(2, 0));
    let order = led_order(s.world(), Order::Route { unit: id, from, to });
    s.act(order).unwrap();
    let (mut trips, mut homing, mut back, mut done) = (0, false, false, None);
    for turn in 1..=40 {
        stock(&mut s, me);
        s.act(Action::EndTurn).unwrap();
        let u = &s.world().units[&id];
        let sc = u.scout().unwrap();
        assert_eq!(u.hungry, 0, "never hungry (turn {turn})");
        if sc.homing && !homing && done.is_none() {
            trips += 1;
        }
        homing = sc.homing;
        back |= matches!(sc.doing, Doing::Back(_));
        if sc.built && done.is_none() {
            done = Some(turn);
        }
    }
    assert_eq!(trips, 1, "one trip to the store before its trails are built");
    assert!(back, "it walked back to its errand");
    assert!(done.is_some_and(|t| t <= 10), "its trails are built by turn 10: {done:?}");
    for q in -4..=-1 {
        assert!(s.trail(at_big(q, 0)), "a trail at ({q}, 0)");
    }
}

/// The rival's scout, put on `tile` with a pack and a destination.
fn rival_scout(s: &mut Stage, civ: u32, tile: usize, dest: usize) -> u32 {
    let id = units::scout(s.world(), civ).unwrap().id;
    let u = s.world_mut().units.get_mut(&id).unwrap();
    u.actor.home = tile;
    u.actor.territory = vec![tile];
    u.actor.store = [0.0, 30.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    u.scout_mut().unwrap().dest = Some(dest);
    id
}

#[test]
fn a_rival_army_on_its_tile_kills_a_scout() {
    let (mut s, ids) = plain();
    let tile = at_big(0, 0);
    let id = rival_scout(&mut s, ids[1], tile, tile);
    s.army(ids[0], tile, 3.0);
    s.act(Action::EndTurn).unwrap();
    assert!(!s.world().units.contains_key(&id), "the scout is dead");
    assert_eq!(s.events(EventKind::ScoutKilled), 1);
}

#[test]
fn a_scout_in_a_rivals_zone_stays_until_the_zone_lifts() {
    let (mut s, ids) = plain();
    let (here, camp) = (at_big(0, 0), at_big(1, 0));
    let id = rival_scout(&mut s, ids[1], here, at_big(0, 4));
    let army = s.army(ids[0], camp, 4.0);
    let order = led_order(s.world(), Order::Zone { unit: army, tiles: vec![here] });
    s.act(order).unwrap();
    s.turn(2).unwrap();
    assert!(units::shut(s.world(), ids[1]).contains_key(&here), "the army holds the scout's tile");
    let u = &s.world().units[&id];
    assert_eq!(u.home, here, "it stays");
    assert_eq!(u.scout().unwrap().doing, Doing::Trapped(ids[0]));
    assert!(units::doing_text(s.world(), u).starts_with("trapped"));
    let order = led_order(s.world(), Order::Zone { unit: army, tiles: vec![] });
    s.act(order).unwrap();
    s.turn(2).unwrap();
    assert_ne!(s.world().units[&id].home, here, "the zone lifted: it walks on");
}

/// A scout outside a rival's zone walks round it, not through.
#[test]
fn a_scout_walks_round_a_rivals_zone() {
    let (mut s, ids) = plain();
    let id = rival_scout(&mut s, ids[1], at_big(3, 0), at_big(-3, 0));
    let army = s.army(ids[0], at_big(0, 0), 6.0);
    let order = led_order(s.world(), Order::Zone { unit: army, tiles: vec![at_big(0, -1), at_big(0, 1)] });
    s.act(order).unwrap();
    for _ in 0..6 {
        s.act(Action::EndTurn).unwrap();
        let at = s.world().units[&id].home;
        assert!(!units::shut(s.world(), ids[1]).contains_key(&at), "never on a held tile");
    }
    assert_eq!(s.world().units[&id].home, at_big(-3, 0), "it gets there");
}
