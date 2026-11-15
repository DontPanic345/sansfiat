//! The end of a game: the `over` step, the three ways a game ends, the refusals after it, the
//! limit, Standing over time, and a played game past its end replaying to the same world.

mod common;

use common::*;
use engine::data::Fighter;
use engine::over::{self, Why};
use engine::stage::Stage;
use engine::{stats, Action, Config, EventKind, Order, Phase, Save, UnitKind, World};

/// A flat map of radius `BIG` with a civ on each of `n` tiles, the first led (none when `lead` is
/// false). Each capital's clan has `people`.
fn civs_on_flat(n: usize, people: f64, lead: bool) -> (Stage, Vec<u32>) {
    let mut s = Stage::flat(BIG);
    let tiles = [at_big(-5, 0), at_big(5, 0), at_big(0, -10)];
    let ids = s.civs(&tiles[..n], people);
    if lead {
        s.lead(ids[0]).unwrap();
    }
    (s, ids)
}

/// Makes `civ`'s capital fall to an army of `people` warriors placed on it by `taker`: the capital's
/// own clan is cut to a half person and the civ has no camp.
fn capital_open_to(s: &mut Stage, civ: u32, taker: u32, people: f64) {
    let cap = s.world().civs[&civ].capital;
    for k in s.world_mut().clans.values_mut().filter(|k| k.home == cap && k.civ == Some(civ)) {
        k.people = 0.5;
    }
    s.army(taker, cap, people);
    let id = s.camp(taker).expect("the taker's army").id;
    s.world_mut().units.get_mut(&id).unwrap().camp_mut().unwrap().dest = Some(cap);
}

/// The phase's end, if the world has one.
fn ended(w: &World) -> Option<over::Over> {
    match w.phase {
        Phase::Over(o) => Some(o),
        _ => None,
    }
}

#[test]
fn a_led_civ_that_takes_the_last_capital_wins_by_conquest() {
    let (mut s, ids) = civs_on_flat(2, 14.0, true);
    let (me, rival) = (ids[0], ids[1]);
    capital_open_to(&mut s, rival, me, 5.0);
    let turns = s.until(5, |w| ended(w).is_some()).unwrap();
    assert!(turns.is_some(), "the rival's capital falls within a few turns");
    let o = ended(s.world()).unwrap();
    assert_eq!(o.why, Why::Conquest);
    assert_eq!(o.winner, me);
    assert_eq!(s.world().civs[&rival].fallen.map(|f| f.1), Some(me));
    assert_eq!(s.events(EventKind::GameOver), 1);
    assert_eq!(s.over().map(|o| o.winner), Some(me));

    // Orders are refused with the game's end; ending turns goes on.
    let raise = Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None };
    let refused = s.act(Action::Order { civ: me, order: raise.clone() });
    assert_eq!(refused, Err("the game is over".to_string()));
    assert_eq!(s.act(Action::SetGift { food: 1.0 }), Err("the game is over".to_string()));
    assert_eq!(s.act(Action::SetKeep { food: Some(1.0) }), Err("the game is over".to_string()));
    let turn = s.world().turn;
    s.turn(2).unwrap();
    assert_eq!(s.world().turn, turn + 2);
    assert!(matches!(s.world().phase, Phase::Over(_)), "the result is fixed");
}

#[test]
fn a_led_civ_whose_capital_a_rival_takes_is_lost_and_the_last_standing_civ_wins() {
    let (mut s, ids) = civs_on_flat(3, 14.0, true);
    let (me, taker, third) = (ids[0], ids[1], ids[2]);
    capital_open_to(&mut s, me, taker, 5.0);
    let turns = s.until(5, |w| ended(w).is_some()).unwrap();
    assert!(turns.is_some(), "the led capital falls within a few turns");
    let o = ended(s.world()).unwrap();
    assert_eq!(o.why, Why::Fell { by: taker });
    let standing = stats::standing(s.world());
    let expect = if standing[&taker] >= standing[&third] { taker } else { third };
    assert_eq!(o.winner, expect, "the highest Standing among the civs left");
    assert_eq!(s.events(EventKind::GameOver), 1);
    let refused = s.act(Action::Order {
        civ: me,
        order: Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None },
    });
    assert_eq!(refused, Err("the game is over".to_string()));
}

#[test]
fn a_led_civ_taken_by_a_rival_in_a_two_civ_game_is_a_fall_and_the_rival_wins() {
    let (mut s, ids) = civs_on_flat(2, 14.0, true);
    let (me, rival) = (ids[0], ids[1]);
    capital_open_to(&mut s, me, rival, 5.0);
    s.until(5, |w| ended(w).is_some()).unwrap();
    let o = ended(s.world()).unwrap();
    assert_eq!(o.why, Why::Fell { by: rival });
    assert_eq!(o.winner, rival);
}

#[test]
fn watching_one_capital_falling_in_three_civs_ends_nothing() {
    let (mut s, ids) = civs_on_flat(3, 14.0, false);
    let (taker, victim) = (ids[1], ids[0]);
    capital_open_to(&mut s, victim, taker, 5.0);
    s.act(Action::ChooseCiv { civ: None }).unwrap();
    s.turn(3).unwrap();
    assert!(s.world().civs[&victim].is_out(), "its capital fell");
    assert_eq!(ended(s.world()), None, "two civs are still standing");
}

#[test]
fn a_game_with_one_civ_never_ends_by_conquest() {
    let (mut s, _) = civs_on_flat(1, 14.0, true);
    s.turn(10).unwrap();
    assert_eq!(ended(s.world()), None);
}

