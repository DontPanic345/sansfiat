//! The world, its turn, and the actions that change it.
//!
//! A turn runs the steps in `TURN` in order. A mechanic adds its state as a field on `World`,
//! its steps to `TURN`, and its actions to `Action`. Every change to the world goes through
//! `act`, which checks an action, applies it and logs it, so `replay(config, log)` rebuilds the
//! same world exactly: the log is the save file.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::civs::{self, Civ};
use crate::clans::{self, Clan};
use crate::data::Config;
use crate::land;
use crate::map::{self, Tile};
use crate::markets::{self, Market};
use crate::pregame::{self, End};
use crate::units::{self, Unit};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct World {
    pub config: Config,
    pub turn: u32,
    /// The only source of randomness: `rng::rand(&mut world.rng)`.
    pub rng: u32,
    pub tiles: Vec<Tile>,
    pub clans: BTreeMap<u32, Clan>,
    pub markets: BTreeMap<u32, Market>,
    /// People moved between clans at festivals, over the whole run.
    pub moved: f64,
    /// The next id for a clan, a market or a civ, from 1.
    pub next_id: u32,
    /// Set once, when the pre-game ends.
    pub end: Option<End>,
    /// Pregame, then Choose once founding has made the civs, then Main once the player has picked.
    pub phase: Phase,
    /// The civ the player leads; `None` before the pick and when watching.
    pub player: Option<u32>,
    pub civs: BTreeMap<u32, Civ>,
    pub units: BTreeMap<u32, Unit>,
    /// Every action applied, in order.
    pub log: Vec<Action>,
    /// What happened, in order: derived from the turns, so not saved; a replay rebuilds it.
    #[serde(skip)]
    pub events: Vec<Event>,
    /// Every market that has been among the centres before the end, so each is announced once.
    #[serde(skip)]
    pub centres_seen: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    Pregame,
    Choose,
    Main,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventKind {
    ClanDivided,
    ClanStarving,
    ClanMoved,
    ClanMerged,
    MarketFormed,
    MarketFaded,
    HearthTamed,
    CentreFormed,
    PregameEnded,
    CivFounded,
    UnitMade,
    UnitHungry,
    UnitDisbanded,
    UnitStarved,
}

/// Something that happened in a turn. `tile` is where; `other` is the daughter of a division;
/// `unit` is for units.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub turn: u32,
    pub kind: EventKind,
    pub tile: usize,
    pub clan: Option<u32>,
    pub market: Option<u32>,
    pub other: Option<u32>,
    pub civ: Option<u32>,
    pub unit: Option<u32>,
}

impl Event {
    /// The one wording of an event, for the CLI and the page.
    pub fn text(&self, w: &World) -> String {
        let h = w.tiles[self.tile].hex();
        let at = format!("({}, {})", h.q, h.r);
        let clan = self.clan.unwrap_or(0);
        let market = self.market.unwrap_or(0);
        match self.kind {
            EventKind::ClanDivided => {
                format!("Clan {clan} divides; clan {} settles at {at}", self.other.unwrap_or(0))
            }
            EventKind::ClanStarving => format!("Clan {clan} is starving"),
            EventKind::ClanMerged => format!("Clan {clan} merges into clan {}", self.other.unwrap_or(0)),
            EventKind::ClanMoved => format!("Clan {clan} moves on to {at}"),
            EventKind::MarketFormed => format!("A market forms at {at}"),
            EventKind::MarketFaded => format!("Market {market} fades"),
            EventKind::HearthTamed => format!("A hearth is tamed at {at}"),
            EventKind::CentreFormed => format!("Market {market} becomes a centre"),
            EventKind::CivFounded => {
                let name = self.civ.and_then(|id| w.civs.get(&id)).map_or("A civ", |c| c.name.as_str());
                format!("{name} is founded at {at}, on market {market}")
            }
            EventKind::UnitMade => {
                let (civ, unit) = (self.civ_name(w), self.unit.unwrap_or(0));
                match self.clan {
                    Some(c) => format!("{civ} makes warrior {unit} from clan {c}"),
                    None => format!("{civ} makes warrior {unit}"),
                }
            }
            EventKind::UnitHungry => format!("{}'s warrior {} goes hungry", self.civ_name(w), self.unit.unwrap_or(0)),
            EventKind::UnitDisbanded | EventKind::UnitStarved => {
                let why = if self.kind == EventKind::UnitStarved { " from hunger" } else { "" };
                let person = match self.clan {
                    Some(c) => format!("its person rejoins clan {c}"),
                    None => "its person is lost".to_string(),
                };
                format!("{}'s warrior {} disbands{why}; {person}", self.civ_name(w), self.unit.unwrap_or(0))
            }
            EventKind::PregameEnded => match &w.end {
                Some(e) => {
                    let n = e.centres.len();
                    let centres = format!("{n} centre{}", if n == 1 { "" } else { "s" });
                    if e.topped_up.is_empty() {
                        format!("The pre-game ends: {centres}")
                    } else {
                        format!("The pre-game ends: {centres}, {} topped up", e.topped_up.len())
                    }
                }
                None => "The pre-game ends".to_string(),
            },
        }
    }
}

impl Event {
    /// The name of the event's civ; civs never go away.
    fn civ_name<'a>(&self, w: &'a World) -> &'a str {
        self.civ.and_then(|id| w.civs.get(&id)).map_or("A civ", |c| c.name.as_str())
    }

    /// The event as JSON, with its wording in `text`.
    pub fn json(&self, w: &World) -> serde_json::Value {
        let mut v = serde_json::to_value(self).unwrap();
        v["text"] = self.text(w).into();
        v
    }
}

