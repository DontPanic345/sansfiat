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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct World {
    pub config: Config,
    pub turn: u32,
    /// The only source of randomness: `rng::rand(&mut world.rng)`.
    pub rng: u32,
    pub tiles: Vec<Tile>,
    pub clans: BTreeMap<u32, Clan>,
    /// The next id for a clan (and, later, a market), from 1.
    pub next_id: u32,
    /// Every action applied, in order.
    pub log: Vec<Action>,
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
    Step { name: "harvest", run: clans::harvest },
    Step { name: "eat", run: clans::eat },
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
        let mut w = World { config, turn: 0, rng, tiles, clans: BTreeMap::new(), next_id: 1, log: Vec::new() };
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