#[test]
fn the_limit_ends_the_game_on_the_turn_it_falls_with_the_highest_standing() {
    let mut s = Stage::flat(BIG);
    let ids = s.civs(&[at_big(-5, 0), at_big(5, 0)], 14.0);
    s.world_mut().config.limit = Some(3);
    s.lead(ids[0]).unwrap();
    let picked = s.world().picked.unwrap();
    assert_eq!(over::ends_at(s.world()), Some(picked + 3));
    assert_eq!(over::limit(s.world()), 3);
    s.turn(2).unwrap();
    assert_eq!(ended(s.world()), None, "not yet at two turns");
    s.turn(1).unwrap();
    let o = ended(s.world()).unwrap();
    assert_eq!(o.why, Why::Limit);
    assert_eq!(o.turn, 3);
    assert_eq!(s.world().turn, picked + 3);
    let standing = stats::standing(s.world());
    let best = if standing[&ids[0]] >= standing[&ids[1]] { ids[0] } else { ids[1] };
    assert_eq!(o.winner, best, "the highest Standing; ties to the lower id");
    assert_eq!(s.events(EventKind::GameOver), 1);
}

#[test]
fn a_limit_tie_goes_to_the_lower_id() {
    let mut s = Stage::flat(BIG);
    let ids = s.civs(&[at_big(-5, 0), at_big(5, 0)], 14.0);
    // With no people anywhere, every civ's Standing is 0.
    let w = s.world_mut();
    for k in w.clans.values_mut() {
        k.people = 0.0;
    }
    w.config.limit = Some(2);
    s.lead(ids[1]).unwrap();
    s.turn(2).unwrap();
    let o = ended(s.world()).unwrap();
    assert_eq!(o.why, Why::Limit);
    assert_eq!(o.winner, ids[0].min(ids[1]));
}

#[test]
fn standing_has_a_count_per_turn_since_the_pick_plus_one() {
    let (mut s, _) = civs_on_flat(2, 14.0, true);
    assert_eq!(s.world().standing.len(), 1, "index 0 is at the pick");
    s.turn(4).unwrap();
    assert_eq!(s.world().standing.len(), 5);
    assert!(s.world().standing.iter().all(|m| m.len() == 2));
}

#[test]
fn standing_keeps_counting_after_the_end() {
    let mut s = Stage::flat(BIG);
    let ids = s.civs(&[at_big(-5, 0), at_big(5, 0)], 14.0);
    s.world_mut().config.limit = Some(2);
    s.lead(ids[0]).unwrap();
    s.turn(4).unwrap();
    assert!(ended(s.world()).is_some());
    assert_eq!(s.world().standing.len(), 1 + 4, "the pick, then one for each of the four turns");
}

#[test]
fn the_world_runs_on_after_the_end_and_the_ai_civs_orders_still_go_through() {
    let (mut s, ids) = civs_on_flat(3, 14.0, true);
    let (me, taker, other) = (ids[0], ids[1], ids[2]);
    capital_open_to(&mut s, me, taker, 5.0);
    s.until(5, |w| ended(w).is_some()).unwrap();
    assert_eq!(ended(s.world()).map(|o| o.why), Some(Why::Fell { by: taker }));
    let turn = s.world().turn;
    let before: Vec<(u32, usize)> = s.world().clans.values().map(|k| (k.id, k.home)).collect();
    let people: f64 = s.world().clans.values().map(|k| k.people).sum();
    s.turn(10).unwrap();
    assert_eq!(s.world().turn, turn + 10, "the turns go on");
    let w = s.world();
    let after: Vec<(u32, usize)> = w.clans.values().map(|k| (k.id, k.home)).collect();
    let moved = w.clans.values().map(|k| k.people).sum::<f64>() != people;
    assert!(before != after || moved, "the clans keep living and moving");
    // An AI civ still gives orders through the world's door, which the end doesn't refuse (the
    // taker's army may have marched on the third capital since: a fallen civ gives none).
    let other = if w.civs[&other].is_out() { taker } else { other };
    let raise = Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None };
    let checked = w.check_order(other, &raise);
    assert_ne!(checked, Err("the game is over".to_string()));
    assert_ne!(checked, Err("the game has not begun".to_string()));
    let claim = w.civs[&other].claim.clone();
    assert_eq!(w.check_order(other, &Order::Claim { tiles: claim }), Ok(()), "an AI civ's claim still checks");
}

#[test]
fn a_game_past_its_end_saves_and_replays_to_the_same_world() {
    let config = Config { seed: 2, radius: 16, limit: Some(6), ..Config::default() };
    let mut w = World::new(config);
    while w.phase == Phase::Pregame {
        w.act(Action::EndTurn).unwrap();
    }
    let first = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(first) }).unwrap();
    w.run(12);
    assert!(matches!(w.phase, Phase::Over(_)), "the game ended at its limit");
    let save: Save = w.save();
    let back = World::replay(&save).unwrap();
    assert_eq!(stats::digest(&back), stats::digest(&w));
    assert_eq!(back.standing, w.standing);
    assert_eq!(back.phase, w.phase);
    assert_eq!(back.standing.len(), 12 + 1, "the pick, then one for each turn");
}

#[test]
fn the_limit_of_a_size_takes_the_next_size_up_and_the_largest_past_it() {
    let mut w = World::new(Config { radius: 16, ..Config::default() });
    assert_eq!(over::limit(&w), 200);
    w.config.radius = 20;
    assert_eq!(over::limit(&w), 250);
    w.config.radius = 40;
    assert_eq!(over::limit(&w), 300);
    w.config.limit = Some(7);
    assert_eq!(over::limit(&w), 7);
    assert_eq!(Config::default().limit, None);
}
