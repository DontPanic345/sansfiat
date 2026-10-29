mod common;

use common::{at, check_territories, flat, out, put, steps};
use engine::clans::{adjust, eat, growth, harvest, nutrition, reap, site_value, surplus, value, walks};
use engine::data::*;
use engine::land::neighbours;
use engine::map::{landmasses, Zone};
use engine::{Basket, Config, World, CLAN, TERRAIN};

const EPS: f64 = 1e-9;

fn land(w: &World, i: usize) -> bool {
    TERRAIN[w.tiles[i].terrain].land
}

/// A world with no clans and no food, and a land tile with a land neighbour.
fn bare() -> (World, usize, usize) {
    let mut w = World::new(Config { seed: 1, radius: 6, army: true });
    w.clans.clear();
    for t in &mut w.tiles {
        t.stock = [0.0; N];
        t.claim = None;
    }
    let i = (0..w.tiles.len()).find(|&i| land(&w, i) && neighbours(&w, i).all(|j| land(&w, j))).unwrap();
    let j = neighbours(&w, i).next().unwrap();
    (w, i, j)
}

fn total(b: &Basket) -> f64 {
    b.iter().sum()
}

#[test]
fn seeds_land_on_every_big_landmass() {
    for seed in 1..=8 {
        let w = World::new(Config { seed, radius: 16, army: true });
        // A sealed valley is a landmass of its own, and nobody is seeded there.
        let big: Vec<Vec<usize>> = landmasses(&w.tiles, 16)
            .into_iter()
            .filter(|m| m.len() >= CLAN.min_landmass && m.iter().any(|&i| w.tiles[i].zone != Zone::Sealed))
            .collect();
        assert_eq!(w.clans.len(), CLAN.seeds.max(big.len()), "seed {seed}");
        let mut homes: Vec<usize> = w.clans.values().map(|c| c.home).collect();
        homes.sort_unstable();
        homes.dedup();
        assert_eq!(homes.len(), w.clans.len());
        for c in w.clans.values() {
            assert!(land(&w, c.home));
            assert_eq!(c.people, CLAN.start_pop);
            assert_eq!(c.territory, vec![c.home]);
            assert_eq!(w.tiles[c.home].claim, Some(c.id));
        }
        for m in &big {
            assert!(homes.iter().any(|h| m.binary_search(h).is_ok()), "seed {seed}: a landmass without a seed");
        }
    }
}

#[test]
fn nutrition_rewards_variety() {
    assert!((nutrition(&[10.0, 0.0, 0.0, 0.0, 0.0], 10.0) - 5.8).abs() < EPS);
    assert!((nutrition(&[2.5, 2.5, 2.5, 2.5, 0.0], 10.0) - 10.0).abs() < EPS);
}

/// What `pop` people with no store or route would reap from `territory` held from its first tile.
fn reaped(w: &World, territory: &[usize], pop: f64) -> Vec<(usize, Basket)> {
    let mut t = territory.to_vec();
    t.sort_unstable();
    let (_, yearly) = walks(w, territory[0], &t);
    t.iter().copied().zip(reap(w, pop, &[0.0; N], &[], &t, &yearly)).collect()
}

#[test]
fn work_takes_a_tiles_foods_in_proportion_to_stock() {
    let (mut w, i, _) = bare();
    w.tiles[i].stock[GRAIN] = 30.0;
    w.tiles[i].stock[GAME] = 10.0;
    // One person stops at the target: three parts grain to one of game.
    let t = reaped(&w, &[i], 1.0)[0].1;
    let food = CLAN.surplus_target;
    assert!((t[GRAIN] - 0.75 * food).abs() < EPS && (t[GAME] - 0.25 * food).abs() < EPS, "{t:?}");
    // Many people could take more than `take_rate` of the stock, and don't.
    let pop = CLAN.take_rate * 40.0;
    let t = reaped(&w, &[i], pop)[0].1;
    assert!((total(&t) - CLAN.take_rate * 40.0).abs() < EPS, "{t:?}");
    assert!((t[GRAIN] - 3.0 * t[GAME]).abs() < EPS);
}

