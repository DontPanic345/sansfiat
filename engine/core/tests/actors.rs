mod common;

use std::collections::BTreeMap;

use common::*;
use engine::actors::{best_drop, change, connected, divide, edge, middle, recentre, walk_cost, Route, Walks};
use engine::clans::{adjust, at_target, value, Clan};
use engine::data::*;
use engine::{Config, World, TURN};

const EPS: f64 = 1e-9;

fn sorted(tiles: &[usize]) -> Vec<usize> {
    let mut t = tiles.to_vec();
    t.sort_unstable();
    t
}

#[test]
fn a_walk_is_there_and_back_at_its_weight() {
    assert_eq!(walk_cost(3.0, 1.0), 6.0);
    assert_eq!(walk_cost(3.0, 0.25), 1.5);
    assert_eq!(walk_cost(0.0, 1.0), 0.0);
}

#[test]
fn labour_is_divided_evenly_and_each_walk_paid_from_its_share() {
    // Four tiles: home, two near, one far; 12 labour.
    let walks = [0.0, 1.0, 2.0, 5.0];
    assert_eq!(divide(12.0, &[], &walks), vec![3.0, 2.0, 1.0, 0.0], "the far tile's walk eats its share");
    // A route is paid before the division: 4 of it leaves 2 a tile.
    let route = Route { to: 0, path: vec![], cost: 4.0 };
    assert_eq!(divide(12.0, std::slice::from_ref(&route), &walks), vec![2.0, 1.0, 0.0, 0.0]);
    // A route dearer than the labour leaves nothing.
    let dear = Route { cost: 20.0, ..route };
    assert_eq!(divide(12.0, &[dear], &walks), vec![0.0; 4]);
    assert!(divide(12.0, &[], &[]).is_empty());
}

#[test]
fn walks_go_over_the_territory_only_cheapest_first() {
    let mut w = flat();
    let o = at(0, 0);
    // A forest in the way, and a way round it over plains.
    let (f, x) = (out(0, 1), out(0, 2));
    w.tiles[f].terrain = MARSH;
    let round_by = [out(1, 1), at(2, -1)];
    let straight = sorted(&[o, f, x]);
    let walks = Walks::new(&w, o, &straight);
    let k = |t: usize, tiles: &[usize]| tiles.iter().position(|&i| i == t).unwrap();
    assert_eq!(walks.cost[k(x, &straight)], 3.0 + 1.0);
    assert_eq!(walks.path(k(x, &straight)), vec![f, x]);
    assert_eq!(walks.path(k(o, &straight)), Vec::<usize>::new());
    assert_eq!(walks.yearly(CLAN.inside)[k(x, &straight)], walk_cost(4.0, CLAN.inside));
    // With the plains tiles too, it goes round.
    let wide = sorted(&[o, f, x, round_by[0], round_by[1]]);
    assert!(steps(&w, round_by[0], round_by[1]) == 1 && steps(&w, round_by[1], x) == 1);
    let walks = Walks::new(&w, o, &wide);
    assert_eq!(walks.cost[k(x, &wide)], 3.0);
    assert_eq!(walks.path(k(x, &wide)), vec![round_by[0], round_by[1], x]);
    // A tile with no way to it over the territory is cut off.
    let cut = sorted(&[o, x]);
    let walks = Walks::new(&w, o, &cut);
    assert!(!walks.connected() && !connected(&w, o, &cut));
    assert!(connected(&w, o, &straight));
}

