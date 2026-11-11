//! The cases the owner's notes and the M3 retro name, each on the retro maps: the scout's settled
//! route and its search order, and Disband all. `relay/specs/151/cases.md` says what each prints on
//! main and what must change. A `should_panic` case is red on main on purpose: its expected text is
//! the failure main gives, and the fix removes the attribute.

use engine::data::{Fighter, FOREST, HILLS, MOUNTAIN, N};
use engine::land::{neighbours, steps};
use engine::stage::Stage;
use engine::units::{self, Plan};
use engine::{Action, Order, Unit, World};

/// The slack a cost comparison allows: the engine's own.
const EPS: f64 = 1e-9;

/// The turns an errand may take before it counts as unsettled: a scout walks at one person's pace, and
/// the slowest errand (large 8 to the rival) settles in 133.
const TURNS: u32 = 200;

/// The seven retro maps: `(size, seed)`.
const MAPS: [(&str, u32); 7] =
    [("small", 2), ("small", 3), ("small", 6), ("small", 9), ("medium", 8), ("medium", 9), ("large", 8)];

/// A scout's errand on a retro map: its first civ leads, its scout is routed from its capital to the
/// nearest rival capital (`rival`) or to `Lead`'s target, the first forest or hills tile seven steps
/// out. `name` is the map and the target, as the messages say them.
struct Errand {
    map: String,
    name: String,
    stage: Stage,
    civ: u32,
    scout: u32,
    cap: usize,
    to: usize,
}

/// The turns a settled route has to build a trail the land wore away in its settling turn.
const GRACE: u32 = 3;

fn errand(size: &str, seed: u32, rival: bool) -> Errand {
    let mut stage = Stage::retro(size, seed).unwrap();
    let me = *stage.world().civs.keys().next().unwrap();
    stage.lead(me).unwrap();
    let w = stage.world();
    let cap = w.civs[&me].capital;
    let to = if rival {
        w.civs
            .values()
            .filter(|c| c.id != me)
            .min_by_key(|c| (steps(w, cap, c.capital), c.id))
            .map(|c| c.capital)
            .unwrap()
    } else {
        // A forest or hills tile seven steps out, or the nearest to seven where the pick has none there.
        (0..w.tiles.len())
            .filter(|&i| matches!(w.tiles[i].terrain, FOREST | HILLS) && steps(w, i, cap) > 0)
            .min_by_key(|&i| (steps(w, i, cap).abs_diff(7), i))
            .unwrap()
    };
    let scout = units::scout(w, me).unwrap().id;
    stage.act(Action::Order { civ: me, order: Order::Route { unit: scout, from: cap, to } }).unwrap();
    let target = if rival { "to the rival" } else { "to Lead's target" };
    Errand { map: format!("{size} {seed}"), name: format!("{size} {seed} {target}"), stage, civ: me, scout, cap, to }
}

/// Before a turn: the civ's store and the scout's pack are refilled to the pack's capacity, so a
/// scout does not starve for want of food the errand never gives it.
fn refill(e: &mut Errand) {
    let w = e.stage.world_mut();
    let Some(u) = w.units.get_mut(&e.scout) else { return };
    let cap = units::pack_cap(u.people);
    u.actor.store = [cap / N as f64; N];
    let civ = w.civs.get_mut(&e.civ).unwrap();
    if civ.actor.store.iter().sum::<f64>() < cap {
        civ.actor.store = [cap / N as f64; N];
    }
}

/// Whether the scout has settled: its trails are built, or its route is cleared. `Err` when the
/// scout is gone.
fn settled(e: &Errand) -> Result<bool, String> {
    let u = e.stage.world().units.get(&e.scout).ok_or("the scout is gone")?;
    let s = u.scout().expect("a scout");
    Ok(s.built || s.route.is_none())
}

