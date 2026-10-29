mod common;

use common::*;
use engine::clans::eat;
use engine::data::*;
use engine::leaning::{gift_leaning, lean};
use engine::markets::trade;
use engine::{civs, units, Action, Basket, Config, End, EventKind, Phase, World};

/// Three civs founded by hand on the big world, in `Phase::Main`; the first civ's id and market.
/// No clans, so no camp; the first civ's store holds the endowment (20 of good 1).
fn scene() -> (World, u32, u32) {
    let mut w = big();
    let ms = [open(&mut w, at_big(-5, 0)), open(&mut w, at_big(5, 0)), open(&mut w, at_big(0, -10))];
    w.end = Some(End { turn: 0, centres: ms.to_vec(), cover: 1.0, by_limit: false, topped_up: Vec::new() });
    civs::found(&mut w);
    w.phase = Phase::Main;
    let civ = *w.civs.keys().next().unwrap();
    let mid = w.civs[&civ].market;
    (w, civ, mid)
}

/// An unaligned clan of `people` on `tile` that came to market `mid` with `basket`.
fn visitor(w: &mut World, mid: u32, tile: usize, people: f64, basket: Basket) -> u32 {
    let id = put(w, &[tile], people);
    let c = w.clans.get_mut(&id).unwrap();
    c.market = Some(mid);
    c.basket = basket;
    id
}

fn total(b: &Basket) -> f64 {
    b.iter().sum()
}

/// Every good held by the civ's store and the clans' baskets.
fn goods(w: &World, civ: u32) -> Basket {
    std::array::from_fn(|f| w.civs[&civ].store[f] + w.clans.values().map(|c| c.basket[f]).sum::<f64>())
}

#[test]
fn the_default_keep_is_the_camps_full_pack() {
    let (mut w, civ, _) = scene();
    // No camp: a new camp's pack.
    assert_eq!(civs::keep(&w, civ), units::pack_cap(f64::from(UNIT.start)));
    // A camp: its pack to fill.
    for k in 0..4 {
        let t = at_big(-5 + k, 1);
        let id = put(&mut w, &[t], 6.0);
        w.clans.get_mut(&id).unwrap().civ = Some(civ);
    }
    units::raise_at_founding(&mut w, civ);
    let warriors = units::camp(&w, civ).unwrap().people;
    assert!(warriors > 0.0);
    assert_eq!(civs::keep(&w, civ), units::pack_cap(warriors));
    // A set keep overrides it, and None puts it back.
    w.civs.get_mut(&civ).unwrap().keep = Some(2.5);
    assert_eq!(civs::keep(&w, civ), 2.5);
    w.civs.get_mut(&civ).unwrap().keep = None;
    assert_eq!(civs::keep(&w, civ), units::pack_cap(warriors));
    // With the army off it is 0.
    w.config = Config { army: false, ..w.config };
    assert_eq!(civs::keep(&w, civ), 0.0);
}

#[test]
fn a_new_civ_gives_the_default_gift() {
    let (w, civ, _) = scene();
    let c = &w.civs[&civ];
    assert_eq!((c.gift, c.keep, c.posted, c.given, c.joined, c.gifted), (LEANING.gift_default, None, 0.0, 0.0, 0, 0.0));
}

#[test]
fn nothing_is_posted_or_given_below_keep() {
    let (mut w, civ, mid) = scene();
    let a = visitor(&mut w, mid, at_big(-5, 1), 6.0, [4.0, 0.0, 0.0, 0.0, 0.0]);
    w.civs.get_mut(&civ).unwrap().keep = Some(25.0);
    let store = w.civs[&civ].store;
    trade(&mut w);
    let c = &w.civs[&civ];
    assert_eq!((c.posted, c.given, c.gifted), (0.0, 0.0, 0.0));
    assert_eq!(c.store, store);
    assert!(w.clans[&a].leaning.is_empty());
}

