//! The generated map: sizes, features, deserts in the rain shadow, hidden trail costs.
mod common;

use engine::data::*;
use engine::hex::Hex;
use engine::land::step_cost;
use engine::map::{self, generate, generate_with, index, Tile, Zone};
use engine::{Config, World};

const SEEDS: std::ops::RangeInclusive<u32> = 1..=20;

fn made(radius: i32, seed: u32) -> Vec<Tile> {
    let mut rng = seed;
    generate(radius, &mut rng)
}

fn count(tiles: &[Tile], terrain: usize) -> usize {
    tiles.iter().filter(|t| t.terrain == terrain).count()
}

fn land(tiles: &[Tile]) -> usize {
    tiles.iter().filter(|t| TERRAIN[t.terrain].land).count()
}

fn rim_is_sea(tiles: &[Tile], radius: i32) -> bool {
    tiles.iter().filter(|t| t.hex().distance(Hex::ORIGIN) == radius).all(|t| t.terrain == SEA)
}

#[test]
fn sizes_are_named_and_about_double() {
    let tiles: Vec<usize> = SIZES.iter().map(|&(_, r)| (1 + 3 * r * (r + 1)) as usize).collect();
    assert_eq!(tiles, [SMALL_TILES, 1657, 3367]);
    for pair in tiles.windows(2) {
        let ratio = pair[1] as f64 / pair[0] as f64;
        assert!((1.9..=2.1).contains(&ratio), "area ratio {ratio}");
    }
    assert_eq!(size_radius("medium"), Some(23));
    assert_eq!(size_radius("huge"), None);
    assert_eq!(scaled(3, SMALL_TILES), 3);
    assert_eq!(scaled(3, 3367), 12);
    assert_eq!(scaled(1, 127), 0);
}

#[test]
fn every_size_has_a_sea_rim_and_a_fair_share_of_land() {
    // Found over these seeds: land is 48%..75% of the tiles.
    let (mut lo, mut hi) = (1.0f64, 0.0f64);
    for &(name, radius) in &SIZES {
        for seed in SEEDS {
            let tiles = made(radius, seed);
            assert_eq!(tiles.len() as i32, 1 + 3 * radius * (radius + 1), "{name} seed {seed}");
            assert!(rim_is_sea(&tiles, radius), "{name} seed {seed}: rim");
            let share = land(&tiles) as f64 / tiles.len() as f64;
            lo = lo.min(share);
            hi = hi.max(share);
            assert!((0.40..=0.80).contains(&share), "{name} seed {seed}: land share {share:.2}");
        }
    }
    println!("land share over sizes and seeds 1..=20: {lo:.3}..{hi:.3}");
}

#[test]
fn every_size_has_its_features() {
    for &(name, radius) in &SIZES {
        for seed in 1..=10 {
            let tiles = made(radius, seed);
            let say = format!("{name} seed {seed}");
            let floor = scaled(MAP.ridges, tiles.len()).max(MAP.ridges_floor);
            assert!(count(&tiles, MOUNTAIN) >= floor * 5, "{say}: mountains {}", count(&tiles, MOUNTAIN));
            for (terrain, what) in
                [(HILLS, "hills"), (FOREST, "forest"), (PLAINS, "plains"), (DESERT, "desert"), (COAST, "coast")]
            {
                assert!(count(&tiles, terrain) > 0, "{say}: no {what}");
            }
            assert!(tiles.iter().any(|t| t.lake), "{say}: no lake");
            // The lake flag is exactly the sea the rim's sea doesn't reach.
            let mut reached = vec![false; tiles.len()];
            let mut todo: Vec<usize> =
                (0..tiles.len()).filter(|&i| tiles[i].hex().distance(Hex::ORIGIN) == radius).collect();
            for &i in &todo {
                reached[i] = true;
            }
            while let Some(i) = todo.pop() {
                for n in tiles[i].hex().neighbors().filter_map(|n| index(radius, n)) {
                    if !reached[n] && tiles[n].terrain == SEA {
                        reached[n] = true;
                        todo.push(n);
                    }
                }
            }
            for (i, t) in tiles.iter().enumerate() {
                assert_eq!(t.lake, t.terrain == SEA && !reached[i], "{say}: tile {i}");
            }
        }
    }
}

