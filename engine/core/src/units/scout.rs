//! The scout: it searches a route for its civ, pricing it as the civ knows the map (`known`) and as
//! the unrevealed tiles could still be (`possible`), revealing what could change the answer, and
//! building the trails the route needs. Its labour is its people's, spent on steps, reveals and
//! trails, after it has fed itself.

use super::line::{line, path_between, walkable};
use super::search::{route_plan, search};
use super::{rival_of, room, strip, strip_at, supply, EPS};
use crate::actors::{stride, walk_cost, Route};
use crate::claim;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land;
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
    /// A scout has said its trails are built, so says it once while the route stays set.
    pub built: bool,
    /// Whether it forages (pillages on a rival's land) round it for what its pack lacks, and
    /// refills its pack from the store along its line, each turn.
    pub auto_forage: bool,
    pub auto_supply: bool,
    /// One-shot orders for this turn's end.
    pub resupply: bool,
    pub forage: bool,
    pub pillage: bool,
}

impl ScoutState {
    /// A scout with no route and its switches on.
    pub fn new() -> Self {
        ScoutState {
            route: None,
            heard: false,
            blocked: false,
            built: false,
            auto_forage: true,
            auto_supply: true,
            resupply: false,
            forage: false,
            pillage: false,
        }
    }
}

impl Default for ScoutState {
    fn default() -> Self {
        ScoutState::new()
    }
}

/// The scout state of unit `id` (the unit is a scout; the callers have checked).
pub(super) fn sc(w: &World, id: u32) -> &ScoutState {
    w.units[&id].scout().expect("a scout")
}

pub(super) fn sc_mut(w: &mut World, id: u32) -> &mut ScoutState {
    w.units.get_mut(&id).and_then(|u| u.scout_mut()).expect("a scout")
}

