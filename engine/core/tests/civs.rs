mod common;

use common::*;
use engine::civs::{self, cards, Civ};
use engine::clans::eat;
use engine::data::*;
use engine::units::fight;
use engine::{Action, Config, EventKind, Order, Phase, Save, UnitKind, World};

/// A big world with three markets, ended by hand: the civs in `End.centres` order.
fn founded() -> (World, [u32; 3]) {
    let mut w = big();
    let ms = markets3(&mut w);
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    (w, [ids[0], ids[1], ids[2]])
}

fn civ_of(w: &World, clan: u32) -> Option<u32> {
    w.clans[&clan].civ
}

#[test]
fn founding_makes_a_civ_on_each_centre() {
    let mut w = big();
    let ms = markets3(&mut w);
    w.turn = 77;
    found(&mut w, &ms);
    assert_eq!(w.civs.len(), 3);
    let names: Vec<&str> = w.civs.values().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["Red", "Blue", "Gold"]);
    let colors: Vec<&str> = w.civs.values().map(|c| c.color.as_str()).collect();
    assert_eq!(colors, ["#d64545", "#3b6fd6", "#d6a63b"]);
    for (c, m) in w.civs.values().zip(ms) {
        assert_eq!((c.market, c.capital, c.founded), (m, w.markets[&m].tile, 77));
        assert_eq!(c.store, CIV.endowment);
        assert_eq!(c.lost, 0.0);
    }
    let founded: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::CivFounded).collect();
    assert_eq!(founded.len(), 3);
    for (e, c) in founded.iter().zip(w.civs.values()) {
        assert_eq!((e.civ, e.market, e.tile, e.turn), (Some(c.id), Some(c.market), c.capital, 77));
    }
    let h = w.tiles[w.civs.values().next().unwrap().capital].hex();
    assert_eq!(founded[0].text(&w), format!("Red is founded at ({}, {}), on market {}", h.q, h.r, ms[0]));
    assert_eq!((w.phase, w.player), (Phase::Choose, None));
}

#[test]
fn a_real_end_founds_and_waits_for_the_pick() {
    let mut w = big();
    markets3(&mut w);
    assert_eq!(w.phase, Phase::Pregame);
    w.turn = PREGAME.max_turns - 1;
    w.run(1);
    let end = w.end.clone().expect("ended");
    assert_eq!(w.phase, Phase::Choose);
    assert_eq!(w.civs.values().map(|c| c.market).collect::<Vec<_>>(), end.centres);
    assert!(w.civs.values().all(|c| c.founded == end.turn));
    assert_eq!(w.events.iter().filter(|e| e.kind == EventKind::CivFounded).count(), 3);
    assert!(w.act(Action::EndTurn).is_err());
    // Running on watches.
    w.run(1);
    assert_eq!((w.phase, w.player), (Phase::Main, None));
    assert_eq!(w.log, vec![Action::EndTurn, Action::ChooseCiv { civ: None }, Action::EndTurn]);
}

#[test]
fn fewer_than_three_centres_found_fewer_civs() {
    for n in 0..3 {
        let mut w = big();
        let ms = markets3(&mut w);
        let id = put(&mut w, &[at_big(-5, 0)], 6.0);
        found(&mut w, &ms[..n]);
        assert_eq!(w.civs.len(), n);
        assert_eq!(w.phase, Phase::Choose);
        assert_eq!(w.civs.values().map(|c| c.name.as_str()).collect::<Vec<_>>(), ["Red", "Blue"][..n]);
        assert_eq!(civ_of(&w, id), w.civs.keys().next().copied(), "the one clan joins the first civ if there is one");
        let mut v = World::new(Config { seed: 1, radius: 6, army: true, gifts: true, zone: true, ..Config::default() });
        v.clans.clear();
        found(&mut v, &[]);
        assert_eq!((v.phase, v.civs.len()), (Phase::Choose, 0));
        assert!(cards(&v).is_empty());
        v.run(2);
    }
}