#[test]
fn deserts_are_a_capped_share_and_sit_in_shadow() {
    let (mut desert, mut other) = ((0.0, 0usize), (0.0, 0usize));
    for seed in SEEDS {
        let radius = 16;
        let mut rng = seed;
        let g = generate_with(radius, &mut rng, 0);
        let terrain: Vec<usize> = g.tiles.iter().map(|t| t.terrain).collect();
        let non_sea = terrain.iter().filter(|&&t| t != SEA).count();
        let deserts = count(&g.tiles, DESERT);
        assert!(deserts as f64 <= MAP.desert_share * non_sea as f64 + 1.0, "seed {seed}: {deserts} of {non_sea}");
        for t in &g.tiles {
            let s = map::shadow(radius, t.hex(), g.wind, &terrain) as f64;
            match t.terrain {
                DESERT => {
                    desert.0 += s;
                    desert.1 += 1;
                }
                PLAINS => {
                    other.0 += s;
                    other.1 += 1;
                }
                _ => {}
            }
        }
    }
    let (d, o) = (desert.0 / desert.1 as f64, other.0 / other.1 as f64);
    println!("mean shadow: desert {d:.3} ({} tiles), other plains {o:.3} ({} tiles)", desert.1, other.1);
    assert!(d > 2.0 * o && d > o + 0.2, "desert {d:.3} vs plains {o:.3}");
}

#[test]
fn hidden_costs_follow_the_terrain() {
    let (mut mountains, mut crossable, mut hills, mut easy) = (0, 0, 0, 0);
    for seed in SEEDS {
        for t in made(33, seed) {
            match t.terrain {
                FOREST => assert!(matches!(t.trail_cost, Some(c) if c == 1.0 || c == 2.0) && t.span.is_none()),
                HILLS => {
                    assert!(matches!(t.trail_cost, Some(c) if c == 1.0 || c == 2.0) && t.span.is_none());
                    hills += 1;
                    easy += (t.trail_cost == Some(1.0)) as usize;
                }
                MOUNTAIN => {
                    mountains += 1;
                    match t.span {
                        Some(s) => {
                            assert!(s == 1 || s == 2);
                            assert_eq!(t.trail_cost, Some(3.0));
                            crossable += 1;
                        }
                        None => assert_eq!(t.trail_cost, None),
                    }
                }
                _ => assert!(t.trail_cost.is_none() && t.span.is_none(), "terrain {}", t.terrain),
            }
        }
    }
    let (c, e) = (crossable as f64 / mountains as f64, easy as f64 / hills as f64);
    println!("large, 20 seeds: {crossable} of {mountains} mountains crossable ({c:.3}), {easy} of {hills} hills at 1 ({e:.3})");
    assert!((0.1..=0.35).contains(&c), "crossable {c}");
    assert!((0.2..=0.5).contains(&e), "easy hills {e}");
}

#[test]
fn the_hidden_rolls_change_nothing_else() {
    for &(_, radius) in &SIZES {
        for seed in [1, 2, 3] {
            let (mut a, mut b) = (seed, seed);
            let (ga, gb) = (generate_with(radius, &mut a, 0x5bd1e995), generate_with(radius, &mut b, 0x1234567));
            assert_eq!(a, b, "the main stream moved");
            assert!(ga.tiles.iter().zip(&gb.tiles).any(|(x, y)| x.trail_cost != y.trail_cost || x.span != y.span));
            let blank = |tiles: Vec<Tile>| -> Vec<Tile> {
                tiles.into_iter().map(|t| Tile { trail_cost: None, span: None, ..t }).collect()
            };
            assert_eq!(blank(ga.tiles), blank(gb.tiles));
        }
    }
}

