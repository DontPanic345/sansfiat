mod common;

use common::*;
use engine::data::{JUDGE, N, UNIT};
use engine::judge::{rank_of, Watch};
use engine::stage::Stage;
use engine::stats::{self, standing};
use engine::units::{self, UnitKind};
use engine::{Action, Config, Order, Phase, World};
use std::collections::BTreeMap;

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
    let w = World::new(Config { seed: 2, radius: 6, army: true, gifts: true, zone: true, ..Config::default() });
    assert!(standing(&w).values().all(|&s| s == 0.0));
    assert!(stats::summary(&w).standing.values().all(|&s| s == 0.0));
}

#[test]
fn raising_a_camp_moves_no_standing() {
    let (mut w, ids, _) = led();
    fill_stores(&mut w);
    let before = standing(&w);
    w.order(ids[0], Order::Raise { n: 2, kind: UnitKind::Camp, fighter: Default::default(), into: None }).unwrap();
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
    w.act(Action::Order {
        civ: ids[0],
        order: Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Default::default(), into: None },
    })
    .unwrap();
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
    w.act(Action::Order {
        civ: ids[0],
        order: Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Default::default(), into: None },
    })
    .unwrap();
    assert!(!serde_json::to_string(&w).unwrap().contains("orders"));
}

#[test]
fn the_ai_orders_are_counted_too() {
    let mut w = World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true, ..Config::default() });
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
        Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Default::default(), into: None },
        Order::Disband { unit: 1, n: None, fighter: None },
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
        let mut w =
            World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true, ..Config::default() });
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

/// The economy group reads a staged world: a store at a pack's worth is a civ-turn at pack, the largest
/// army is the most people in one camp, and the levy pays in the first turns after the pick.
#[test]
fn a_staged_store_at_a_pack_is_a_turn_at_pack() {
    let mut s = Stage::retro("small", 2).unwrap();
    let civs: Vec<u32> = s.world().civs.keys().copied().collect();
    let pack = units::pack_cap(UNIT.first as f64);
    for &c in &civs {
        s.world_mut().civs.get_mut(&c).unwrap().store = [0.0; N];
    }
    s.world_mut().civs.get_mut(&civs[0]).unwrap().store[0] = pack;
    let mut watch = Watch::new(s.world());
    watch.turn(s.world());
    let e = watch.scores(s.world(), &civs).economy;
    assert_eq!(e.turns_at_pack, 1, "one civ-turn at pack: {e:?}");
    assert!((e.store_max - pack).abs() < 1e-9, "{e:?}");
}

#[test]
fn a_staged_army_sets_the_largest_army() {
    let mut s = Stage::retro("small", 2).unwrap();
    let civs: Vec<u32> = s.world().civs.keys().copied().collect();
    let civ = civs[0];
    let cap = s.world().civs[&civ].capital;
    s.army(civ, cap, 40.0);
    let mut watch = Watch::new(s.world());
    watch.turn(s.world());
    assert_eq!(watch.scores(s.world(), &civs).economy.largest_army, 40.0);
}

#[test]
fn the_levy_pays_in_the_first_turns_after_the_pick() {
    let mut s = Stage::retro("small", 2).unwrap();
    let civs: Vec<u32> = s.world().civs.keys().copied().collect();
    s.lead(civs[0]).unwrap();
    let mut watch = Watch::new(s.world());
    for _ in 0..3 {
        s.turn(1).unwrap();
        watch.turn(s.world());
    }
    let e = watch.scores(s.world(), &civs).economy;
    assert!(e.levy_per_turn > 0.0, "{e:?}");
}

/// Two or three civs on a flat map with a civ on each of `n` tiles, the first led; each capital's
/// clan has `people`. As `over.rs`'s helper of the same name.
fn civs_on_flat(n: usize, people: f64) -> (Stage, Vec<u32>) {
    let mut s = Stage::flat(BIG);
    let tiles = [at_big(-5, 0), at_big(5, 0), at_big(0, -10)];
    let ids = s.civs(&tiles[..n], people);
    s.lead(ids[0]).unwrap();
    (s, ids)
}

/// Makes `civ`'s capital fall to an army of `people` warriors placed on it by `taker`.
fn capital_open_to(s: &mut Stage, civ: u32, taker: u32, people: f64) {
    let cap = s.world().civs[&civ].capital;
    for k in s.world_mut().clans.values_mut().filter(|k| k.home == cap && k.civ == Some(civ)) {
        k.people = 0.5;
    }
    s.army(taker, cap, people);
    let id = s.camp(taker).expect("the taker's army").id;
    s.world_mut().units.get_mut(&id).unwrap().camp_mut().unwrap().dest = Some(cap);
}

#[test]
fn a_game_ended_by_the_limit_reads_its_end_and_the_ranks_at_it() {
    let mut s = Stage::flat(BIG);
    let ids = s.civs(&[at_big(-5, 0), at_big(5, 0)], 14.0);
    s.world_mut().config.limit = Some(3);
    s.lead(ids[0]).unwrap();
    let mut watch = Watch::new(s.world());
    for _ in 0..3 {
        s.turn(1).unwrap();
        watch.turn(s.world());
    }
    let e = watch.scores(s.world(), &ids).end;
    let over = e.over.clone().expect("the game ended at its limit");
    assert_eq!((over.why.as_str(), over.turn), ("limit", 3));
    assert_eq!(over.winner, s.over().unwrap().winner);
    assert!(!e.early, "the limit is not early");
    assert_eq!(e.capitals_fallen, 0);
    let at_end = &s.world().standing[3];
    let expect: BTreeMap<u32, usize> = ids.iter().map(|&id| (id, rank_of(at_end, id))).collect();
    assert_eq!(e.ranks, expect, "the ranks are read from World.standing at the end");
    assert_eq!(e.led_rank, Some(expect[&ids[0]]), "the led civ's rank");
}

#[test]
fn a_watched_game_has_no_led_rank_and_a_game_still_on_has_no_end() {
    let mut s = Stage::flat(BIG);
    let ids = s.civs(&[at_big(-5, 0), at_big(5, 0)], 14.0);
    s.act(Action::ChooseCiv { civ: None }).unwrap();
    let mut watch = Watch::new(s.world());
    s.turn(2).unwrap();
    watch.turn(s.world());
    watch.turn(s.world());
    let e = watch.scores(s.world(), &ids).end;
    assert!(e.over.is_none(), "the game goes on");
    assert!(!e.early);
    assert_eq!(e.led_rank, None, "no civ is led when watching");
    assert_eq!(e.ranks.len(), ids.len(), "every civ has a rank at the watch's last turn");
}

#[test]
fn a_conquest_reads_as_early_with_its_capital_taken() {
    let (mut s, ids) = civs_on_flat(2, 14.0);
    let (me, rival) = (ids[0], ids[1]);
    capital_open_to(&mut s, rival, me, 5.0);
    let mut watch = Watch::new(s.world());
    for _ in 0..5 {
        if matches!(s.world().phase, Phase::Over(_)) {
            break;
        }
        s.turn(1).unwrap();
        watch.turn(s.world());
    }
    let e = watch.scores(s.world(), &ids).end;
    let over = e.over.clone().expect("the rival's capital fell and the game ended");
    assert_eq!(over.why, "conquest");
    assert_eq!(over.winner, me);
    assert!(e.early, "a conquest is before the limit");
    assert_eq!(e.capitals_fallen, 1, "one capital taken after the pick");
    let at_end = &s.world().standing[over.turn as usize];
    assert_eq!(e.ranks[&me], rank_of(at_end, me), "the rank at the end, at Over");
}