impl World {
    /// Records an event at the current turn.
    pub fn event(&mut self, kind: EventKind, tile: usize, clan: Option<u32>, market: Option<u32>, other: Option<u32>) {
        self.events.push(Event { turn: self.turn, kind, tile, clan, market, other, civ: None, unit: None });
    }

    /// Records an event about a civ (and its market, if any) at the current turn.
    pub fn civ_event(&mut self, kind: EventKind, tile: usize, civ: u32, market: Option<u32>) {
        self.events.push(Event {
            turn: self.turn,
            kind,
            tile,
            clan: None,
            market,
            other: None,
            civ: Some(civ),
            unit: None,
        });
    }
}

impl World {
    /// Records an event about a unit of a civ (and the clan that gave or took its person).
    pub fn unit_event(&mut self, kind: EventKind, tile: usize, civ: u32, unit: u32, clan: Option<u32>) {
        self.events.push(Event {
            turn: self.turn,
            kind,
            tile,
            clan,
            market: None,
            other: None,
            civ: Some(civ),
            unit: Some(unit),
        });
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    EndTurn,
    /// Lead a civ, or `None` to watch all three on the AI. Only at the pick.
    ChooseCiv {
        civ: Option<u32>,
    },
    /// Make a warrior at the capital of the civ the player leads.
    MakeUnit,
    /// Send a unit to a tile; it moves when the turn ends.
    MoveUnit {
        unit: u32,
        to: usize,
    },
    /// Disband a unit at once; its person rejoins the nearest clan of its civ.
    DisbandUnit {
        unit: u32,
    },
}

pub struct Step {
    pub name: &'static str,
    pub run: fn(&mut World),
}

/// One turn, in order.
pub const TURN: &[Step] = &[
    Step { name: "regrow", run: land::regrow },
    Step { name: "move", run: clans::adjust },
    Step { name: "go", run: markets::go },
    Step { name: "harvest", run: clans::harvest },
    Step { name: "trade", run: markets::trade },
    Step { name: "eat", run: clans::eat },
    Step { name: "people", run: markets::people },
    Step { name: "wear", run: land::wear },
    Step { name: "civs", run: civs::upkeep },
    Step { name: "units", run: units::march },
    Step { name: "hold", run: land::hold },
    Step { name: "markets", run: markets::form_and_fade },
    Step { name: "end", run: pregame::check_end },
];

/// What a save holds: enough to replay the world.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Save {
    pub config: Config,
    pub log: Vec<Action>,
}

impl World {
    pub fn new(config: Config) -> World {
        let mut rng = config.seed;
        let tiles = map::generate(config.radius, &mut rng);
        let mut w = World {
            config,
            turn: 0,
            rng,
            tiles,
            clans: BTreeMap::new(),
            markets: BTreeMap::new(),
            moved: 0.0,
            next_id: 1,
            end: None,
            phase: Phase::Pregame,
            player: None,
            civs: BTreeMap::new(),
            units: BTreeMap::new(),
            log: Vec::new(),
            events: Vec::new(),
            centres_seen: Vec::new(),
        };
        clans::seed(&mut w);
        w
    }

    /// Checks `action` against the world; `Err` says why it can't happen.
    pub fn validate(&self, action: &Action) -> Result<(), String> {
        match action {
            Action::EndTurn if self.phase == Phase::Choose => Err("choose a civ to lead, or watch".to_string()),
            Action::EndTurn => Ok(()),
            Action::ChooseCiv { .. } if self.phase != Phase::Choose => Err("there is no civ to choose now".to_string()),
            Action::ChooseCiv { civ: Some(id) } if !self.civs.contains_key(id) => Err(format!("{id} is not a civ")),
            Action::ChooseCiv { .. } => Ok(()),
            Action::MakeUnit => units::check_make(self),
            Action::MoveUnit { unit, to } => units::check_move(self, *unit, *to),
            Action::DisbandUnit { unit } => units::check_disband(self, *unit),
        }
    }

    /// Validates, applies and logs `action`. On `Err` the world is unchanged.
    pub fn act(&mut self, action: Action) -> Result<(), String> {
        self.validate(&action)?;
        match &action {
            Action::EndTurn => self.end_turn(),
            Action::ChooseCiv { civ } => {
                self.player = *civ;
                self.phase = Phase::Main;
            }
            Action::MakeUnit => {
                let civ = self.player.expect("validated: the player leads a civ");
                units::make(self, civ).expect("validated: a clan can give a person");
            }
            Action::MoveUnit { unit, to } => units::order_move(self, *unit, *to),
            Action::DisbandUnit { unit } => units::disband(self, *unit, false),
        }
        self.log.push(action);
        Ok(())
    }

    fn end_turn(&mut self) {
        for step in TURN {
            (step.run)(self);
        }
        self.turn += 1;
    }

    /// Ends `turns` turns. At the pick it first watches (`ChooseCiv { civ: None }`, logged), so a
    /// run goes on through the main game with every civ on the AI.
    pub fn run(&mut self, turns: u32) {
        for _ in 0..turns {
            if self.phase == Phase::Choose {
                self.act(Action::ChooseCiv { civ: None }).expect("watching is always allowed at the pick");
            }
            self.act(Action::EndTurn).expect("ending a turn is always allowed");
        }
    }

    pub fn save(&self) -> Save {
        Save { config: self.config, log: self.log.clone() }
    }

    /// Rebuilds the world a save describes; `Err` names the first action that no longer applies.
    pub fn replay(save: &Save) -> Result<World, String> {
        let mut w = World::new(save.config);
        for (i, a) in save.log.iter().enumerate() {
            w.act(a.clone()).map_err(|e| format!("action {i} ({a:?}): {e}"))?;
        }
        Ok(w)
    }
}
