mod common;

use common::*;
use engine::data::JUDGE;
use engine::judge::Watch;
use engine::stats::{self, standing};
use engine::{Action, Config, Order, Phase, World};

/// Three civs' clans from `three`, with no units; the first two clans are the civs' with the given
/// people, the third is unaligned with `unaligned` people. Every other clan is empty.
fn world_with(people: [f64; 2], unaligned: f64) -> (World, [u32; 2], [u32; 3]) {
    let (mut w, ids, clans) = three();
    w.units.clear();
    for k in w.clans.values_mut() {
        k.people = 0.0;
        k.civ = None;
    }
    for i in 0..2 {
        let k = w.clans.get_mut(&clans[i]).unwrap();
        k.civ = Some(ids[i]);
        k.people = people[i];
    }
    w.clans.get_mut(&clans[2]).unwrap().people = unaligned;
    (w, [ids[0], ids[1]], clans)
}

#[test]
fn standing_is_the_civs_share_of_all_people() {
    let (w, _, _) = led();
    let all: f64 = w.clans.values().map(|k| k.people).sum::<f64>() + w.units.values().map(|u| u.people).sum::<f64>();
    let mine: f64 = w.clans.values().filter(|k| k.civ.is_some()).map(|k| k.people).sum::<f64>()
        + w.units.values().map(|u| u.people).sum::<f64>();
    let s = standing(&w);
    assert_eq!(s.len(), w.civs.len(), "every civ is there");
    let sum: f64 = s.values().sum();
    assert!((sum - mine / all).abs() < 1e-12, "civs' shares sum to the civs' share of the world");
}

#[test]
fn a_civ_with_no_people_has_no_standing() {
    let (mut w, ids, _) = led();
    let gone = ids[2];
    w.units.retain(|_, u| u.civ != gone);
    for k in w.clans.values_mut().filter(|k| k.civ == Some(gone)) {
        k.people = 0.0;
    }
    assert_eq!(standing(&w)[&gone], 0.0);
}

#[test]
fn standing_is_empty_for_a_world_with_nobody() {
    let w = World::new(Config { seed: 2, radius: 6, army: true, gifts: true, zone: true });
    assert!(standing(&w).values().all(|&s| s == 0.0));
    assert!(stats::summary(&w).standing.values().all(|&s| s == 0.0));
}

#[test]
fn raising_a_camp_moves_no_standing() {
    let (mut w, ids, _) = led();
    fill_stores(&mut w);
    let before = standing(&w);
    w.order(ids[0], Order::Raise { n: 2 }).unwrap();
    assert!(engine::units::camp(&w, ids[0]).is_some(), "the raise made a camp");
    let after = standing(&w);
    for id in ids {
        assert!((before[&id] - after[&id]).abs() < 1e-12, "civ {id}: standing moved");
    }
}

#[test]
fn an_accepted_order_counts_under_its_kind_and_a_refused_one_does_not() {
    let (mut w, ids, _) = led();
    fill_stores(&mut w);
    w.act(Action::Order { civ: ids[0], order: Order::Raise { n: 1 } }).unwrap();
    w.act(Action::SetGift { food: 1.0 }).unwrap();
    assert_eq!(w.orders[&ids[0]]["raise"], 1);
    assert_eq!(w.orders[&ids[0]]["setGift"], 1);
    let refused = w.act(Action::Order { civ: ids[0], order: Order::Move { unit: 9999, to: 0 } });
    assert!(refused.is_err());
    assert!(!w.orders[&ids[0]].contains_key("move"));
}

#[test]
fn the_orders_counter_is_not_in_the_save_or_the_json() {
    let (mut w, ids, _) = led();
    fill_stores(&mut w);
    w.act(Action::Order { civ: ids[0], order: Order::Raise { n: 1 } }).unwrap();
    assert!(!serde_json::to_string(&w).unwrap().contains("orders"));
}

#[test]
fn the_ai_orders_are_counted_too() {
    let mut w = World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true });
    while w.phase == Phase::Pregame {
        w.run(1);
    }
    w.run(60);
    let counted: u32 = w.orders.values().flat_map(|k| k.values()).sum();
    assert!(counted > 0, "the AI gave orders, and they were counted");
}

#[test]
fn each_order_kind_is_its_wire_name() {
    let samples = [
        Order::Raise { n: 1 },
        Order::Disband { unit: 1, n: None },
        Order::Move { unit: 1, to: 0 },
        Order::Zone { unit: 1, tiles: vec![] },
        Order::Switches { unit: 1, forage: true, supply: true },
        Order::Resupply { unit: 1 },
        Order::Forage { unit: 1 },
        Order::Pillage { unit: 1 },
        Order::Route { unit: 1, from: 0, to: 1 },
        Order::ClearRoute { unit: 1 },
    ];
    for o in samples {
        let v = serde_json::to_value(&o).unwrap();
        assert_eq!(v["type"], o.kind());
    }
}

#[test]
fn a_near_tie_swapping_the_lead_is_no_change_and_a_real_one_is() {
    let (mut w, [a, b], clans) = world_with([10.0, 10.0], 0.0);
    let mut watch = Watch::new(&w);
    assert_eq!(watch.leader_at(0), Some(a), "a tie goes to the lowest id");
    w.clans.get_mut(&clans[1]).unwrap().people = 10.0 + 2.0 * JUDGE.lead_margin * 10.0 / 2.0;
    watch.turn(&w);
    assert_eq!(watch.lead_changes(), 0, "a lead within the margin is no change");
    w.clans.get_mut(&clans[1]).unwrap().people = 11.0;
    watch.turn(&w);
    assert_eq!(watch.lead_changes(), 1);
    assert_eq!(watch.leader_at(2), Some(b));
}

#[test]
fn a_clan_joining_a_civ_counts_its_people() {
    let (mut w, [a, _], clans) = world_with([10.0, 10.0], 5.0);
    let mut watch = Watch::new(&w);
    w.clans.get_mut(&clans[2]).unwrap().civ = Some(a);
    watch.turn(&w);
    assert_eq!(watch.clans_changed(), 1);
    assert_eq!(watch.people_changed(), 5.0);
}

#[test]
fn a_clan_gone_or_new_is_not_a_change() {
    let (mut w, _, clans) = world_with([10.0, 10.0], 5.0);
    let mut watch = Watch::new(&w);
    w.clans.remove(&clans[2]);
    watch.turn(&w);
    assert_eq!(watch.clans_changed(), 0);
}

#[test]
fn equal_worlds_give_equal_scores() {
    let run = || {
        let (mut w, ids, clans) = world_with([10.0, 6.0], 4.0);
        let mut watch = Watch::new(&w);
        for k in 1..=3 {
            w.clans.get_mut(&clans[1]).unwrap().people = 6.0 + f64::from(k) * 2.0;
            watch.turn(&w);
        }
        watch.scores(&w, &ids)
    };
    assert_eq!(run(), run());
}

#[test]
fn a_played_game_scores_the_same_twice() {
    let run = || {
        let mut w = World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true });
        while w.phase == Phase::Pregame {
            w.run(1);
        }
        w.act(Action::ChooseCiv { civ: None }).unwrap();
        let civs: Vec<u32> = w.civs.keys().copied().collect();
        let mut watch = Watch::new(&w);
        for _ in 0..30 {
            w.run(1);
            watch.turn(&w);
        }
        watch.scores(&w, &civs)
    };
    let (a, b) = (run(), run());
    assert_eq!(a, b);
    assert_eq!(a.standing.civs.len(), 3);
    assert!(a.standing.civs.iter().all(|c| c.at_100.is_none()), "30 turns: no +100 yet");
}
