//! The army: a civ's camp and its scout are units (`docs/design/supply.md`). A unit holds what every
//! unit has (its warriors, tile, pack and labour) and its kind's own state: a camp's zone, orders
//! and line, or a scout's route. The camp is a pack it eats from, a tile it stands on and a line to
//! the capital, where the civ's store is. Its territory is its zone: the tiles its warriors spread
//! over one ring a turn and hold (`hold`), guarding them with labour set aside, so a rival's line
//! can't pass them. Warriors are raised from the civ's clans, one person from each, and disbanded to
//! the civ's own clan. The `ai` step keeps each AI camp posted; the `units` step runs every unit.
//!
//! The files: `line` (the paths and the line to the capital), `camp` (the camp's turn and orders and
//! its zone), `scout` (the scout's turn and feeding), `search` (the route search), `forage` (what a
//! unit eats, and the labour it spends on food, `Ladder`), `guard` (the civ's guard), `fight` and `ai`
//! (the AI's posts and zones). `muster` raises, disbands and gives up people. The types, the JSON,
//! the queries and the `units` step are here.

mod ai;
mod camp;
pub mod fight;
mod forage;
pub mod guard;
mod line;
mod muster;
mod scout;
mod search;

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::Actor;
use crate::claim;
use crate::data::*;
use crate::world::World;

pub use ai::{ai, claim_plan, guard_order, outweighs, plan, post, Decision, Give};
pub use camp::{zone_cap, CampState};
pub(crate) use camp::{zone_of, zone_ok};
pub use line::{cut_at, cut_by_rival, cut_off, line, line_from, past_reach, path_between, ring, route, turns};
pub use scout::{ground_worth, rich_ground, Hunger, ScoutState};
pub use search::{route_plan, search_view, Plan, SearchView};

use camp::{camp_march, camp_turn, hold};
use forage::room;
pub use muster::{
    disband, disband_kind, give_up, givers, nearest_clan, raise, raise_scout, raise_scout_at_founding, surrender,
};
pub(crate) use muster::{make_camp, take_people};
use scout::scout_turn;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnitKind {
    #[default]
    Camp,
    Scout,
    Guard,
}

impl UnitKind {
    pub fn name(self) -> &'static str {
        match self {
            UnitKind::Camp => "camp",
            UnitKind::Scout => "scout",
            UnitKind::Guard => "guard",
        }
    }
}

/// What a civ's army has done, kept on the civ so it outlives any one camp. The sweep reads it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Counters {
    /// Camps raised (founding, the AI's re-raising, the player's).
    pub raised: u32,
    /// Camps starved out to nothing.
    pub lost: u32,
    /// Turns the camp was hungry.
    pub hungry: u32,
    /// Turns the camp was cut off from its capital.
    pub cut_off: u32,
    /// Food foraged, and the tiles stripped doing it.
    pub foraged: f64,
    pub foraged_tiles: u32,
    /// Food pillaged from rivals, and the tiles stripped.
    pub pillaged: f64,
    pub pillaged_tiles: u32,
    /// Food brought along the line.
    pub resupplied: f64,
    /// Warriors lost to hunger.
    pub starved: u32,
    /// Tile-turns held beyond the camp's own tile.
    #[serde(default)]
    pub held: u32,
    /// Turns the line was cut by a rival's held ground (and would carry the camp's food without it).
    #[serde(default)]
    pub cut_by_army: u32,
    /// People lost in fights: its armies', guard's and (build-2) militia's.
    #[serde(default)]
    pub fallen: f64,
    /// Tiles it won by fighting.
    #[serde(default)]
    pub taken: u32,
    /// Its armies that surrendered (build-2).
    #[serde(default)]
    pub surrendered: u32,
}

/// A unit: what every unit has, and its kind's own state. Its JSON is one flat object (see
/// `UnitFlat`), so the page and the digests see the fields they always have.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "UnitFlat", into = "UnitFlat")]
pub struct Unit {
    pub id: u32,
    pub civ: u32,
    /// Its warriors (`people`), the tile it stands on (`home`), its pack (`store`) and its line to
    /// the capital (`routes`: one `Route`, or none when cut off or already there).
    pub actor: Actor,
    /// Labour per warrior put towards the next step and not yet enough to take it.
    pub banked: f64,
    /// Hungry turns in a row.
    pub hungry: u32,
    pub state: UnitState,
}

/// What a unit is, and the state only that kind has. A guard has none: its people and hunger are the
/// unit's (`claim.rs` divides them over the claim).
#[derive(Clone, Debug, PartialEq)]
pub enum UnitState {
    Camp(CampState),
    Scout(ScoutState),
    Guard,
}

