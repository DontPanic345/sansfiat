//! Exchange (`exchange.rs`): an army's line may end at a market that isn't a rival's. Its civ pays it in wares
//! along its line home, it posts them in that market's clearing, the food it buys waits at its stall there, and
//! its line brings it.

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
fn a_camp_with_salt_on_a_festival_buys_food_into_its_stall_and_the_clans_hold_where_it_came_from() {
    let (mut w, civs, mid, tile) = scene();
    let source = at_big(3, 3);
    let id = pack_salt(&mut w, civs[0], tile, 3.0, source);
    let salt_before: f64 = 3.0;
    trade(&mut w);
    let u = &w.units[&id];
    let stall = u.stall.as_ref().expect("the food waits at its stall");
    assert_eq!(stall.market, mid);
    assert!(total(&stall.food) > 0.5, "it bought food: {}", total(&stall.food));
    assert!(total(&u.store) < 1e-9, "none of it is in its pack until its line brings it");
    assert!(u.store[SALT] < salt_before - 0.1, "and sold salt: {}", u.store[SALT]);
    let t = u.traded.as_ref().expect("its trade is kept");
    assert_eq!(t.market, mid);
    assert!((t.food - total(&stall.food)).abs() < 1e-9);
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

/// An army of civ 0 alone on a tile past its line's reach, with no food in its pack, its capital's store holding
/// 40 salt and food, and a festival on the next tile out whose four clans, with food to spare, want salt and last
/// traded it at a food's price. Returns the world, the civ, the army, the festival's market and tile.
fn far() -> (World, u32, u32, u32, usize) {
    let (mut w, civs, _) = three();
    let civ = civs[0];
    let capital = w.civs[&civ].capital;
    let id = put_camp(&mut w, civ, capital);
    camp_mut(&mut w, id).actor.store = [0.0; N];
    // The nearest tile straight out from the capital, away from the rivals, past the reach, and the next one out.
    let h = w.tiles[capital].hex();
    let out = |k: i32| at_big(h.q, h.r + k);
    let k = (1..10).find(|&k| engine::units::past_reach(&w, id, out(k))).expect("a tile past the reach");
    let (tile, at) = (out(k), out(k + 1));
    put_camp(&mut w, civ, tile);
    let mid = open(&mut w, at);
    let m = w.markets.get_mut(&mid).unwrap();
    m.goods = vec![GRAIN, SALT];
    m.prices[GRAIN] = 1.0;
    m.prices[SALT] = 1.0;
    let free: Vec<usize> = engine::land::neighbours(&w, at).filter(|&t| t != tile).take(4).collect();
    for t in free {
        let c = put(&mut w, &[t], 8.0);
        let c = w.clans.get_mut(&c).unwrap();
        c.market = Some(mid);
        c.basket[GRAIN] = 80.0;
    }
    let store = &mut w.civs.get_mut(&civ).unwrap().store;
    store[SALT] = 40.0;
    store[GRAIN] = 60.0;
    (w, civ, id, mid, at)
}

#[test]
fn an_army_past_its_lines_reach_is_paid_salt_along_its_line_home_and_its_line_ends_at_the_festival() {
    let (mut w, civ, id, _, at) = far();
    let capital = w.civs[&civ].capital;
    assert!(engine::units::past_reach(&w, id, w.units[&id].home), "food can't come down its line home");
    engine::units::step(&mut w);
    let u = &w.units[&id];
    let paid = u.store[SALT];
    assert!(paid > 1.0, "its civ paid it salt: {paid}");
    assert!((w.civs[&civ].store[SALT] - (40.0 - paid)).abs() < 1e-9, "from its capital's store");
    let stall = u.stall.as_ref().expect("a stall at the festival");
    assert_eq!(w.markets[&stall.market].tile, at, "its stall is at the festival");
    assert_eq!(engine::units::store_of(&w, id), Some(capital), "and its line home still ends at its capital");
    assert!(w.notes[&id].iter().any(|n| n.text.contains("paid it") && n.text.contains("salt")));
}

#[test]
fn the_food_it_buys_waits_at_its_stall_and_its_line_brings_it() {
    let (mut w, _, id, mid, at) = far();
    engine::units::step(&mut w);
    trade(&mut w);
    let bought = engine::exchange::stall_at(&w.units[&id], mid);
    assert!(bought > 1.0, "it bought food at the festival: {bought}");
    engine::units::step(&mut w);
    let u = &w.units[&id];
    assert!(engine::exchange::stall_at(u, mid) < bought * 0.5, "its line took the food from the stall");
    assert!(w.notes[&id].iter().any(|n| n.text.contains("its line brings food")));
    assert_eq!(u.routes.first().map(|r| r.to), Some(at), "its line ended at the festival this turn");
    assert_eq!(u.hungry, 0, "and it ate");
}

#[test]
fn a_festival_that_didnt_trade_a_ware_doesnt_make_it_pay() {
    let (mut w, civ, id, mid, _) = far();
    w.markets.get_mut(&mid).unwrap().goods = vec![GRAIN];
    engine::units::step(&mut w);
    assert_eq!(w.units[&id].store[SALT], 0.0);
    assert_eq!(w.civs[&civ].store[SALT], 40.0);
}

#[test]
fn a_ware_that_buys_less_than_its_weight_in_food_isnt_pay() {
    let (mut w, _, id, mid, _) = far();
    w.markets.get_mut(&mid).unwrap().prices[SALT] = 0.5 * GOOD[SALT].weight;
    engine::units::step(&mut w);
    assert_eq!(w.units[&id].store[SALT], 0.0, "food is cheaper carried than bought with it");
}

#[test]
fn a_rivals_market_never_ends_its_line() {
    let (mut w, civs, _) = three();
    let rival = w.civs[&civs[1]].market;
    let theirs = w.civs[&civs[1]].capital;
    let id = put_camp(&mut w, civs[0], theirs);
    let mut food = [0.0; N];
    food[GRAIN] = 10.0;
    camp_mut(&mut w, id).stall = Some(engine::exchange::Stall { market: rival, food });
    assert!(engine::exchange::market_end(&w, &w.units[&id]).is_none());
}

#[test]
fn the_store_keeps_its_armies_pay_back_from_its_keeping() {
    let (mut w, civ, id, _, _) = far();
    let capital = w.civs[&civ].capital;
    let back = engine::exchange::pay_back(&w, civ, capital);
    let plan = engine::exchange::pay_plan(&w, id).expect("it is paid");
    assert!((back - plan.wares[SALT]).abs() < 1e-9 && back > 1.0, "it keeps back the pay: {back}");
    engine::civs::upkeep(&mut w);
    assert!(w.civs[&civ].store[SALT] + 1e-9 >= back, "its keeping used none of it: {}", w.civs[&civ].store[SALT]);
}

#[test]
fn with_exchange_off_an_army_isnt_paid() {
    let (mut w, civ, id, _, _) = far();
    w.config.exchange = false;
    engine::units::step(&mut w);
    assert_eq!(w.units[&id].store[SALT], 0.0);
    assert_eq!(w.civs[&civ].store[SALT], 40.0);
    assert!(w.units[&id].routes.first().is_none_or(|r| r.to == w.civs[&civ].capital));
}

#[test]
fn the_levy_takes_the_salt_and_obsidian_a_clan_brings_whatever_the_food_room() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let mid = w.civs[&red].market;
    hold_land(&mut w, red, &[at_big(-4, 0)]);
    let on = put(&mut w, &[at_big(-4, 0)], 4.0);
    let c = w.clans.get_mut(&on).unwrap();
    c.market = Some(mid);
    c.basket = [0.0; N];
    c.basket[GRAIN] = 10.0;
    c.basket[SALT] = 4.0;
    c.basket[OBSIDIAN] = 2.0;
    let full = &mut w.civs.get_mut(&red).unwrap().store;
    *full = [0.0; N];
    full[GRAIN] = CIV.store_cap;
    engine::leaning::land(&mut w, false);
    engine::civs::levy(&mut w, mid);
    let store = &w.civs[&red].store;
    assert_eq!(store[GRAIN], CIV.store_cap, "no room for food");
    // It keeps back the salt its own spare food will use and its people's tools.
    let mut basket = [0.0; N];
    (basket[GRAIN], basket[SALT], basket[OBSIDIAN]) = (10.0, 4.0, 2.0);
    let back = engine::actors::keeps_back(&basket, w.clans[&on].people);
    assert!(back[SALT] > 0.0 && back[OBSIDIAN] > 0.0);
    assert!((store[SALT] - CIV.levy_goods * (4.0 - back[SALT])).abs() < 1e-9);
    assert!((store[OBSIDIAN] - CIV.levy_goods * (2.0 - back[OBSIDIAN])).abs() < 1e-9);
}