#[test]
fn a_clan_joins_the_cheaper_market_and_a_tie_goes_to_the_lower_civ() {
    let mut w = big();
    let ms = markets3(&mut w);
    // Round tiles 2 steps from Red's market and 3 from Blue's, and the reverse, and 3 and 3.
    let red = put(&mut w, &[at_big(-3, 0), at_big(2, 0)], 6.0);
    let blue = put(&mut w, &[at_big(3, 0), at_big(-2, 0)], 6.0);
    // Round tiles 3 steps from each: a tie.
    let tie = put(&mut w, &[at_big(2, 1), at_big(-2, -1)], 6.0);
    let gold = put(&mut w, &[at_big(0, -8)], 6.0);
    // Out of every reach, but Gold has a clan, so it stays unaligned.
    let far = put(&mut w, &[at_big(0, 5)], 6.0);
    found(&mut w, &ms);
    let [r, b, g] = [0, 1, 2].map(|k| w.civs.keys().nth(k).copied());
    assert_eq!([civ_of(&w, red), civ_of(&w, blue), civ_of(&w, gold)], [r, b, g]);
    assert_eq!(civ_of(&w, tie), r, "ties to the lower civ");
    assert_eq!(civ_of(&w, far), None);
}

#[test]
fn a_civ_with_no_clan_takes_the_nearest_unaligned_one() {
    let mut w = big();
    let ms = markets3(&mut w);
    let red = put(&mut w, &[at_big(-4, 0)], 6.0);
    let blue = put(&mut w, &[at_big(4, 0)], 6.0);
    // Out of every reach; 15, 12 and 12 steps from Gold's capital.
    let far = put(&mut w, &[at_big(0, 5)], 6.0);
    let near = put(&mut w, &[at_big(0, 2)], 6.0);
    let also = put(&mut w, &[at_big(1, 1)], 6.0);
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    assert_eq!([civ_of(&w, red), civ_of(&w, blue)], [Some(ids[0]), Some(ids[1])]);
    assert_eq!(civ_of(&w, near), Some(ids[2]), "ties to the lower clan id");
    assert_eq!([civ_of(&w, also), civ_of(&w, far)], [None, None]);
}

