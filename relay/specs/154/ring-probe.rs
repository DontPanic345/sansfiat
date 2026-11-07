// The spec-review's throwaway probe for #154 (relay/specs/154/probe-ring.md): run on main as
// engine/core/tests/zz_ringprobe.rs with --ignored --nocapture. Not on the branch.
use engine::data::*;
use engine::stage::Stage;
use engine::units;

#[test]
#[ignore]
fn ring_probe() {
    let bins = [0.0, 0.5, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 2.0, 9.0];
    let mut tot = vec![(0u32, 0u32, 0u32, 0u32); bins.len()];
    let maps = [("small", 2), ("small", 3), ("small", 6), ("small", 9), ("medium", 8), ("medium", 9), ("large", 8)];
    for (size, seed) in maps {
        let civs: Vec<u32> = Stage::retro(size, seed).unwrap().world().civs.keys().copied().collect();
        for &me in &civs {
            let mut s0 = Stage::retro(size, seed).unwrap();
            s0.lead(me).unwrap();
            let cap = s0.world().civs[&me].capital;
            let id = s0.army(me, cap, 3.0);
            let w = s0.world();
            // shares for every tile within 8 steps
            let mut by_bin: Vec<Vec<usize>> = vec![vec![]; bins.len()];
            for i in 0..w.tiles.len() {
                if engine::land::steps(w, cap, i) > 8 { continue; }
                let Some((_, carry)) = units::line_from(w, id, i) else { continue };
                if !TERRAIN[w.tiles[i].terrain].land { continue; }
                let share = 3.0 * UNIT.food / carry;
                let b = bins.iter().rposition(|&x| share >= x).unwrap();
                by_bin[b].push(i);
            }
            for (b, tiles) in by_bin.iter().enumerate() {
                for &t in tiles.iter().take(1) {
                    let mut s = Stage::retro(size, seed).unwrap(); s.lead(me).unwrap();
                                        let id = s.army(me, t, 3.0);
                    let mut hungry = 0;
                    let mut turns = 0; let mut dry = 0; let mut gone = 0;
                    for _ in 0..20 {
                        if s.turn(1).is_err() { break; }
                        let Some(u) = s.world().units.get(&id) else { if bins[b] < 1.0 { eprintln!("GONE {size} {seed} civ {me} bin {} turn {turns} store {:.1}", bins[b], s.store(me)); } gone += 1; break };
                        turns += 1;
                        if s.store(me) < 3.0 * UNIT.food { dry += 1; continue; }
                        if u.hungry > 0 { hungry += 1; }
                    }
                    if bins[b] < 1.0 && hungry > 0 { eprintln!("IN {size} {seed} civ {me} bin {} tile {t} store0 {:.1} store_end {:.1} hungry {hungry} first_hungry_turns {}", bins[b], s0.store(me), s.store(me), turns); }
                    let e = &mut tot[b];
                    e.0 += 1; e.1 += turns - dry; e.2 += hungry; e.3 += gone;
                }
            }
        }
    }
    for (b, e) in tot.iter().enumerate() {
        eprintln!("share >= {:>4}: armies {:>3} turns {:>4} hungry {:>4} rate {:.3} gone {}", bins[b], e.0, e.1, e.2, e.2 as f64 / e.1.max(1) as f64, e.3);
    }
}
