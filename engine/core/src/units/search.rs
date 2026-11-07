//! The scout's route search: the plan for a route as the civ knows the map (`known`) and as the
//! unrevealed tiles could be (`possible`), the order it works through the tiles on it, and the
//! labour it spends revealing them and building the trails the route needs.

use std::collections::{BTreeMap, BTreeSet};

use super::forage::Ladder;
use super::line::{path_between, walkable};
use super::scout::{blocked, clear_route, march, sc, sc_mut, scout_event};
use super::EPS;
use crate::data::*;
use crate::events::EventKind;
use crate::land::{self, build_trail, cost_map, cost_map_by, path_by, steps, trailable};
use crate::world::World;

// ---- the route search ---------------------------------------------------------------------------
//
// The search: a route's plan, the tiles it works on, and the labour it spends on them.

/// Whether tile `j` is revealed to `civ`.
pub(super) fn revealed(w: &World, civ: u32, j: usize) -> bool {
    w.civs[&civ].revealed.contains(&j)
}

/// Whether a mountain can be bridged by some span the map can roll, by `CIV.bridge_span`.
pub(super) fn mountain_may_bridge() -> bool {
    MAP.mountain_span.iter().any(|&(s, _)| s.is_some_and(|s| s <= CIV.bridge_span))
}

/// What entering tile `j` costs `civ`, ignoring any river: as it is known (`possible` false) or as
/// it could be (`possible` true). A tile with a trail, or revealed, is priced with its trail where
/// one could go (`min` of bare and hidden, a mountain only by its trail). An unrevealed tile is its
/// bare cost as known; as possible, its cheapest hidden trail if it could hide one. `None` is
/// impassable.
pub(super) fn tile_cost(w: &World, civ: u32, j: usize, possible: bool) -> Option<f64> {
    let t = &w.tiles[j];
    let bare = TERRAIN[t.terrain].move_cost;
    if t.trail || revealed(w, civ, j) {
        let hidden = if t.trail || trailable(w, j) { t.trail_cost } else { None };
        return match hidden {
            Some(h) => Some(bare.map_or(h, |c| c.min(h))),
            None => bare,
        };
    }
    if !possible {
        return bare;
    }
    let hidden = match t.terrain {
        FOREST | HILLS => {
            let pairs = if t.terrain == FOREST { MAP.forest_cost } else { MAP.hills_cost };
            Some(pairs.iter().map(|p| f64::from(p.0)).fold(f64::INFINITY, f64::min))
        }
        MOUNTAIN if mountain_may_bridge() => Some(MAP.mountain_cost as f64),
        _ => None,
    };
    match hidden {
        Some(h) => Some(bare.map_or(h, |c| c.min(h))),
        None => bare,
    }
}

/// Whether tile `j` could be trailed as `civ` knows it.
pub(super) fn may_trail(w: &World, civ: u32, j: usize) -> bool {
    let t = &w.tiles[j];
    if t.trail || revealed(w, civ, j) {
        return t.trail || trailable(w, j);
    }
    match t.terrain {
        FOREST | HILLS => true,
        MOUNTAIN => mountain_may_bridge(),
        _ => false,
    }
}

/// The step into `j` as `civ` prices it (`tile_cost` plus a river crossing, which a trail bridges:
/// known keeps it until a trail is built, possible drops it where one could go).
pub(super) fn step_as(w: &World, civ: u32, from: usize, to: usize, possible: bool) -> Option<f64> {
    let t = &w.tiles[to];
    let along = w.tiles[from].flow == Some(to) || t.flow == Some(from);
    let bridged = if possible { may_trail(w, civ, to) } else { t.trail };
    let cross = if t.river > 0 && !bridged && !along { RIVER.cross } else { 0.0 };
    Some(tile_cost(w, civ, to, possible)? + cross)
}

/// Whether a trail on revealed tile `j` would cut what entering it costs: it is trailable, not
/// built, and its hidden cost is less than its bare one (or it bridges a river).
pub(super) fn needs_trail(w: &World, civ: u32, j: usize) -> bool {
    let t = &w.tiles[j];
    if t.trail || !revealed(w, civ, j) || !trailable(w, j) || !TRAIL.work.iter().any(|&(k, _)| k == t.terrain) {
        return false;
    }
    let cut = TERRAIN[t.terrain].move_cost.is_none_or(|c| t.trail_cost.is_some_and(|h| h < c));
    cut || t.river > 0
}