/// Puts a scout's walk on `path` into its labour `left`: steps taken while each costs no more than
/// what is left. A step part-paid carries to next turn as `banked`. Returns whether it reached the
/// end of the path.
pub(super) fn march(w: &mut World, id: u32, path: &[usize], left: &mut f64, banked: &mut f64) -> bool {
    let (taken, rest) = stride(w, w.units[&id].home, path, *left, EPS);
    let people = w.units[&id].people;
    for &t in &path[..taken] {
        w.tiles[t].traffic += people;
        w.tiles[t].used = true;
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
pub(super) fn scout_event(w: &mut World, kind: EventKind, tile: usize, civ: u32, id: u32, cost: Option<f64>) {
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
pub(super) fn clear_route(w: &mut World, id: u32) {
    if let Some(s) = w.units.get_mut(&id).and_then(|u| u.scout_mut()) {
        s.route = None;
        s.heard = false;
        s.blocked = false;
        s.built = false;
    }
}

/// Reports a scout is blocked, once until it moves on.
pub(super) fn blocked(w: &mut World, id: u32, civ: u32) {
    if sc(w, id).blocked {
        return;
    }
    let tile = w.units[&id].home;
    sc_mut(w, id).blocked = true;
    scout_event(w, EventKind::ScoutBlocked, tile, civ, id, None);
}

/// The tiles within `reach` steps of `from`, `from` first, then by tile.
fn near(w: &World, from: usize, reach: usize) -> Vec<usize> {
    let mut seen = vec![from];
    let mut frontier = vec![from];
    for _ in 0..reach {
        let mut next = Vec::new();
        for t in frontier {
            for n in land::neighbours(w, t) {
                if !seen.contains(&n) {
                    seen.push(n);
                    next.push(n);
                }
            }
        }
        frontier = next;
    }
    let rest = seen.split_off(1);
    let mut rest = rest;
    rest.sort_unstable();
    seen.extend(rest);
    seen
}

/// Whether tile `t` is a rival's land to scout `id`'s civ.
fn rival_tile(w: &World, id: u32, t: usize) -> bool {
    claim::tile_civ(w, t).is_some_and(|c| c != w.units[&id].civ)
}

/// The labour a unit of food costs on the scout's line: the line's cost over what a unit of labour
/// carries. `None` when the store holds nothing to bring, or no line reaches it.
fn line_price(w: &World, id: u32) -> Option<f64> {
    let civ = w.units[&id].civ;
    let (_, cost, _) = line(w, id)?;
    (total(&w.civs[&civ].store) > EPS).then_some(cost / UNIT.carry)
}

/// The scout forages (pillages, on a rival's land) its own tile and the tiles within `SCOUT.reach`
/// steps for up to `want` of food, richest first, paying each walk there and back from its labour
/// `left`. A tile that costs more labour a unit of food than the line does is left while the store
/// has food to bring. Returns the food taken.
fn forage_round(w: &mut World, id: u32, left: &mut f64, want: f64) -> f64 {
    let home = w.units[&id].home;
    let line_price = if sc(w, id).auto_supply { line_price(w, id) } else { None };
    let mut tiles: Vec<(f64, usize)> = near(w, home, SCOUT.reach)
        .into_iter()
        .filter(|&t| t == home || walkable(w, t))
        .map(|t| (total(&w.tiles[t].stock), t))
        .filter(|&(held, t)| held > 0.0 && w.tiles[t].fallow == 0)
        .collect();
    tiles.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut got = 0.0;
    for (held, t) in tiles {
        let need = want - got;
        if need <= EPS || *left <= EPS {
            break;
        }
        let walk = if t == home {
            0.0
        } else {
            match path_between(w, home, t) {
                Some((_, cost)) => walk_cost(cost, 1.0),
                None => continue,
            }
        };
        if walk >= *left {
            continue;
        }
        let rate = if rival_tile(w, id, t) { UNIT.pillage } else { UNIT.forage };
        let price = walk / held.min(need) + 1.0 / rate;
        if line_price.is_some_and(|p| price > p + EPS) {
            continue;
        }
        let (used, food) = strip_at(w, id, t, *left - walk, need.min(room(&w.units[&id])), rate);
        if food > 0.0 {
            *left -= walk + used;
            got += food;
        }
    }
    got
}

/// Brings up to `want` of food along the line into the pack on `left` labour, the line's tiles
/// counting as walked.
fn bring(w: &mut World, id: u32, left: &mut f64, want: f64) {
    let (used, food) = supply(w, id, *left, want);
    *left -= used;
    if food > 0.0 {
        let people = w.units[&id].people;
        for &t in line(w, id).iter().flat_map(|(p, _, _)| p) {
            w.tiles[t].traffic += people;
            w.tiles[t].used = true;
        }
    }
}

/// While a scout's route is settled (its trails built, or none needed), the tiles of the path it
/// knows count as walked, so its trail holds.
fn keep_route(w: &mut World, id: u32) {
    let civ = w.units[&id].civ;
    let s = sc(w, id);
    let (Some((from, to)), true) = (s.route, s.heard || s.built) else { return };
    if let Some((path, _)) = route_plan(w, civ, from, to).known {
        for t in path {
            w.tiles[t].used = true;
        }
    }
}

/// One scout's turn: it spoils, walks home if it has no route, feeds itself (its one-shot orders,
/// then foraging round it for what its pack lacks, then the line to fill the pack), searches on the
/// labour left, and eats. Its people go hungry as a camp's do; a scout that starves to nothing is
/// gone.
pub(super) fn scout_turn(w: &mut World, id: u32) {
    let civ = w.units[&id].civ;
    w.units.get_mut(&id).unwrap().spoil();
    let people = w.units[&id].people;
    let need = people * UNIT.food;
    let mut left = people * UNIT.labour + w.units[&id].banked;
    let mut banked = 0.0;
    w.units.get_mut(&id).unwrap().banked = 0.0;

    // Without a route it walks home to the capital, to be fed there rather than starve out in the
    // country.
    if sc(w, id).route.is_none() {
        let (home, capital) = (w.units[&id].home, w.civs[&civ].capital);
        if let Some((p, _)) = path_between(w, home, capital) {
            if !p.is_empty() {
                march(w, id, &p, &mut left, &mut banked);
            }
        }
    }

    // One-shot orders, on this turn's labour.
    let (once_forage, once_pillage, once_supply) = {
        let s = sc_mut(w, id);
        let once = (s.forage, s.pillage, s.resupply);
        (s.forage, s.pillage, s.resupply) = (false, false, false);
        once
    };
    let here = rival_of(w, &w.units[&id]);
    if (once_forage && !here) || (once_pillage && here) {
        let rate = if here { UNIT.pillage } else { UNIT.forage };
        let (used, _) = strip(w, id, left, room(&w.units[&id]), rate);
        left -= used;
    }
    if once_supply {
        bring(w, id, &mut left, room(&w.units[&id]));
    }
    // It feeds itself first: auto-forage covers what eating needs that the pack doesn't, round the
    // scout's tile, and auto-supply the rest of it along the line.
    let short = |w: &World| (need - total(&w.units[&id].store)).max(0.0).min(room(&w.units[&id]));
    if sc(w, id).auto_forage {
        let want = short(w);
        forage_round(w, id, &mut left, want);
    }
    if sc(w, id).auto_supply {
        let want = short(w);
        bring(w, id, &mut left, want);
    }

    // Searches on what labour is left.
    let start = left;
    if sc(w, id).route.is_some() {
        banked = search(w, id, &mut left, start);
        keep_route(w, id);
    }
    // Auto-supply refills the pack to full on what the search left.
    if sc(w, id).auto_supply {
        bring(w, id, &mut left, room(&w.units[&id]));
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
