//! The world, its turn, and the actions that change it.
//!
//! A turn runs the steps in `TURN` in order. A mechanic adds its state as a field on `World`,
//! its steps to `TURN`, and its actions to `Action`. Every change to the world goes through
//! `act`, which checks an action, applies it and logs it, so `replay(config, log)` rebuilds the
//! same world exactly: the log is the save file.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::clans::{self, Clan};
use crate::data::Config;
use crate::land;
use crate::map::{self, Tile};
use crate::markets::{self, Market};
use crate::pregame::{self, End};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct World {
    pub config: Config,
    pub turn: u32,
    /// The only source of randomness: `rng::rand(&mut world.rng)`.
    pub rng: u32,
    pub tiles: Vec<Tile>,
    pub clans: BTreeMap<u32, Clan>,
    pub markets: BTreeMap<u32, Market>,
    /// The next id for a clan or a market, from 1.
    pub next_id: u32,
    /// Set once, when the pre-game ends.
    pub end: Option<End>,
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
pub enum EventKind {
    ClanDivided,
    ClanStarving,
    ClanMoved,
    MarketFormed,
    MarketFaded,
    HearthTamed,
    CentreFormed,
    PregameEnded,
}

/// Something that happened in a turn. `tile` is where; `other` is the daughter of a division.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub turn: u32,
    pub kind: EventKind,
    pub tile: usize,
    pub clan: Option<u32>,
    pub market: Option<u32>,
    pub other: Option<u32>,
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
            EventKind::ClanMoved => format!("Clan {clan} moves on to {at}"),
            EventKind::MarketFormed => format!("A market forms at {at}"),
            EventKind::MarketFaded => format!("Market {market} fades"),
            EventKind::HearthTamed => format!("A hearth is tamed at {at}"),
            EventKind::CentreFormed => format!("Market {market} becomes a centre"),
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
        self.events.push(Event { turn: self.turn, kind, tile, clan, market, other });
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    EndTurn,
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
            next_id: 1,
            end: None,
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
            Action::EndTurn => Ok(()),
        }
    }

    /// Validates, applies and logs `action`. On `Err` the world is unchanged.
    pub fn act(&mut self, action: Action) -> Result<(), String> {
        self.validate(&action)?;
        match &action {
            Action::EndTurn => self.end_turn(),
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

    pub fn run(&mut self, turns: u32) {
        for _ in 0..turns {
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
