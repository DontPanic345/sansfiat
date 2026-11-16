//! The scout: it searches a route for its civ, pricing it as the civ knows the map (`known`) and as
//! the unrevealed tiles could still be (`possible`), revealing what could change the answer, and
//! building the trails the route needs. Its labour is its people's, spent on steps, reveals and
//! trails. It eats from its pack and fills it at a store; a rival army's zone shuts it in or out,
//! and a rival army on its tile kills it.

use super::forage::Ladder;
use super::line::{turns, walkable};
use super::search::{route_plan, search};
use super::{cap_of, room, EPS};
use crate::actors::{stride, Route};
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land;
use crate::market::{add, take, total};
use crate::settle;
use crate::world::World;
use std::collections::BTreeMap;

/// What a scout is doing this turn, as its line in the view and its panel say it: derived, set each
/// turn and not saved.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Doing {
    /// No errand: it stands where it is.
    #[default]
    Idle,
    /// On its errand: searching its route, or walking to its destination.
    Searching,
    /// Its pack ran low: it walks to the store on this tile, this many turns away.
    ToStore(usize, u32),
    /// At the store on this tile, its pack too light to set out on yet.
    Filling(usize),
    /// At the store on this tile, its errand further than a full pack takes it and brings it back.
    TooFar(usize),
    /// Fed, walking back to the tile it left its errand on.
    Back(usize),
    /// On a tile of this rival's zone: it stays until the zone lifts or the army leaves.
    Trapped(u32),
}

/// A scout's own state: its route (the tile it searches from and the tile it searches for), and what
/// it has said about it.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoutState {
    /// The tile a scout marches to when it has no route (`Move`), and stays at.
    pub dest: Option<usize>,
    /// The tile the scout searches from and the tile it searches for.
    pub route: Option<(usize, usize)>,
    /// A scout has said what its route costs; it is in `heard` once a route is found.
    pub heard: bool,
    /// A scout has reported that it is blocked, so says it once.
    pub blocked: bool,
    /// A scout has said its trails are built, so says it once while the route stays set.
    pub built: bool,
    /// Its pack ran low and it is walking to a store.
    pub homing: bool,
    /// The tile it left its errand on, which it walks back to once fed.
    pub back: Option<usize>,
    /// What it did this turn (not saved).
    pub doing: Doing,
}