#[test]
fn the_edge_is_free_passable_land_next_to_the_territory_within_the_radius() {
    let mut w = flat();
    let o = at(0, 0);
    let held = put(&mut w, &[out(3, 2)], 3.0);
    w.tiles[out(1, 1)].terrain = MOUNTAIN;
    w.tiles[out(2, 1)].terrain = SEA;
    let e = edge(&w, o, &[o], 1);
    assert_eq!(e, sorted(&[out(0, 1), out(3, 1), out(4, 1), out(5, 1)]));
    // The tile next to another clan's isn't taken from it, and the radius holds.
    let e = edge(&w, o, &sorted(&[o, out(3, 1)]), 2);
    assert!(!e.contains(&out(3, 2)) && w.tiles[out(3, 2)].claim == Some(held));
    assert!(e.iter().all(|&t| steps(&w, o, t) <= 2));
    assert!(edge(&w, o, &sorted(&[o, out(3, 1)]), 1).iter().all(|&t| steps(&w, o, t) <= 1));
}

/// A line out from home, `tiles` long: home, then `out(0, 1)`, `out(0, 2)` …
fn line(n: i32) -> Vec<usize> {
    (0..n).map(|k| if k == 0 { at(0, 0) } else { out(0, k) }).collect()
}

#[test]
fn the_rule_never_drops_home_or_disconnects() {
    let w = flat();
    let o = at(0, 0);
    let t = sorted(&line(4));
    // A value that rises with every tile dropped: it drops the end of the line, never home or the middle.
    let fewer = |r: &[usize]| -(r.len() as f64);
    let (dropped, _) = best_drop(&w, o, &t, None, &fewer).unwrap();
    assert_eq!(dropped, sorted(&line(3)));
    let after = change(&w, o, &t, false, 2, 0.0, fewer);
    assert_eq!(after, sorted(&line(3)));
    // Even a value that wants the middle gone can't have it.
    let mid = out(0, 1);
    let no_mid = |r: &[usize]| if r.contains(&mid) { 0.0 } else { 1.0 };
    let after = change(&w, o, &t, false, 2, 0.0, no_mid);
    assert!(after.contains(&mid) && after.contains(&o));
    assert!(connected(&w, o, &after));
    // Home alone: nothing to drop.
    assert_eq!(change(&w, o, &[o], false, 2, 0.0, fewer), vec![o]);
}

#[test]
fn the_rule_adds_only_on_a_rise_of_the_margin() {
    let w = flat();
    let o = at(0, 0);
    let target = out(2, 1);
    let likes = |gain: f64| move |r: &[usize]| if r.contains(&target) { gain } else { 0.0 };
    assert_eq!(change(&w, o, &[o], true, 2, 0.1, likes(0.1)), vec![o], "a rise of just the margin");
    assert_eq!(change(&w, o, &[o], true, 2, 0.1, likes(0.2)), sorted(&[o, target]));
    // A fed actor never adds.
    assert_eq!(change(&w, o, &[o], false, 2, 0.1, likes(5.0)), vec![o]);
    // No rise from adding, but swapping a worse tile for it rises: one in, one out.
    let worse = out(5, 1);
    let swap = |r: &[usize]| {
        let mut v = 0.0;
        if r.contains(&target) {
            v += 1.0;
        }
        if r.contains(&worse) {
            v -= 0.5;
        }
        // Holding more costs: adding alone doesn't pay.
        v - (r.len() as f64 - 2.0).max(0.0)
    };
    let t = sorted(&[o, worse]);
    let after = change(&w, o, &t, true, 2, 0.1, swap);
    assert_eq!(after, sorted(&[o, target]));
}

/// A hungry clan of `pop` holding home with grain on it, and nothing else.
fn hungry_clan(pop: f64) -> (World, u32) {
    let mut w = flat();
    let o = at(0, 0);
    w.tiles[o].stock[GRAIN] = 2.0;
    let id = put(&mut w, &[o], pop);
    w.clans.get_mut(&id).unwrap().fed = false;
    (w, id)
}