/// The labour a trail on tile `j` still needs, or `None` where none can be built.
pub(super) fn trail_left(w: &World, j: usize) -> Option<f64> {
    let t = &w.tiles[j];
    TRAIL.work.iter().find(|&&(k, _)| k == t.terrain).map(|&(_, need)| (need - t.trail_work).max(0.0))
}

/// What a route plan says: `known` and `possible` are the cheapest paths from `from` to `to` (tiles
/// after `from`, and cost), and `candidates` are the unrevealed tiles on the possible path whose
/// known and possible costs differ. None when the known path is already as cheap as the possible one.
#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub known: Option<(Vec<usize>, f64)>,
    pub possible: Option<(Vec<usize>, f64)>,
    pub candidates: Vec<usize>,
}

/// The route plan for `civ` from `from` to `to`: see `Plan`.
pub fn route_plan(w: &World, civ: u32, from: usize, to: usize) -> Plan {
    let known = path_by(w, from, to, |i, j| step_as(w, civ, i, j, false));
    let possible = path_by(w, from, to, |i, j| step_as(w, civ, i, j, true));
    let mut candidates = Vec::new();
    if let Some((pp, pc)) = &possible {
        let kc = known.as_ref().map_or(f64::INFINITY, |k| k.1);
        if kc > pc + EPS {
            let mut prev = from;
            for &j in pp {
                let differ = step_as(w, civ, prev, j, false) != step_as(w, civ, prev, j, true);
                if differ && !revealed(w, civ, j) && !w.tiles[j].trail {
                    candidates.push(j);
                }
                prev = j;
            }
        }
    }
    Plan { known, possible, candidates }
}

/// The labour a scout's walk costs, from `home`: `cost_map` over walkable tiles.
pub(super) fn walk_costs(w: &World, home: usize) -> BTreeMap<usize, f64> {
    cost_map(w, &[home], f64::INFINITY, |i| walkable(w, i))
}

/// The cheapest tile to stand on to work on tile `j` (`j` itself or a neighbour) that the walk
/// reaches: its walk cost and the tile.
pub(super) fn beside(w: &World, costs: &BTreeMap<usize, f64>, j: usize) -> Option<(f64, usize)> {
    std::iter::once(j)
        .chain(land::neighbours(w, j))
        .filter_map(|n| costs.get(&n).map(|&c| (c, n)))
        .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)))
}

/// The walk to a tile beside `j` from `home`: empty when it stands there already, `None` when no
/// walk reaches one.
pub(super) fn approach(w: &World, costs: &BTreeMap<usize, f64>, home: usize, j: usize) -> Option<Vec<usize>> {
    if steps(w, home, j) <= 1 {
        return Some(Vec::new());
    }
    let (_, n) = beside(w, costs, j)?;
    if n == home {
        return Some(Vec::new());
    }
    path_between(w, home, n).map(|(p, _)| p)
}

/// The cheapest possible cost from `from` to each tile, and from each tile into `to`, within `max`:
/// priced as `route_plan` prices a possible path. A mountain's cost through is its two entries.
fn through_maps(w: &World, civ: u32, from: usize, to: usize, max: f64) -> (BTreeMap<usize, f64>, BTreeMap<usize, f64>) {
    let out = cost_map_by(w, &[from], max, |_| true, |i, j| step_as(w, civ, i, j, true));
    let into = cost_map_by(w, &[to], max, |_| true, |i, j| step_as(w, civ, j, i, true));
    (out, into)
}

