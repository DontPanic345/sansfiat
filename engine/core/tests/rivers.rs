//! Rivers: they run to the water and never double back, they feed and drain lakes, they make
//! land richer, the big ones flood and end in deltas, and crossing one costs labour.
mod common;

use std::collections::VecDeque;

use common::*;
use engine::data::*;
use engine::land::{cost_map, step_cost};
use engine::map::{generate, index, Tile};
use engine::markets::reach;
use engine::round::{cost, leg};
use engine::World;

const SEEDS: std::ops::RangeInclusive<u32> = 1..=15;

fn made(radius: i32, seed: u32) -> Vec<Tile> {
    let mut rng = seed;
    generate(radius, &mut rng)
}

fn nbrs(tiles: &[Tile], radius: i32, i: usize) -> Vec<usize> {
    tiles[i].hex().neighbors().filter_map(|n| index(radius, n)).collect()
}

fn land(t: &Tile) -> bool {
    TERRAIN[t.terrain].land
}

fn ocean(t: &Tile) -> bool {
    t.terrain == SEA && !t.lake
}

/// Steps to the nearest `water` tile over land, as the generator's rivers should see it.
fn distance(tiles: &[Tile], radius: i32, water: impl Fn(&Tile) -> bool) -> Vec<Option<u32>> {
    let mut dist = vec![None; tiles.len()];
    let mut todo = VecDeque::new();
    for (i, t) in tiles.iter().enumerate() {
        if water(t) {
            dist[i] = Some(0);
            todo.push_back(i);
        }
    }
    while let Some(i) = todo.pop_front() {
        for j in nbrs(tiles, radius, i) {
            if dist[j].is_none() && land(&tiles[j]) {
                dist[j] = Some(dist[i].unwrap() + 1);
                todo.push_back(j);
            }
        }
    }
    dist
}

/// The tiles from `i` down its river, ending with the water it empties into.
fn chain(tiles: &[Tile], i: usize) -> Vec<usize> {
    let mut out = vec![i];
    let mut here = i;
    while let Some(j) = tiles[here].flow {
        assert!(!out.contains(&j), "river through {i} revisits {j}");
        out.push(j);
        here = j;
    }
    out
}

fn upstream(tiles: &[Tile], i: usize) -> Vec<usize> {
    (0..tiles.len()).filter(|&j| tiles[j].flow == Some(i)).collect()
}

#[test]
fn rivers_run_down_to_the_water_and_never_double_back() {
    for &(name, radius) in &SIZES {
        let mut joins = 0;
        for seed in SEEDS {
            let tiles = made(radius, seed);
            let say = format!("{name} seed {seed}");
            let all = distance(&tiles, radius, |t| t.terrain == SEA);
            let sea = distance(&tiles, radius, ocean);
            let biggest = tiles.iter().map(|t| t.river).max().unwrap();
            assert!(biggest >= 5, "{say}: the biggest river has {biggest} tiles");
            for (i, t) in tiles.iter().enumerate() {
                assert_eq!(t.river > 0, t.flow.is_some(), "{say}: tile {i}");
                if t.river == 0 {
                    continue;
                }
                assert!(land(t), "{say}: river on terrain {} at {i}", t.terrain);
                let feeders = upstream(&tiles, i);
                assert_eq!(t.river, 1 + feeders.iter().map(|&j| tiles[j].river).sum::<u32>(), "{say}: {i}");
                joins += (feeders.len() >= 2) as usize;
                let path = chain(&tiles, i);
                let (&end, course) = path.split_last().unwrap();
                assert_eq!(tiles[end].terrain, SEA, "{say}: the river through {i} ends on land");
                for pair in path.windows(2) {
                    let (a, b) = (pair[0], pair[1]);
                    assert!(course.contains(&a) && tiles[a].river > 0);
                    assert!(nbrs(&tiles, radius, a).contains(&b), "{say}: {a} flows to {b}, not a neighbour");
                    // A lake's outflow runs down the distance to the ocean; every other river
                    // down the distance to any water.
                    let down = |d: &[Option<u32>]| d[b].is_some_and(|x| d[a].is_some_and(|y| x <= y));
                    assert!(down(&all) || down(&sea), "{say}: {a} to {b} climbs away from the water");
                }
            }
        }
        println!("{name}: {joins} confluences over {} seeds", SEEDS.count());
        if name == "large" {
            assert!(joins > 0, "no rivers join on large maps");
        }
    }
}