#[test]
fn a_civ_posts_what_it_holds_beyond_keep_and_the_clearing_conserves_every_good() {
    let (mut w, civ, mid) = scene();
    visitor(&mut w, mid, at_big(-5, 1), 6.0, [6.0, 0.0, 0.0, 0.0, 0.0]);
    visitor(&mut w, mid, at_big(-4, 1), 6.0, [0.0, 0.0, 6.0, 0.0, 0.0]);
    let c = w.civs.get_mut(&civ).unwrap();
    c.keep = Some(5.0);
    c.gift = 0.0;
    let before = goods(&w, civ);
    trade(&mut w);
    let c = &w.civs[&civ];
    assert!((c.posted - 15.0).abs() < 1e-9, "the store beyond keep: {}", c.posted);
    assert_eq!(c.given, 0.0);
    for (f, (b, a)) in before.iter().zip(goods(&w, civ)).enumerate() {
        assert!((a - b).abs() < 1e-6, "good {f}: {b} -> {a}");
    }
    // The civ's basket came back: more than the 5 it kept is in the store only if it traded
    // goods back, but at least what it kept is.
    assert!(total(&c.store) >= 5.0 - 1e-9);
}

#[test]
fn the_pool_is_at_most_the_gift_and_at_most_the_store_beyond_keep() {
    let (mut w, civ, mid) = scene();
    let a = visitor(&mut w, mid, at_big(-5, 1), 6.0, [0.0; N]);
    w.civs.get_mut(&civ).unwrap().keep = Some(5.0);
    w.civs.get_mut(&civ).unwrap().gift = 3.0;
    trade(&mut w);
    assert!((w.civs[&civ].given - 3.0).abs() < 1e-9, "the gift");
    assert!((total(&w.clans[&a].basket) - 3.0).abs() < 1e-6);
    // Spare 1 beyond keep: gives only 1.
    let (mut w, civ, mid) = scene();
    visitor(&mut w, mid, at_big(-5, 1), 6.0, [0.0; N]);
    w.civs.get_mut(&civ).unwrap().keep = Some(19.0);
    w.civs.get_mut(&civ).unwrap().gift = 3.0;
    trade(&mut w);
    let c = &w.civs[&civ];
    assert!((c.given - 1.0).abs() < 1e-9 && (c.gifted - 1.0).abs() < 1e-9);
    assert_eq!(c.posted, 0.0);
}

#[test]
fn the_gift_is_split_by_people_weighted_to_the_starving() {
    let (mut w, civ, mid) = scene();
    let a = visitor(&mut w, mid, at_big(-5, 1), 4.0, [0.0; N]);
    let b = visitor(&mut w, mid, at_big(-4, 1), 8.0, [0.0; N]);
    let s = visitor(&mut w, mid, at_big(-3, 1), 4.0, [0.0; N]);
    w.clans.get_mut(&s).unwrap().starving = true;
    // An aligned clan at the market is given nothing.
    let mine = visitor(&mut w, mid, at_big(-2, 1), 4.0, [0.0; N]);
    w.clans.get_mut(&mine).unwrap().civ = Some(civ);
    let c = w.civs.get_mut(&civ).unwrap();
    c.keep = Some(0.0);
    c.gift = 10.0;
    trade(&mut w);
    let weights = [4.0, 8.0, 4.0 * LEANING.starving];
    let sum: f64 = weights.iter().sum();
    for (id, wt) in [a, b, s].into_iter().zip(weights) {
        let got = total(&w.clans[&id].basket);
        assert!((got - 10.0 * wt / sum).abs() < 1e-6, "clan {id}: {got}");
    }
    assert_eq!(total(&w.clans[&mine].basket), 0.0);
    assert!(w.clans[&mine].leaning.is_empty(), "a civ clan never leans");
}

#[test]
fn a_gift_with_nobody_to_take_it_is_not_given() {
    let (mut w, civ, mid) = scene();
    let mine = visitor(&mut w, mid, at_big(-2, 1), 4.0, [0.0; N]);
    w.clans.get_mut(&mine).unwrap().civ = Some(civ);
    w.civs.get_mut(&civ).unwrap().keep = Some(0.0);
    trade(&mut w);
    assert_eq!(w.civs[&civ].given, 0.0);
}