#[test]
fn walking_comes_off_each_tiles_share_first() {
    let (mut w, i, j) = bare();
    w.tiles[i].stock[GRAIN] = 40.0;
    w.tiles[j].stock[GRAIN] = 40.0;
    w.tiles[j].terrain = FOREST;
    let pop = 1.0;
    let takes = reaped(&w, &[i, j], pop);
    let share = pop * CLAN.work / 2.0;
    let walk = 2.0 * 2.0 * CLAN.inside;
    let at = |t: usize| takes.iter().find(|x| x.0 == t).unwrap().1[GRAIN];
    // Both scaled to the target by one factor: in proportion to their work.
    assert!((at(j) / at(i) - (share - walk) / share).abs() < EPS);
    assert!((at(i) + at(j) - CLAN.surplus_target * pop).abs() < EPS);
}

#[test]
fn reaping_keeps_to_labour_stock_and_target_on_real_maps() {
    for seed in 1..=3 {
        let mut w = World::new(Config { seed, radius: 16, army: true });
        w.run(60);
        for c in w.clans.values() {
            let (_, yearly) = walks(&w, c.home, &c.territory);
            for (pop, store) in [(c.people, c.store), (2.0, [0.0; N]), (40.0, [9.0, 0.0, 0.0, 0.0, 0.0])] {
                let takes = reap(&w, pop, &store, &c.routes, &c.territory, &yearly);
                let food = total(&std::array::from_fn(|f| takes.iter().map(|t| t[f]).sum()));
                let labour = (pop * CLAN.work - c.trip()).max(0.0);
                assert!(CLAN.harvest_labour * food <= labour + EPS);
                assert!(food <= (CLAN.surplus_target * pop - total(&store)).max(0.0) + EPS);
                for (&i, t) in c.territory.iter().zip(&takes) {
                    for (x, s) in t.iter().zip(&w.tiles[i].stock) {
                        assert!(*x <= CLAN.take_rate * s + EPS);
                    }
                }
            }
        }
    }
}

#[test]
fn a_clan_harvests_nothing_off_its_territory() {
    let mut w = flat();
    let o = at(0, 0);
    w.tiles[o].stock[GRAIN] = 2.0;
    for d in 0..6 {
        w.tiles[out(d, 1)].stock = [20.0, 20.0, 20.0, 0.0, 0.0];
    }
    let id = put(&mut w, &[o], 5.0);
    let before: Vec<Basket> = w.tiles.iter().map(|t| t.stock).collect();
    harvest(&mut w);
    let c = &w.clans[&id];
    assert!((total(&c.harvest) - CLAN.take_rate * 2.0).abs() < EPS);
    for (i, t) in w.tiles.iter().enumerate() {
        if i != o {
            assert_eq!(t.stock, before[i], "tile {i}");
        }
    }
    assert!(!w.tiles.iter().any(|t| t.walked), "a home-only clan walks nowhere");
}

#[test]
fn a_far_tile_costs_its_walk_and_its_tiles_are_walked() {
    let mut w = flat();
    let (o, a, b) = (at(0, 0), out(0, 1), out(0, 2));
    w.tiles[a].terrain = FOREST;
    w.tiles[b].terrain = MARSH;
    for t in [o, a, b] {
        w.tiles[t].stock[GRAIN] = 40.0;
    }
    let id = put(&mut w, &[o, a, b], 1.0);
    harvest(&mut w);
    let c = &w.clans[&id];
    // b is 5 away, through forest into marsh: its walk eats its third of one person's labour.
    assert_eq!(c.walk, 2.0 * CLAN.inside * (2.0 + 5.0));
    const { assert!(CLAN.work / 3.0 < 2.0 * CLAN.inside * 5.0) };
    assert!(w.tiles[a].walked && w.tiles[b].walked && !w.tiles[o].walked);
    let got = total(&c.harvest);
    let share = CLAN.work / 3.0;
    let want = (share + (share - 2.0 * CLAN.inside * 2.0)) / CLAN.harvest_labour;
    assert!((got - want).abs() < EPS, "{got} vs {want}");
}

fn eater(pop: f64, basket: Basket, since: u32) -> World {
    let mut w = World::new(Config { seed: 1, radius: 6, army: true });
    let c = w.clans.get_mut(&1).unwrap();
    c.people = pop;
    c.basket = basket;
    c.since = since;
    w
}

/// Too few people to divide or send a daughter, so `eat` only eats.
const P: f64 = 4.0;
const _: () = assert!(P < CLAN.split_at && P < CLAN.hungry_split);

/// `basket` scaled from a need of 10 to a need of `P`.
fn per(basket: Basket) -> Basket {
    basket.map(|v| v * P / 10.0)
}