/// Runs the errand's turns until it settles, and returns the turn it settled on (from 1). Panics
/// with its own message when it does not settle within `TURNS`.
fn settle(e: &mut Errand) -> u32 {
    for t in 1..=TURNS {
        refill(e);
        e.stage.turn(1).unwrap();
        match settled(e) {
            Ok(true) => return t,
            Ok(false) => {}
            Err(m) => panic!("{} turn {t}: {m}", e.name),
        }
    }
    panic!("{}: the scout has not settled in {TURNS} turns", e.name)
}

/// The known cost of `civ`'s route from `cap` to `to` (infinite when the known path is none).
fn known_cost(w: &World, civ: u32, cap: usize, to: usize) -> f64 {
    units::route_plan(w, civ, cap, to).known.map_or(f64::INFINITY, |k| k.1)
}

/// A mountain's cost through: the cheapest possible route from `from` to it, plus from it to `to`
/// (`None` when either has no route).
fn through(w: &World, civ: u32, from: usize, to: usize, m: usize) -> Option<f64> {
    let a = units::route_plan(w, civ, from, m).possible?.1;
    let b = units::route_plan(w, civ, m, to).possible?.1;
    Some(a + b)
}

/// The best case the search is ordered by: a mountain's cost through it plus the scout's walk to it,
/// the least known cost from the scout's tile `home` to a passable tile beside it, 0 when it stands
/// beside it. Its `through` is given, so the caller prices it once.
fn best_case(w: &World, civ: u32, home: usize, m: usize, through: f64) -> f64 {
    let walk = if steps(w, home, m) <= 1 {
        0.0
    } else {
        neighbours(w, m)
            .filter_map(|n| units::route_plan(w, civ, home, n).known.map(|k| k.1))
            .fold(f64::INFINITY, f64::min)
    };
    through + walk
}

/// The mountains open at the start of a turn: unrevealed, and their cost through below the known
/// route's cost by more than `EPS`, each with its best case.
fn open_mountains(w: &World, civ: u32, cap: usize, to: usize, home: usize) -> Vec<(usize, f64)> {
    let known = known_cost(w, civ, cap, to);
    (0..w.tiles.len())
        .filter(|&m| w.tiles[m].terrain == MOUNTAIN && !w.civs[&civ].revealed.contains(&m))
        .filter_map(|m| {
            let x = through(w, civ, cap, to, m)?;
            (x < known - EPS).then(|| (m, best_case(w, civ, home, m, x)))
        })
        .collect()
}

/// Walks the scout's search turn by turn, and checks each turn that reveals a mountain: at most one,
/// and it was open at the turn's start with a best case within `EPS` of the lowest of the open set.
/// The turn's start is a clone of the world, so the open set is priced only on a turn that reveals.
fn check_search_order(e: &mut Errand) {
    let (civ, cap, to) = (e.civ, e.cap, e.to);
    for t in 1..=TURNS {
        refill(e);
        let start = e.stage.world().clone();
        e.stage.turn(1).unwrap();
        let end = e.stage.world();
        let new: Vec<usize> = end.civs[&civ]
            .revealed
            .iter()
            .copied()
            .filter(|&m| end.tiles[m].terrain == MOUNTAIN && !start.civs[&civ].revealed.contains(&m))
            .collect();
        if new.len() > 1 {
            panic!("{} turn {t}: the scout revealed {} mountains in one turn", e.map, new.len());
        }
        if let Some(&m) = new.first() {
            let home = start.units[&e.scout].home;
            let open = open_mountains(&start, civ, cap, to, home);
            let lowest = open.iter().copied().min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
            let mine = through(&start, civ, cap, to, m).map_or(f64::INFINITY, |x| best_case(&start, civ, home, m, x));
            let open_m = open.iter().any(|&(j, _)| j == m);
            let ok = open_m && lowest.is_some_and(|(_, b)| mine <= b + EPS);
            if !ok {
                let msg = match lowest {
                    Some((m2, b2)) if open_m => {
                        format!("{} turn {t}: the scout revealed mountain {m} (best case {mine:.1}) while {m2} (best case {b2:.1}) was open", e.map)
                    }
                    Some((m2, b2)) => format!(
                        "{} turn {t}: the scout revealed mountain {m} (best case {mine:.1}), which was not open, while {m2} (best case {b2:.1}) was open",
                        e.map
                    ),
                    None => format!("{} turn {t}: the scout revealed mountain {m} (best case {mine:.1}) while no mountain was open", e.map),
                };
                panic!("{msg}");
            }
        }
        match settled(e) {
            Ok(true) => return,
            Ok(false) => {}
            Err(m) => panic!("{} turn {t}: {m}", e.map),
        }
    }
    panic!("{}: the scout has not settled in {TURNS} turns", e.map);
}