#[test]
fn the_pick_and_the_phases() {
    let (w, ids) = founded();
    let choose = |civ| Action::ChooseCiv { civ };
    // Refused in the pre-game.
    let mut p = big();
    assert!(p.act(choose(None)).is_err());
    assert!(p.log.is_empty());
    // EndTurn is refused at the pick, and the world is unchanged.
    let mut v = w.clone();
    assert_eq!(v.act(Action::EndTurn), Err("choose a civ to lead, or watch".to_string()));
    assert_eq!(v, w);
    // Not a civ.
    assert!(v.act(choose(Some(999))).is_err());
    assert!(v.act(choose(Some(w.markets.keys().next().copied().unwrap()))).is_err());
    assert_eq!(v, w);
    v.act(choose(Some(ids[1]))).unwrap();
    assert_eq!((v.phase, v.player), (Phase::Main, Some(ids[1])));
    assert!(v.act(choose(Some(ids[1]))).is_err(), "refused in Main");
    assert!(v.act(choose(None)).is_err());
    v.act(Action::EndTurn).unwrap();
    let mut u = w.clone();
    u.act(choose(None)).unwrap();
    assert_eq!((u.phase, u.player), (Phase::Main, None));
    assert_eq!(serde_json::to_string(&choose(Some(12))).unwrap(), r#"{"type":"chooseCiv","civ":12}"#);
    assert_eq!(serde_json::to_string(&choose(None)).unwrap(), r#"{"type":"chooseCiv","civ":null}"#);
    assert_eq!(serde_json::to_string(&Phase::Choose).unwrap(), r#""choose""#);
}

#[test]
fn a_daughter_belongs_to_her_mothers_civ() {
    // A clan that divides its land, and one that splits a daughter off.
    for civ in [Some(7), None] {
        let mut w = flat();
        let round: Vec<usize> = std::iter::once(at(0, 0)).chain((0..6).map(|d| out(d, 1))).collect();
        let id = put(&mut w, &round, CLAN.split_at);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = [10.0, 10.0, 10.0, 0.0, 0.0];
        c.civ = civ;
        eat(&mut w);
        assert_eq!(w.clans.len(), 2, "divided");
        assert!(w.clans.values().all(|c| c.civ == civ));

        let mut w = flat();
        let rich = out(1, 3);
        w.tiles[rich].stock = [20.0, 20.0, 20.0, 0.0, 0.0];
        let id = put(&mut w, &[at(0, 0)], 9.0);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = [3.6, 2.7, 0.0, 0.0, 0.0];
        c.civ = civ;
        eat(&mut w);
        assert_eq!(w.clans.len(), 2, "split");
        assert!(w.clans.values().all(|c| c.civ == civ));
    }
}

#[test]
fn the_store_spoils_and_is_capped() {
    let (mut w, ids) = founded();
    let red = ids[0];
    civs::upkeep(&mut w);
    let kept = CIV.endowment[GRAIN] * (1.0 - CLAN.spoilage);
    assert!((w.civs[&red].store[GRAIN] - kept).abs() < 1e-9);
    assert!((w.civs[&ids[2]].store[GRAIN] - kept).abs() < 1e-9);

    // The upkeep no longer levies: a clan's store stays its own.
    let fed = put(&mut w, &[at_big(-4, 0)], 6.0);
    {
        let c = w.clans.get_mut(&fed).unwrap();
        (c.civ, c.fed, c.store) = (Some(red), true, [4.0, 2.0, 0.0, 6.0, 0.0]);
    }
    w.civs.get_mut(&red).unwrap().store = [0.0, 2.0 * CIV.store_cap, CIV.store_cap, 0.0, 0.0];
    civs::upkeep(&mut w);
    assert_eq!(w.clans[&fed].store, [4.0, 2.0, 0.0, 6.0, 0.0]);

    // Past the cap, every good scales down so the store holds the cap.
    let c = &w.civs[&red];
    let total: f64 = c.store.iter().sum();
    assert!((total - CIV.store_cap).abs() < 1e-9, "{total}");
    // Three caps spoil, and the rest over the cap is lost.
    let over = 3.0 * CIV.store_cap * (1.0 - CLAN.spoilage) - CIV.store_cap;
    assert!((c.lost - over).abs() < 1e-9, "{}", c.lost);
    assert!((c.store[GRAIN] / c.store[FRUIT] - 2.0).abs() < 1e-9);
    civs::upkeep(&mut w);
    assert_eq!(w.civs[&red].lost, 0.0, "nothing lost once under the cap");
}

#[test]
fn a_cut_store_says_how_much_was_lost() {
    let (mut w, ids) = founded();
    w.civs.get_mut(&ids[0]).unwrap().store = [0.0, 3.0 * CIV.store_cap, 0.0, 0.0, 0.0];
    civs::upkeep(&mut w);
    let c = &w.civs[&ids[0]];
    assert!((c.store[GRAIN] - CIV.store_cap).abs() < 1e-9);
    assert!((c.lost - (3.0 * CIV.store_cap * (1.0 - CLAN.spoilage) - CIV.store_cap)).abs() < 1e-9);
}

#[test]
fn a_civs_market_never_fades() {
    let (mut w, _) = founded();
    let mids: Vec<u32> = w.civs.values().map(|c| c.market).collect();
    let doomed = open(&mut w, at_big(0, 8));
    for m in w.markets.values_mut() {
        m.activity = 0.0;
    }
    w.run(6);
    assert!(!w.markets.contains_key(&doomed));
    for m in mids {
        assert!(w.markets.contains_key(&m));
    }
}

#[test]
fn a_card_counts_clans_people_hearths_terrain_and_rivals() {
    let mut w = big();
    // Red's site is its bigger clan's home, as the pre-game leaves it.
    let ms = [open(&mut w, at_big(-4, 1)), open(&mut w, at_big(7, 0)), open(&mut w, at_big(0, -10))];
    // Red's land (round the capital at (-4, 1)): the 19 tiles round it: 15 plains, 2 forest, 2 hills.
    for (t, k) in [(at_big(-5, 1), FOREST), (at_big(-6, 1), FOREST), (at_big(-4, -1), HILLS), (at_big(-4, 0), HILLS)] {
        w.tiles[t].terrain = k;
    }
    w.tiles[at_big(-6, 0)].terrain = DESERT;
    hearth(&mut w, at_big(-5, -1), 3.0);
    hearth(&mut w, at_big(-5, 2), 3.0);
    hearth(&mut w, at_big(-1, 0), 3.0);
    let a = put(&mut w, &[at_big(-4, 1)], 12.0);
    let b = put(&mut w, &[at_big(-1, 1)], 2.5);
    found(&mut w, &ms);
    // Red's land is the 19 tiles within two steps of its capital, held by its guard.
    let ring: Vec<usize> =
        w.tiles[at_big(-4, 1)].hex().within(2).into_iter().filter_map(|h| engine::map::index(BIG, h)).collect();
    let red_id = *w.civs.keys().next().unwrap();
    hold_land(&mut w, red_id, &ring);
    assert_eq!([civ_of(&w, a), civ_of(&w, b)], [w.civs.keys().next().copied(); 2]);
    let cs = cards(&w);
    assert_eq!(cs.len(), 3);
    let red = &cs[0];
    // The bigger clan is the civ, its home the capital.
    assert_eq!((red.name.as_str(), red.capital, red.q, red.r), ("Red", at_big(-4, 1), -4, 1));
    assert_eq!(w.civs.values().next().unwrap().clan, a);
    // 14.5 people, less the two the scout took from the clans and the three the guard `hold_land` raised.
    assert_eq!((red.clans, red.people, red.hearths), (2, 9.5, 1));
    let names: Vec<&str> = red.terrain.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["plains", "forest", "hills"], "ties by terrain index");
    // Of the 19 land tiles: 15 plains, 2 forest, 2 hills (the clan's tile beyond is not Red's land now).
    assert!((red.terrain[0].1 - 15.0 / 19.0).abs() < 1e-12 && (red.terrain[1].1 - 2.0 / 19.0).abs() < 1e-12);
    assert_eq!([cs[0].rival_steps, cs[1].rival_steps, cs[2].rival_steps], [Some(11), Some(11), Some(11)]);
    assert_eq!((cs[1].clans, cs[1].people, cs[1].hearths), (0, 0.0, 0));
    assert!(cs.iter().all(|c| c.rival_by_land));
    let json = serde_json::to_value(red).unwrap();
    assert_eq!(json["rival_steps"], 11);
    assert_eq!(json["rival_by_land"], true);
}

#[test]
fn a_rival_across_the_sea_is_not_by_land() {
    let mut w = big();
    for t in &mut w.tiles {
        if t.q == 0 {
            t.terrain = SEA;
        }
    }
    let ms = [open(&mut w, at_big(-5, 0)), open(&mut w, at_big(5, 0)), open(&mut w, at_big(3, -10))];
    found(&mut w, &ms);
    let by_land: Vec<bool> = cards(&w).iter().map(|c| c.rival_by_land).collect();
    assert_eq!(by_land, [false, true, true]);
}

#[test]
fn watching_and_leading_replay_to_the_same_world() {
    let mut w = World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true, ..Config::default() });
    while w.end.is_none() {
        w.run(1);
    }
    assert!(w.end.is_some());
    let first = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(first) }).unwrap();
    w.run(20);
    let json = serde_json::to_string(&w.save()).unwrap();
    let save: Save = serde_json::from_str(&json).unwrap();
    let r = World::replay(&save).unwrap();
    assert_eq!(r, w);
    assert_eq!(r.events, w.events);
    assert_eq!(r.player, Some(first));
    // The world's JSON carries the phase, the player and the civs.
    let v = serde_json::to_value(&w).unwrap();
    assert_eq!(v["phase"], "main");
    assert_eq!(v["player"], first);
    assert!(v["civs"][first.to_string()]["market"].is_u64());
    assert!(v["clans"].as_object().unwrap().values().all(|c| c.get("civ").is_some()));
}

