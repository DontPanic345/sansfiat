//! The scout: it searches a route for its civ, pricing it as the civ knows the map (`known`) and as
//! the unrevealed tiles could still be (`possible`), revealing what could change the answer, and
//! building the trails the route needs. Its labour is its people's, spent on steps, reveals and
//! trails, after it has fed itself.

use std::collections::{BTreeMap, BTreeSet};

use super::line::{line, path_between, walkable};
use super::{rival_of, room, strip, supply, EPS};
use crate::actors::{stride, Route};
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::{self, build_trail, cost_map, path_by, steps, trailable};
use crate::market::total;
use crate::world::World;

/// A scout's own state: its route (the tile it searches from and the tile it searches for), and what
/// it has said about it.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoutState {
    /// The tile the scout searches from and the tile it searches for.
    pub route: Option<(usize, usize)>,
    /// A scout has said what its route costs; it is in `heard` once a route is found.
    pub heard: bool,
    /// A scout has reported that it is blocked, so says it once.
    pub blocked: bool,
}

/// The scout state of unit `id` (the unit is a scout; the callers have checked).
fn sc(w: &World, id: u32) -> &ScoutState {
    w.units[&id].scout().expect("a scout")
}

fn sc_mut(w: &mut World, id: u32) -> &mut ScoutState {
    w.units.get_mut(&id).and_then(|u| u.scout_mut()).expect("a scout")
}

// ---- the route search ---------------------------------------------------------------------------
//
// The search: a route's plan, the tiles it works on, and the labour it spends on them.

/// Whether tile `j` is revealed to `civ`.
fn revealed(w: &World, civ: u32, j: usize) -> bool {
    w.civs[&civ].revealed.contains(&j)
}

/// Whether a mountain can be bridged by some span the map can roll, by `CIV.bridge_span`.
fn mountain_may_bridge() -> bool {
    MAP.mountain_span.iter().any(|&(s, _)| s.is_some_and(|s| s <= CIV.bridge_span))
}

/// What entering tile `j` costs `civ`, ignoring any river: as it is known (`possible` false) or as
/// it could be (`possible` true). A tile with a trail, or revealed, is priced with its trail where
/// one could go (`min` of bare and hidden, a mountain only by its trail). An unrevealed tile is its
/// bare cost as known; as possible, its cheapest hidden trail if it could hide one. `None` is
/// impassable.
fn tile_cost(w: &World, civ: u32, j: usize, possible: bool) -> Option<f64> {
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
fn may_trail(w: &World, civ: u32, j: usize) -> bool {
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
fn step_as(w: &World, civ: u32, from: usize, to: usize, possible: bool) -> Option<f64> {
    let t = &w.tiles[to];
    let along = w.tiles[from].flow == Some(to) || t.flow == Some(from);
    let bridged = if possible { may_trail(w, civ, to) } else { t.trail };
    let cross = if t.river > 0 && !bridged && !along { RIVER.cross } else { 0.0 };
    Some(tile_cost(w, civ, to, possible)? + cross)
}

/// Whether a trail on revealed tile `j` would cut what entering it costs: it is trailable, not
/// built, and its hidden cost is less than its bare one (or it bridges a river).
fn needs_trail(w: &World, civ: u32, j: usize) -> bool {
    let t = &w.tiles[j];
    if t.trail || !revealed(w, civ, j) || !trailable(w, j) || !TRAIL.work.iter().any(|&(k, _)| k == t.terrain) {
        return false;
    }
    let cut = TERRAIN[t.terrain].move_cost.is_none_or(|c| t.trail_cost.is_some_and(|h| h < c));
    cut || t.river > 0
}

/// The labour a trail on tile `j` still needs, or `None` where none can be built.
fn trail_left(w: &World, j: usize) -> Option<f64> {
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
fn walk_costs(w: &World, home: usize) -> BTreeMap<usize, f64> {
    cost_map(w, &[home], f64::INFINITY, |i| walkable(w, i))
}

/// The cheapest tile to stand on to work on tile `j` (`j` itself or a neighbour) that the walk
/// reaches: its walk cost and the tile.
fn beside(w: &World, costs: &BTreeMap<usize, f64>, j: usize) -> Option<(f64, usize)> {
    std::iter::once(j)
        .chain(land::neighbours(w, j))
        .filter_map(|n| costs.get(&n).map(|&c| (c, n)))
        .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)))
}

/// The walk to a tile beside `j` from `home`: empty when it stands there already, `None` when no
/// walk reaches one.
fn approach(w: &World, costs: &BTreeMap<usize, f64>, home: usize, j: usize) -> Option<Vec<usize>> {
    if steps(w, home, j) <= 1 {
        return Some(Vec::new());
    }
    let (_, n) = beside(w, costs, j)?;
    if n == home {
        return Some(Vec::new());
    }
    path_between(w, home, n).map(|(p, _)| p)
}