// ---- the scout: the settled route ----------------------------------------------------------------

/// Owner's invariant, green on main: on every retro errand, at settle (or the turn after, when a
/// trail faded under the settled route) no tile on the possible path that is unrevealed and has no
/// trail has a possible step cheaper than its known one.
#[test]
fn settled_scouts_leave_nothing_unrevealed_that_could_beat_them() {
    let mut failures = Vec::new();
    for (size, seed) in MAPS {
        for rival in [true, false] {
            let mut e = errand(size, seed, rival);
            let t = settle(&mut e);
            let mut plan: Plan = e.stage.route(e.civ, e.cap, e.to);
            // The scout settles in its own turn, and the land's trails wear after it in the same
            // turn (`tend`): a trail that fades then opens a tile the scout reveals on its next turn
            // (medium 8 to the rival, tile 1019, with the planner's armies off that trail).
            if !plan.candidates.is_empty() {
                refill(&mut e);
                e.stage.turn(1).unwrap();
                plan = e.stage.route(e.civ, e.cap, e.to);
            }
            if !plan.candidates.is_empty() {
                failures
                    .push(format!("{} turn {t}: unrevealed tiles {:?} could beat the route", e.name, plan.candidates));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The search orders a range's mountains by best case: on each turn that reveals a mountain, it is
/// open at the turn's start (its cost through beats the known route) and its best case is the least
/// of the open set, within `EPS`. Red after build-2: the search orders only the mountains joined to
/// its candidates' ranges, so on small 9 to the rival it reveals 131 (best case 32) while 178 (best
/// case 28, in no candidate's range) is open. Widening the set to every open mountain makes the
/// errand unsettled in 120 turns (`settled_scouts_leave_nothing_unrevealed_that_could_beat_them`), so
/// the fix needs the owner's call (`relay/specs/153/search.md`).
#[test]
#[should_panic(expected = "small 9 turn 22: the scout revealed mountain 131")]
#[ignore = "red on main's picks only; the pre-game's picks moved with the day's scale (relay/specs/154/day.md)"]
fn a_scout_searches_a_range_by_best_case() {
    for (size, seed) in [("small", 9), ("medium", 8)] {
        check_search_order(&mut errand(size, seed, true));
    }
}

/// The control: small 9 to Lead's forest, two reveals, both in order. Passes on main.
#[test]
fn a_scouts_search_on_small_9_to_the_forest_is_in_order() {
    check_search_order(&mut errand("small", 9, false));
}

/// A settled route has no cheaper trail: the scout builds trails on the possible path too, once no
/// reveal is left, so a bridge on a revealed river tile that would cut the route is built. On main,
/// small 7 to the rival settles on turn 26 at 20.0 known against 19.67 possible, and it stays so;
/// medium 8 to the rival settles on turn 14 at 26.33 against 26.0. The land's trails wear after the
/// scout settles in the same turn (`tend`), so a route that is cheaper at settle gets the turns it takes
/// to build the trail (`GRACE`) before it fails.
#[test]
fn a_settled_route_has_no_cheaper_trail() {
    for (size, seed, rival) in [("small", 7, true), ("medium", 8, true)] {
        let mut e = errand(size, seed, rival);
        let t = settle(&mut e);
        let cheaper = |e: &Errand| {
            let plan = e.stage.route(e.civ, e.cap, e.to);
            let known = plan.known.as_ref().map_or(f64::INFINITY, |k| k.1);
            let possible = plan.possible.as_ref().map_or(f64::INFINITY, |p| p.1);
            (plan, known, possible, possible < known - EPS)
        };
        let (mut plan, mut known, mut possible, mut red) = cheaper(&e);
        for _ in 0..GRACE {
            if !red {
                break;
            }
            refill(&mut e);
            e.stage.turn(1).unwrap();
            (plan, known, possible, red) = cheaper(&e);
        }
        if red {
            // The first step of the possible path that is dearer known: its two costs, from the tile before.
            let (path, _) = plan.possible.as_ref().unwrap();
            let mut prev = e.cap;
            let mut step = None;
            for &j in path {
                let one = e.stage.route(e.civ, prev, j);
                let k = one.known.map_or(f64::INFINITY, |p| p.1);
                let p = one.possible.map_or(f64::INFINITY, |p| p.1);
                if p < k - EPS {
                    step = Some((j, k, p));
                    break;
                }
                prev = j;
            }
            let (j, k, p) = step.expect("a cheaper possible step");
            panic!(
                "{} turn {t}: the settled route costs {known:.1} known and {possible:.1} possible; the step into tile {j} costs {p:.1} where known it costs {k:.1}",
                e.name
            );
        }
    }
}

// ---- Disband all: the engine's control ----------------------------------------------------------

/// Sets a unit's people to `k`, its stack (an army's warriors) and its pack to match.
fn set_people(u: &mut Unit, k: f64) {
    u.actor.people = k;
    u.actor.store = [units::pack_cap(k) / N as f64; N];
    if let Some(c) = u.camp_mut() {
        c.stack = [0.0; 2];
        c.stack[Fighter::Warriors.index()] = k;
    }
}

fn people_in(w: &World) -> f64 {
    w.clans.values().map(|c| c.people).sum::<f64>() + w.units.values().map(|u| u.actor.people).sum::<f64>()
}

/// Control, green on main: `Order::Disband` with no count sends every person of a unit home at once,
/// for an army, a scout and a guard with fractional people: each unit is gone, the clans gain exactly
/// its people, and the world's people are conserved. The page's count is the red case (page arm).
#[test]
fn disband_all_sends_every_person_home() {
    let mut stage = Stage::retro("small", 2).unwrap();
    let me = *stage.world().civs.keys().next().unwrap();
    stage.lead(me).unwrap();
    // A civ starts with no army or guard: the stage places its army and raises its guard.
    let cap = stage.world().civs[&me].capital;
    stage.army(me, cap, 3.0);
    units::guard::raise(stage.world_mut(), me, 3);
    let w = stage.world_mut();
    let camp = units::camp(w, me).unwrap().id;
    let scout = units::scout(w, me).unwrap().id;
    let guard = units::guard_of(w, me).unwrap().id;
    for (id, k) in [(camp, 4.6), (scout, 1.5), (guard, 2.5)] {
        set_people(w.units.get_mut(&id).unwrap(), k);
    }
    for id in [camp, scout, guard] {
        let k = stage.world().units[&id].people;
        let (clans, all) = (stage.world().clans.values().map(|c| c.people).sum::<f64>(), people_in(stage.world()));
        stage.order(me, Order::Disband { unit: id, n: None, fighter: None }).unwrap();
        let w = stage.world();
        assert!(!w.units.contains_key(&id), "unit {id} is still there after disband all");
        let rose = w.clans.values().map(|c| c.people).sum::<f64>() - clans;
        assert!((rose - k).abs() < EPS, "the clans rose by {rose} for unit {id} of {k} people");
        assert!((people_in(w) - all).abs() < EPS, "people were not conserved disbanding unit {id}");
    }
}