#[test]
fn step_cost_ignores_the_hidden_rolls() {
    let mut w = common::flat();
    let (from, i) = (common::at(1, 0), common::at(1, 1));
    w.tiles[i].terrain = FOREST;
    let base = step_cost(&w, from, i);
    assert_eq!(base, Some(2.0));
    for (cost, span) in [(Some(1.0), None), (Some(3.0), Some(2)), (None, Some(1))] {
        w.tiles[i].trail_cost = cost;
        w.tiles[i].span = span;
        assert_eq!(step_cost(&w, from, i), base);
    }
    // Worn trails still bring forest and hills down to their hidden cost.
    for terrain in [FOREST, HILLS] {
        w.tiles[i].terrain = terrain;
        w.tiles[i].trail_cost = Some(1.0);
        w.tiles[i].trail = true;
        assert_eq!(step_cost(&w, from, i), Some(1.0));
    }
}

#[test]
fn mountains_are_not_land_and_split_landmasses() {
    assert!(!TERRAIN[MOUNTAIN].land && TERRAIN[MOUNTAIN].move_cost.is_none());
    assert!(TERRAIN[DESERT].land && TERRAIN[DESERT].move_cost == Some(1.0));
    assert!(TERRAIN[MARSH].land && TERRAIN[MARSH].move_cost == Some(3.0));
    assert_eq!(TERRAIN[DESERT].food, [0.0; N]);
    let tiles = made(16, 1);
    let masses = map::landmasses(&tiles, 16);
    assert!(masses.len() > 1);
    assert_eq!(masses, map::landmasses(&tiles, 16));
    assert_eq!(masses.iter().map(Vec::len).sum::<usize>(), land(&tiles));
}

#[test]
fn caps_follow_terrain_and_fertility() {
    for t in made(16, 4) {
        let river = if t.river > 0 { FEATURES.river } else { [0.0; N] };
        for (f, extra) in river.into_iter().enumerate() {
            let want = ((TERRAIN[t.terrain].food[f] + extra) * t.fertility * 10.0).round() / 10.0;
            assert_eq!(t.cap[f], want);
        }
        assert_eq!(t.stock, t.cap);
        assert_eq!(t.wild_grain, t.cap[GRAIN]);
        if t.flood {
            assert_eq!(t.fertility, MAP.flood_fertility);
        } else {
            assert!((FERTILITY.0..=FERTILITY.1).contains(&t.fertility));
        }
    }
}

#[test]
fn tiny_maps_generate() {
    for radius in 0..=8 {
        for seed in 1..=20 {
            let tiles = made(radius, seed);
            assert_eq!(tiles.len() as i32, 1 + 3 * radius * (radius + 1));
            assert!(rim_is_sea(&tiles, radius));
        }
    }
}

#[test]
fn every_landmass_that_can_feed_has_a_clan() {
    for &(name, radius) in &SIZES {
        for seed in 1..=5 {
            let w = World::new(Config { seed, radius, army: true, gifts: true });
            for mass in map::landmasses(&w.tiles, radius) {
                // Nobody is seeded in a sealed valley (a landmass of its own).
                let fed =
                    mass.iter().any(|&i| w.tiles[i].cap.iter().any(|&c| c > 0.0) && w.tiles[i].zone != Zone::Sealed);
                if mass.len() >= CLAN.min_landmass && fed {
                    assert!(
                        w.clans.values().any(|c| mass.contains(&c.home)),
                        "{name} seed {seed}: empty landmass of {}",
                        mass.len()
                    );
                }
            }
            assert!(w.clans.len() >= CLAN.seeds, "{name} seed {seed}");
        }
    }
}

#[test]
fn same_seed_same_tiles_and_replay_matches() {
    for &(_, radius) in &SIZES {
        assert_eq!(made(radius, 7), made(radius, 7));
        assert_ne!(made(radius, 7), made(radius, 8));
    }
    let mut w = World::new(Config { seed: 3, radius: 16, army: true, gifts: true });
    w.run(40);
    assert_eq!(World::replay(&w.save()).unwrap(), w);
}