#[test]
fn rivers_start_inland_and_never_touch_a_mountain() {
    for &(name, radius) in &SIZES {
        for seed in SEEDS {
            let tiles = made(radius, seed);
            let dist = distance(&tiles, radius, |t| t.terrain == SEA);
            let deepest = dist.iter().flatten().copied().max().unwrap();
            let least = MAP.river_min.min(deepest - 1);
            for (i, t) in tiles.iter().enumerate() {
                if t.river == 0 {
                    continue;
                }
                assert!(t.flow.is_some_and(|j| tiles[j].terrain != MOUNTAIN), "{name} seed {seed}: {i}");
                // A source is a river's top: nothing flows into it. Beside a lake it is the
                // lake's outflow; any other starts `river_min` from water, or as far as the land allows.
                let lake = nbrs(&tiles, radius, i).iter().any(|&j| tiles[j].lake);
                if upstream(&tiles, i).is_empty() && !lake {
                    assert!(dist[i].unwrap() >= least, "{name} seed {seed}: source {i} at {:?}", dist[i]);
                }
            }
        }
    }
}

#[test]
fn lakes_are_fed_and_drained() {
    let (mut fed, mut drained) = (0, 0);
    for seed in 1..=20 {
        let tiles = made(16, seed);
        fed += tiles.iter().filter(|t| t.flow.is_some_and(|j| tiles[j].lake)).count();
        for (i, t) in tiles.iter().enumerate() {
            let beside = nbrs(&tiles, 16, i).iter().any(|&j| tiles[j].lake);
            // Only an outflow starts beside water: a river's source is far from it.
            if t.river > 0 && beside && upstream(&tiles, i).is_empty() {
                drained += 1;
            }
        }
    }
    println!("small, 20 seeds: {fed} rivers end in a lake, {drained} lakes drain");
    assert!(fed > 0 && drained > 0);
}

#[test]
fn river_tiles_are_richer_than_their_banks() {
    for &(name, radius) in &SIZES {
        let (mut river, mut banks) = ((0.0, 0), (0.0, 0));
        for seed in 1..=20 {
            let tiles = made(radius, seed);
            for (i, t) in tiles.iter().enumerate() {
                if t.river == 0 || t.terrain == COAST || t.flood {
                    continue;
                }
                river.0 += t.cap.iter().sum::<f64>();
                river.1 += 1;
                for j in nbrs(&tiles, radius, i) {
                    if land(&tiles[j]) && tiles[j].river == 0 {
                        banks.0 += tiles[j].cap.iter().sum::<f64>();
                        banks.1 += 1;
                    }
                }
            }
        }
        let (r, b) = (river.0 / river.1 as f64, banks.0 / banks.1 as f64);
        println!("{name}: a river tile's caps {r:.2} ({} tiles), its banks' {b:.2}", river.1);
        assert!(r > b, "{name}: river {r:.2} vs banks {b:.2}");
    }
}

