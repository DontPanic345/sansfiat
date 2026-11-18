//! Exchange (`exchange.rs`): a unit on a market's tile joins its clearing with its pack's wares and wants
//! food; a market that is a rival civ's takes none.

mod common;

use common::*;
use engine::data::*;
use engine::lots::Lots;
use engine::markets::trade;
use engine::World;

/// Three civs with armies (`three`), a festival on a free tile, and two unaligned clans there with food to
/// spare, which want salt. Returns the world, the civs, and the festival's market and tile.
fn scene() -> (World, [u32; 3], u32, usize) {
    let (mut w, civs, _) = three();
    let tile = at_big(0, 3);
    let mid = open(&mut w, tile);
    for t in [at_big(0, 4), at_big(1, 3)] {
        let id = put(&mut w, &[t], 4.0);
        let c = w.clans.get_mut(&id).unwrap();
        c.market = Some(mid);
        c.basket[GRAIN] = 30.0;
        c.basket[GAME] = 10.0;
    }
    (w, civs, mid, tile)
}

/// Puts `civ`'s camp on `tile` with a pack of `salt` dug at `source` and no food.
fn pack_salt(w: &mut World, civ: u32, tile: usize, salt: f64, source: usize) -> u32 {
    let id = put_camp(w, civ, tile);
    let u = camp_mut(w, id);
    u.actor.store = [0.0; N];
    u.actor.store[SALT] = salt;
    u.actor.lots = Lots::dug(SALT, source);
    id
}

#[test]
fn a_camp_with_salt_on_a_festival_buys_food_and_the_clans_hold_where_it_came_from() {
    let (mut w, civs, mid, tile) = scene();
    let source = at_big(3, 3);
    let id = pack_salt(&mut w, civs[0], tile, 3.0, source);
    let salt_before: f64 = 3.0;
    trade(&mut w);
    let u = &w.units[&id];
    assert!(total(&u.store) > 0.5, "it bought food: {}", total(&u.store));
    assert!(u.store[SALT] < salt_before - 0.1, "and sold salt: {}", u.store[SALT]);
    let t = u.traded.as_ref().expect("its trade is kept");
    assert_eq!(t.market, mid);
    assert!((t.food - total(&u.store)).abs() < 1e-9);
    // Salt is conserved between the pack and the clans, and the clans' lots say it came from the source.
    let held: f64 =
        u.store[SALT] + w.clans.values().filter(|c| c.market == Some(mid)).map(|c| c.basket[SALT]).sum::<f64>();
    assert!((held - salt_before).abs() < 1e-9, "salt held {held}");
    for c in w.clans.values().filter(|c| c.market == Some(mid) && c.basket[SALT] > 1e-9) {
        let lot = c.lots.of(SALT).next().expect("a lot");
        assert_eq!((lot.source, lot.markets.clone()), (source, vec![mid]));
    }
    assert!(w.notes[&id].iter().any(|n| n.text.contains("bought") && n.text.contains("food at the festival")));
}

#[test]
fn a_camp_on_a_rivals_capital_market_trades_nothing() {
    let (mut w, civs, _, _) = scene();
    let rival = w.civs[&civs[1]].capital;
    let id = pack_salt(&mut w, civs[0], rival, 3.0, at_big(3, 3));
    trade(&mut w);
    let u = &w.units[&id];
    assert_eq!(u.store[SALT], 3.0, "its salt stays");
    assert!(u.traded.is_none());
}

#[test]
fn with_exchange_off_a_camp_trades_nothing() {
    let (mut w, civs, _, tile) = scene();
    w.config.exchange = false;
    let id = pack_salt(&mut w, civs[0], tile, 3.0, at_big(3, 3));
    trade(&mut w);
    assert_eq!(w.units[&id].store[SALT], 3.0);
    assert!(w.units[&id].traded.is_none());
}

/// The seven retro maps and how long the sweep plays each past the pick.
const MAPS: [(&str, u32, u32); 7] = [
    ("small", 2, 200),
    ("small", 3, 200),
    ("small", 6, 200),
    ("small", 9, 200),
    ("medium", 8, 250),
    ("medium", 9, 250),
    ("large", 8, 300),
];