#[test]
fn a_real_map_founds_three_civs_that_live_on() {
    let mut w = World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true, ..Config::default() });
    while w.end.is_none() {
        w.run(1);
    }
    let end = w.end.clone().unwrap();
    assert_eq!((w.phase, w.player), (Phase::Choose, None));
    assert_eq!(w.civs.len(), 3);
    assert_eq!(w.civs.values().map(|c| c.market).collect::<Vec<_>>(), end.centres);
    for c in w.civs.values() {
        assert_eq!(c.capital, w.markets[&c.market].tile);
    }
    for c in w.clans.values() {
        assert!(c.civ.is_none_or(|id| w.civs.contains_key(&id)));
    }
    let unaligned = w.clans.values().filter(|c| c.civ.is_none()).count();
    for civ in w.civs.keys() {
        let n = w.clans.values().filter(|c| c.civ == Some(*civ)).count();
        assert!(n > 0 || unaligned == 0, "civ {civ} has no clan while {unaligned} are unaligned");
    }
    assert_eq!(cards(&w).len(), 3);

    let first = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(first) }).unwrap();
    for _ in 0..100 {
        w.act(Action::EndTurn).unwrap();
        for c in w.civs.values() {
            assert!(c.store.iter().sum::<f64>() <= CIV.store_cap + 1e-9);
            assert!(w.markets.contains_key(&c.market));
        }
        for c in w.clans.values() {
            assert!(c.civ.is_none_or(|id| w.civs.contains_key(&id)));
        }
    }
    assert!(w.civs.values().any(|c| c.store.iter().sum::<f64>() > 0.0));
}