#[test]
fn the_levy_stops_at_what_the_store_holds() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let mid = w.civs[&red].market;
    hold_land(&mut w, red, &[at_big(-4, 0)]);
    let on = put(&mut w, &[at_big(-4, 0)], 4.0);
    let c = w.clans.get_mut(&on).unwrap();
    c.market = Some(mid);
    c.basket = [0.0; N];
    c.basket[SALT] = 100.0;
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    engine::leaning::land(&mut w, false);
    engine::civs::levy(&mut w, mid);
    let holds = engine::actors::civ_holds(CIV.store_cap)[SALT];
    assert!((w.civs[&red].store[SALT] - holds).abs() < 1e-9);
    assert!((w.clans[&on].basket[SALT] - (100.0 - holds)).abs() < 1e-9, "the rest stays with the clan");
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

/// What armies do with exchange on the retro maps, a map a line: run with `cargo test --release --test exchange
/// army_flow -- --ignored --nocapture`. Army-turns, those past their line home's reach, those paid, those whose
/// line ends at a market, those that bought food there and the food, the salt and obsidian a civ's store held a turn,
/// and the festival-turns trading each and the food a unit bought there.
#[test]
#[ignore]
fn army_flow() {
    let lines: Vec<String> = std::thread::scope(|s| {
        let jobs: Vec<_> = MAPS
            .iter()
            .map(|&(size, seed, turns)| {
                s.spawn(move || {
                    let mut w = engine::stage::Stage::retro(size, seed).unwrap().into_world();
                    w.act(engine::Action::ChooseCiv { civ: None }).unwrap();
                    let (mut camps, mut past, mut paid, mut ends, mut bought, mut food) = (0, 0, 0, 0, 0, 0.0);
                    let (mut civt, mut salt, mut obsidian) = (0, 0.0, 0.0);
                    let (mut pn, mut ps, mut bad) = ([0usize; 2], [0.0f64; 2], 0);
                    for _ in 0..turns {
                        let before: std::collections::BTreeMap<u32, f64> =
                            w.units.values().map(|u| (u.id, u.store[SALT] + u.store[OBSIDIAN])).collect();
                        w.run(1);
                        for u in w.units.values().filter(|u| u.camp().is_some()) {
                            camps += 1;
                            if engine::units::past_reach(&w, u.id, u.home) {
                                past += 1;
                            }
                            if u.store[SALT] + u.store[OBSIDIAN] > before.get(&u.id).copied().unwrap_or(0.0) + 1e-9 {
                                paid += 1;
                            }
                            let end = u.routes.first().map(|r| r.to);
                            let stall = u.stall.as_ref().and_then(|s| w.markets.get(&s.market)).map(|m| m.tile);
                            if stall.is_some() && stall == end {
                                ends += 1;
                            }
                            if let Some(t) = u.traded.as_ref().filter(|t| t.food > 1e-6) {
                                bought += 1;
                                food += t.food;
                            }
                        }
                        for m in w.markets.values() {
                            let foods: Vec<f64> = m.goods.iter().filter(|&&f| GOOD[f].eaten).map(|&f| m.prices[f]).collect();
                            let mean = foods.iter().sum::<f64>() / foods.len().max(1) as f64;
                            if mean <= 1e-9 {
                                continue;
                            }
                            for (k, g) in [SALT, OBSIDIAN].into_iter().enumerate() {
                                let r = m.prices[g] / mean;
                                if m.goods.contains(&g) && r.is_finite() {
                                    pn[k] += 1;
                                    ps[k] += r;
                                } else if m.goods.contains(&g) {
                                    bad += 1;
                                }
                            }
                        }
                        for c in w.civs.values().filter(|c| !c.is_out()) {
                            civt += 1;
                            salt += c.store[SALT];
                            obsidian += c.store[OBSIDIAN];
                        }
                    }
                    format!(
                        "{size} {seed}: army-turns {camps}, past reach {past}, paid {paid}, line at a market {ends}, bought {bought} (food {food:.1}); mean store salt {:.2}, obsidian {:.2}; festival-turns trading salt {} at {:.2} food, obsidian {} at {:.2}, {} unpriced",
                        salt / civt.max(1) as f64,
                        obsidian / civt.max(1) as f64,
                        pn[0], ps[0] / pn[0].max(1) as f64, pn[1], ps[1] / pn[1].max(1) as f64, bad
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