#[test]
fn a_hungry_clan_adds_a_tile_whose_food_is_worth_the_walk() {
    let (mut w, id) = hungry_clan(5.0);
    let before = w.clans[&id].territory.clone();
    adjust(&mut w);
    assert_eq!(w.clans[&id].territory, before, "added a tile with no food");

    let (mut w, id) = hungry_clan(5.0);
    let t = out(2, 1);
    w.tiles[t].stock[FRUIT] = 20.0;
    let c = w.clans[&id].clone();
    let now = value(&w, &c, &c.territory);
    adjust(&mut w);
    let c = &w.clans[&id];
    assert!(c.territory.contains(&t), "{:?}", c.territory);
    assert_eq!(c.territory.len(), 2);
    assert!(value(&w, c, &c.territory) > now + CLAN.land_margin);
    check_territories(&w);
}

#[test]
fn a_poor_far_tile_lowers_the_value_and_is_not_added() {
    // Home has the food; the only edge tile is a marsh with a crumb of game, dear to walk to.
    let (o, m) = (at(0, 0), out(0, 1));
    let mut w = island(&[o, m]);
    w.tiles[o].stock = [10.0, 10.0, 10.0, 0.0, 0.0];
    w.tiles[m].terrain = MARSH;
    w.tiles[m].stock[GAME] = 0.05;
    let id = put(&mut w, &[o], 4.0);
    // A route that leaves it 2 labour: short of its target, so it wants land.
    w.clans.get_mut(&id).unwrap().routes = vec![Route { to: o, path: vec![], cost: 4.0 * CLAN.work - 2.0 }];
    let c = w.clans[&id].clone();
    assert!(value(&w, &c, &sorted(&[o, m])) < value(&w, &c, &[o]), "half the labour to a crumb");
    adjust(&mut w);
    assert_eq!(w.clans[&id].territory, vec![o]);
    assert_eq!(w.tiles[m].claim, None);
}

#[test]
fn a_clan_at_its_target_drops_land_it_cannot_use() {
    let mut w = flat();
    let (o, n) = (at(0, 0), out(0, 1));
    w.tiles[o].stock = [20.0, 20.0, 20.0, 0.0, 0.0];
    w.tiles[n].terrain = FOREST;
    let id = put(&mut w, &[o, n], 5.0);
    // A store that leaves half its labour enough to reach its target.
    w.clans.get_mut(&id).unwrap().store = [CLAN.surplus_target * 5.0 - CLAN.work * 5.0 / 2.0, 0.0, 0.0, 0.0, 0.0];
    assert!(at_target(&w, &w.clans[&id], &[o, n]));
    adjust(&mut w);
    let c = &w.clans[&id];
    assert_eq!(c.territory, vec![o]);
    assert_eq!(w.tiles[n].claim, None);
}

#[test]
fn a_hungry_clan_swaps_its_worst_tile() {
    // An island: home with grain, a fruit tile beside it, and a bare tile beside home.
    let (o, bare, fruit) = (at(0, 0), out(0, 1), out(1, 1));
    let mut w = island(&[o, bare, fruit]);
    w.tiles[o].stock[GRAIN] = 4.0;
    w.tiles[fruit].stock[FRUIT] = 20.0;
    let id = put(&mut w, &[o, bare], 4.0);
    let c = w.clans.get_mut(&id).unwrap();
    // A route that leaves it little labour, so a third tile costs more than it gives.
    c.routes = vec![Route { to: o, path: vec![], cost: 4.0 * CLAN.work - 2.0 }];
    let c = c.clone();
    let all = sorted(&[o, bare, fruit]);
    assert!(value(&w, &c, &all) <= value(&w, &c, &c.territory) + CLAN.land_margin, "adding alone");
    adjust(&mut w);
    assert_eq!(w.clans[&id].territory, sorted(&[o, fruit]), "one tile in, one out");
    assert_eq!(w.tiles[bare].claim, None);
    check_territories(&w);
}

