//! Closed country: valleys behind a pass, sealed valleys, peninsulas on a neck, and the open land
//! that always holds three sites for a centre.

use engine::data::*;
use engine::hex::Hex;
use engine::map::{generate_with, index, open_sites, zone_groups, Generated, Tile, Zone};
use engine::{Config, World};

const KINDS: [(Zone, &str); 3] = [(Zone::Valley, "valleys"), (Zone::Sealed, "sealed"), (Zone::Neck, "necks")];

fn made(radius: i32, seed: u32) -> Generated {
    let mut rng = seed;
    generate_with(radius, &mut rng, 0x5bd1e995)
}

/// The per-small count of a kind and its floor.
fn target(zone: Zone, tiles: usize) -> usize {
    match zone {
        Zone::Valley => scaled(MAP.valleys, tiles).max(MAP.valleys_floor),
        Zone::Sealed => scaled(MAP.sealed, tiles).max(MAP.sealed_floor),
        _ => scaled(MAP.necks, tiles).max(MAP.necks_floor),
    }
}

fn nbrs(tiles: &[Tile], radius: i32, i: usize) -> Vec<usize> {
    tiles[i].hex().neighbors().filter_map(|h| index(radius, h)).collect()
}

fn passable(t: &Tile) -> bool {
    TERRAIN[t.terrain].move_cost.is_some()
}

/// The tiles reached over passable tiles from `from`, never entering a `barred` one.
fn reach(tiles: &[Tile], radius: i32, from: &[usize], barred: impl Fn(usize) -> bool) -> Vec<bool> {
    let mut seen = vec![false; tiles.len()];
    let mut todo: Vec<usize> = from.to_vec();
    for &i in from {
        seen[i] = true;
    }
    while let Some(i) = todo.pop() {
        for j in nbrs(tiles, radius, i) {
            if !seen[j] && passable(&tiles[j]) && !barred(j) {
                seen[j] = true;
                todo.push(j);
            }
        }
    }
    seen
}

/// The largest connected group of passable open tiles: the open outside.
fn outside(tiles: &[Tile], radius: i32) -> Vec<usize> {
    let open = |i: usize| tiles[i].zone == Zone::Open && TERRAIN[tiles[i].terrain].land;
    let mut seen = vec![false; tiles.len()];
    let mut best = Vec::new();
    for s in 0..tiles.len() {
        if seen[s] || !open(s) {
            continue;
        }
        let r = reach(tiles, radius, &[s], |j| !open(j));
        let group: Vec<usize> = (0..tiles.len()).filter(|&j| r[j]).collect();
        for &j in &group {
            seen[j] = true;
        }
        if group.len() > best.len() {
            best = group;
        }
    }
    best
}