#[test]
fn at_founding_a_clan_as_close_to_a_capital_as_its_festival_switches_to_it() {
    let mut w = big();
    let ms = markets3(&mut w);
    // A busy festival two steps east of Red's capital at (-5, 0).
    let fest = open(&mut w, at_big(-3, 0));
    w.markets.get_mut(&fest).unwrap().traders = 6;
    let own = put(&mut w, &[at_big(-5, 0)], 10.0);
    // One step from each: it switches. One step from the festival and three from the capital: it stays.
    let tie = put(&mut w, &[at_big(-4, 0)], 3.0);
    let near = put(&mut w, &[at_big(-2, 0)], 3.0);
    for id in [tie, near] {
        w.clans.get_mut(&id).unwrap().market = Some(fest);
    }
    found(&mut w, &ms);
    let red = w.civs.values().next().unwrap();
    assert_eq!((red.clan, red.capital), (own, at_big(-5, 0)));
    assert_eq!(w.clans[&tie].market, Some(red.market));
    assert_eq!(w.clans[&tie].routes[0].to, red.capital);
    assert_eq!(w.clans[&near].market, Some(fest));
    // Traders are counted again as the clans now stand.
    assert_eq!((w.markets[&red.market].traders, w.markets[&fest].traders), (2, 1));
}

#[test]
fn a_civ_serialises_its_store_where_it_always_was_and_reads_back() {
    const KEYS: [&str; 22] = [
        "id", "name", "color", "market", "capital", "founded", "store", "lost", "clan", "army", "keep", "gift",
        "posted", "given", "joined", "gifted", "revealed", "claim", "guard", "pressed", "besieged", "fallen",
    ];
    let (w, ids) = founded();
    let civ = &w.civs[&ids[0]];
    let json = serde_json::to_string(civ).unwrap();
    assert_eq!(keys_in_order(&json), KEYS.map(String::from), "{json}");
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["store"], serde_json::to_value(civ.store).unwrap());
    // The actor is read from the capital (its home) and the store, and it writes back what it read.
    let back: Civ = serde_json::from_str(&json).unwrap();
    assert_eq!((back.home, back.store), (civ.capital, civ.store));
    assert_eq!(serde_json::to_string(&back).unwrap(), json);
}

/// Red's army of `people` on Blue's capital, Blue's own camp taken away: a rival on the capital with
/// nobody of Blue's there but its clan.
fn invade_capital(w: &mut World, red: u32, blue: u32, people: f64) -> usize {
    let cap = w.civs[&blue].capital;
    let own = camp_id(w, blue);
    w.units.remove(&own);
    let r = put_camp(w, red, cap);
    let u = camp_mut(w, r);
    u.actor.people = people;
    u.actor.store = [0.0; N];
    let c = u.camp_mut().unwrap();
    c.stack = [people, 0.0];
    c.dest = None;
    w.civs.get_mut(&blue).unwrap().store = CIV.endowment;
    cap
}