#[test]
fn home_follows_the_middle_a_tile_a_turn() {
    let mut w = flat();
    let o = at(0, 0);
    // A territory off to one side of home, with no food: nothing to add, swap or drop.
    let t = [o, out(0, 1), out(0, 2), at(2, -1), at(1, -1)];
    let id = put(&mut w, &t, 5.0);
    w.clans.get_mut(&id).unwrap().fed = false;
    let mut moved = 0;
    for _ in 0..4 {
        let before = w.clans[&id].clone();
        let m = middle(&w, &before.territory, before.home);
        adjust(&mut w);
        let c = &w.clans[&id];
        assert!(steps(&w, before.home, c.home) <= 1);
        if c.home != before.home {
            moved += 1;
            assert!(steps(&w, c.home, m) < steps(&w, before.home, m));
        }
        check_territories(&w);
    }
    assert!(moved >= 1);
    let c = &w.clans[&id];
    assert_eq!(middle(&w, &c.territory, c.home), c.home);
}

#[test]
fn recentring_drops_tiles_too_far_or_cut_off() {
    let w = flat();
    let o = at(0, 0);
    // A line of four out from home: from home's neighbour the far end is 2 away, but home's other
    // side would be 3.
    let t = sorted(&[out(3, 1), o, out(0, 1), out(0, 2), out(0, 3)]);
    let (home, kept) = recentre(&w, o, &t, 2);
    assert_eq!(home, out(0, 1));
    assert_eq!(kept, sorted(&[out(3, 1), o, out(0, 1), out(0, 2), out(0, 3)]));
    let (home, kept) = recentre(&w, home, &kept, 2);
    assert_eq!(home, out(0, 1), "already in the middle");
    assert!(kept.iter().all(|&t| steps(&w, home, t) <= 2));
    // Radius 1 from the new home cuts the ends off.
    let (_, kept) = recentre(&w, out(0, 1), &kept, 1);
    assert_eq!(kept, sorted(&[o, out(0, 1), out(0, 2)]));
}

/// The territory tiles gained and lost between two territories.
fn changes(a: &[usize], b: &[usize]) -> (usize, usize) {
    (b.iter().filter(|t| !a.contains(t)).count(), a.iter().filter(|t| !b.contains(t)).count())
}

#[test]
fn territories_keep_their_invariants_every_step() {
    for seed in 1..=3 {
        let mut w = World::new(Config { seed, radius: 16 });
        for turn in 0..300 {
            for step in TURN {
                let before: BTreeMap<u32, Clan> = w.clans.clone();
                (step.run)(&mut w);
                check_territories(&w);
                for (id, c) in &w.clans {
                    let Some(b) = before.get(id) else { continue };
                    let (gained, lost) = changes(&b.territory, &c.territory);
                    let at = format!(
                        "seed {seed} turn {turn} {}: clan {id} {:?} -> {:?}",
                        step.name, b.territory, c.territory
                    );
                    match step.name {
                        "move" => {
                            let moved_on = b.hungry >= CLAN.patience && c.territory == [c.home] && c.since == 0;
                            if !moved_on {
                                assert!(gained <= 1, "{at}");
                                assert!(steps(&w, b.home, c.home) <= 1, "{at}");
                                // Only a home that moved drops more than one.
                                if c.home == b.home {
                                    assert!(lost <= 1, "{at}");
                                }
                            }
                        }
                        // Dividing only gives tiles away.
                        "eat" => assert_eq!(gained, 0, "{at}"),
                        _ => assert_eq!(b.territory, c.territory, "{at}"),
                    }
                }
                if step.name == "harvest" {
                    for c in w.clans.values() {
                        for &t in &c.territory {
                            assert!(t == c.home || w.tiles[t].walked, "{seed} {turn}: tile {t} of clan {}", c.id);
                        }
                        for r in &c.routes {
                            assert!(r.path.iter().all(|&t| w.tiles[t].walked));
                            assert!((c.trip() - r.cost).abs() < EPS);
                        }
                    }
                }
            }
        }
        assert!(w.clans.values().all(|c| c.people > 0.0 && c.store.iter().all(|&s| s >= -EPS)));
    }
}