/// A unit's JSON: the flat object every unit has always written, the fields in their old order.
/// A camp writes the scout's fields at their defaults, and a scout writes the switches and one-shot
/// orders it shares with the camp, and the camp's other fields as `scout_camp_fields` has them.
#[derive(Clone, Serialize, Deserialize)]
struct UnitFlat {
    id: u32,
    civ: u32,
    kind: UnitKind,
    #[serde(flatten)]
    actor: Actor,
    dest: Option<usize>,
    banked: f64,
    hungry: u32,
    auto_forage: bool,
    auto_supply: bool,
    resupply: bool,
    forage: bool,
    pillage: bool,
    cut: bool,
    foraging: bool,
    supplying: bool,
    #[serde(default)]
    held: Vec<usize>,
    #[serde(default)]
    guard: Vec<f64>,
    #[serde(default)]
    full: bool,
    #[serde(default)]
    cut_by: Option<u32>,
    #[serde(default)]
    route: Option<(usize, usize)>,
    #[serde(default)]
    heard: bool,
    #[serde(default)]
    blocked: bool,
    #[serde(default)]
    built: bool,
    #[serde(default)]
    stack: [f64; 2],
    #[serde(default)]
    still: u32,
    #[serde(default)]
    bows: bool,
    #[serde(default)]
    wander: Option<usize>,
}

impl From<Unit> for UnitFlat {
    fn from(u: Unit) -> Self {
        let kind = u.kind();
        let dest = match &u.state {
            UnitState::Camp(c) => c.dest,
            UnitState::Scout(s) => s.dest,
            UnitState::Guard => None,
        };
        let (camp, scout) = match u.state {
            UnitState::Camp(c) => (c, ScoutState::new()),
            UnitState::Scout(s) => (
                CampState {
                    auto_forage: s.auto_forage,
                    auto_supply: s.auto_supply,
                    resupply: s.resupply,
                    forage: s.forage,
                    pillage: s.pillage,
                    ..scout_camp_fields()
                },
                s,
            ),
            UnitState::Guard => (scout_camp_fields(), ScoutState::new()),
        };
        UnitFlat {
            id: u.id,
            civ: u.civ,
            kind,
            actor: u.actor,
            dest,
            banked: u.banked,
            hungry: u.hungry,
            auto_forage: camp.auto_forage,
            auto_supply: camp.auto_supply,
            resupply: camp.resupply,
            forage: camp.forage,
            pillage: camp.pillage,
            cut: camp.cut,
            foraging: camp.foraging,
            supplying: camp.supplying,
            held: camp.held,
            guard: camp.guard,
            full: camp.full,
            cut_by: camp.cut_by,
            route: scout.route,
            heard: scout.heard,
            blocked: scout.blocked,
            built: scout.built,
            stack: camp.stack,
            still: camp.still,
            bows: camp.bows,
            wander: scout.wander,
        }
    }
}

impl From<UnitFlat> for Unit {
    /// Reads the state its `kind` names; the other kind's fields are dropped.
    fn from(f: UnitFlat) -> Self {
        let state = match f.kind {
            UnitKind::Camp => UnitState::Camp(CampState {
                dest: f.dest,
                auto_forage: f.auto_forage,
                auto_supply: f.auto_supply,
                resupply: f.resupply,
                forage: f.forage,
                pillage: f.pillage,
                cut: f.cut,
                foraging: f.foraging,
                supplying: f.supplying,
                held: f.held,
                guard: f.guard,
                full: f.full,
                cut_by: f.cut_by,
                stack: f.stack,
                still: f.still,
                bows: f.bows,
            }),
            UnitKind::Scout => UnitState::Scout(ScoutState {
                dest: f.dest,
                route: f.route,
                heard: f.heard,
                blocked: f.blocked,
                built: f.built,
                auto_forage: f.auto_forage,
                auto_supply: f.auto_supply,
                resupply: f.resupply,
                forage: f.forage,
                pillage: f.pillage,
                wander: f.wander,
                hunger: None,
            }),
            UnitKind::Guard => UnitState::Guard,
        };
        Unit { id: f.id, civ: f.civ, actor: f.actor, banked: f.banked, hungry: f.hungry, state }
    }
}

/// The camp fields a scout carries beyond the switches and one-shot orders it has of its own (not
/// `Default`, which would write `auto_forage` false).
fn scout_camp_fields() -> CampState {
    CampState {
        dest: None,
        auto_forage: true,
        auto_supply: true,
        resupply: false,
        forage: false,
        pillage: false,
        cut: false,
        foraging: false,
        supplying: false,
        held: Vec::new(),
        guard: Vec::new(),
        full: false,
        cut_by: None,
        stack: [0.0; 2],
        still: 0,
        bows: false,
    }
}

impl Deref for Unit {
    type Target = Actor;
    fn deref(&self) -> &Actor {
        &self.actor
    }
}

impl DerefMut for Unit {
    fn deref_mut(&mut self) -> &mut Actor {
        &mut self.actor
    }
}

impl Unit {
    /// The tile the unit stands on.
    pub fn tile(&self) -> usize {
        self.actor.home
    }