/// What Carry moves on the retro maps: run with `cargo test --release --test exchange carry_flow -- --ignored
/// --nocapture`. One line a map: second routes walked (clan-turns), clan-turns that sold wares there, the
/// salt and obsidian sold, and the festivals where a carrier sold.
#[test]
#[ignore]
fn carry_flow() {
    let lines: Vec<String> = std::thread::scope(|s| {
        let jobs: Vec<_> = MAPS
            .iter()
            .map(|&(size, seed, turns)| {
                s.spawn(move || {
                    let mut w = engine::stage::Stage::retro(size, seed).unwrap().into_world();
                    w.act(engine::Action::ChooseCiv { civ: None }).unwrap();
                    let (mut walked, mut sold, mut salt, mut obsidian) = (0, 0, 0.0, 0.0);
                    let mut festivals = std::collections::BTreeSet::new();
                    for _ in 0..turns {
                        w.run(1);
                        walked += w.clans.values().filter(|c| c.routes.len() > 1).count();
                        for (_, m, b) in &w.carried {
                            if b[SALT] + b[OBSIDIAN] > 1e-9 {
                                sold += 1;
                                festivals.insert(*m);
                            }
                            salt += b[SALT];
                            obsidian += b[OBSIDIAN];
                        }
                    }
                    format!(
                        "{size} {seed}: walked {walked} clan-turns, sold on {sold}, salt {salt:.0}, obsidian {obsidian:.0}, {} festivals",
                        festivals.len()
                    )
                })
            })
            .collect();
        jobs.into_iter().map(|j| j.join().unwrap()).collect()
    });
    for l in lines {
        println!("{l}");
    }
}

/// Two festivals three steps apart on the big world, a clan between them on festival A, holding salt A's
/// last clearing didn't take (A trades grain only, B salt too), and two clans at B that want salt.
fn two_festivals() -> (World, u32, u32, u32) {
    let mut w = big();
    let (a, b) = (open(&mut w, at_big(0, 0)), open(&mut w, at_big(3, 0)));
    w.markets.get_mut(&a).unwrap().goods = vec![GRAIN];
    w.markets.get_mut(&b).unwrap().goods = vec![GRAIN, SALT];
    let id = put(&mut w, &[at_big(1, 0)], 8.0);
    let c = w.clans.get_mut(&id).unwrap();
    c.market = Some(a);
    c.store[GRAIN] = 10.0;
    c.store[SALT] = 4.0;
    c.lots = Lots::dug(SALT, at_big(-2, 1));
    for t in [at_big(4, 0), at_big(3, 1)] {
        let k = put(&mut w, &[t], 4.0);
        let c = w.clans.get_mut(&k).unwrap();
        c.market = Some(b);
        c.basket[GRAIN] = 30.0;
        c.basket[GAME] = 10.0;
    }
    (w, id, a, b)
}

#[test]
fn a_clan_walks_a_second_route_for_salt_its_festival_left_unsold() {
    let (w, id, a, b) = two_festivals();
    let c = &w.clans[&id];
    let festivals = engine::markets::festival_costs(&w, &mut engine::reach::Scratch::new(&w), c.home);
    let route = engine::exchange::carry_route(&w, c, &festivals).expect("worth the walk");
    assert_eq!(route.to, w.markets[&b].tile);
    assert!(route.cost > 0.0);
    // Salt A trades is no second walk.
    let mut w = w;
    w.markets.get_mut(&a).unwrap().goods = vec![GRAIN, SALT];
    let c = &w.clans[&id];
    assert!(engine::exchange::carry_route(&w, c, &festivals).is_none());
}

#[test]
fn a_carrier_trades_its_wares_at_the_second_festival_and_its_lots_go_with_them() {
    let (mut w, id, a, b) = two_festivals();
    let c = &w.clans[&id];
    let festivals = engine::markets::festival_costs(&w, &mut engine::reach::Scratch::new(&w), c.home);
    let route = engine::exchange::carry_route(&w, c, &festivals).unwrap();
    let c = w.clans.get_mut(&id).unwrap();
    c.routes = vec![engine::Route { to: w.markets[&a].tile, path: vec![], cost: 0.0 }, route];
    c.basket = c.store;
    c.store = [0.0; N];
    let salt: f64 = w.clans.values().map(|c| c.basket[SALT]).sum();
    trade(&mut w);
    let sold = w.carried.iter().find(|x| x.0 == id).expect("it carried");
    assert_eq!(sold.1, b);
    assert!(sold.2[SALT] > 0.5, "it sold salt: {}", sold.2[SALT]);
    assert!(w.clans[&id].basket[SALT] < 4.0);
    let after: f64 = w.clans.values().map(|c| c.basket[SALT]).sum();
    assert!((after - salt).abs() < 1e-9, "salt is conserved");
    let buyer = w.clans.values().find(|c| c.market == Some(b) && c.basket[SALT] > 1e-9).expect("a buyer");
    let lot = buyer.lots.of(SALT).next().unwrap();
    assert_eq!((lot.source, lot.markets.clone()), (at_big(-2, 1), vec![b]));
}