#[test]
fn with_gifts_off_nothing_is_given() {
    let (mut w, civ, mid) = scene();
    let a = visitor(&mut w, mid, at_big(-5, 1), 6.0, [0.0; N]);
    w.config = Config { gifts: false, ..w.config };
    let c = w.civs.get_mut(&civ).unwrap();
    c.keep = Some(0.0);
    c.gift = 10.0;
    trade(&mut w);
    let c = &w.civs[&civ];
    assert_eq!((c.given, c.gifted), (0.0, 0.0));
    assert!(c.posted > 0.0, "the whole surplus is posted");
    assert!(w.clans[&a].leaning.get(&civ).is_none_or(|&l| l > 0.0), "only trade leaning, if any");
}

#[test]
fn a_starving_clan_leans_more_and_twice_the_gift_buys_less_than_twice_the_leaning() {
    assert!(gift_leaning(1.0, 6.0, true) > gift_leaning(1.0, 6.0, false));
    assert!((gift_leaning(1.0, 6.0, true) / gift_leaning(1.0, 6.0, false) - LEANING.starving).abs() < 1e-9);
    assert!(gift_leaning(2.0, 6.0, false) < 2.0 * gift_leaning(1.0, 6.0, false));
    assert!(gift_leaning(2.0, 6.0, false) > gift_leaning(1.0, 6.0, false));
    // The same through the market: two clans, one starving, the same food.
    let (mut w, civ, mid) = scene();
    let a = visitor(&mut w, mid, at_big(-5, 1), 6.0, [0.0; N]);
    let s = visitor(&mut w, mid, at_big(-4, 1), 6.0, [0.0; N]);
    w.clans.get_mut(&s).unwrap().starving = true;
    w.config = Config { gifts: true, ..w.config };
    let c = w.civs.get_mut(&civ).unwrap();
    c.keep = Some(0.0);
    c.gift = 2.0;
    trade(&mut w);
    assert!(w.clans[&s].leaning[&civ] > w.clans[&a].leaning[&civ]);
}

#[test]
fn trade_leaning_comes_only_at_the_civs_market_on_a_turn_it_posted() {
    let build = |keep: f64, own_market: bool| {
        let (mut w, civ, mid) = scene();
        w.config = Config { gifts: false, ..w.config };
        let other = open(&mut w, at_big(0, 5));
        let at = if own_market { mid } else { other };
        let a = visitor(&mut w, at, at_big(-5, 1), 6.0, [6.0, 0.0, 0.0, 0.0, 0.0]);
        w.civs.get_mut(&civ).unwrap().keep = Some(keep);
        trade(&mut w);
        (w.clans[&a].leaning.get(&civ).copied().unwrap_or(0.0), w.civs[&civ].posted)
    };
    let (lean_here, posted) = build(0.0, true);
    assert!(posted > 0.0 && lean_here > 0.0, "posted {posted}, leaning {lean_here}");
    let (lean_none, posted) = build(25.0, true);
    assert_eq!((lean_none, posted), (0.0, 0.0), "nothing posted: no leaning");
    let (lean_away, _) = build(0.0, false);
    assert_eq!(lean_away, 0.0, "another market");
}

#[test]
fn leanings_fade_and_those_under_the_floor_go() {
    let (mut w, civ, _) = scene();
    let a = put(&mut w, &[at_big(-5, 1)], 6.0);
    let other = *w.civs.keys().nth(1).unwrap();
    let c = w.clans.get_mut(&a).unwrap();
    c.leaning.insert(civ, 0.5);
    c.leaning.insert(other, LEANING.floor * 1.01);
    lean(&mut w);
    let c = &w.clans[&a];
    assert!((c.leaning[&civ] - 0.5 * (1.0 - LEANING.fade)).abs() < 1e-12);
    assert!(!c.leaning.contains_key(&other), "under the floor after fading");
    // Before the main game nothing changes.
    w.phase = Phase::Choose;
    let before = w.clans[&a].leaning.clone();
    lean(&mut w);
    assert_eq!(w.clans[&a].leaning, before);
}