#[test]
fn floodplains_are_the_richest_land_and_deltas_are_marsh_on_the_ocean() {
    let mut found = Vec::new();
    for &(name, radius) in &SIZES {
        let (mut floods, mut deltas) = (0, 0);
        for seed in SEEDS {
            let tiles = made(radius, seed);
            let say = format!("{name} seed {seed}");
            let richest = tiles.iter().filter(|t| land(t) && !t.flood).map(|t| t.fertility).fold(0.0, f64::max);
            let poorest = tiles.iter().filter(|t| t.flood).map(|t| t.fertility).fold(f64::INFINITY, f64::min);
            assert!(poorest >= richest, "{say}: floodplain {poorest} vs {richest}");
            let mouth = |i: usize| {
                let t: &Tile = &tiles[i];
                t.river >= MAP.delta_river && t.flow.is_some_and(|j| ocean(&tiles[j]))
            };
            let to_mouth = |i: usize| chain(&tiles, i).len() as u32 - 2;
            for (i, t) in tiles.iter().enumerate() {
                assert!(!t.flood || land(t), "{say}: {i}");
                if t.terrain == MARSH {
                    // Marsh is only ever a delta.
                    assert!(t.flood, "{say}: {i}");
                    assert!(
                        nbrs(&tiles, radius, i).iter().any(|&j| ocean(&tiles[j])),
                        "{say}: delta {i} off the ocean"
                    );
                    assert!(
                        mouth(i) || nbrs(&tiles, radius, i).into_iter().any(mouth),
                        "{say}: delta {i} by no big mouth"
                    );
                }
                if t.river >= MAP.big_river && to_mouth(i) <= MAP.flood_reach {
                    assert!(t.flood, "{say}: {i} should flood");
                    for j in nbrs(&tiles, radius, i) {
                        assert!(tiles[j].flood || !land(&tiles[j]), "{say}: {j} beside {i} should flood");
                    }
                }
                if t.flood && t.terrain != MARSH {
                    assert!(!matches!(t.terrain, COAST | DESERT), "{say}: {i}");
                }
            }
            floods += tiles.iter().any(|t| t.flood && t.terrain != MARSH) as usize;
            deltas += tiles.iter().any(|t| t.terrain == MARSH) as usize;
        }
        println!("{name}: {floods} of {} seeds have a floodplain, {deltas} a delta", SEEDS.count());
        found.push((floods, deltas));
    }
    // Found over seeds 1..=15: small 11 and 4, medium 15 and 8, large 15 and 15.
    let [small, medium, large] = found[..] else { unreachable!() };
    assert!(small.0 >= 8 && small.1 >= 3, "small {small:?}");
    assert!(medium.0 >= 12 && medium.1 >= 5, "medium {medium:?}");
    assert!(large.0 >= 14 && large.1 >= 12, "large {large:?}");
}

// Movement, on a hand-built river: the column q = 0 from rim to rim, flowing to r = +6 (sea).

/// `flat()` with a river down the middle, from `at(0, -5)` to `at(0, 5)`, into the sea at `at(0, 6)`.
fn river_world() -> World {
    let mut w = flat();
    for r in -5..=5 {
        let t = &mut w.tiles[at(0, r)];
        t.river = (r + 6) as u32;
        t.flow = Some(at(0, r + 1));
    }
    assert_eq!(w.tiles[at(0, 6)].terrain, SEA);
    w
}

#[test]
fn entering_a_river_from_the_bank_costs_the_crossing() {
    let mut w = river_world();
    let (bank, here, up, down) = (at(-1, 0), at(0, 0), at(0, -1), at(0, 1));
    assert_eq!(step_cost(&w, bank, here), Some(1.0 + RIVER.cross));
    assert_eq!(step_cost(&w, up, here), Some(1.0), "down the course");
    assert_eq!(step_cost(&w, down, here), Some(1.0), "up the course");
    assert_eq!(step_cost(&w, here, at(1, 0)), Some(1.0), "leaving to the bank");
    // A tributary flowing in is along the course; a river beside it that doesn't flow in isn't.
    let (side, other) = (at(1, -1), at(-1, 1));
    w.tiles[side].river = 1;
    w.tiles[side].flow = Some(here);
    w.tiles[other].river = 1;
    w.tiles[other].flow = Some(at(-2, 1));
    assert_eq!(step_cost(&w, side, here), Some(1.0));
    assert_eq!(step_cost(&w, here, side), Some(1.0));
    assert_eq!(step_cost(&w, here, other), Some(1.0 + RIVER.cross), "into a river not this one's course");
    assert_eq!(step_cost(&w, other, here), Some(1.0 + RIVER.cross));
    // The crossing adds to the terrain; a trail bridges it and caps the terrain.
    w.tiles[here].terrain = FOREST;
    assert_eq!(step_cost(&w, bank, here), Some(2.0 + RIVER.cross));
    assert_eq!(step_cost(&w, up, here), Some(2.0));
    w.tiles[here].trail = true;
    assert_eq!(step_cost(&w, bank, here), Some(TRAIL.cost));
    assert_eq!(step_cost(&w, up, here), Some(TRAIL.cost));
}

