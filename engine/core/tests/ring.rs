//! The ring: the tiles from which a civ's line feeds an army with days to spare (`units::ring`). Its
//! shares agree with the line itself (`line_from`) to 1e-9 on the retro pick and on bare plains, and
//! a staged army walked out along plains goes hungry from the first tile whose share is one or more,
//! and not inside. The retro maps' table by band is a report: `cargo test --release -p engine --test
//! ring -- --ignored --nocapture` (`relay/specs/154/ring.md`).

mod common;

use engine::data::*;
use engine::land::passable;
use engine::stage::Stage;
use engine::units;

const EPS: f64 = 1e-9;

/// The seven retro maps: `(size, seed)`.
const MAPS: [(&str, u32); 7] =
    [("small", 2), ("small", 3), ("small", 6), ("small", 9), ("medium", 8), ("medium", 9), ("large", 8)];

/// Whether an army can stand on tile `i` (`line::walkable`): passable land, or a built pass.
fn standable(w: &engine::World, i: usize) -> bool {
    passable(w, i) && (TERRAIN[w.tiles[i].terrain].land || w.tiles[i].trail)
}

/// The ring of `civ` in `stage` agrees with the line a 3-person camp of `civ` has from every tile:
/// same tiles, and each share is `eats / carry` from `line_from`.
fn agree(stage: &mut Stage, civ: u32) -> usize {
    let cap = stage.world().civs[&civ].capital;
    let id = stage.army(civ, cap, 3.0);
    let w = stage.world();
    let ring = units::ring(w, civ);
    let mut inside = 0;
    for i in 0..w.tiles.len() {
        let line = units::line_from(w, id, i);
        match (line, standable(w, i)) {
            (Some((_, carry)), true) => {
                let share = 3.0 * UNIT.food / carry;
                let got = *ring.get(&i).unwrap_or_else(|| panic!("tile {i} is on the line but not in the ring"));
                assert!((got - share).abs() < EPS, "tile {i}: ring {got} against line {share}");
                inside += 1;
            }
            _ => assert!(!ring.contains_key(&i), "tile {i} is in the ring with no line from it"),
        }
    }
    assert_eq!(inside, ring.len());
    assert_eq!(ring.get(&cap), Some(&0.0), "the capital is 0");
    inside
}

#[test]
fn ring_is_the_line_on_the_retro_pick() {
    let mut stage = Stage::retro("small", 2).unwrap();
    let me = *stage.world().civs.keys().next().unwrap();
    stage.lead(me).unwrap();
    let n = agree(&mut stage, me);
    assert!(n > 50, "a civ's ring is most of its near land: {n} tiles");
}

/// Three civs on bare plains, the first led, as `day.rs` stages them: the stage and the civs' ids.
fn plains() -> (Stage, Vec<u32>) {
    let mut s = Stage::flat(common::BIG);
    let ids = s.civs(&[common::at_big(-5, 0), common::at_big(5, 0), common::at_big(0, -10)], 14.0);
    s.lead(ids[0]).unwrap();
    s.plain();
    (s, ids)
}

#[test]
fn ring_is_the_line_for_every_civ_on_plains() {
    let (_, ids) = plains();
    for civ in ids {
        let (mut t, _) = plains();
        agree(&mut t, civ);
    }
}

#[test]
fn a_fallen_civ_has_no_ring() {
    let (s, _) = plains();
    assert!(units::ring(s.world(), 9999).is_empty(), "an unknown civ has no ring");
}

/// A straight line of plains from the led civ's capital: a 3-person army staged `k` steps out, with
/// a full pack and both switches on, goes hungry within 40 turns from the first tile whose share is
/// one or more, and never inside the ring.
#[test]
fn the_walk_out_goes_hungry_past_the_ring() {
    let mut first_past = None;
    // Past 8 steps the walk is on Blue's land, where no line runs: the ring ends there.
    for k in 1..=8 {
        let (mut s, _) = plains();
        let me = s.world().player.unwrap();
        let tile = common::at_big(-5 + k, 0);
        let id = s.army(me, tile, 3.0);
        let share = {
            let w = s.world();
            let (_, carry) = units::line_from(w, id, tile).expect("a line on plains");
            3.0 * UNIT.food / carry
        };
        let mut hungry_at = None;
        for turn in 1..=40 {
            // Bare plains give the civ's store nothing, and its line drains it: it is topped up to its
            // cap each turn, so only the line's reach can starve the army.
            s.world_mut().civs.get_mut(&me).unwrap().actor.store = [CIV.store_cap / N as f64; N];
            s.turn(1).unwrap();
            let Some(u) = s.world().units.get(&id) else { break };
            if u.hungry > 0 {
                hungry_at = Some(turn);
                break;
            }
        }
        if share < 1.0 {
            assert!(hungry_at.is_none(), "{k} steps (share {share:.3}) went hungry at turn {hungry_at:?}");
        } else if first_past.is_none() {
            first_past = Some(k);
            assert!(hungry_at.is_some_and(|t| t < 40), "{k} steps (share {share:.3}) never went hungry in 40 turns");
        }
    }
    assert_eq!(first_past, Some(6), "the first tile past the ring is six steps out on plains");
}