/// The tiles a scout works on next, in order. Mountains come first, by best case: the cost through
/// them plus the scout's walk to one beside them, the walk priced as the known route prices it (0
/// when the scout stands next to the mountain). Ties go to the lower tile. Then the forest and hills,
/// the nearest to the scout first, then the biggest saving. A mountain candidate stands for its
/// range: the unrevealed mountains joined to it whose cheapest possible route through them beats the
/// known one. So the search sweeps along a range from where it stands.
pub(super) fn ordered(
    w: &World,
    civ: u32,
    plan: &Plan,
    from: usize,
    to: usize,
    home: usize,
    costs: &BTreeMap<usize, f64>,
) -> Vec<usize> {
    let known_cost = plan.known.as_ref().map_or(f64::INFINITY, |k| k.1);
    let mut set: BTreeSet<usize> = plan.candidates.iter().copied().collect();
    let (out, into) = through_maps(w, civ, from, to, known_cost);
    let through = |m: usize| match (out.get(&m), into.get(&m)) {
        (Some(a), Some(b)) => a + b,
        _ => f64::INFINITY,
    };
    for &c in plan.candidates.iter().filter(|&&c| w.tiles[c].terrain == MOUNTAIN) {
        let mut range = BTreeSet::new();
        let mut stack = vec![c];
        while let Some(m) = stack.pop() {
            if !range.insert(m) {
                continue;
            }
            for n in land::neighbours(w, m) {
                if w.tiles[n].terrain == MOUNTAIN && !revealed(w, civ, n) && !range.contains(&n) {
                    stack.push(n);
                }
            }
        }
        set.extend(range.into_iter().filter(|&m| through(m) < known_cost - EPS));
    }
    let walks = cost_map_by(w, &[home], f64::INFINITY, |_| true, |i, j| step_as(w, civ, i, j, false));
    let mut keyed: Vec<(u8, f64, f64, usize)> = set
        .into_iter()
        .map(|j| {
            if w.tiles[j].terrain == MOUNTAIN {
                let walk = if steps(w, home, j) <= 1 {
                    0.0
                } else {
                    land::neighbours(w, j).filter_map(|n| walks.get(&n).copied()).fold(f64::INFINITY, f64::min)
                };
                return (0, through(j) + walk, 0.0, j);
            }
            let reach = beside(w, costs, j).map_or(f64::INFINITY, |(c, _)| c);
            let saving = match tile_cost(w, civ, j, false) {
                Some(k) => k - tile_cost(w, civ, j, true).unwrap_or(k),
                None => f64::INFINITY,
            };
            (1, reach, -saving, j)
        })
        .collect();
    keyed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)).then(a.2.total_cmp(&b.2)).then(a.3.cmp(&b.3)));
    keyed.into_iter().map(|k| k.3).collect()
}

/// Spends the scout's labour (`ladder`) on its route, as `scout/`'s `scoutWork`: it walks to the first
/// reveal it can reach, reveals it, and builds the trails its route needs; a mountain takes a whole
/// turn's search labour, begun beside it (`start` is what the search had this turn). Returns the
/// labour a part-paid step carries to next turn.
pub(super) fn search(w: &mut World, id: u32, ladder: &mut Ladder, start: f64) -> f64 {
    let civ = w.units[&id].civ;
    let mut banked = 0.0;
    let mut guard = 0;
    while ladder.left > EPS && guard < 100_000 {
        guard += 1;
        let Some((from, to)) = sc(w, id).route else { break };
        let home = w.units[&id].home;
        let plan = route_plan(w, civ, from, to);
        if plan.possible.is_none() {
            let tile = home;
            clear_route(w, id);
            scout_event(w, EventKind::ScoutNoRoute, tile, civ, id, None);
            break;
        }
        let costs = walk_costs(w, home);
        let order = ordered(w, civ, &plan, from, to, home, &costs);
        let reach = order.iter().find_map(|&j| approach(w, &costs, home, j).map(|p| (j, p)));
        if let Some((j, path)) = reach {
            if !path.is_empty() {
                march(w, id, &path, ladder, &mut banked);
                sc_mut(w, id).blocked = false;
                continue;
            }
            if w.tiles[j].terrain == MOUNTAIN {
                // A mountain needs the whole turn's search labour, begun beside it.
                if ladder.left + EPS < start {
                    break;
                }
                reveal(w, id, civ, j);
                ladder.left = 0.0;
            } else {
                if ladder.left + EPS < SCOUT.reveal {
                    break;
                }
                ladder.spend(SCOUT.reveal);
                reveal(w, id, civ, j);
            }
            continue;
        }
        // No reveal can be reached. Once the candidates are none, the known route is settled and
        // its trails are built; while candidates remain unreached, the first reachable trail on the
        // possible path is built instead (a pass that opens the land beyond).
        let known_cost = plan.known.as_ref().map(|k| k.1);
        let no_reveal = !plan.candidates.is_empty();
        let trail_on: &[usize] = match (&plan.known, &plan.possible) {
            (Some((p, _)), _) if !no_reveal => p,
            (_, Some((p, _))) if no_reveal => p,
            _ => &[],
        };
        let needs: Vec<usize> = {
            let mut v: Vec<usize> = trail_on.iter().copied().filter(|&j| needs_trail(w, civ, j)).collect();
            v.sort_by_key(|&j| (steps(w, home, j), j));
            v
        };
        if !no_reveal {
            let pcost = plan.possible.as_ref().map_or(f64::INFINITY, |p| p.1);
            if known_cost.is_some_and(|k| k <= pcost + EPS) && !sc(w, id).heard && !needs.is_empty() {
                let cost = known_cost.unwrap_or(0.0);
                sc_mut(w, id).heard = true;
                scout_event(w, EventKind::ScoutRouteFound, home, civ, id, Some(cost));
            }
        }
        let trail = needs.iter().find_map(|&j| approach(w, &costs, home, j).map(|p| (j, p)));
        let Some((j, path)) = trail else {
            match known_cost {
                // Its trails are built: the route stays set until cleared, and is said once.
                Some(cost) if !no_reveal && needs.is_empty() => {
                    if !sc(w, id).built {
                        sc_mut(w, id).built = true;
                        scout_event(w, EventKind::ScoutRouteFound, home, civ, id, Some(cost));
                    }
                }
                _ => blocked(w, id, civ),
            }
            break;
        };
        if !path.is_empty() {
            march(w, id, &path, ladder, &mut banked);
            sc_mut(w, id).blocked = false;
            continue;
        }
        let Some(need) = trail_left(w, j) else { break };
        let used = need.min(ladder.left);
        if used <= EPS {
            break;
        }
        ladder.spend(used);
        if build_trail(w, j, used) {
            scout_event(w, EventKind::ScoutTrailBuilt, j, civ, id, None);
        }
        sc_mut(w, id).blocked = false;
    }
    banked
}

