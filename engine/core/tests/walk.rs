//! The per-person walk on the retro maps: a clan that carries more pays more days to its festival.
//! The report (`cargo test --release -p engine --test walk -- --ignored --nocapture`,
//! `relay/specs/154/walk.md`) runs each of the seven maps on the AI to +100 and bins every clan's
//! festival walk by what it brought the turn before (and per person).

use engine::actors::walk_cost;
use engine::data::*;
use engine::land::{passable, path};
use engine::stage::Stage;
use engine::Action;

const MAPS: [(&str, u32); 7] =
    [("small", 2), ("small", 3), ("small", 6), ("small", 9), ("medium", 8), ("medium", 9), ("large", 8)];

#[test]
#[ignore = "a report: run it with --ignored --nocapture"]
fn festival_walk_by_brought() {
    // Bins of what a clan brought (food). `trade` caps it at `MARKET.carry` a person, and nearly every clan
    // brings that cap, so what a person brings is the same for all and what a clan brings is its people.
    let edges = [0.0, 0.5, 1.0, 1.5, 2.0, f64::INFINITY];
    let names = ["under 0.5", "0.5 to 1", "1 to 1.5", "1.5 to 2", "2 and more"];
    // (clans, sum brought, sum brought a person, sum people, sum walkers, sum walk days, sum one-walker days)
    let mut tot = vec![(0u32, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64); names.len()];
    for (size, seed) in MAPS {
        let mut s = Stage::retro(size, seed).unwrap();
        s.act(Action::ChooseCiv { civ: None }).unwrap();
        s.turn(99).unwrap();
        // What each clan brought the turn before: its basket as `go` reads it, capped as `trade` caps it.
        let before: std::collections::BTreeMap<u32, (f64, f64)> = s
            .world()
            .clans
            .values()
            .map(|c| (c.id, (c.basket.iter().sum::<f64>().min(MARKET.carry * c.people), c.people)))
            .collect();
        s.turn(1).unwrap();
        let w = s.world();
        for c in w.clans.values() {
            let (Some(r), Some(&(brought, people))) = (c.routes.first(), before.get(&c.id)) else { continue };
            if r.path.is_empty() {
                continue;
            }
            let Some((_, cost)) = path(w, c.home, r.to, |i| passable(w, i)) else { continue };
            let one = walk_cost(cost, 1.0);
            let b = edges.iter().rposition(|&e| brought >= e).unwrap().min(names.len() - 1);
            let t = &mut tot[b];
            t.0 += 1;
            t.1 += brought;
            t.2 += brought / people;
            t.3 += people;
            t.4 += r.cost / one;
            t.5 += r.cost / DAY;
            t.6 += one / DAY;
        }
    }
    eprintln!("| brought (food) | clans | mean brought | mean brought a person | mean people | mean walkers | festival walk, days | at one walker, days |");
    eprintln!("|---|---|---|---|---|---|---|---|");
    let (mut last, mut rising) = (0.0, true);
    for (b, t) in tot.iter().enumerate() {
        let n = t.0.max(1) as f64;
        eprintln!(
            "| {} | {} | {:.2} | {:.3} | {:.1} | {:.2} | {:.2} | {:.2} |",
            names[b],
            t.0,
            t.1 / n,
            t.2 / n,
            t.3 / n,
            t.4 / n,
            t.5 / n,
            t.6 / n
        );
        if t.0 > 0 {
            rising &= t.4 / n >= last;
            last = t.4 / n;
        }
    }
    assert!(rising, "the walkers do not rise with what a clan brings");
    let big = &tot[names.len() - 1];
    assert!(big.0 == 0 || big.5 > big.6, "a clan bringing a load or more pays no more than one walker");
}