    pub fn kind(&self) -> UnitKind {
        match self.state {
            UnitState::Camp(_) => UnitKind::Camp,
            UnitState::Scout(_) => UnitKind::Scout,
            UnitState::Guard => UnitKind::Guard,
        }
    }

    /// The tile a camp or a scout is marching to: its `dest`.
    pub fn dest(&self) -> Option<usize> {
        match &self.state {
            UnitState::Camp(c) => c.dest,
            UnitState::Scout(s) => s.dest,
            UnitState::Guard => None,
        }
    }

    pub fn camp(&self) -> Option<&CampState> {
        match &self.state {
            UnitState::Camp(c) => Some(c),
            UnitState::Scout(_) | UnitState::Guard => None,
        }
    }

    pub fn camp_mut(&mut self) -> Option<&mut CampState> {
        match &mut self.state {
            UnitState::Camp(c) => Some(c),
            UnitState::Scout(_) | UnitState::Guard => None,
        }
    }

    pub fn scout(&self) -> Option<&ScoutState> {
        match &self.state {
            UnitState::Scout(s) => Some(s),
            UnitState::Camp(_) | UnitState::Guard => None,
        }
    }

    pub fn scout_mut(&mut self) -> Option<&mut ScoutState> {
        match &mut self.state {
            UnitState::Scout(s) => Some(s),
            UnitState::Camp(_) | UnitState::Guard => None,
        }
    }

    /// The tiles a camp's warriors hold (none for a scout).
    pub fn held(&self) -> &[usize] {
        self.camp().map_or(&[][..], |c| c.held.as_slice())
    }
}

pub(crate) const EPS: f64 = 1e-9;

/// `civ`'s first army by id, if it has one.
pub fn camp(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Camp)
}

/// `civ`'s scout, if it has one.
pub fn scout(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Scout)
}

/// `civ`'s guard, if it has one.
pub fn guard_of(w: &World, civ: u32) -> Option<&Unit> {
    w.units.values().find(|u| u.civ == civ && u.kind() == UnitKind::Guard)
}

/// `civ`'s guard, if it has one, for changing.
pub(crate) fn guard_of_mut(w: &mut World, civ: u32) -> Option<&mut Unit> {
    w.units.values_mut().find(|u| u.civ == civ && u.kind() == UnitKind::Guard)
}

/// The food `people` warriors' packs hold.
pub fn pack_cap(people: f64) -> f64 {
    people * UNIT.pack * UNIT.food
}

/// The name of `civ`, for a note.
pub(crate) fn name_of(w: &World, civ: u32) -> String {
    w.civs.get(&civ).map_or_else(|| "a rival".to_string(), |c| c.name.clone())
}

/// The civ whose land the camp stands on, if it isn't its own.
pub(crate) fn rival_of(w: &World, u: &Unit) -> bool {
    claim::tile_civ(w, u.home).is_some_and(|c| c != u.civ)
}

/// The `units` step, after `civs`: fallow tiles count down, each camp in id order spoils and
/// marches (stopping where a rival has people, as they stood before the marches), every camp's
/// warriors spread and hold (`hold`), the fight resolves every contested tile (`fight::step`), then
/// each unit, in id order, takes the rest of its turn.
pub fn step(w: &mut World) {
    for t in &mut w.tiles {
        t.fallow = t.fallow.saturating_sub(1);
    }
    let ids: Vec<u32> = w.units.keys().copied().collect();
    let camps: Vec<u32> = ids.iter().copied().filter(|id| w.units[id].kind() == UnitKind::Camp).collect();
    let holders = fight::army_holders(w);
    let rivals: BTreeMap<usize, Vec<u32>> = fight::presence(w)
        .into_iter()
        .map(|(t, parts)| (t, parts.iter().filter(|p| p.people > FIGHT.floor).map(|p| p.civ).collect()))
        .collect();
    let left: BTreeMap<u32, f64> = camps.iter().map(|&id| (id, camp_march(w, id, &rivals))).collect();
    let spent = hold(w);
    fight::step(w, &holders);
    for id in ids {
        match w.units.get(&id).map(|u| u.kind()) {
            Some(UnitKind::Camp) => {
                let left = left.get(&id).copied().unwrap_or(0.0) - spent.get(&id).copied().unwrap_or(0.0);
                camp_turn(w, id, left.max(0.0));
            }
            Some(UnitKind::Scout) => scout_turn(w, id),
            Some(UnitKind::Guard) => guard::turn(w, id),
            None => {}
        }
    }
}

/// The civ the player leads, or why there is none.
pub fn leader(w: &World) -> Result<u32, String> {
    if !w.phase.playing() {
        return Err("the game has not begun".to_string());
    }
    let civ = w.player.ok_or_else(|| "you're watching: you lead no civ".to_string())?;
    if w.civs.get(&civ).is_some_and(|c| c.is_out()) {
        return Err("your civ has fallen".to_string());
    }
    Ok(civ)
}