/// The tiles a scout works on next, in order: mountains first (the ones that decide whether a
/// route exists), then the nearest to the scout, then the biggest saving. A mountain candidate
/// stands for its range: the unrevealed mountains joined to it whose cheapest possible route
/// through them beats the known one. So the search sweeps along a range from where it stands.
fn ordered(w: &World, civ: u32, plan: &Plan, from: usize, to: usize, costs: &BTreeMap<usize, f64>) -> Vec<usize> {
    let known_cost = plan.known.as_ref().map_or(f64::INFINITY, |k| k.1);
    let mut set: BTreeSet<usize> = plan.candidates.iter().copied().collect();
    let through = |m: usize| {
        let a = path_by(w, from, m, |i, j| step_as(w, civ, i, j, true));
        let b = path_by(w, m, to, |i, j| step_as(w, civ, i, j, true));
        match (a, b) {
            (Some((_, x)), Some((_, y))) => x + y,
            _ => f64::INFINITY,
        }
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
    let mut keyed: Vec<(u8, f64, f64, usize)> = set
        .into_iter()
        .map(|j| {
            let reach = beside(w, costs, j).map_or(f64::INFINITY, |(c, _)| c);
            let tier = u8::from(w.tiles[j].terrain != MOUNTAIN);
            let saving = match tile_cost(w, civ, j, false) {
                Some(k) => k - tile_cost(w, civ, j, true).unwrap_or(k),
                None => f64::INFINITY,
            };
            (tier, reach, saving, j)
        })
        .collect();
    keyed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)).then(b.2.total_cmp(&a.2)).then(a.3.cmp(&b.3)));
    keyed.into_iter().map(|k| k.3).collect()
}

/// Puts a scout's walk on `path` into its labour `left`: steps taken while each costs no more than
/// what is left. A step part-paid carries to next turn as `banked`. Returns whether it reached the
/// end of the path.
fn march(w: &mut World, id: u32, path: &[usize], left: &mut f64, banked: &mut f64) -> bool {
    let (taken, rest) = stride(w, w.units[&id].home, path, *left, EPS);
    for &t in &path[..taken] {
        w.tiles[t].walked = true;
    }
    *left = rest;
    let u = w.units.get_mut(&id).unwrap();
    if taken > 0 {
        let at = path[taken - 1];
        u.actor.home = at;
        u.actor.territory = vec![at];
    }
    let done = taken == path.len();
    if done {
        *banked = 0.0;
    } else {
        *banked = *left;
        *left = 0.0;
    }
    done
}

/// Logs a scout's event about its tile, with a route cost where it has one.
fn scout_event(w: &mut World, kind: EventKind, tile: usize, civ: u32, id: u32, cost: Option<f64>) {
    w.emit(Event::new(kind, tile).civ(civ).unit(id).cost(cost));
    let at = w.at(tile);
    let text = match kind {
        EventKind::ScoutRouteFound => format!("has its route: cost {:.1}", cost.unwrap_or(0.0)),
        EventKind::ScoutNoRoute => "no route can exist to its goal".to_string(),
        EventKind::ScoutTrailBuilt => format!("builds a trail at {at}"),
        EventKind::ScoutPassFound => format!("finds a pass at {at}"),
        EventKind::ScoutBlocked => format!("is blocked at {at}"),
        EventKind::ScoutHungry => format!("goes hungry at {at}"),
        EventKind::ScoutStarved => format!("loses a person to hunger at {at}"),
        _ => return,
    };
    w.note(id, text);
}

/// Clears a scout's route and what it has reported about it.
fn clear_route(w: &mut World, id: u32) {
    if let Some(s) = w.units.get_mut(&id).and_then(|u| u.scout_mut()) {
        s.route = None;
        s.heard = false;
        s.blocked = false;
    }
}

/// Reports a scout is blocked, once until it moves on.
fn blocked(w: &mut World, id: u32, civ: u32) {
    if sc(w, id).blocked {
        return;
    }
    let tile = w.units[&id].home;
    sc_mut(w, id).blocked = true;
    scout_event(w, EventKind::ScoutBlocked, tile, civ, id, None);
}