/// Civ `civ`'s capital holds `salt` dug at `source`, with its lots.
fn capital_salt(w: &mut World, civ: u32, salt: f64, source: usize) {
    let c = w.civs.get_mut(&civ).unwrap();
    c.store[SALT] = salt;
    c.actor.lots = Lots::dug(SALT, source);
}

#[test]
fn fill_on_a_capital_moves_salt_from_the_store_into_the_pack_with_its_lots() {
    let (mut w, civs, _) = led();
    let source = at_big(3, 3);
    let capital = w.civs[&civs[0]].capital;
    capital_salt(&mut w, civs[0], 5.0, source);
    let id = put_camp(&mut w, civs[0], capital);
    camp_mut(&mut w, id).actor.store = [0.0; N];
    let room = UNIT.salt * w.units[&id].people;
    w.order(civs[0], engine::orders::Order::Fill { unit: id }).expect("a store with salt, and room");
    let u = &w.units[&id];
    let took = room.min(5.0);
    assert!((u.store[SALT] - took).abs() < 1e-9, "the pack took {took}: {}", u.store[SALT]);
    assert!((w.civs[&civs[0]].store[SALT] - (5.0 - took)).abs() < 1e-9, "the store kept the rest");
    let lot = u.lots.of(SALT).next().expect("the pack has its lot");
    assert_eq!((lot.source, lot.markets.clone()), (source, Vec::<u32>::new()));
    // A full pack is refused with the reason.
    let err = w.order(civs[0], engine::orders::Order::Fill { unit: id }).unwrap_err();
    assert!(err.contains("full"), "{err}");
}

#[test]
fn fill_off_a_store_is_refused_and_with_exchange_off_it_is_refused_too() {
    let (mut w, civs, _) = led();
    let tile = at_big(0, 3);
    capital_salt(&mut w, civs[0], 5.0, at_big(3, 3));
    let id = put_camp(&mut w, civs[0], tile);
    let err = w.order(civs[0], engine::orders::Order::Fill { unit: id }).unwrap_err();
    assert!(err.contains("off a store"), "{err}");
    let capital = w.civs[&civs[0]].capital;
    put_camp(&mut w, civs[0], capital);
    w.config.exchange = false;
    let err = w.order(civs[0], engine::orders::Order::Fill { unit: id }).unwrap_err();
    assert!(err.contains("exchange is off"), "{err}");
}

#[test]
fn a_store_keeps_its_pack_salt_back_from_its_keeping_and_a_clans_store_does_not() {
    let mut store = [0.0; N];
    store[GAME] = 40.0;
    store[SALT] = 12.0;
    let mut clan = store;
    engine::actors::spoil_keeping(&mut store, KEEPING.packs);
    // The store's 12 salt keeps 2 beyond its reserve: all 2 are used, and the reserve of 10 stays whole.
    assert!((store[SALT] - KEEPING.packs).abs() < 1e-9, "the reserve stays: {}", store[SALT]);
    engine::actors::spoil(&mut clan);
    assert!(clan[SALT] < 12.0 - 1e-9, "a clan's salt keeps its food: {}", clan[SALT]);
    assert!(clan[SALT] < store[SALT], "a clan's salt is used where a store's reserve is not");
}

#[test]
fn an_army_with_salt_and_no_food_marches_to_a_festival_to_buy_food() {
    let (mut w, civs, _, tile) = scene();
    w.act(engine::Action::ChooseCiv { civ: Some(civs[0]) }).unwrap();
    // The camp stands a step from the festival with salt and no food; its line may feed it, so the test
    // checks the planner's order, not its feeding.
    let start = engine::land::neighbours(&w, tile).next().unwrap();
    let id = pack_salt(&mut w, civs[0], start, 3.0, at_big(3, 3));
    let mut give = |w: &mut World, civ: u32, d: engine::units::Decision| match d {
        engine::units::Decision::Order(o) => w.order(civ, o),
        engine::units::Decision::Gift(_) => Ok(()),
    };
    engine::units::plan(&mut w, civs[0], &mut give);
    let u = &w.units[&id];
    assert!(
        u.dest() == Some(tile) || u.home == tile,
        "it marches to the festival to buy food: dest {:?}, home {}",
        u.dest(),
        u.home
    );
}
