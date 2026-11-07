//! The scout: it searches a route for its civ, pricing it as the civ knows the map (`known`) and as
//! the unrevealed tiles could still be (`possible`), revealing what could change the answer, and
//! building the trails the route needs. Its labour is its people's, spent on steps, reveals and
//! trails, after it has fed itself.

use super::forage::Ladder;
use super::line::{line, walkable};
use super::search::{route_plan, search, walk_costs};
use super::{pack_cap, rival_of, room, EPS};
use crate::actors::{stride, walk_cost, Route};
use crate::claim;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land;
use crate::market::total;
use crate::world::World;
use std::collections::BTreeMap;

/// What a scout's last turn brought against what it eats, when it went hungry: derived, rebuilt each turn
/// and not saved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hunger {
    /// Food its foraging brought, and its line.
    pub forage: f64,
    pub line: f64,
    /// What it eats a turn.
    pub eats: f64,
    /// The line from where it stands: its cost and what it carries a turn, if one reaches it.
    pub cost: Option<f64>,
    pub carry: Option<f64>,
}

/// A scout's own state: its route (the tile it searches from and the tile it searches for), and what
/// it has said about it.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoutState {
    /// The tile a scout marches to when it has no route (`Move`), and forages at on arriving.
    pub dest: Option<usize>,
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
    /// The tile it is going to forage at: rich ground. A routed scout leaves its errand for it while
    /// it runs low; an idle scout waits there.
    pub wander: Option<usize>,
    /// Why it went hungry this turn (not saved).
    pub hunger: Option<Hunger>,
}

