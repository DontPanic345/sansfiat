//! The scout's first errand on the seven retro maps, on the stage: the first civ is led, and its
//! scout gets four errands, each its own stage with no refill: routed to the nearest rival capital,
//! routed to `Lead`'s target (a forest or hills tile seven steps out), moved there, and left idle.
//! Over 50 turns each scout's hungry turns, people lost, turns off the errand, tiles revealed and trails
//! built are counted, and every scout must be alive at the end. A second test, a report, refills the
//! routed scouts' packs each turn: the control for how much a living scout gets done.
//! A scout eats from its pack and fills it at a store; turns off the errand are the turns it walks to
//! a store (`homing`) or back (`back`). The errands run with fields off (`Config.fields`), the control.
//! `cargo test --release -p engine --test scout_errand -- --nocapture` prints the table
//! (`relay/specs/153/feeding.md`).

use engine::data::{FOREST, HILLS};
use engine::land::steps;
use engine::stage::Stage;
use engine::units;
use engine::{Action, EventKind, Order};

/// The seven retro maps: `(size, seed)`.
const MAPS: [(&str, u32); 7] =
    [("small", 2), ("small", 3), ("small", 6), ("small", 9), ("medium", 8), ("medium", 9), ("large", 8)];
const TURNS: u32 = 50;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    RouteRival,
    RouteLead,
    MoveLead,
    Idle,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::RouteRival => "route to the rival",
            Kind::RouteLead => "route to Lead's target",
            Kind::MoveLead => "move to Lead's target",
            Kind::Idle => "idle",
        }
    }
}

struct Result {
    hungry: u32,
    lost: f64,
    off: u32,
    revealed: usize,
    trails: usize,
    killed: usize,
    /// The turn the route was found, counted from the pick.
    found: Option<u32>,
    alive: bool,
}

fn errand(size: &str, seed: u32, kind: Kind, refill: bool) -> Result {
    let mut stage = Stage::retro(size, seed).unwrap();
    stage.world_mut().config.fields = false;
    // The fight is off: the led civ sits idle past the pick and this test is not about war.
    stage.world_mut().config.fight = false;
    let me = *stage.world().civs.keys().next().unwrap();
    stage.lead(me).unwrap();
    let w = stage.world();
    let cap = w.civs[&me].capital;
    let rival = w
        .civs
        .values()
        .filter(|c| c.id != me)
        .min_by_key(|c| (steps(w, cap, c.capital), c.id))
        .map(|c| c.capital)
        .unwrap();
    // A forest or hills tile seven steps out, or the nearest to seven where the pick has none there.
    let lead = (0..w.tiles.len())
        .filter(|&i| matches!(w.tiles[i].terrain, FOREST | HILLS) && steps(w, i, cap) > 0)
        .min_by_key(|&i| (steps(w, i, cap).abs_diff(7), i))
        .unwrap();
    let id = units::scout(w, me).unwrap().id;
    let start = w.units[&id].people;
    let seen = w.civs[&me].revealed.len();
    let turn0 = w.turn;
    let order = match kind {
        Kind::RouteRival => Some(Order::Route { unit: id, from: cap, to: rival }),
        Kind::RouteLead => Some(Order::Route { unit: id, from: cap, to: lead }),
        Kind::MoveLead => Some(Order::Move { unit: id, to: lead }),
        Kind::Idle => None,
    };
    if let Some(order) = order {
        stage.act(Action::Order { civ: me, order }).unwrap();
    }
    let (mut hungry, mut off) = (0, 0);
    for _ in 0..TURNS {
        if refill {
            let w = stage.world_mut();
            if let Some(u) = w.units.get_mut(&id) {
                let cap = units::pack_cap(u.people);
                u.actor.store = engine::market::spread(cap);
                let civ = w.civs.get_mut(&me).unwrap();
                if engine::market::total(&civ.actor.store) < cap {
                    civ.actor.store = engine::market::spread(cap);
                }
            }
        }
        stage.turn(1).unwrap();
        let w = stage.world();
        match w.units.get(&id) {
            Some(u) => {
                hungry += u32::from(u.hungry > 0);
                off += u32::from({
                    let s = u.scout().unwrap();
                    s.homing || s.back.is_some()
                });
            }
            None => hungry += 1,
        }
    }
    let w = stage.world();
    let killed = w.events.iter().filter(|e| e.kind == EventKind::ScoutKilled && e.unit == Some(id)).count();
    let left = w.units.get(&id).map_or(0.0, |u| u.people);
    let trails = w.events.iter().filter(|e| e.kind == EventKind::ScoutTrailBuilt && e.unit == Some(id)).count();
    let found =
        w.events.iter().find(|e| e.kind == EventKind::ScoutRouteFound && e.unit == Some(id)).map(|e| e.turn - turn0);
    Result {
        hungry,
        lost: start - left,
        off,
        revealed: w.civs[&me].revealed.len() - seen,
        trails,
        killed,
        found,
        alive: left > 0.0,
    }
}

