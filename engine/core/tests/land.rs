mod common;

use common::{at, flat, island};
use engine::data::*;
use engine::hex::Hex;
use engine::land::{build_trail, neighbours, passable, path, path_by, regrow, step_cost, trailable};
use engine::map::index;
use engine::{units, Config, World, GOODS, TERRAIN};

#[test]
fn goods_are_the_five_foods() {
    assert_eq!(GOODS, ["game", "grain", "fruit", "fish", "shellfish"]);
    assert_eq!((GAME, GRAIN, FRUIT, FISH, SHELLFISH), (0, 1, 2, 3, 4));
}

#[test]
fn caps_are_terrain_food_times_fertility() {
    for seed in 1..=3 {
        let w = World::new(Config { seed, radius: 16, army: true });
        for t in &w.tiles {
            let terrain = &TERRAIN[t.terrain];
            let river = if t.river > 0 { FEATURES.river } else { [0.0; N] };
            for (f, extra) in river.into_iter().enumerate() {
                let want = ((terrain.food[f] + extra) * t.fertility * 10.0).round() / 10.0;
                assert!((t.cap[f] - want).abs() < 1e-9);
            }
            if t.terrain == SEA {
                assert!(t.cap.iter().all(|&c| c == 0.0));
            } else if t.flood {
                assert_eq!(t.fertility, MAP.flood_fertility);
            } else {
                assert!((FERTILITY.0..=FERTILITY.1).contains(&t.fertility), "{}", t.fertility);
            }
            assert_eq!(t.stock, t.cap);
            assert_eq!(t.wild_grain, t.cap[GRAIN]);
        }
    }
}

#[test]
fn fertility_is_patchy() {
    let w = World::new(Config { seed: 1, radius: 16, army: true });
    let f: Vec<f64> = w.tiles.iter().map(|t| t.fertility).collect();
    let (mut near, mut n) = (0.0, 0);
    for i in 0..f.len() {
        for j in engine::land::neighbours(&w, i) {
            near += (f[i] - f[j]).abs();
            n += 1;
        }
    }
    near /= n as f64;
    let (mut far, mut m) = (0.0, 0);
    for i in (0..f.len()).step_by(7) {
        for j in (0..f.len()).step_by(11) {
            far += (f[i] - f[j]).abs();
            m += 1;
        }
    }
    far /= m as f64;
    assert!(near < 0.5 * far, "neighbours {near:.3}, random pairs {far:.3}");
    let lo = f.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = f.iter().copied().fold(0.0, f64::max);
    assert!(lo <= 0.7 && hi >= 1.5, "{lo}..{hi}");
}

#[test]
fn index_matches_within() {
    for radius in [0, 1, 5, 16] {
        for (i, h) in Hex::ORIGIN.within(radius).into_iter().enumerate() {
            assert_eq!(index(radius, h), Some(i), "radius {radius}, {h:?}");
        }
        for h in Hex::ORIGIN.within(radius + 2) {
            if h.distance(Hex::ORIGIN) > radius {
                assert_eq!(index(radius, h), None);
            }
        }
    }
}

#[test]
fn step_costs() {
    let mut w = World::new(Config { seed: 1, radius: 8, army: true });
    let find = |w: &World, t: usize| w.tiles.iter().position(|x| x.terrain == t && x.river == 0).unwrap();
    let sea = find(&w, SEA);
    let plains = find(&w, PLAINS);
    // Off any river, the tile left doesn't matter.
    let from = neighbours(&w, plains).next().unwrap();
    assert_eq!(step_cost(&w, from, sea), None);
    assert_eq!(step_cost(&w, from, plains), Some(1.0));
    w.tiles[plains].terrain = FOREST;
    assert_eq!(step_cost(&w, from, plains), Some(2.0));
    w.tiles[plains].trail = true;
    assert_eq!(step_cost(&w, from, plains), Some(2.0), "a trail with no hidden cost gains nothing");
}

#[test]
fn a_trail_brings_a_tile_to_its_hidden_cost_and_never_raises_it() {
    let mut w = flat();
    let (from, i) = (at(1, 0), at(1, 1));
    w.tiles[i].terrain = FOREST;
    w.tiles[i].trail = true;
    w.tiles[i].trail_cost = Some(1.0);
    assert_eq!(step_cost(&w, from, i), Some(1.0));
    w.tiles[i].trail_cost = Some(3.0);
    assert_eq!(step_cost(&w, from, i), Some(2.0), "a costlier hidden cost leaves the terrain");
    w.tiles[i].terrain = HILLS;
    w.tiles[i].trail_cost = Some(1.0);
    assert_eq!(step_cost(&w, from, i), Some(1.0));
    w.tiles[i].trail = false;
    assert_eq!(step_cost(&w, from, i), Some(2.0), "without a trail, the terrain");
}

#[test]
fn a_mountain_is_passable_only_with_a_trail_and_costs_its_hidden_cost() {
    let mut w = island(&[at(-1, 0), at(0, 0), at(1, 0)]);
    let (west, pass) = (at(-1, 0), at(0, 0));
    w.tiles[pass].terrain = MOUNTAIN;
    w.tiles[pass].span = Some(1);
    w.tiles[pass].trail_cost = Some(MAP.mountain_cost as f64);
    assert!(!passable(&w, pass));
    assert_eq!(step_cost(&w, west, pass), None);
    w.tiles[pass].trail = true;
    assert!(passable(&w, pass));
    assert_eq!(step_cost(&w, west, pass), Some(MAP.mountain_cost as f64));
    w.tiles[pass].trail_cost = None;
    assert!(!passable(&w, pass), "a trail with no hidden cost crosses nothing");
    assert_eq!(step_cost(&w, west, pass), None);
}