#[test]
fn a_clan_that_harvests_just_its_need_holds() {
    let mut w = eater(P, per([2.5, 2.5, 2.5, 2.5, 0.0]), 5);
    w.clans.get_mut(&1).unwrap().harvest = per([2.5, 2.5, 2.5, 2.5, 0.0]);
    eat(&mut w);
    let c = &w.clans[&1];
    assert!(c.fed && !c.starving);
    assert_eq!(c.since, 6);
    assert_eq!(c.people, P);
}

#[test]
fn a_clan_grows_on_its_harvest_beyond_need() {
    for extra in [0.5, 1.0, 10.0] {
        // 4 of game and of grain and 2 of fruit meet a need of 10; more game counts at `excess_value`.
        let b = per([4.0 + extra, 4.0, 2.0, 0.0, 0.0]);
        let mut w = eater(P, b, 5);
        w.clans.get_mut(&1).unwrap().harvest = b;
        eat(&mut w);
        let c = &w.clans[&1];
        let s = surplus(&b, P);
        assert!((s - CLAN.excess_value * extra * P / 10.0).abs() < EPS);
        let want = P * (1.0 + growth(6) * (s / (CLAN.grow_full * P)).min(1.0));
        assert!(c.fed && (c.people - want).abs() < EPS, "{extra}: {} vs {want}", c.people);
        assert!(c.people > P);
    }
}

#[test]
fn a_store_alone_does_not_make_a_clan_grow() {
    // Fed well from its store, with nothing harvested.
    let mut w = eater(P, per([5.0, 5.0, 5.0, 5.0, 0.0]), 5);
    eat(&mut w);
    let c = &w.clans[&1];
    assert!(c.fed);
    assert_eq!(c.people, P);
}

#[test]
fn a_clan_short_of_food_holds_steady() {
    let mut w = eater(P, per([4.0, 4.0, 1.0, 0.0, 0.0]), 5);
    eat(&mut w);
    let c = &w.clans[&1];
    assert!((c.fed_ratio - 0.9).abs() < EPS);
    assert!(!c.fed && !c.starving);
    assert_eq!(c.people, P);
}

#[test]
fn a_starving_clan_shrinks() {
    let mut w = eater(P, per([4.0, 0.0, 0.0, 0.0, 0.0]), 5);
    eat(&mut w);
    let c = &w.clans[&1];
    assert!((c.fed_ratio - 0.4).abs() < EPS);
    assert!(c.starving);
    assert_eq!(c.hungry, 1);
    assert!((c.people - P * (1.0 - CLAN.starve * (1.0 - 0.4 / 0.8))).abs() < EPS);
}

#[test]
fn the_store_is_what_is_left_less_spoilage() {
    let mut w = eater(P, per([10.0, 10.0, 0.0, 0.0, 0.0]), 5);
    eat(&mut w);
    let c = &w.clans[&1];
    // 8 tenths of need count in full; 2 tenths more need 2 / 0.3 of the 12 tenths left over.
    let eaten = 0.2 * P / CLAN.excess_value;
    let left = 0.6 * P * (1.0 - eaten / (1.2 * P)) * (1.0 - CLAN.spoilage);
    assert!(c.fed);
    assert!((c.store[GAME] - left).abs() < EPS && (c.store[GRAIN] - left).abs() < EPS, "{:?}", c.store);
}

#[test]
fn a_clan_under_one_person_dies() {
    let mut w = eater(1.5, [0.0; N], 5);
    let home = w.clans[&1].home;
    eat(&mut w);
    assert!(!w.clans.contains_key(&1));
    assert_eq!(w.tiles[home].claim, None);
}

#[test]
fn clans_live_for_fifty_turns() {
    for seed in 1..=3 {
        let mut w = World::new(Config { seed, radius: 16, army: true });
        w.run(50);
        assert!(!w.clans.is_empty(), "seed {seed}");
        for c in w.clans.values() {
            assert!(c.people > 0.0);
        }
        for t in &w.tiles {
            for f in 0..N {
                assert!(t.stock[f] >= 0.0 && t.stock[f] <= t.cap[f] + EPS);
            }
        }
    }
}