#[test]
fn a_path_across_costs_more_and_a_path_along_does_not() {
    let w = river_world();
    let plain = flat();
    let all = |_| true;
    let across = |w: &World| cost_map(w, &[at(-2, 0)], f64::INFINITY, all)[&at(2, 0)];
    assert_eq!(across(&plain), 4.0);
    assert_eq!(across(&w), 4.0 + RIVER.cross);
    let along = |w: &World| cost_map(w, &[at(0, -4)], f64::INFINITY, all)[&at(0, 4)];
    assert_eq!(along(&plain), 8.0);
    assert_eq!(along(&w), 8.0);
    // Out of a bank tile, the river and everything past it cost the crossing.
    let out = cost_map(&w, &[at(-1, 0)], f64::INFINITY, all);
    assert_eq!(out[&at(0, 0)], 1.0 + RIVER.cross);
    assert_eq!(out[&at(1, 0)], 2.0 + RIVER.cross);
    assert_eq!(out[&at(-2, 0)], 1.0);
}

#[test]
fn a_round_costs_the_same_either_way_round() {
    let w = river_world();
    let reversed = |round: &[usize]| -> Vec<usize> {
        let mut r = round.to_vec();
        r[1..].reverse();
        r
    };
    let rounds = [
        // Over the river and back.
        vec![at(-1, 0), at(1, 0), at(1, -1)],
        // Onto the river from the bank, down it a step, off to the same bank: crossed once.
        vec![at(-1, 0), at(0, 0), at(0, 1), at(-1, 1)],
        // Along the river from a home on it.
        vec![at(0, 0), at(0, 1), at(0, 2), at(1, 1)],
        vec![at(0, 0), at(0, -1), at(-1, 0), at(-1, 1)],
        // Through the river from one bank to the other.
        vec![at(-1, 0), at(0, 0), at(1, 0), at(1, -1), at(0, -1)],
    ];
    for round in rounds {
        let (there, back) = (cost(&w, &round), cost(&w, &reversed(&round)));
        assert_eq!(there, back, "{round:?}");
        assert!(there.is_finite());
    }
    // A single leg differs by direction only at a river endpoint.
    assert_eq!(leg(&w, at(-1, 0), at(-1, 0), at(0, 0)), Some(1.0 + RIVER.cross));
    assert_eq!(leg(&w, at(-1, 0), at(0, 0), at(-1, 0)), Some(1.0));
    assert_eq!(leg(&w, at(-1, 0), at(-1, 0), at(1, 0)), leg(&w, at(-1, 0), at(1, 0), at(-1, 0)));
}

#[test]
fn a_trail_on_the_river_bridges_rounds_and_market_reach() {
    let mut w = river_world();
    let (west, ford, east) = (at(-1, 0), at(0, 0), at(1, 0));
    let round = [west, east];
    assert_eq!(cost(&w, &round), 2.0 * (2.0 + RIVER.cross));
    assert!(!reach(&w, west).contains_key(&east), "across the river is out of reach");
    w.tiles[ford].trail = true;
    assert_eq!(cost(&w, &round), 4.0);
    assert_eq!(reach(&w, west)[&east], 2.0);
    // A trail at the course changes nothing along it.
    w.tiles[at(0, 1)].trail = true;
    assert_eq!(step_cost(&w, ford, at(0, 1)), Some(1.0));
}