#[test]
fn a_trail_goes_only_where_a_hidden_cost_is_rolled_and_a_mountain_span_is_bridged() {
    let mut w = flat();
    let t = at(0, 0);
    assert!(!trailable(&w, t), "plains have no hidden cost");
    w.tiles[t].terrain = FOREST;
    w.tiles[t].trail_cost = Some(1.0);
    assert!(trailable(&w, t));
    w.tiles[t].terrain = MOUNTAIN;
    w.tiles[t].trail_cost = Some(MAP.mountain_cost as f64);
    w.tiles[t].span = Some(CIV.bridge_span);
    assert!(trailable(&w, t));
    w.tiles[t].span = Some(CIV.bridge_span + 1);
    assert!(!trailable(&w, t));
    w.tiles[t].span = None;
    assert!(!trailable(&w, t));
}

#[test]
fn building_a_trail_takes_its_terrains_work_and_starts_the_rest_run_over() {
    let mut w = flat();
    let t = at(0, 0);
    w.tiles[t].terrain = HILLS;
    w.tiles[t].trail_cost = Some(1.0);
    w.tiles[t].rest_run = 200;
    let need = TRAIL.work.iter().find(|&&(terrain, _)| terrain == HILLS).unwrap().1;
    assert!(!build_trail(&mut w, t, need - 0.5));
    assert!(!w.tiles[t].trail);
    assert!(build_trail(&mut w, t, 0.5));
    assert!(w.tiles[t].trail);
    assert_eq!((w.tiles[t].trail_work, w.tiles[t].rest_run), (0.0, 0));

    let plains = at(1, 0);
    assert!(!build_trail(&mut w, plains, 100.0), "plains have no trail to build");
    assert!(!w.tiles[plains].trail);

    let m = at(0, 1);
    w.tiles[m].terrain = MOUNTAIN;
    w.tiles[m].trail_cost = Some(MAP.mountain_cost as f64);
    w.tiles[m].span = Some(CIV.bridge_span + 1);
    assert!(!build_trail(&mut w, m, 100.0), "a span too wide for the bridge");
    w.tiles[m].span = Some(CIV.bridge_span);
    assert!(build_trail(&mut w, m, TRAIL.work[2].1));
    assert!(passable(&w, m));
}

#[test]
fn a_camp_crosses_a_built_pass_and_no_unbuilt_one() {
    let (west, pass, east) = (at(-1, 0), at(0, 0), at(1, 0));
    let mut w = island(&[at(-2, 0), west, east, at(2, 0)]);
    w.tiles[pass].terrain = MOUNTAIN;
    w.tiles[pass].span = Some(1);
    w.tiles[pass].trail_cost = Some(MAP.mountain_cost as f64);
    let (a, b) = (at(-2, 0), at(2, 0));
    assert!(units::path_between(&w, a, b).is_none(), "no way over an unbuilt mountain");
    w.tiles[pass].trail = true;
    let (tiles, cost) = units::path_between(&w, a, b).unwrap();
    assert_eq!(tiles, vec![west, pass, east, b]);
    assert_eq!(cost, 1.0 + MAP.mountain_cost as f64 + 1.0 + 1.0);
}

#[test]
fn path_by_is_path_on_the_real_cost() {
    let mut w = World::new(Config { seed: 2, radius: 12, army: true });
    for i in 0..w.tiles.len() {
        if i % 3 == 0 && w.tiles[i].trail_cost.is_some() {
            w.tiles[i].trail = true;
        }
    }
    let n = w.tiles.len();
    let mut found = 0;
    for from in (0..n).step_by(17) {
        for to in (0..n).step_by(29) {
            let by = path_by(&w, from, to, |i, j| step_cost(&w, i, j));
            assert_eq!(path(&w, from, to, |_| true), by, "{from} to {to}");
            let some = |j: usize| j % 4 != 1;
            let only = path_by(&w, from, to, |i, j| if some(j) { step_cost(&w, i, j) } else { None });
            assert_eq!(path(&w, from, to, some), only, "{from} to {to}, some tiles closed");
            found += by.is_some() as usize;
        }
    }
    assert!(found > 0);
}

#[test]
fn food_regrows_and_game_breeds() {
    let mut w = World::new(Config { seed: 1, radius: 8, army: true });
    let i = w.tiles.iter().position(|t| t.cap[GRAIN] > 0.0 && t.cap[GAME] > 0.0).unwrap();
    w.clans.clear();
    let cap = w.tiles[i].cap;
    w.tiles[i].stock[GRAIN] = 0.0;
    w.tiles[i].stock[GAME] = 0.1 * cap[GAME];
    regrow(&mut w);
    let s = w.tiles[i].stock;
    assert!((s[GRAIN] - 0.25 * cap[GRAIN]).abs() < 1e-12);
    let g = 0.1 * cap[GAME];
    let want = g + cap[GAME] * 0.02 + 0.35 * g * (1.0 - g / cap[GAME]);
    assert!((s[GAME] - want).abs() < 1e-12);

    let turns_to_cap = |w: &mut World, f: usize| {
        let mut n = 1;
        while w.tiles[i].stock[f] < cap[f] - 1e-9 {
            regrow(w);
            n += 1;
        }
        n
    };
    let game = turns_to_cap(&mut w.clone(), GAME);
    let grain = turns_to_cap(&mut w, GRAIN);
    assert!(game > grain, "game {game} turns, grain {grain}");
}