#[test]
fn every_size_has_each_kind_kept_apart() {
    let mut totals = Vec::new();
    for &(name, radius) in &SIZES {
        let mut total = 0;
        let mut line = format!("{name}:");
        for (zone, what) in KINDS {
            let (mut pooled, mut maps) = (0, 0);
            let mut want = 0;
            for seed in 1..=15 {
                let g = made(radius, seed);
                want = target(zone, g.tiles.len());
                let k = zone_groups(&g.tiles, radius, zone).len();
                pooled += k;
                maps += (k > 0) as usize;
            }
            line += &format!(" {what} {pooled} (target {}), {} of 15 maps lack them;", 15 * want, 15 - maps);
            assert!(pooled as f64 >= 15.0 * want as f64 * 0.7, "{name}: {pooled} {what}, want {}", 15 * want);
            assert!(maps >= 12, "{name}: only {maps} of 15 maps have {what}");
            total += pooled;
        }
        println!("{line}");
        totals.push(total);
    }
    assert!(totals.windows(2).all(|w| w[1] > w[0]), "larger maps have more closed country: {totals:?}");

    // Features don't touch or overlap: tiles of different features are 3+ steps apart, and 3+ from the rim.
    for &(name, radius) in &SIZES {
        for seed in 1..=15 {
            let g = made(radius, seed);
            let groups: Vec<Vec<usize>> = KINDS.iter().flat_map(|&(z, _)| zone_groups(&g.tiles, radius, z)).collect();
            for (a, ga) in groups.iter().enumerate() {
                for &i in ga {
                    assert!(
                        radius - g.tiles[i].hex().distance(Hex::ORIGIN) >= 3,
                        "{name} seed {seed}: tile {i} by the rim"
                    );
                }
                for gb in &groups[a + 1..] {
                    for &i in ga {
                        for &j in gb {
                            let d = g.tiles[i].hex().distance(g.tiles[j].hex());
                            assert!(d >= MAP.closed_clearance, "{name} seed {seed}: tiles {i} and {j} {d} apart");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn a_valley_is_entered_only_by_its_passes() {
    let mut seen = 0;
    for &(name, radius) in &SIZES {
        for seed in 1..=15 {
            let g = made(radius, seed);
            let tiles = &g.tiles;
            let out = outside(tiles, radius);
            for group in zone_groups(tiles, radius, Zone::Valley) {
                let say = format!("{name} seed {seed} valley at {}", group[0]);
                let passes: Vec<usize> = group.iter().copied().filter(|&i| tiles[i].pass).collect();
                assert!((1..=2).contains(&passes.len()), "{say}: {} passes", passes.len());
                for &p in &passes {
                    assert_eq!(tiles[p].terrain, HILLS, "{say}: pass {p}");
                }
                let inside: Vec<usize> =
                    group.iter().copied().filter(|&i| TERRAIN[tiles[i].terrain].land && !tiles[i].pass).collect();
                // The wall: everything round the inside is mountain, water or a pass.
                for &i in &inside {
                    for j in nbrs(tiles, radius, i) {
                        let t = &tiles[j];
                        assert!(
                            inside.contains(&j) || t.pass || matches!(t.terrain, MOUNTAIN | SEA),
                            "{say}: {j} on the wall is {}",
                            TERRAIN[t.terrain].name
                        );
                        if t.terrain == MOUNTAIN || t.pass {
                            assert_eq!(t.zone, Zone::Valley, "{say}: wall {j} untagged");
                        }
                    }
                }
                let through = reach(tiles, radius, &out, |_| false);
                assert!(inside.iter().all(|&i| through[i]), "{say}: not reached through its pass");
                let barred = reach(tiles, radius, &out, |j| tiles[j].pass);
                assert!(inside.iter().all(|&i| !barred[i]), "{say}: reached without its pass");
                seen += 1;
            }
            // Passes are on valley walls only.
            assert!(tiles.iter().all(|t| !t.pass || t.zone == Zone::Valley));
        }
    }
    println!("{seen} valleys checked");
}

#[test]
fn a_sealed_valley_has_no_way_in_but_a_bridge() {
    let mut seen = 0;
    for &(name, radius) in &SIZES {
        for seed in 1..=15 {
            let g = made(radius, seed);
            let tiles = &g.tiles;
            let out = outside(tiles, radius);
            let from_out = reach(tiles, radius, &out, |_| false);
            for group in zone_groups(tiles, radius, Zone::Sealed) {
                let say = format!("{name} seed {seed} sealed valley at {}", group[0]);
                assert!(group.iter().all(|&i| !tiles[i].pass), "{say}: a pass");
                let inside: Vec<usize> = group.iter().copied().filter(|&i| TERRAIN[tiles[i].terrain].land).collect();
                assert!(inside.len() >= 7, "{say}: {} land tiles", inside.len());
                assert!(
                    inside
                        .iter()
                        .any(|&i| matches!(tiles[i].terrain, PLAINS | FOREST) && tiles[i].cap.iter().any(|&c| c > 0.0)),
                    "{say}: no food"
                );
                assert!(inside.iter().all(|&i| !from_out[i]), "{say}: walked into");
                for &i in &inside {
                    for j in nbrs(tiles, radius, i) {
                        assert!(
                            inside.contains(&j) || matches!(tiles[j].terrain, MOUNTAIN | SEA),
                            "{say}: {j} on the wall"
                        );
                    }
                }
                let bridges =
                    group.iter().filter(|&&i| tiles[i].terrain == MOUNTAIN && tiles[i].span == Some(1)).count();
                assert!(bridges >= 1, "{say}: no wall mountain to bridge");
                for &i in group.iter().filter(|&&i| tiles[i].span == Some(1)) {
                    assert_eq!(tiles[i].trail_cost, Some(MAP.mountain_cost as f64));
                }
                seen += 1;
            }
        }
    }
    println!("{seen} sealed valleys checked");
}

#[test]
fn a_peninsula_hangs_on_a_neck_one_or_two_wide() {
    let mut widths = [0; 3];
    for &(name, radius) in &SIZES {
        for seed in 1..=15 {
            let g = made(radius, seed);
            let tiles = &g.tiles;
            let land = |i: usize| TERRAIN[tiles[i].terrain].land;
            let from_out = reach(tiles, radius, &outside(tiles, radius), |_| false);
            for group in zone_groups(tiles, radius, Zone::Neck) {
                let say = format!("{name} seed {seed} neck at {}", group[0]);
                assert!(group.iter().all(|&i| land(i)), "{say}: tagged water or mountain");
                // The neck: its tiles with land outside the zone.
                let neck: Vec<usize> = group
                    .iter()
                    .copied()
                    .filter(|&i| nbrs(tiles, radius, i).into_iter().any(|j| land(j) && tiles[j].zone != Zone::Neck))
                    .collect();
                assert!((1..=2).contains(&neck.len()), "{say}: neck {} wide", neck.len());
                widths[neck.len()] += 1;
                let blob: Vec<usize> = group.iter().copied().filter(|i| !neck.contains(i)).collect();
                assert!((MAP.neck_land.0..=MAP.neck_land.1 + 2).contains(&group.len()), "{say}: {} tiles", group.len());
                assert!(group.iter().all(|&i| from_out[i]), "{say}: not joined to the open land");
                // Without the neck, the blob is cut off.
                let cut = reach(tiles, radius, &blob[..1], |j| neck.contains(&j));
                assert!((0..tiles.len()).all(|j| !cut[j] || blob.contains(&j)), "{say}: joined without its neck");
                assert!(blob.iter().all(|&j| cut[j]), "{say}: blob in pieces");
            }
        }
    }
    println!("necks 1 wide: {}, 2 wide: {}", widths[1], widths[2]);
}

#[test]
fn the_open_land_always_holds_three_sites() {
    for &(name, radius) in &SIZES {
        let (mut rerolled, mut dropped, mut maps, mut layouts) = (0, 0, 0, 0);
        for seed in 1..=40 {
            let g = made(radius, seed);
            let tiles = &g.tiles;
            let sites = open_sites(tiles, radius);
            assert_eq!(sites.len(), 3, "{name} seed {seed}: {sites:?}");
            for &s in &sites {
                assert_eq!(tiles[s].zone, Zone::Open, "{name} seed {seed}");
                assert!(TERRAIN[tiles[s].terrain].land);
            }
            for (a, &s) in sites.iter().enumerate() {
                for &t in &sites[a + 1..] {
                    assert!(tiles[s].hex().distance(tiles[t].hex()) >= PREGAME.centre_spacing);
                }
            }
            rerolled += g.tries.rerolled() as usize;
            dropped += g.tries.all_open() as usize;
            maps += g.tries.maps;
            layouts += g.tries.layouts;
        }
        println!(
            "{name}, seeds 1..=40: {rerolled} maps needed a re-roll ({layouts} layouts tried in all), \
             {maps} whole maps re-rolled, {dropped} maps kept with no closed country"
        );
        assert_eq!(dropped, 0, "{name}: a map kept with its closed features dropped");
    }
}

#[test]
fn small_radii_have_no_closed_country() {
    for radius in [0, 1, 6, 12, MAP.closed_from - 1] {
        let g = made(radius, 3);
        assert!(g.tiles.iter().all(|t| t.zone == Zone::Open && !t.pass), "radius {radius}");
        assert_eq!((g.tries.layouts, g.tries.dropped, g.tries.maps), (0, 0, 0));
    }
}

#[test]
fn nobody_is_seeded_in_a_sealed_valley() {
    let mut sealed = 0;
    for seed in 1..=40 {
        let w = World::new(Config { seed, radius: 16 });
        sealed += w.tiles.iter().any(|t| t.zone == Zone::Sealed) as usize;
        for c in w.clans.values() {
            assert_ne!(w.tiles[c.home].zone, Zone::Sealed, "seed {seed}: clan {}", c.id);
        }
    }
    assert!(sealed >= 30, "only {sealed} of 40 small maps have a sealed valley");
}

#[test]
fn a_sealed_valley_stays_empty_through_the_pregame() {
    for seed in 1..=5 {
        let mut w = World::new(Config { seed, radius: 16 });
        assert!(w.tiles.iter().any(|t| t.zone == Zone::Sealed), "seed {seed}: no sealed valley");
        while w.end.is_none() && w.turn < PREGAME.max_turns {
            w.run(1);
            for c in w.clans.values() {
                for &t in c.round.iter().chain([&c.home]) {
                    assert_ne!(w.tiles[t].zone, Zone::Sealed, "seed {seed} turn {}: clan {} at {t}", w.turn, c.id);
                }
            }
            assert!(w.tiles.iter().all(|t| t.zone != Zone::Sealed || t.claim.is_none()));
        }
        println!("seed {seed}: ended turn {}", w.turn);
    }
}