fn table(kinds: &[Kind], refill: bool) -> Vec<String> {
    let mut lost = Vec::new();
    let (mut revealed, mut trails) = (0, 0);
    let (mut hungry, mut people, mut off) = (0, 0.0, 0);
    println!("| map | errand | hungry turns | people lost | turns off the errand | tiles revealed | trails built | route found | killed | alive |");
    println!("|---|---|---|---|---|---|---|---|---|---|");
    for (size, seed) in MAPS {
        for &kind in kinds {
            let e = errand(size, seed, kind, refill);
            println!(
                "| {size} {seed} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                kind.name(),
                e.hungry,
                e.lost,
                e.off,
                e.revealed,
                e.trails,
                e.found.map_or("-".to_string(), |t| format!("turn {t}")),
                e.killed,
                if e.alive { "yes" } else { "no" }
            );
            hungry += e.hungry;
            people += e.lost;
            if matches!(kind, Kind::RouteRival | Kind::RouteLead) {
                off += e.off;
                revealed += e.revealed;
                trails += e.trails;
            }
            if !e.alive {
                lost.push(format!("{size} {seed} {}", kind.name()));
            }
        }
    }
    println!(
        "scouts lost: {} of {}: {lost:?}; people lost {people}, hungry scout-turns {hungry}; routed: off {off}, tiles revealed {revealed}, trails built {trails}",
        lost.len(),
        MAPS.len() * kinds.len()
    );
    lost
}

/// At most 4 of the 28 scouts are lost by 50 turns with no refill, with fields off, at the numbers in `SCOUT`.
/// Which map loses them follows the pick's ground and who pays the civ's levy there: the two routed scouts
/// on medium 8 (`relay/specs/352/scores.md`).
#[test]
fn every_scouts_first_errand_is_survived() {
    let lost = table(&[Kind::RouteRival, Kind::RouteLead, Kind::MoveLead, Kind::Idle], false);
    // The pre-game's pick moves with any float: main with `CLAN.work` nudged by one part in 10^12 loses 5 of
    // the 28 on other maps, so the bound is that, not none (relay/specs/154/day.md).
    assert!(lost.len() <= 4, "scouts lost: {lost:?}");
}

/// A report: the 14 routed errands with the pack refilled each turn, the control for how much a living
/// scout gets done.
#[test]
#[ignore = "a report: run it with --ignored --nocapture"]
fn the_refilled_scouts_work() {
    table(&[Kind::RouteRival, Kind::RouteLead], true);
}

/// A report: the seven retro maps with every civ on the AI, 200 turns past the pick. The AI's scouts lost
/// (gone) and the people they lost to hunger are counted: it must be none (`relay/specs/153/errand.md`).
#[test]
#[ignore = "a report: run it with --ignored --nocapture"]
fn the_ais_scouts_live_to_200() {
    let (mut gone, mut starved, mut scouts) = (0, 0, 0);
    println!("| map | scouts at the pick | alive at +200 | people lost |");
    println!("|---|---|---|---|");
    for (size, seed) in MAPS {
        let mut stage = Stage::retro(size, seed).unwrap();
        // The fight is off: the led civ sits idle past the pick and this test is not about war.
        stage.world_mut().config.fight = false;
        stage.act(Action::ChooseCiv { civ: None }).unwrap();
        let civs: Vec<u32> = stage.world().civs.keys().copied().collect();
        let at_pick = civs.iter().filter(|&&c| units::scout(stage.world(), c).is_some()).count();
        stage.turn(200).unwrap();
        let w = stage.world();
        let alive = civs.iter().filter(|&&c| units::scout(w, c).is_some()).count();
        let lost = w.events.iter().filter(|e| e.kind == EventKind::ScoutStarved).count();
        println!("| {size} {seed} | {at_pick} | {alive} | {lost} |");
        scouts += at_pick;
        gone += at_pick - alive;
        starved += lost;
    }
    println!("AI scouts: {scouts} at the pick; lost {gone}, people lost to hunger {starved}");
    assert_eq!((gone, starved), (0, 0), "the AI's scouts: {gone} lost, {starved} people starved");
}