impl ScoutState {
    /// A scout with no route.
    pub fn new() -> Self {
        ScoutState {
            dest: None,
            route: None,
            heard: false,
            blocked: false,
            built: false,
            homing: false,
            back: None,
            doing: Doing::Idle,
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
        EventKind::ScoutHungry => format!("goes hungry at {at}: its pack is empty"),
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
        s.back = None;
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

/// The built passes between a scout and its nearest store count as walked, so the way home stays
/// open behind it: a pass it built and left would wear away, and strand it past a wall with no store
/// to walk to.
fn keep_way(w: &mut World, id: u32) {
    let Some((_, path, _)) = store_walk_by(w, id, land::passable) else { return };
    for t in path {
        if w.tiles[t].trail {
            w.tiles[t].used = true;
        }
    }
}

/// The tiles a scout of `civ` can't enter or leave: every tile a rival army holds or stands on.
pub fn shut(w: &World, civ: u32) -> BTreeMap<usize, u32> {
    let mut out = BTreeMap::new();
    for u in w.units.values().filter(|u| u.civ != civ && u.camp().is_some() && u.people > EPS) {
        for &t in u.held().iter().chain(std::iter::once(&u.home)) {
            out.entry(t).or_insert(u.civ);
        }
    }
    out
}

/// The walk from scout `id`'s tile to the store of its civ it reaches for the least, among those
/// holding a turn of its food (with none so stocked, among them all): the store's tile, the path and
/// the turns the walk takes. A scout walks any land and built pass outside a rival army's zone.
pub fn store_walk(w: &World, id: u32) -> Option<(usize, Vec<usize>, u32)> {
    store_walk_by(w, id, walkable)
}

/// `store_walk` over the tiles `open` lets a scout onto, outside a rival army's zone.
fn store_walk_by(w: &World, id: u32, open: impl Fn(&World, usize) -> bool) -> Option<(usize, Vec<usize>, u32)> {
    let u = w.units.get(&id)?;
    let shut = shut(w, u.civ);
    let eats = u.people * UNIT.food;
    let mut best: Option<(bool, f64, usize, Vec<usize>)> = None;
    for store in settle::roots(w, u.civ) {
        let stocked = settle::store(w, u.civ, store).is_some_and(|s| total(s) + EPS >= eats);
        if best.as_ref().is_some_and(|b| b.0 && !stocked) {
            continue;
        }
        let Some((tiles, cost)) = land::path(w, u.home, store, |i| open(w, i) && !shut.contains_key(&i)) else {
            continue;
        };
        if best.as_ref().is_none_or(|b| (stocked && !b.0) || (stocked == b.0 && cost < b.1)) {
            best = Some((stocked, cost, store, tiles));
        }
    }
    let (_, _, store, tiles) = best?;
    let n = turns(w, &tiles, u.home, 0.0);
    Some((store, tiles, n))
}

/// What scout `u` is doing, in words: its line in the view and its panel.
pub fn doing_text(w: &World, u: &super::Unit) -> String {
    let Some(s) = u.scout() else { return String::new() };
    let store = |t: usize| settle::store_name(w, u.civ, t);
    match s.doing {
        Doing::Idle if s.route.is_some() && s.built => "its route is built".to_string(),
        Doing::Idle => "idle".to_string(),
        Doing::Searching if s.route.is_some() => "searching".to_string(),
        Doing::Searching => "walking to its destination".to_string(),
        Doing::ToStore(t, n) => format!("walking to {} to fill its pack, {n} turns away", store(t)),
        Doing::Filling(t) => format!("waiting at {} for a pack", store(t)),
        Doing::TooFar(t) => {
            format!("waiting at {}: its errand is further than a full pack takes it and back", store(t))
        }
        Doing::Back(t) => format!("walking back to its errand at {}", w.at(t)),
        Doing::Trapped(by) => format!("trapped in {}'s zone", super::name_of(w, by)),
    }
}

/// The turns of eating in scout `id`'s pack.
pub fn pack_turns(w: &World, id: u32) -> f64 {
    let u = &w.units[&id];
    total(&u.store) / (u.people * UNIT.food)
}

/// Takes up to `want` of food from `civ`'s store on `tile` into scout `id`'s pack.
fn fill(w: &mut World, id: u32, tile: usize, want: f64) {
    let civ = w.units[&id].civ;
    let want = want.min(room(&w.units[&id]));
    if want <= EPS {
        return;
    }
    let got = settle::store_mut(w, civ, tile).map(|s| take(s, want)).unwrap_or_default();
    add(&mut w.units.get_mut(&id).unwrap().store, &got);
}

/// One scout's turn. Its food is its pack, filled at a store of its civ: standing on one it eats from
/// it, and takes a full pack when it has an errand. Away from one, when the pack holds no more than
/// the walk to the nearest store and `SCOUT.spare` turns over, it walks there, fills and walks back
/// to the tile it left. It sets out from a store with at least `SCOUT.out` of a pack, and not at all
/// for a tile a full pack can't reach and come back from. On a rival army's zone it stays where it
/// is. Its people go hungry as a camp's do; a scout that starves to nothing is gone.
pub(super) fn scout_turn(w: &mut World, id: u32) {
    let civ = w.units[&id].civ;
    let people = w.units[&id].people;
    let need = people * UNIT.food;
    let mut ladder = Ladder::new(people * UNIT.labour + w.units[&id].banked);
    let mut banked = 0.0;
    w.units.get_mut(&id).unwrap().banked = 0.0;
    let shut = shut(w, civ);
    let home = w.units[&id].home;
    // Its errand: a route still to build, or a destination still to reach.
    let errand = {
        let s = sc(w, id);
        (s.route.is_some() && !s.built) || s.dest.is_some_and(|d| d != home)
    };
    let walk = store_walk(w, id);
    let was = sc(w, id).doing;
    let mut doing = if errand { Doing::Searching } else { Doing::Idle };

    if let Some(&by) = shut.get(&home) {
        doing = Doing::Trapped(by);
        if was != doing {
            let (at, name) = (w.at(home), super::name_of(w, by));
            w.note(
                id,
                format!("trapped at {at}: it can't leave {name}'s zone until the zone lifts or the army leaves"),
            );
        }
    } else {
        // At a store it fills; walking, it turns for one when the pack runs low.
        if let Some((store, path, n)) = &walk {
            if path.is_empty() {
                let leaving = errand;
                let want =
                    if leaving { room(&w.units[&id]) } else { (2.0 * need - total(&w.units[&id].store)).max(0.0) };
                fill(w, id, *store, want);
                sc_mut(w, id).homing = false;
                if leaving {
                    let cap = cap_of(&w.units[&id]);
                    let far = sc(w, id)
                        .back
                        .and_then(|b| land::path(w, home, b, |i| walkable(w, i) && !shut.contains_key(&i)));
                    let out = far.map_or(0, |(p, _)| turns(w, &p, home, 0.0));
                    if f64::from(2 * out) + SCOUT.spare + 1.0 > cap / need + EPS {
                        doing = Doing::TooFar(*store);
                        if was != doing {
                            let at = sc(w, id).back.map(|b| w.at(b)).unwrap_or_default();
                            w.note(id, format!("its errand at {at} is {out} turns' walk from its store: further than a full pack takes it and brings it back"));
                            blocked(w, id, civ);
                        }
                    } else if total(&w.units[&id].store) + EPS < SCOUT.out * cap {
                        doing = Doing::Filling(*store);
                        if was != doing {
                            w.note(
                                id,
                                format!(
                                    "waits at its store for a pack: it holds {:.1} of {cap:.0}",
                                    total(&w.units[&id].store)
                                ),
                            );
                        }
                    }
                }
            } else if !sc(w, id).homing && pack_turns(w, id) <= f64::from(*n) + SCOUT.spare + EPS {
                let s = sc_mut(w, id);
                s.homing = true;
                s.back = errand.then_some(home);
                let (at, pack) = (w.at(*store), pack_turns(w, id));
                w.note(id, format!("its pack holds {pack:.1} turns: it walks to its store at {at}, {n} turns away"));
            }
        }
        if sc(w, id).homing {
            match &walk {
                Some((store, path, n)) if !path.is_empty() => {
                    doing = Doing::ToStore(*store, *n);
                    if march(w, id, path, &mut ladder, &mut banked) {
                        // It fills on arriving, and sets out again next turn.
                        let want = if errand { room(&w.units[&id]) } else { need };
                        fill(w, id, *store, want);
                        sc_mut(w, id).homing = false;
                    }
                    ladder.left = 0.0;
                }
                _ => sc_mut(w, id).homing = false,
            }
        }
        if !errand {
            sc_mut(w, id).back = None;
        }
        // Fed, it walks back to the tile it left, and takes up its errand there.
        if doing == Doing::Searching {
            if let Some(b) = sc(w, id).back {
                let here = w.units[&id].home;
                let path = land::path(w, here, b, |i| walkable(w, i) && !shut.contains_key(&i)).map(|(p, _)| p);
                match path {
                    Some(p) if !p.is_empty() => {
                        if march(w, id, &p, &mut ladder, &mut banked) {
                            sc_mut(w, id).back = None;
                        } else {
                            doing = Doing::Back(b);
                            ladder.left = 0.0;
                        }
                    }
                    _ => sc_mut(w, id).back = None,
                }
            }
        }
        // The errand, on the labour left.
        if doing == Doing::Searching && ladder.left > EPS {
            let start = ladder.left;
            if sc(w, id).route.is_some() {
                banked = search(w, id, &mut ladder, start, &shut);
            } else if let Some(dest) = sc(w, id).dest {
                let here = w.units[&id].home;
                if here != dest {
                    let path = land::path(w, here, dest, |i| walkable(w, i) && !shut.contains_key(&i));
                    if let Some((p, _)) = path {
                        if march(w, id, &p, &mut ladder, &mut banked) {
                            let at = w.at(dest);
                            w.note(id, format!("at {at}"));
                        }
                    }
                }
            }
        }
    }
    if !w.units.contains_key(&id) {
        return;
    }
    if sc(w, id).route.is_some() {
        keep_route(w, id);
    }
    keep_way(w, id);
    w.units.get_mut(&id).unwrap().banked = banked;
    sc_mut(w, id).doing = doing;

    // The walk to its store, for the page.
    let routes = match store_walk(w, id) {
        Some((store, p, _)) if !p.is_empty() => {
            let mut at = w.units[&id].home;
            let mut cost = 0.0;
            for &t in &p {
                cost += land::step_cost(w, at, t).unwrap_or(0.0);
                at = t;
            }
            vec![Route { to: store, path: p, cost }]
        }
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

/// Every scout an army of another civ stands on dies: the `units` step runs this after the marches
/// and the fight. With `Config.fight` off nobody dies: the control.
pub(super) fn kill(w: &mut World) {
    if !w.config.fight {
        return;
    }
    let scouts: Vec<(u32, u32, usize)> =
        w.units.values().filter(|u| u.scout().is_some()).map(|u| (u.id, u.civ, u.home)).collect();
    for (id, civ, tile) in scouts {
        let by = w.units.values().find(|a| a.civ != civ && a.camp().is_some() && a.people > EPS && a.home == tile);
        let Some(by) = by.map(|a| a.civ) else { continue };
        w.units.remove(&id);
        w.emit(Event::new(EventKind::ScoutKilled, tile).civ(civ).unit(id).other(by));
    }
}