#[test]
fn a_clan_joins_the_civ_it_leans_to_most_and_a_tie_goes_to_the_lower_id() {
    let (mut w, civ, _) = scene();
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    let strong = LEANING.join / (1.0 - LEANING.fade) * 1.001;
    let a = put(&mut w, &[at_big(-5, 1)], 6.0);
    let b = put(&mut w, &[at_big(-4, 1)], 6.0);
    let c = put(&mut w, &[at_big(-3, 1)], 6.0);
    let d = put(&mut w, &[at_big(-2, 1)], 6.0);
    // a: leans to two, the second stronger. b: a tie. c: under join. d: aligned already.
    w.clans.get_mut(&a).unwrap().leaning.extend([(ids[0], strong), (ids[2], strong * 1.5)]);
    w.clans.get_mut(&b).unwrap().leaning.extend([(ids[1], strong), (ids[2], strong)]);
    w.clans.get_mut(&c).unwrap().leaning.insert(ids[0], LEANING.join * 0.9);
    w.clans.get_mut(&d).unwrap().civ = Some(ids[1]);
    w.events.clear();
    lean(&mut w);
    assert_eq!(w.clans[&a].civ, Some(ids[2]));
    assert_eq!(w.clans[&b].civ, Some(ids[1]));
    assert_eq!(w.clans[&c].civ, None);
    assert!(w.clans[&c].leaning.contains_key(&ids[0]));
    assert!(w.clans[&a].leaning.is_empty() && w.clans[&b].leaning.is_empty(), "cleared on joining");
    assert_eq!((w.civs[&ids[2]].joined, w.civs[&ids[1]].joined, w.civs[&civ].joined), (1, 1, 0));
    let joined: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClanJoined).collect();
    assert_eq!(joined.len(), 2);
    assert_eq!((joined[0].clan, joined[0].civ, joined[0].tile), (Some(a), Some(ids[2]), w.clans[&a].home));
    assert_eq!(joined[0].text(&w), format!("Clan {a} joins {}", w.civs[&ids[2]].name));
    // It never leans again: it is a civ clan, and the step leaves it be.
    lean(&mut w);
    assert!(w.clans[&a].leaning.is_empty());
}

#[test]
fn a_daughter_takes_her_mothers_leanings() {
    {
        let mut w = flat();
        let round: Vec<usize> = std::iter::once(at(0, 0)).chain((0..6).map(|d| out(d, 1))).collect();
        let id = put(&mut w, &round, CLAN.split_at);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = [10.0, 10.0, 10.0, 0.0, 0.0];
        c.leaning.insert(1, 0.4);
        eat(&mut w);
        assert_eq!(w.clans.len(), 2, "divided");
        assert!(w.clans.values().all(|c| c.leaning.get(&1) == Some(&0.4)));

        let mut w = flat();
        w.tiles[out(1, 3)].stock = [20.0, 20.0, 20.0, 0.0, 0.0];
        let id = put(&mut w, &[at(0, 0)], 9.0);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = [3.6, 2.7, 0.0, 0.0, 0.0];
        c.leaning.insert(1, 0.4);
        eat(&mut w);
        assert_eq!(w.clans.len(), 2, "split");
        assert!(w.clans.values().all(|c| c.leaning.get(&1) == Some(&0.4)));
    }
}