#[test]
fn a_capital_a_rival_army_stands_on_with_no_army_of_its_own_falls_and_its_civ_is_out() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = invade_capital(&mut w, red, blue, 5.0);
    // Its clan there, however many, is no defence.
    let bc = w.civs[&blue].clan;
    w.clans.get_mut(&bc).unwrap().people = 14.0;
    let before = total_people(&w);
    let holders = fight::army_holders(&w);
    fight::step(&mut w, &holders);
    let fallen: f64 = w.civs.values().map(|c| c.army.fallen).sum();
    assert!(w.civs[&blue].is_out(), "its capital fell");
    assert_eq!(w.civs[&blue].fallen, Some((w.turn, red)));
    assert!((before - total_people(&w) - fallen).abs() < 1e-9, "people are conserved");
    assert!(w.clans.values().all(|k| k.civ != Some(blue)), "its clans are unaligned");
    assert!(w.clans.values().filter(|k| k.home == cap).all(|k| k.leaning.is_empty()));
    assert!(w.civs[&blue].claim.is_empty());
    assert_ne!(engine::claim::tile_civ(&w, cap), Some(blue));
    assert_eq!(engine::stats::standing(&w)[&blue], 0.0);
    assert!(w.units.values().all(|u| u.civ != blue), "its units are gone");
    assert!(w.civs[&red].army.taken == 0, "a fall is not a tile taken");
    assert!(w.units[&camp_id(&w, red)].store.iter().sum::<f64>() > 0.0, "its store fills the taker's pack");
    assert_eq!(w.civs[&blue].store, [0.0; N], "the rest is lost");
    assert_eq!(kinds(&w, EventKind::CapitalFell), 1);
    assert!(
        w.notes[&blue].iter().any(|n| n.text.contains("fell")) && w.notes[&red].iter().any(|n| n.text.contains("took"))
    );
    // Its orders are refused, and the game goes on.
    let refused =
        w.check_order(blue, &Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None });
    assert_eq!(refused, Err("your civ has fallen".to_string()));
    end_turn(&mut w);
    end_turn(&mut w);
}

#[test]
fn a_capital_its_own_army_stands_on_does_not_fall() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&blue].capital;
    let own = camp_id(&w, blue);
    assert_eq!(w.units[&own].home, cap);
    let r = put_camp(&mut w, red, cap);
    let u = camp_mut(&mut w, r);
    u.actor.people = 1.0;
    u.camp_mut().unwrap().stack = [1.0, 0.0];
    let holders = fight::army_holders(&w);
    fight::step(&mut w, &holders);
    assert!(!w.civs[&blue].is_out(), "its army is on the capital");
    assert_eq!(engine::claim::tile_civ(&w, cap), Some(blue));
}

#[test]
fn a_fallen_civs_units_go_to_the_nearest_clan_and_a_rival_pack_is_filled() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = invade_capital(&mut w, red, blue, 5.0);
    let r = camp_id(&w, red);
    let scout = led_scout_of(&w, blue);
    let before = total_people(&w);
    civs::fall(&mut w, blue, red);
    assert!(!w.units.contains_key(&scout), "its scout disbanded");
    assert!((before - total_people(&w)).abs() < 1e-9, "people are conserved");
    assert!(w.units[&r].store.iter().sum::<f64>() > 0.0, "the taker's army takes the store");
    assert_eq!(w.units[&r].home, cap);
}

fn led_scout_of(w: &World, civ: u32) -> u32 {
    engine::units::scout(w, civ).expect("a scout").id
}

fn total_people(w: &World) -> f64 {
    w.clans.values().map(|c| c.people).sum::<f64>() + w.units.values().map(|u| u.people).sum::<f64>()
}

#[test]
fn a_fallen_capitals_market_fades_like_any_festival() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let mid = w.civs[&blue].market;
    invade_capital(&mut w, red, blue, 5.0);
    w.markets.get_mut(&mid).unwrap().activity = 0.0;
    engine::markets::fade(&mut w);
    assert!(w.markets.contains_key(&mid), "a civ's market is kept while it stands");
    let holders = fight::army_holders(&w);
    fight::step(&mut w, &holders);
    assert!(w.civs[&blue].is_out());
    engine::markets::fade(&mut w);
    assert!(!w.markets.contains_key(&mid), "no longer kept: it fades like any festival");
}