/// Spends the scout's labour `left` on its route, as `scout/`'s `scoutWork`: it walks to the first
/// reveal it can reach, reveals it, and builds the trails its route needs; a mountain takes a whole
/// turn's search labour, begun beside it (`start` is what the search had this turn). Returns the
/// labour a part-paid step carries to next turn.
fn search(w: &mut World, id: u32, left: &mut f64, start: f64) -> f64 {
    let civ = w.units[&id].civ;
    let mut banked = 0.0;
    let mut guard = 0;
    while *left > EPS && guard < 100_000 {
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
        let order = ordered(w, civ, &plan, from, to, &costs);
        let reach = order.iter().find_map(|&j| approach(w, &costs, home, j).map(|p| (j, p)));
        if let Some((j, path)) = reach {
            if !path.is_empty() {
                march(w, id, &path, left, &mut banked);
                sc_mut(w, id).blocked = false;
                continue;
            }
            if w.tiles[j].terrain == MOUNTAIN {
                // A mountain needs the whole turn's search labour, begun beside it.
                if *left + EPS < start {
                    break;
                }
                reveal(w, id, civ, j);
                *left = 0.0;
            } else {
                if *left + EPS < SCOUT.reveal {
                    break;
                }
                *left -= SCOUT.reveal;
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
                Some(cost) if !no_reveal && needs.is_empty() => {
                    clear_route(w, id);
                    scout_event(w, EventKind::ScoutRouteFound, home, civ, id, Some(cost));
                }
                _ => blocked(w, id, civ),
            }
            break;
        };
        if !path.is_empty() {
            march(w, id, &path, left, &mut banked);
            sc_mut(w, id).blocked = false;
            continue;
        }
        let Some(need) = trail_left(w, j) else { break };
        let used = need.min(*left);
        if used <= EPS {
            break;
        }
        *left -= used;
        if build_trail(w, j, used) {
            scout_event(w, EventKind::ScoutTrailBuilt, j, civ, id, None);
        }
        sc_mut(w, id).blocked = false;
    }
    banked
}

/// Reveals tile `j` for `civ`; a mountain that can be bridged is a pass found.
fn reveal(w: &mut World, id: u32, civ: u32, j: usize) {
    w.civs.get_mut(&civ).unwrap().revealed.insert(j);
    sc_mut(w, id).blocked = false;
    if w.tiles[j].terrain == MOUNTAIN && trailable(w, j) {
        scout_event(w, EventKind::ScoutPassFound, j, civ, id, None);
    }
}
/// One scout's turn: it spoils, feeds itself (forage only on a route and off rival land, then the
/// line for what is still short), searches on the labour left, and eats. Its people go hungry as a
/// camp's do; a scout that starves to nothing is gone.
pub(super) fn scout_turn(w: &mut World, id: u32) {
    let civ = w.units[&id].civ;
    w.units.get_mut(&id).unwrap().spoil();
    let people = w.units[&id].people;
    let need = people * UNIT.food;
    let mut left = people * UNIT.labour + w.units[&id].banked;
    let mut banked = 0.0;
    w.units.get_mut(&id).unwrap().banked = 0.0;

    // Without a route it walks home to the capital, to be fed there rather than starve out in the
    // country (where it forages nothing).
    if sc(w, id).route.is_none() {
        let (home, capital) = (w.units[&id].home, w.civs[&civ].capital);
        if let Some((p, _)) = path_between(w, home, capital) {
            if !p.is_empty() {
                march(w, id, &p, &mut left, &mut banked);
            }
        }
    }

    // Feeds first: foraging only while it has a route, and never on rival land.
    if sc(w, id).route.is_some() && !rival_of(w, &w.units[&id]) {
        let short = (need - total(&w.units[&id].store)).max(0.0);
        let (used, _) = strip(w, id, left, short.min(room(&w.units[&id])), UNIT.forage);
        left -= used;
    }
    let short = (need - total(&w.units[&id].store)).max(0.0);
    let (used, _) = supply(w, id, left, short.min(room(&w.units[&id])));
    left -= used;

    // Searches on what labour is left.
    let start = left;
    if sc(w, id).route.is_some() {
        banked = search(w, id, &mut left, start);
    }
    w.units.get_mut(&id).unwrap().banked = banked;

    // The line, for the page.
    let routes = match line(w, id) {
        Some((p, cost, _)) if !p.is_empty() => vec![Route { to: w.civs[&civ].capital, path: p, cost }],
        _ => Vec::new(),
    };
    w.units.get_mut(&id).unwrap().actor.routes = routes;

    // Eating, and hunger.
    let u = w.units.get_mut(&id).unwrap();
    let eaten = u.eat_by_share(need);
    let fed = total(&eaten) + EPS >= need;
    u.hungry = if fed { 0 } else { u.hungry + 1 };
    let (hungry, tile) = (u.hungry, u.home);
    if hungry == 1 {
        scout_event(w, EventKind::ScoutHungry, tile, civ, id, None);
    }
    if hungry >= UNIT.hungry_turns {
        let u = w.units.get_mut(&id).unwrap();
        u.people -= 1.0;
        scout_event(w, EventKind::ScoutStarved, tile, civ, id, None);
        if w.units[&id].people <= EPS {
            w.units.remove(&id);
        }
    }
}