#[test]
fn the_actions_are_the_leaders_and_replay() {
    let (mut w, civ, _) = scene();
    w.phase = Phase::Choose;
    let set = Action::SetGift { food: 3.0 };
    assert!(w.act(set.clone()).is_err(), "before the pick");
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    assert!(w.act(set.clone()).is_err(), "a watcher");
    assert!(w.act(Action::SetKeep { food: None }).is_err(), "a watcher");

    let (mut w, civ2, _) = scene();
    assert_eq!(civ, civ2);
    w.player = Some(civ);
    for bad in [-1.0, f64::NAN, f64::INFINITY, CIV.store_cap + 0.1] {
        assert!(w.act(Action::SetGift { food: bad }).is_err(), "gift {bad}");
    }
    for bad in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(w.act(Action::SetKeep { food: Some(bad) }).is_err(), "keep {bad}");
    }
    assert!(w.log.is_empty());
    w.act(Action::SetGift { food: CIV.store_cap }).unwrap();
    w.act(Action::SetKeep { food: Some(0.0) }).unwrap();
    assert_eq!((w.civs[&civ].gift, w.civs[&civ].keep), (CIV.store_cap, Some(0.0)));
    w.act(Action::SetKeep { food: None }).unwrap();
    assert_eq!(w.civs[&civ].keep, None);
    assert_eq!(serde_json::to_string(&Action::SetKeep { food: None }).unwrap(), r#"{"type":"setKeep","food":null}"#);
    assert_eq!(serde_json::to_string(&Action::SetGift { food: 2.0 }).unwrap(), r#"{"type":"setGift","food":2.0}"#);
}

#[test]
fn old_saves_load_with_gifts_on_and_the_actions_replay_to_the_same_world() {
    let old = r#"{"seed":1,"radius":16,"army":true}"#;
    assert!(serde_json::from_str::<Config>(old).unwrap().gifts);

    let mut w =
        World::new(Config { seed: 2, radius: engine::data::size_radius("small").unwrap(), ..Config::default() });
    while w.phase != Phase::Choose {
        w.run(1);
    }
    let civ = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(civ) }).unwrap();
    w.act(Action::SetGift { food: 6.0 }).unwrap();
    for n in 0..60 {
        if n == 20 {
            w.act(Action::SetKeep { food: Some(4.0) }).unwrap();
        }
        w.act(Action::EndTurn).unwrap();
    }
    let json = serde_json::to_string(&w.save()).unwrap();
    let again = World::replay(&serde_json::from_str(&json).unwrap()).unwrap();
    assert_eq!(w.civs, again.civs);
    assert_eq!(w.clans, again.clans);
}

#[test]
fn a_small_seed_runs_a_hundred_turns_past_the_pick_with_leanings_rising_and_a_forced_join() {
    let mut w =
        World::new(Config { seed: 2, radius: engine::data::size_radius("small").unwrap(), ..Config::default() });
    while w.phase != Phase::Choose {
        w.run(1);
    }
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    let pick = w.turn;
    let mut leaned = false;
    while w.turn < pick + 100 {
        w.run(1);
        leaned |= w.clans.values().any(|c| c.civ.is_none() && !c.leaning.is_empty());
        for c in w.clans.values().filter(|c| c.civ.is_some()) {
            assert!(c.leaning.is_empty(), "a civ clan never leans");
        }
    }
    let gifted: f64 = w.civs.values().map(|c| c.gifted).sum();
    let joined: u32 = w.civs.values().map(|c| c.joined).sum();
    assert!(leaned || joined > 0, "leanings rose");
    assert!(gifted > 0.0, "the civs gave");
    // Force a join if one is possible: the step does the rest.
    let civ = *w.civs.keys().next().unwrap();
    let Some(id) = w.clans.values().find(|c| c.civ.is_none()).map(|c| c.id) else { return };
    w.clans.get_mut(&id).unwrap().leaning.insert(civ, LEANING.join * 2.0);
    let before = w.civs[&civ].joined;
    w.events.clear();
    lean(&mut w);
    assert_eq!(w.clans[&id].civ, Some(civ));
    assert_eq!(w.civs[&civ].joined, before + 1);
    assert!(w.events.iter().any(|e| e.kind == EventKind::ClanJoined && e.clan == Some(id)));
}