/// The retro maps' table: each civ led in turn, a 3-person army staged on the first land tile within
/// 8 steps of the capital in each band of the ring's share, 20 turns. A turn counts unless the civ's
/// store ends under the army's turn of food (dry). When the army is gone the run stops: a turn its
/// civ's store was dry counts no more, else the turns left count as starving. The bounds: under
/// share 1 the starving rate is under 0.05; at 1.25 and up it is at least 0.2 and four times the rate
/// under 1. Main gives 0.000 under 1 and 0.24–0.38 from 1.25 (`relay/specs/154/probe-ring.md`).
#[test]
#[ignore = "a report: run it with --ignored --nocapture"]
fn retro_bands() {
    let bins = [0.0, 0.5, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 2.0];
    let names = ["under 0.5", "0.5-0.75", "0.75-0.9", "0.9-1", "1-1.1", "1.1-1.25", "1.25-1.5", "1.5-2", "2 and up"];
    // (armies, counted turns, starving, dry, gone) per band.
    let mut tot = vec![(0u32, 0u32, 0u32, 0u32, 0u32); bins.len()];
    for (size, seed) in MAPS {
        let civs: Vec<u32> = Stage::retro(size, seed).unwrap().world().civs.keys().copied().collect();
        for &me in &civs {
            let mut s0 = Stage::retro(size, seed).unwrap();
            s0.lead(me).unwrap();
            let cap = s0.world().civs[&me].capital;
            s0.army(me, cap, 3.0);
            let w = s0.world();
            let ring = units::ring(w, me);
            let mut by_bin: Vec<Vec<usize>> = vec![vec![]; bins.len()];
            for (&i, &share) in &ring {
                if engine::land::steps(w, cap, i) > 8 || !TERRAIN[w.tiles[i].terrain].land {
                    continue;
                }
                let b = bins.iter().rposition(|&x| share >= x).unwrap();
                by_bin[b].push(i);
            }
            for (b, tiles) in by_bin.iter().enumerate() {
                // The first land tile in the band, in tile order.
                let Some(&t) = tiles.iter().min() else { continue };
                let mut s = Stage::retro(size, seed).unwrap();
                s.lead(me).unwrap();
                let id = s.army(me, t, 3.0);
                let (mut turns, mut hungry, mut dry, mut gone) = (0u32, 0u32, 0u32, 0u32);
                for t in 1..=20 {
                    if s.turn(1).is_err() {
                        break;
                    }
                    let dry_now = s.store(me) < 3.0 * UNIT.food;
                    match s.world().units.get(&id) {
                        Some(_) if dry_now => dry += 1,
                        Some(u) => {
                            turns += 1;
                            if u.hungry > 0 {
                                hungry += 1;
                            }
                        }
                        None => {
                            // Gone: a dry store is the store's failing and counts no more turns; else
                            // the turns left are starving.
                            gone = 1;
                            if !dry_now {
                                turns += 20 - t;
                                hungry += 20 - t;
                            }
                            break;
                        }
                    }
                }
                let e = &mut tot[b];
                e.0 += 1;
                e.1 += turns;
                e.2 += hungry;
                e.3 += dry;
                e.4 += gone;
            }
        }
    }
    eprintln!("| share | armies | counted turns | starving | rate | dry turns | gone |");
    eprintln!("|---|---|---|---|---|---|---|");
    for (b, e) in tot.iter().enumerate() {
        eprintln!(
            "| {} | {} | {} | {} | {:.3} | {} | {} |",
            names[b],
            e.0,
            e.1,
            e.2,
            e.2 as f64 / e.1.max(1) as f64,
            e.3,
            e.4
        );
    }
    let under_one = (0..4).map(|b| tot[b].2).sum::<u32>() as f64 / (0..4).map(|b| tot[b].1).sum::<u32>().max(1) as f64;
    let from_125 = (6..9).map(|b| tot[b].2).sum::<u32>() as f64 / (6..9).map(|b| tot[b].1).sum::<u32>().max(1) as f64;
    assert!(under_one < 0.05, "armies starve inside the ring: {under_one:.3}");
    assert!(from_125 >= 0.2, "armies past the ring do not starve: {from_125:.3}");
    assert!(from_125 >= 4.0 * under_one, "past the ring {from_125:.3} is not four times inside {under_one:.3}");
}