impl ScoutState {
    /// A scout with no route and its switches on.
    pub fn new() -> Self {
        ScoutState {
            dest: None,
            route: None,
            heard: false,
            blocked: false,
            built: false,
            auto_forage: true,
            auto_supply: true,
            resupply: false,
            forage: false,
            pillage: false,
            wander: None,
            hunger: None,
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

/// Puts a scout's walk on `path` into its labour `left`: a unit walks together, so each step is paid
/// by every one of its people (it costs `step_cost × people`), and is taken while it costs no more
/// than what is left. A step part-paid carries to next turn as `banked`. Returns whether it reached
/// the end of the path.
pub(super) fn march(w: &mut World, id: u32, path: &[usize], ladder: &mut Ladder, banked: &mut f64) -> bool {
    let people = w.units[&id].people;
    let (taken, each) = stride(w, w.units[&id].home, path, ladder.left / people, EPS / people);
    for &t in &path[..taken] {
        w.tiles[t].traffic += people;
        w.tiles[t].used = true;
    }
    ladder.left = each * people;
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
        *banked = ladder.left;
        ladder.left = 0.0;
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

/// One tile's take in a foraging round: the walk to it and back, the food, and the rate it comes at.
struct Take {
    tile: usize,
    walk: f64,
    food: f64,
    rate: f64,
}

/// The foraging round a scout standing on `at` would make with `labour`: its own tile and the tiles
/// within `SCOUT.reach` steps it can walk to, richest spare first, up to `want` of food, each walk
/// there and back paid from the labour by every one of its people. A tile that costs more labour a unit of food than
/// `line_price` is left. The one costing `forage_round` takes and `rich_ground` values.
fn round_plan(w: &World, id: u32, at: usize, labour: f64, want: f64, line_price: Option<f64>) -> Vec<Take> {
    let costs = walk_costs(w, at);
    let people = w.units[&id].people;
    let mut tiles: Vec<(f64, usize)> = near(w, at, SCOUT.reach)
        .into_iter()
        .filter(|&t| t == at || walkable(w, t))
        .map(|t| (total(&land::spare(&w.tiles[t])), t))
        .filter(|&(held, _)| held > 0.0)
        .collect();
    tiles.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let (mut left, mut got) = (labour, 0.0);
    let mut plan = Vec::new();
    for (held, t) in tiles {
        let need = want - got;
        if need <= EPS || left <= EPS {
            break;
        }
        let walk = if t == at {
            0.0
        } else {
            match costs.get(&t) {
                Some(&cost) => walk_cost(cost, 1.0) * people,
                None => continue,
            }
        };
        if walk >= left {
            continue;
        }
        let rate = if rival_tile(w, id, t) { UNIT.pillage } else { UNIT.forage };
        let price = walk / held.min(need) + 1.0 / rate;
        if line_price.is_some_and(|p| price > p + EPS) {
            continue;
        }
        let food = ((left - walk) * rate).min(held).min(need);
        if food <= EPS {
            continue;
        }
        left -= walk + food / rate;
        got += food;
        plan.push(Take { tile: t, walk, food, rate });
    }
    plan
}

/// The scout forages (pillages, on a rival's land) its own tile and the tiles within `SCOUT.reach`
/// steps for up to `want` of food, richest spare first, paying each walk there and back from its
/// labour `left`, and taking only what each tile spares. A tile that costs more labour a unit of food
/// than the line does is left while the store has food to bring. Returns the food taken.
fn forage_round(w: &mut World, id: u32, ladder: &mut Ladder, want: f64) -> f64 {
    let home = w.units[&id].home;
    let line_price = if sc(w, id).auto_supply { line_price(w, id) } else { None };
    let want = want.min(room(&w.units[&id]));
    let mut got = 0.0;
    for Take { tile, walk, food, rate } in round_plan(w, id, home, ladder.left, want, line_price) {
        got += ladder.graze_walked(w, id, tile, walk, food, rate);
    }
    got
}

/// What a turn's foraging brings a scout standing on `at`: its whole labour on a round that fills its
/// pack. The worth of ground.
pub fn ground_worth(w: &World, id: u32, at: usize) -> f64 {
    let u = &w.units[&id];
    round_plan(w, id, at, u.people * UNIT.labour, pack_cap(u.people), None).iter().map(|t| t.food).sum()
}

/// The ground a scout would go to feed: among the tiles within `SCOUT.wander` steps that it can walk
/// to, the nearest by walk cost where a turn's foraging brings at least `SCOUT.rich` turns of its
/// eating, else the one that brings the most. `None` when nothing in range brings anything.
pub fn rich_ground(w: &World, id: u32) -> Option<usize> {
    let u = &w.units[&id];
    let home = u.home;
    let costs: BTreeMap<usize, f64> = walk_costs(w, home);
    let rich = SCOUT.rich * u.people * UNIT.food;
    let worth: Vec<(f64, usize, f64)> = near(w, home, SCOUT.wander)
        .into_iter()
        .filter(|&t| t == home || land::steps(w, home, t) as usize <= SCOUT.wander)
        .filter_map(|t| {
            let cost = if t == home { 0.0 } else { *costs.get(&t)? };
            Some((cost, t, ground_worth(w, id, t)))
        })
        .collect();
    let near_rich =
        worth.iter().filter(|&&(_, _, v)| v + EPS >= rich).min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let best = worth.iter().filter(|&&(_, _, v)| v > EPS).max_by(|a, b| a.2.total_cmp(&b.2).then(b.1.cmp(&a.1)));
    near_rich.or(best).map(|&(_, t, _)| t)
}

/// Brings up to `want` of food along the line into the pack on `left` labour, the line's tiles
/// counting as walked.
fn bring(w: &mut World, id: u32, ladder: &mut Ladder, want: f64) -> f64 {
    let food = ladder.supply(w, id, want);
    if food > 0.0 {
        let people = w.units[&id].people;
        for &t in line(w, id).iter().flat_map(|(p, _, _)| p) {
            w.tiles[t].traffic += people;
            w.tiles[t].used = true;
        }
    }
    food
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

/// Sets where a scout wanders to feed, and says so when it is somewhere new.
fn wander_to(w: &mut World, id: u32, to: usize, text: impl Fn(&str, i32) -> String) {
    sc_mut(w, id).wander = Some(to);
    let n = land::steps(w, w.units[&id].home, to);
    let at = w.at(to);
    w.note(id, text(&at, n));
}

/// The start of a scout's feeding: whether it goes to rich ground, stays there or picks again.
/// Ground is read before the scout forages, as the scout's own take lowers what it spares.
fn choose_ground(w: &mut World, id: u32) {
    let u = &w.units[&id];
    let (people, home, hungry) = (u.people, u.home, u.hungry);
    let eats = people * UNIT.food;
    let pack = total(&u.store);
    let s = sc(w, id);
    let errand = s.route.is_some() || s.dest.is_some();
    let (wander, dest) = (s.wander, s.dest);
    let goal = match (s.route, dest) {
        (Some((_, to)), _) => Some(to),
        (None, d) => d,
    };
    let leaves = |at: &str, n: i32| match goal {
        Some(_) if dest.is_none() => format!("leaves its route to forage at {at}, {n} steps away"),
        _ => format!("leaves for {at}, {n} steps away"),
    };
    match (errand, wander) {
        (true, None) => {
            if pack < SCOUT.low * eats || hungry > 0 {
                if let Some(g) = rich_ground(w, id) {
                    wander_to(w, id, g, leaves);
                }
            }
        }
        (true, Some(g)) => {
            if g == home && ground_worth(w, id, home) + EPS < SCOUT.rich * eats {
                match rich_ground(w, id) {
                    Some(n) if n != g => wander_to(w, id, n, leaves),
                    _ => {}
                }
            }
        }
        (false, None) => {
            if let Some(g) = rich_ground(w, id) {
                wander_to(w, id, g, |at, _| format!("waits on rich ground at {at}"));
            }
        }
        (false, Some(g)) => {
            if g == home && ground_worth(w, id, home) + EPS < eats {
                match rich_ground(w, id) {
                    Some(n) if n != g => wander_to(w, id, n, |at, _| format!("waits on rich ground at {at}")),
                    _ => {}
                }
            }
        }
    }
}

/// One scout's turn: it spoils, feeds itself (its one-shot orders, then foraging round it for what its
/// pack lacks, then the line to fill the pack), searches on the labour left, and eats. A scout that
/// runs low leaves its errand for rich ground and comes back fed; an idle one waits on rich ground.
/// Its people go hungry as a camp's do; a scout that starves to nothing is gone.
pub(super) fn scout_turn(w: &mut World, id: u32) {
    let civ = w.units[&id].civ;
    w.units.get_mut(&id).unwrap().spoil();
    let people = w.units[&id].people;
    let need = people * UNIT.food;
    let mut ladder = Ladder::new(people * UNIT.labour + w.units[&id].banked);
    let mut banked = 0.0;
    w.units.get_mut(&id).unwrap().banked = 0.0;
    let (mut by_forage, mut by_line) = (0.0, 0.0);

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
        let home = w.units[&id].home;
        by_forage += ladder.graze(w, id, home, room(&w.units[&id]), rate);
    }
    if once_supply {
        by_line += bring(w, id, &mut ladder, room(&w.units[&id]));
    }

    choose_ground(w, id);
    let wander = sc(w, id).wander;
    let wandering = wander.is_some();
    if let Some(g) = wander {
        // It marches to rich ground first.
        let home = w.units[&id].home;
        if home != g {
            if let Some((p, _)) = super::line::path_between(w, home, g) {
                march(w, id, &p, &mut ladder, &mut banked);
            }
        }
    }

    // It feeds itself first: auto-forage covers what eating needs that the pack doesn't, round the
    // scout's tile (a wanderer fills its pack), and auto-supply the rest of it along the line.
    let short = |w: &World| (need - total(&w.units[&id].store)).max(0.0).min(room(&w.units[&id]));
    if sc(w, id).auto_forage {
        let want = if wandering { room(&w.units[&id]) } else { short(w) };
        by_forage += forage_round(w, id, &mut ladder, want);
    }
    if sc(w, id).auto_supply {
        let want = short(w);
        by_line += bring(w, id, &mut ladder, want);
    }

    // A routed scout fed to `SCOUT.fed` turns goes back to its errand next turn.
    let errand = sc(w, id).route.is_some() || sc(w, id).dest.is_some();
    if wandering && errand && total(&w.units[&id].store) + EPS >= SCOUT.fed * need {
        let s = sc_mut(w, id);
        s.wander = None;
        let back = match (s.route, s.dest) {
            (_, Some(d)) => Some(d),
            _ => None,
        };
        match back {
            Some(d) => {
                let at = w.at(d);
                w.note(id, format!("fed: back to {at}"));
            }
            None => w.note(id, "fed: back to its route".to_string()),
        }
    }

    // Searches on what labour is left, or without a route marches to its destination; a wanderer does
    // neither.
    let start = ladder.left;
    if wandering {
        if sc(w, id).route.is_some() {
            keep_route(w, id);
        }
        banked = banked.max(0.0);
    } else if sc(w, id).route.is_some() {
        banked = search(w, id, &mut ladder, start);
        keep_route(w, id);
    } else if let Some(dest) = sc(w, id).dest {
        // It walks to its destination and stays there: the destination is its errand until another
        // order, so a hungry scout comes back to it fed.
        let home = w.units[&id].home;
        if home != dest {
            if let Some((p, _)) = super::line::path_between(w, home, dest) {
                if march(w, id, &p, &mut ladder, &mut banked) {
                    let at = w.at(dest);
                    w.note(id, format!("at {at}, foraging"));
                }
            }
        }
    }
    // Auto-supply refills the pack to full on what the search left.
    if sc(w, id).auto_supply {
        by_line += bring(w, id, &mut ladder, room(&w.units[&id]));
    }
    w.units.get_mut(&id).unwrap().banked = banked;

    // The line, for the page.
    let line_now = line(w, id);
    let routes = match &line_now {
        Some((p, cost, _)) if !p.is_empty() => vec![Route { to: w.civs[&civ].capital, path: p.clone(), cost: *cost }],
        _ => Vec::new(),
    };
    w.units.get_mut(&id).unwrap().actor.routes = routes;

    // Eating, and hunger.
    let u = w.units.get_mut(&id).unwrap();
    let eaten = u.eat_by_share(need);
    let fed = total(&eaten) + EPS >= need;
    u.hungry = if fed { 0 } else { u.hungry + 1 };
    let (hungry, tile) = (u.hungry, u.home);
    let reason = (!fed).then(|| Hunger {
        forage: by_forage,
        line: by_line,
        eats: need,
        cost: line_now.as_ref().map(|l| l.1),
        carry: line_now.as_ref().map(|l| l.2),
    });
    sc_mut(w, id).hunger = reason;
    if hungry == 1 {
        scout_event(w, EventKind::ScoutHungry, tile, civ, id, None);
    }
    if let Some(h) = reason {
        let at = w.at(tile);
        let tail = match (h.cost, h.carry) {
            (Some(c), Some(k)) => format!("its line costs {c:.1} and carries {k:.1} a turn from here"),
            _ => "no line reaches it".to_string(),
        };
        w.note(
            id,
            format!(
                "goes hungry at {at}: foraging brought {:.1} and its line {:.1} of the {:.0} it eats; {tail}",
                h.forage, h.line, h.eats
            ),
        );
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
