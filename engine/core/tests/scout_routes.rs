//! A report: every civ's scout on every retro map is routed from its capital to each rival capital,
//! with the other civs on the AI and the led civ idle, 200 turns past the pick, with the fight off and
//! on. Counted: of the routed errands, how many have their route built by +50, +100 and +200, the turns
//! off the errand (walking to a store or back), the turns waiting at a store (`Filling`, `TooFar`),
//! scouts starved and scouts killed (`relay/specs/337/errand.md`).
//! `cargo test --release -p engine --test scout_routes -- --ignored --nocapture`

use engine::data::SCOUT;
use engine::stage::Stage;
use engine::units;
use engine::{Action, EventKind, Order};
use std::thread;

const MAPS: [(&str, u32); 7] =
    [("small", 2), ("small", 3), ("small", 6), ("small", 9), ("medium", 8), ("medium", 9), ("large", 8)];
const TURNS: u32 = 200;

#[derive(Default, Clone)]
struct Run {
    label: String,
    routed: bool,
    built: Option<u32>,
    off: u32,
    filling: u32,
    toofar: u32,
    starved: usize,
    killed: usize,
    gone: bool,
    at200: String,
}

fn run(size: &str, seed: u32, me_ix: usize, rival_ix: usize, fight: bool) -> Option<Run> {
    let mut stage = Stage::retro(size, seed).unwrap();
    stage.world_mut().config.fight = fight;
    let ids: Vec<u32> = stage.world().civs.keys().copied().collect();
    let (me, rival) = (ids[me_ix], ids[rival_ix]);
    stage.lead(me).unwrap();
    let w = stage.world();
    let (cap, to) = (w.civs[&me].capital, w.civs[&rival].capital);
    let id = units::scout(w, me)?.id;
    let turn0 = w.turn;
    let mut r = Run { label: format!("{size} {seed} civ {me} to {rival}"), ..Run::default() };
    r.routed = stage.act(Action::Order { civ: me, order: Order::Route { unit: id, from: cap, to } }).is_ok();
    if !r.routed {
        return Some(r);
    }
    for _ in 0..TURNS {
        stage.turn(1).unwrap();
        let w = stage.world();
        let Some(u) = w.units.get(&id) else {
            r.gone = true;
            break;
        };
        let s = u.scout().unwrap();
        if r.built.is_none() && s.built {
            r.built = Some(w.turn - turn0);
        }
        r.off += u32::from(s.homing || s.back.is_some());
        r.filling += u32::from(matches!(s.doing, units::Doing::Filling(_)));
        r.toofar += u32::from(matches!(s.doing, units::Doing::TooFar(_)));
    }
    let w = stage.world();
    r.starved = w.events.iter().filter(|e| e.kind == EventKind::ScoutStarved && e.unit == Some(id)).count();
    r.killed = w.events.iter().filter(|e| e.kind == EventKind::ScoutKilled && e.unit == Some(id)).count();
    r.at200 = w.units.get(&id).map_or("gone".to_string(), |u| format!("tile {}", u.actor.home));
    Some(r)
}

fn report(fight: bool) {
    let mut jobs = Vec::new();
    for (size, seed) in MAPS {
        let n = Stage::retro(size, seed).unwrap().world().civs.len();
        for a in 0..n {
            for b in 0..n {
                if a != b {
                    jobs.push((size, seed, a, b));
                }
            }
        }
    }
    let queue = std::sync::Mutex::new(jobs.iter().enumerate().collect::<Vec<_>>());
    let out = std::sync::Mutex::new(Vec::new());
    thread::scope(|sc| {
        let hs: Vec<_> = (0..4)
            .map(|_| {
                sc.spawn(|| loop {
                    let job = queue.lock().unwrap().pop();
                    let Some((i, &(size, seed, a, b))) = job else { break };
                    if let Some(r) = run(size, seed, a, b, fight) {
                        out.lock().unwrap().push((i, r));
                    }
                })
            })
            .collect();
        for h in hs {
            h.join().unwrap();
        }
    });
    let mut v = out.into_inner().unwrap();
    v.sort_by_key(|x| x.0);
    let results: Vec<Run> = v.into_iter().map(|x| x.1).collect();
    println!(
        "fight {} | pack {} spare {} out {}",
        if fight { "on" } else { "off" },
        SCOUT.pack,
        SCOUT.spare,
        SCOUT.out
    );
    println!("| errand | built | off | filling | too far | starved | killed | at +200 |");
    println!("|---|---|---|---|---|---|---|---|");
    for r in &results {
        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            r.label,
            r.built.map_or("-".into(), |t| format!("+{t}")),
            r.off,
            r.filling,
            r.toofar,
            r.starved,
            r.killed,
            r.at200
        );
    }
    let routed: Vec<&Run> = results.iter().filter(|r| r.routed).collect();
    let by = |n: u32| routed.iter().filter(|r| r.built.is_some_and(|t| t <= n)).count();
    let (off, filling, toofar): (u32, u32, u32) = (
        routed.iter().map(|r| r.off).sum(),
        routed.iter().map(|r| r.filling).sum(),
        routed.iter().map(|r| r.toofar).sum(),
    );
    println!(
        "SUMMARY fight {} pack {} spare {} out {}: routed {}; built by +50 {}, +100 {}, +200 {}; off {off}, filling {filling}, too far {toofar}; starved {}, killed {}, gone {}",
        if fight { "on" } else { "off" },
        SCOUT.pack,
        SCOUT.spare,
        SCOUT.out,
        routed.len(),
        by(50),
        by(100),
        by(200),
        routed.iter().map(|r| r.starved).sum::<usize>(),
        routed.iter().map(|r| r.killed).sum::<usize>(),
        routed.iter().filter(|r| r.gone).count()
    );
}

#[test]
#[ignore = "a report: run it with --ignored --nocapture"]
fn routes_to_every_rival_capital() {
    let only = std::env::var("FIGHT").ok();
    if only.as_deref() != Some("on") {
        report(false);
    }
    if only.as_deref() != Some("off") {
        report(true);
    }
}