#[test]
fn a_clan_starving_too_long_moves_on() {
    for hungry in [CLAN.patience - 1, CLAN.patience] {
        let mut w = flat();
        let o = at(0, 0);
        let rich = out(0, 4);
        w.tiles[rich].stock = [20.0, 20.0, 20.0, 0.0, 0.0];
        let id = put(&mut w, &[o, out(3, 1)], 5.0);
        let c = w.clans.get_mut(&id).unwrap();
        (c.fed, c.starving, c.hungry, c.since) = (false, true, hungry, 9);
        let c = c.clone();
        let now = value(&w, &c, &c.territory);
        adjust(&mut w);
        let after = &w.clans[&id];
        if hungry < CLAN.patience {
            assert!(after.territory.contains(&o), "moved before its patience ran out");
            continue;
        }
        assert_ne!(after.home, o);
        assert_eq!(after.territory, vec![after.home]);
        assert!(steps(&w, o, after.home) <= CLAN.move_range);
        assert!(site_value(&w, after.home, c.people, &c.store) > now + CLAN.move_margin);
        assert_eq!((after.since, after.hungry), (0, 0));
        assert_eq!(w.tiles[o].claim, None);
        assert_eq!(w.tiles[out(3, 1)].claim, None);
        check_territories(&w);
    }
}

#[test]
fn a_big_clan_divides_its_land() {
    let mut w = flat();
    let round: Vec<usize> = std::iter::once(at(0, 0)).chain((0..6).map(|d| out(d, 1))).chain([out(0, 2)]).collect();
    let id = put(&mut w, &round, CLAN.split_at);
    let c = w.clans.get_mut(&id).unwrap();
    c.basket = [10.0, 10.0, 10.0, 0.0, 0.0];
    c.harvest = c.basket;
    c.since = 5;
    let s = surplus(&c.harvest, CLAN.split_at);
    let pop = CLAN.split_at * (1.0 + growth(6) * (s / (CLAN.grow_full * CLAN.split_at)).min(1.0));
    let store: f64 = 3.0 * (10.0 - CLAN.diet_share * CLAN.split_at) * (1.0 - CLAN.spoilage);
    eat(&mut w);
    assert_eq!(w.clans.len(), 2);
    let (m, d) = (&w.clans[&id], w.clans.values().find(|c| c.id != id).unwrap());
    assert_eq!(m.home, at(0, 0));
    assert!(!m.territory.is_empty() && !d.territory.is_empty());
    assert!(m.territory.iter().all(|t| !d.territory.contains(t)));
    assert!(m.territory.len() + d.territory.len() <= round.len());
    assert!((m.people + d.people - pop).abs() < EPS && (m.people - d.people).abs() < EPS);
    assert!((total(&m.store) + total(&d.store) - store).abs() < EPS);
    assert_eq!(d.since, m.since);
    assert!(d.fed);
    check_territories(&w);
}

#[test]
fn a_starving_clan_sends_a_daughter_only_where_she_would_be_fed() {
    for food in [false, true] {
        let mut w = flat();
        let o = at(0, 0);
        let rich = out(1, 3);
        if food {
            w.tiles[rich].stock = [20.0, 20.0, 20.0, 0.0, 0.0];
        }
        let id = put(&mut w, &[o], 9.0);
        // 70% fed: starving, but still at least `hungry_split` after shrinking.
        w.clans.get_mut(&id).unwrap().basket = [3.6, 2.7, 0.0, 0.0, 0.0];
        let pop = 9.0 * (1.0 - CLAN.starve * (1.0 - 0.7 / CLAN.stable_at));
        assert!(pop >= CLAN.hungry_split);
        eat(&mut w);
        let total_pop: f64 = w.clans.values().map(|c| c.people).sum();
        assert!((total_pop - pop).abs() < EPS);
        if !food {
            assert_eq!(w.clans.len(), 1, "a daughter with nowhere to go");
            continue;
        }
        assert_eq!(w.clans.len(), 2);
        let d = w.clans.values().find(|c| c.id != id).unwrap();
        assert!((d.people - pop.min(CLAN.split_at) / 2.0).abs() < EPS);
        assert_eq!(d.territory, vec![d.home]);
        assert!(steps(&w, o, d.home) <= CLAN.scout_range);
        assert!(site_value(&w, d.home, d.people, &d.store) >= CLAN.settle_fed);
        assert_eq!(w.clans[&id].hungry, 0);
        check_territories(&w);
    }
}

#[test]
fn clans_spread() {
    let mut w = World::new(Config { seed: 1, radius: 16, army: true });
    w.run(300);
    assert!(w.clans.len() > 10, "{} clans", w.clans.len());
}
