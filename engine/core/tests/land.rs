use engine::data::*;
use engine::hex::Hex;
use engine::land::{neighbours, regrow, step_cost};
use engine::map::index;
use engine::{Config, World, GOODS, TERRAIN};

#[test]
fn goods_are_the_five_foods() {
    assert_eq!(GOODS, ["game", "grain", "fruit", "fish", "shellfish"]);
    assert_eq!((GAME, GRAIN, FRUIT, FISH, SHELLFISH), (0, 1, 2, 3, 4));
}

#[test]
fn caps_are_terrain_food_times_fertility() {
    for seed in 1..=3 {
        let w = World::new(Config { seed, radius: 16, army: true, gifts: true });
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
    let w = World::new(Config { seed: 1, radius: 16, army: true, gifts: true });
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
    let mut w = World::new(Config { seed: 1, radius: 8, army: true, gifts: true });
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
    assert_eq!(step_cost(&w, from, plains), Some(1.0));
}

#[test]
fn food_regrows_and_game_breeds() {
    let mut w = World::new(Config { seed: 1, radius: 8, army: true, gifts: true });
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
