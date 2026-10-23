mod common;

use common::{at, check_rounds, flat, out, put, steps};
use engine::clans::{adjust, eat, growth, harvest, nutrition, plan_harvest, site_value, value};
use engine::data::*;
use engine::land::neighbours;
use engine::map::landmasses;
use engine::{round, Basket, Config, World, CLAN, TERRAIN};

const EPS: f64 = 1e-9;

fn land(w: &World, i: usize) -> bool {
    TERRAIN[w.tiles[i].terrain].land
}

/// A world with no clans and no food, and a land tile with a land neighbour.
fn bare() -> (World, usize, usize) {
    let mut w = World::new(Config { seed: 1, radius: 6 });
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
        let w = World::new(Config { seed, radius: 16 });
        let big: Vec<Vec<usize>> =
            landmasses(&w.tiles, 16).into_iter().filter(|m| m.len() >= CLAN.min_landmass).collect();
        assert_eq!(w.clans.len(), CLAN.seeds.max(big.len()), "seed {seed}");
        let mut homes: Vec<usize> = w.clans.values().map(|c| c.home).collect();
        homes.sort_unstable();
        homes.dedup();
        assert_eq!(homes.len(), w.clans.len());
        for c in w.clans.values() {
            assert!(land(&w, c.home));
            assert_eq!(c.pop, CLAN.start_pop);
            assert_eq!(c.round, vec![c.home]);
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

#[test]
fn the_round_tile_comes_first() {
    let (mut w, i, j) = bare();
    w.tiles[i].stock[GRAIN] = 10.0;
    w.tiles[j].stock[GRAIN] = 10.0;
    let options = round::options(&w, 1, &[i]);
    assert!(options.contains(&(i, 0.0)));
    assert!(options.iter().any(|&(t, c)| t == j && c > 0.0));
    let plan = plan_harvest(&w, 5.0, &[0.0; N], 2.0, &options);
    assert_eq!(plan.take.len(), 1);
    assert_eq!(plan.take[0].0, i);
    assert!((plan.food[GRAIN] - 2.0).abs() < EPS);
}

#[test]
fn setup_is_paid_once() {
    let (mut w, _, j) = bare();
    w.tiles[j].stock[GRAIN] = 10.0;
    w.tiles[j].stock[GAME] = 10.0;
    let plan = plan_harvest(&w, 5.0, &[0.0; N], 100.0, &[(j, 2.0)]);
    let food = total(&plan.food);
    assert!((food - 7.5).abs() < EPS, "{food}");
    assert!((plan.labour - (2.0 + CLAN.harvest_labour * food)).abs() < EPS, "{}", plan.labour);
}

#[test]
fn plans_keep_to_labour_and_budget() {
    for seed in 1..=3 {
        let mut w = World::new(Config { seed, radius: 16 });
        w.run(20);
        for c in w.clans.values() {
            let opts = round::options(&w, c.id, &c.round);
            for (pop, labour) in [(c.pop, 3.0), (c.pop, 50.0), (2.0, 100.0), (40.0, 7.0)] {
                for store in [[0.0; N], c.store, [9.0, 0.0, 0.0, 0.0, 0.0]] {
                    let plan = plan_harvest(&w, pop, &store, labour, &opts);
                    let budget = (CLAN.surplus_target * pop - total(&store)).max(0.0);
                    assert!(plan.labour <= labour + EPS);
                    assert!(total(&plan.food) <= budget + EPS);
                    for (i, t) in &plan.take {
                        for (x, s) in t.iter().zip(&w.tiles[*i].stock) {
                            assert!(*x <= CLAN.take_rate * s + EPS);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn a_missing_food_comes_before_more_grain() {
    let (mut w, i, j) = bare();
    w.tiles[i].stock[GRAIN] = 10.0;
    w.tiles[j].stock[GAME] = 10.0;
    let store = [0.0, 20.0, 0.0, 0.0, 0.0];
    let plan = plan_harvest(&w, 20.0, &store, 5.0, &[(i, 0.0), (j, 2.0)]);
    assert!((plan.food[GAME] - 3.0).abs() < EPS, "{:?}", plan.food);
    assert_eq!(plan.food[GRAIN], 0.0);
}

#[test]
fn walking_comes_off_first() {
    let mut w = World::new(Config { seed: 1, radius: 16 });
    let id = *w.clans.keys().next().unwrap();
    let home = w.clans[&id].home;
    // The farthest tile within reach of home makes a round that costs more than one person's work.
    let far = (0..w.tiles.len())
        .filter(|&i| land(&w, i) && w.tiles[i].hex().distance(w.tiles[home].hex()) == 2)
        .find(|&i| round::leg(&w, home, home, i).is_some())
        .unwrap();
    let c = w.clans.get_mut(&id).unwrap();
    c.pop = 1.0;
    c.round = vec![home, far];
    let cost = round::cost(&w, &[home, far]);
    assert!(cost > CLAN.work, "{cost}");
    harvest(&mut w);
    let c = &w.clans[&id];
    assert_eq!(c.walk, cost);
    assert_eq!(c.harvest, [0.0; N]);
}

fn eater(pop: f64, basket: Basket, since: u32) -> World {
    let mut w = World::new(Config { seed: 1, radius: 6 });
    let c = w.clans.get_mut(&1).unwrap();
    c.pop = pop;
    c.basket = basket;
    c.since = since;
    w
}

#[test]
fn a_fed_clan_grows() {
    let mut w = eater(10.0, [2.5, 2.5, 2.5, 2.5, 0.0], 5);
    eat(&mut w);
    let c = &w.clans[&1];
    assert!(c.fed && !c.starving);
    assert_eq!(c.since, 6);
    assert!((c.pop - 10.0 * (1.0 + growth(6))).abs() < EPS);
}

#[test]
fn a_clan_short_of_food_holds_steady() {
    let mut w = eater(10.0, [4.0, 4.0, 1.0, 0.0, 0.0], 5);
    eat(&mut w);
    let c = &w.clans[&1];
    assert!((c.fed_ratio - 0.9).abs() < EPS);
    assert!(!c.fed && !c.starving);
    assert_eq!(c.pop, 10.0);
}

#[test]
fn a_starving_clan_shrinks() {
    let mut w = eater(10.0, [4.0, 0.0, 0.0, 0.0, 0.0], 5);
    eat(&mut w);
    let c = &w.clans[&1];
    assert!((c.fed_ratio - 0.4).abs() < EPS);
    assert!(c.starving);
    assert_eq!(c.hungry, 1);
    assert!((c.pop - 10.0 * (1.0 - CLAN.starve * (1.0 - 0.4 / 0.8))).abs() < EPS);
}

#[test]
fn the_store_is_what_is_left_less_spoilage() {
    let mut w = eater(10.0, [10.0, 10.0, 0.0, 0.0, 0.0], 5);
    eat(&mut w);
    let c = &w.clans[&1];
    // 8 counts in full; 2 more needs 2 / 0.3 of the 12 left over.
    let eaten = 2.0 / CLAN.excess_value;
    let left = 6.0 * (1.0 - eaten / 12.0) * (1.0 - CLAN.spoilage);
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
        let mut w = World::new(Config { seed, radius: 16 });
        w.run(50);
        assert!(!w.clans.is_empty(), "seed {seed}");
        for c in w.clans.values() {
            assert!(c.pop > 0.0);
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
        let rich = out(0, 3);
        w.tiles[rich].stock = [20.0, 20.0, 20.0, 0.0, 0.0];
        let id = put(&mut w, &[o, out(3, 1)], 5.0);
        let c = w.clans.get_mut(&id).unwrap();
        (c.fed, c.starving, c.hungry, c.since) = (false, true, hungry, 9);
        let c = c.clone();
        let now = value(&w, &c, &c.round);
        adjust(&mut w);
        let after = &w.clans[&id];
        if hungry < CLAN.patience {
            assert!(after.round.contains(&o), "moved before its patience ran out");
            continue;
        }
        assert_ne!(after.home, o);
        assert_eq!(after.round, vec![after.home]);
        assert!(steps(&w, o, after.home) <= CLAN.move_range);
        assert!(site_value(&w, &c, after.home, c.pop, &c.store) > now + CLAN.move_margin);
        assert_eq!((after.since, after.hungry), (0, 0));
        assert_eq!(w.tiles[o].claim, None);
        assert_eq!(w.tiles[out(3, 1)].claim, None);
        check_rounds(&w);
    }
}

#[test]
fn a_big_clan_divides_its_land() {
    let mut w = flat();
    let round: Vec<usize> = std::iter::once(at(0, 0)).chain((0..6).map(|d| out(d, 1))).chain([out(0, 2)]).collect();
    let id = put(&mut w, &round, CLAN.split_at);
    let c = w.clans.get_mut(&id).unwrap();
    c.basket = [10.0, 10.0, 10.0, 0.0, 0.0];
    c.since = 5;
    let pop = CLAN.split_at * (1.0 + growth(6));
    let store: f64 = 3.0 * (10.0 - CLAN.diet_share * CLAN.split_at) * (1.0 - CLAN.spoilage);
    eat(&mut w);
    assert_eq!(w.clans.len(), 2);
    let (m, d) = (&w.clans[&id], w.clans.values().find(|c| c.id != id).unwrap());
    assert_eq!(m.home, at(0, 0));
    assert!(m.round.len() >= 1 && d.round.len() >= 1);
    assert!(m.round.iter().all(|t| !d.round.contains(t)));
    assert!(m.round.len() + d.round.len() <= round.len());
    assert!((m.pop + d.pop - pop).abs() < EPS && (m.pop - d.pop).abs() < EPS);
    assert!((total(&m.store) + total(&d.store) - store).abs() < EPS);
    assert_eq!(d.since, m.since);
    assert!(d.fed);
    check_rounds(&w);
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
        let total_pop: f64 = w.clans.values().map(|c| c.pop).sum();
        assert!((total_pop - pop).abs() < EPS);
        if !food {
            assert_eq!(w.clans.len(), 1, "a daughter with nowhere to go");
            continue;
        }
        assert_eq!(w.clans.len(), 2);
        let d = w.clans.values().find(|c| c.id != id).unwrap();
        assert!((d.pop - pop / 2.0).abs() < EPS);
        assert_eq!(d.round, vec![d.home]);
        assert!(steps(&w, o, d.home) <= CLAN.scout_range);
        assert!(site_value(&w, d, d.home, d.pop, &d.store) >= CLAN.settle_fed);
        assert_eq!(w.clans[&id].hungry, 0);
        check_rounds(&w);
    }
}

#[test]
fn clans_spread() {
    let mut w = World::new(Config { seed: 1, radius: 16 });
    w.run(300);
    assert!(w.clans.len() > 10, "{} clans", w.clans.len());
}
