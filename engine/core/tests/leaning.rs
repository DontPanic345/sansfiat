mod common;

use common::*;
use engine::clans::eat;
use engine::data::*;
use engine::events::By;
use engine::leaning::{gift_leaning, lean};
use engine::markets::trade;
use engine::{civs, claim, stats, units, Action, Basket, Config, End, EventKind, Order, Phase, World};

/// Three civs founded by hand on the big world, in `Phase::Main`; the first civ's id and market.
/// No clans, so no camp; the first civ's store holds the endowment (20 of good 1).
fn scene() -> (World, u32, u32) {
    let mut w = big();
    let ms = [open(&mut w, at_big(-5, 0)), open(&mut w, at_big(5, 0)), open(&mut w, at_big(0, -10))];
    w.end = Some(End {
        turn: 0,
        centres: ms.to_vec(),
        clans: Vec::new(),
        cover: 1.0,
        by_limit: false,
        topped_up: Vec::new(),
    });
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

/// Every good held by the civ's store and the clans' baskets.
fn goods(w: &World, civ: u32) -> Basket {
    std::array::from_fn(|f| w.civs[&civ].store[f] + w.clans.values().map(|c| c.basket[f]).sum::<f64>())
}

#[test]
fn the_default_keep_is_the_camps_full_pack() {
    let (mut w, civ, _) = scene();
    // No camp: a new camp's pack.
    assert_eq!(civs::keep(&w, civ), units::pack_cap(f64::from(UNIT.first)));
    // A camp: its pack to fill.
    for k in 0..4 {
        let t = at_big(-5 + k, 1);
        let id = put(&mut w, &[t], 6.0);
        let market = Some(w.civs[&civ].market);
        let k = w.clans.get_mut(&id).unwrap();
        k.civ = Some(civ);
        k.market = market;
    }
    units::raise(&mut w, civ, UNIT.first, Fighter::Warriors, None, None).unwrap();
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
    let a = visitor(&mut w, mid, at_big(-5, 2), 6.0, [4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
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
    visitor(&mut w, mid, at_big(-5, 2), 6.0, [6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    visitor(&mut w, mid, at_big(-4, 1), 6.0, [0.0, 0.0, 6.0, 0.0, 0.0, 0.0, 0.0]);
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
        let a = visitor(&mut w, at, at_big(-5, 1), 6.0, [6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
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
    let a = put(&mut w, &[at_big(-5, 2)], 6.0);
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
    let a = put(&mut w, &[at_big(-5, 3)], 6.0);
    let b = put(&mut w, &[at_big(-4, 3)], 6.0);
    let c = put(&mut w, &[at_big(-3, 3)], 6.0);
    let d = put(&mut w, &[at_big(-2, 3)], 6.0);
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
    // A joined clan keeps only its leaning to its civ, at the join line (`lean`).
    for k in [a, b] {
        assert_eq!(w.clans[&k].leaning.len(), 1, "clan {k}: its other leanings are cleared on joining");
        assert_eq!(w.clans[&k].leaning.get(&w.clans[&k].civ.unwrap()), Some(&LEANING.join));
    }
    assert_eq!((w.civs[&ids[2]].joined, w.civs[&ids[1]].joined, w.civs[&civ].joined), (1, 1, 0));
    let joined: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClanJoined).collect();
    assert_eq!(joined.len(), 2);
    assert_eq!((joined[0].clan, joined[0].civ, joined[0].tile), (Some(a), Some(ids[2]), w.clans[&a].home));
    let name = &w.civs[&ids[2]].name;
    assert_eq!(joined[0].text(&w), format!("Clan {a} joins {name}: won by trade at {name}'s market"));
    // It stays with its civ, and as a civ clan its leaning to it only fades (`lean`).
    lean(&mut w);
    assert_eq!(w.clans[&a].civ, Some(ids[2]));
    assert!(w.clans[&a].leaning[&ids[2]] < LEANING.join, "faded from the join line");
}

#[test]
fn a_daughter_takes_her_mothers_leanings() {
    {
        let mut w = flat();
        let round: Vec<usize> = std::iter::once(at(0, 0)).chain((0..6).map(|d| out(d, 1))).collect();
        let id = put(&mut w, &round, CLAN.split_at);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = [10.0, 10.0, 10.0, 0.0, 0.0, 0.0, 0.0];
        c.leaning.insert(1, 0.4);
        eat(&mut w);
        assert_eq!(w.clans.len(), 2, "divided");
        assert!(w.clans.values().all(|c| c.leaning.get(&1) == Some(&0.4)));

        let mut w = flat();
        w.tiles[out(1, 3)].stock = [20.0, 20.0, 20.0, 0.0, 0.0, 0.0, 0.0];
        let id = put(&mut w, &[at(0, 0)], 9.0);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = [3.6, 2.7, 0.0, 0.0, 0.0, 0.0, 0.0];
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
    let mut civ_leaned = false;
    while w.turn < pick + 100 {
        w.run(1);
        civ_leaned |= w.clans.values().any(|c| c.civ.is_some() && !c.leaning.is_empty());
    }
    // Every clan leans, a civ's own clans included: one on its civ's land leans to it (`lean`).
    assert!(civ_leaned, "a civ clan leans too");
    // No gift on this seed: a civ keeps its army's food back from gifts (`civs::keep`), and its store
    // is short, so there is seldom food over the keep to give. The rest of the check is the forced join.
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

#[test]
fn a_home_on_a_civs_land_makes_the_clan_that_civs_the_same_turn() {
    let (mut w, civ, _) = scene();
    let (on, off) = (at_big(-5, 1), at_big(-4, 2));
    hold_land(&mut w, civ, &[on]);
    let a = put(&mut w, &[on], 6.0);
    let b = put(&mut w, &[off], 6.0);
    lean(&mut w);
    assert_eq!(w.clans[&a].civ, Some(civ), "on its land: the civ's, with no leaning to wait for");
    assert_eq!(w.clans[&a].leaning.get(&civ), Some(&LEANING.join));
    assert_eq!(w.clans[&b].civ, None, "off its land: unaligned, and leaning to nobody");
    assert!(w.clans[&b].leaning.is_empty());
    let joined: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClanJoined).collect();
    assert_eq!(joined.len(), 1);
    assert_eq!((joined[0].clan, joined[0].civ, joined[0].by), (Some(a), Some(civ), Some(By::Land)));
    let name = w.civs[&civ].name.clone();
    assert_eq!(joined[0].text(&w), format!("Clan {a} joins {name}: its home is on {name}'s land"));
    assert_eq!(w.civs[&civ].joined, 1);
}

#[test]
fn a_claim_order_over_a_clans_home_makes_it_the_civs_at_once_and_it_pays_the_levy() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let (cap, mid) = (w.civs[&red].capital, w.civs[&red].market);
    let home = at_big(-3, 0);
    let k = visitor(&mut w, mid, home, 6.0, [4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let before = stats::standing(&w)[&red];
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    civs::levy(&mut w, mid);
    assert_eq!(w.clans[&k].basket[0], 4.0, "an unaligned clan off the land pays nothing");
    w.act(led_order(&w, Order::Claim { tiles: vec![cap, at_big(-4, 0), home] })).unwrap();
    assert_eq!(w.clans[&k].civ, Some(red), "the order alone makes it Red's");
    assert!(
        stats::standing(&w)[&red] > before,
        "and it counts to Red's Standing: {} from {before}, claim {:?}",
        stats::standing(&w)[&red],
        w.civs[&red].claim
    );
    civs::levy(&mut w, mid);
    assert!((w.clans[&k].basket[0] - 4.0 * (1.0 - CIV.levy)).abs() < 1e-9, "it pays the levy");
    assert!(units::givers(&w, red).contains(&k), "and Red can raise from it");
}

#[test]
fn a_clan_whose_home_tile_is_taken_changes_civ_with_the_tile() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    hold_land(&mut w, blue, &(-2..=4).map(|q| at_big(q, 0)).collect::<Vec<_>>());
    w.order(red, Order::Claim { tiles: vec![at_big(-4, 0), at_big(-3, 0)] }).unwrap();
    let k = put(&mut w, &[at_big(-3, 0)], 6.0);
    lean(&mut w);
    assert_eq!(w.clans[&k].civ, Some(red));
    put_camp(&mut w, blue, at_big(-3, 0));
    w.events.clear();
    claim::step(&mut w);
    assert_eq!(claim::tile_civ(&w, at_big(-3, 0)), Some(blue), "the tile joins Blue's land");
    assert_eq!(w.clans[&k].civ, Some(blue), "and the clan is Blue's with it");
    let left: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClanLeft).collect();
    assert_eq!(left.len(), 1);
    assert_eq!((left[0].civ, left[0].other, left[0].by), (Some(red), Some(blue), Some(By::Land)));
    let (old, new) = (w.civs[&red].name.clone(), w.civs[&blue].name.clone());
    assert_eq!(left[0].text(&w), format!("Clan {k} leaves {old} for {new}: its home is on {new}'s land"));
}

#[test]
fn a_clan_on_its_civs_land_never_leaves_and_one_whose_tile_is_nobodys_stays_until_it_leans_elsewhere() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&red].capital;
    let home = at_big(-4, 0);
    w.order(red, Order::Claim { tiles: vec![home] }).unwrap();
    let k = put(&mut w, &[home], 6.0);
    lean(&mut w);
    assert_eq!(w.clans[&k].civ, Some(red));
    // Drawn hard to Blue, on Red's land: it stays.
    for _ in 0..(LEANING.leave_run * 2) {
        w.clans.get_mut(&k).unwrap().leaning.insert(blue, 5.0);
        lean(&mut w);
    }
    assert_eq!((w.clans[&k].civ, w.clans[&k].drawn), (Some(red), 0), "its home is on Red's land");
    // Red drops the tile: the clan is still Red's.
    w.clans.get_mut(&k).unwrap().leaning.remove(&blue);
    w.order(red, Order::Claim { tiles: vec![cap] }).unwrap();
    lean(&mut w);
    assert_eq!(w.clans[&k].civ, Some(red), "off the land it stays until it leans elsewhere");
    // Blue's gifts draw it: after `leave_run` turns it leaves, won by gifts.
    w.events.clear();
    for _ in 0..LEANING.leave_run {
        let c = w.clans.get_mut(&k).unwrap();
        c.leaning.insert(blue, 5.0);
        c.gifts.insert(blue, 4.0);
        lean(&mut w);
    }
    assert_eq!(w.clans[&k].civ, Some(blue));
    let left: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClanLeft).collect();
    assert_eq!(left[0].by, Some(By::Gift));
    let (old, new) = (w.civs[&red].name.clone(), w.civs[&blue].name.clone());
    assert_eq!(left[0].text(&w), format!("Clan {k} leaves {old} for {new}: won by {new}'s gifts"));
}

#[test]
fn a_clan_on_nobodys_land_joins_by_its_leaning_and_the_feed_says_gift_or_trade() {
    let (mut w, civ, _) = scene();
    let name = w.civs[&civ].name.clone();
    let strong = LEANING.join / (1.0 - LEANING.fade) * 1.001;
    let a = put(&mut w, &[at_big(-4, 2)], 6.0);
    let b = put(&mut w, &[at_big(-3, 3)], 6.0);
    w.clans.get_mut(&a).unwrap().leaning.insert(civ, strong);
    w.clans.get_mut(&a).unwrap().gifts.insert(civ, strong * 0.8);
    w.clans.get_mut(&b).unwrap().leaning.insert(civ, strong);
    w.clans.get_mut(&b).unwrap().gifts.insert(civ, strong * 0.2);
    lean(&mut w);
    let joined: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClanJoined).collect();
    assert_eq!(
        joined.iter().map(|e| (e.clan, e.by)).collect::<Vec<_>>(),
        vec![(Some(a), Some(By::Gift)), (Some(b), Some(By::Trade))]
    );
    assert_eq!(joined[0].text(&w), format!("Clan {a} joins {name}: won by {name}'s gifts"));
    assert_eq!(joined[1].text(&w), format!("Clan {b} joins {name}: won by trade at {name}'s market"));
    assert!(w.clans[&a].gifts.is_empty(), "a joined clan's gift parts are cleared");
}

#[test]
fn a_gift_adds_to_the_gift_part_of_a_leaning_and_trade_does_not() {
    let (mut w, civ, mid) = scene();
    let a = visitor(&mut w, mid, at_big(-4, 2), 6.0, [0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    trade(&mut w);
    let c = &w.clans[&a];
    let (lean_to, gift) = (c.leaning.get(&civ).copied().unwrap_or(0.0), c.gifts.get(&civ).copied().unwrap_or(0.0));
    assert!(gift > 0.0 && gift <= lean_to + 1e-12, "gift part {gift} of leaning {lean_to}");
    lean(&mut w);
    let c = &w.clans[&a];
    if c.civ.is_none() {
        assert!((c.gifts[&civ] - gift * (1.0 - LEANING.fade)).abs() < 1e-12, "the gift part fades with the leaning");
    }
}

#[test]
fn a_civ_clan_off_its_land_drawn_to_a_rival_for_leave_run_turns_leaves_with_its_daughter_and_its_own_clan_does_not() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    // A Blue clan on nobody's land, drawn to Red.
    let k = put(&mut w, &[at_big(-2, 3)], 6.0);
    w.clans.get_mut(&k).unwrap().civ = Some(blue);
    w.clans.get_mut(&k).unwrap().leaning = [(red, 1.0)].into_iter().collect();
    // Its daughter, far off, leaves with it.
    let d = put(&mut w, &[at_big(-8, 3)], 2.0);
    {
        let c = w.clans.get_mut(&d).unwrap();
        c.civ = Some(blue);
        c.parent = Some(k);
    }
    // Blue's own clan is drawn to Red too, and never leaves.
    let own = w.civs[&blue].clan;
    w.clans.get_mut(&own).unwrap().leaning = [(red, 1.0)].into_iter().collect();
    w.events.clear();
    for turn in 1..=LEANING.leave_run {
        w.clans.get_mut(&k).unwrap().leaning.insert(red, 1.0);
        lean(&mut w);
        if turn < LEANING.leave_run {
            assert_eq!(w.clans[&k].civ, Some(blue), "not before {} turns in a row", LEANING.leave_run);
        }
    }
    assert_eq!(w.clans[&k].civ, Some(red), "the clan left for Red");
    assert_eq!(w.clans[&d].civ, Some(red), "and its daughter went with it");
    assert_eq!(w.clans[&own].civ, Some(blue), "a civ's own clan never leaves");
    let left: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClanLeft).collect();
    assert_eq!(left.len(), 2, "one event for each clan that left");
    assert!(left.iter().all(|e| e.civ == Some(blue) && e.other == Some(red) && e.by == Some(By::Trade)));
    assert_eq!(w.civs[&red].joined, 2, "both joined Red");
}

#[test]
fn a_rival_leaning_under_leave_at_never_draws_a_clan_away() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let k = put(&mut w, &[at_big(-2, 3)], 6.0);
    w.clans.get_mut(&k).unwrap().civ = Some(blue);
    w.clans.get_mut(&k).unwrap().leaning = [(red, LEANING.leave_at / 2.0)].into_iter().collect();
    for _ in 0..(LEANING.leave_run * 3) {
        lean(&mut w);
    }
    assert_eq!(w.clans[&k].civ, Some(blue), "a crumb of leaning is not drawn");
    assert_eq!(w.clans[&k].drawn, 0);
}