/// Reveals tile `j` for `civ`; a mountain that can be bridged is a pass found.
pub(super) fn reveal(w: &mut World, id: u32, civ: u32, j: usize) {
    w.civs.get_mut(&civ).unwrap().revealed.insert(j);
    sc_mut(w, id).blocked = false;
    if w.tiles[j].terrain == MOUNTAIN && trailable(w, j) {
        scout_event(w, EventKind::ScoutPassFound, j, civ, id, None);
    }
}

/// What a scout's search is doing on its route, as `Sim.view()` shows it: the tiles it will try, in
/// its order (`next`); the unrevealed mountains it leaves, each with its cost through, cheapest first
/// and ten at most (`left`); and the known route's cost (`known`).
#[derive(Clone, Debug, PartialEq)]
pub struct SearchView {
    pub next: Vec<usize>,
    pub left: Vec<(usize, f64)>,
    pub known: Option<f64>,
}

/// The search of scout `id` on its route, or `None` without one. `left` holds the unrevealed
/// mountains of every range that touches the possible or known path (on it, beside one of its tiles,
/// or joined to one of those), whose cost through is not below the known cost by more than `EPS`.
pub fn search_view(w: &World, id: u32) -> Option<SearchView> {
    let (from, to) = super::scout::sc(w, id).route?;
    let (civ, home) = (w.units[&id].civ, w.units[&id].home);
    let plan = route_plan(w, civ, from, to);
    let costs = walk_costs(w, home);
    let next = ordered(w, civ, &plan, from, to, home, &costs);
    let known = plan.known.as_ref().map(|k| k.1);
    let known_cost = known.unwrap_or(f64::INFINITY);
    let (out, into) = through_maps(w, civ, from, to, f64::INFINITY);
    let through = |m: usize| match (out.get(&m), into.get(&m)) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    };
    let path: Vec<usize> = std::iter::once(from)
        .chain(plan.known.iter().chain(plan.possible.iter()).flat_map(|p| p.0.iter().copied()))
        .collect();
    let mut range = BTreeSet::new();
    let mut stack: Vec<usize> = path
        .iter()
        .flat_map(|&t| std::iter::once(t).chain(land::neighbours(w, t).collect::<Vec<_>>()))
        .filter(|&m| w.tiles[m].terrain == MOUNTAIN && !revealed(w, civ, m))
        .collect();
    while let Some(m) = stack.pop() {
        if !range.insert(m) {
            continue;
        }
        for n in land::neighbours(w, m) {
            if w.tiles[n].terrain == MOUNTAIN && !revealed(w, civ, n) && !range.contains(&n) {
                stack.push(n);
            }
        }
    }
    let mut left: Vec<(usize, f64)> = range
        .into_iter()
        .filter_map(|m| through(m).map(|x| (m, x)))
        .filter(|&(_, x)| x.is_finite() && x >= known_cost - EPS)
        .collect();
    left.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
    left.truncate(10);
    Some(SearchView { next, left, known })
}
