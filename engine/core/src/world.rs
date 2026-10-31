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
use crate::data::{Config, CIV, NOTES};
use crate::events::{Event, EventKind};
use crate::land;
use crate::leaning;
use crate::map::{self, Tile};
use crate::markets::{self, Market};
use crate::orders::Order;
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
    /// People in all clans at the end of each of the last `PREGAME.settle_turns` pre-game turns,
    /// oldest first: whether the pre-game has settled.
    #[serde(default)]
    pub people_seen: Vec<f64>,
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
    /// Every clan that has been among the centres before the end, so each is announced once.
    #[serde(skip)]
    pub centres_seen: Vec<u32>,
    /// Why each subject (a clan, market, civ or unit, by id) was acted on, oldest first: derived
    /// from the turns, so not saved; a replay rebuilds it. See `note`.
    #[serde(skip)]
    pub notes: BTreeMap<u32, Vec<Note>>,
    /// The alert candidates of the last turn, as (kind, subject): a trouble that goes on from one
    /// turn to the next is an alert once (`flag_alerts`).
    #[serde(skip)]
    pub candidates: Vec<(EventKind, u32)>,
    /// Accepted orders and actions per civ, by kind (`Order::kind`, `setKeep`, `setGift`), since the
    /// world began: the AI's and the player's alike. Derived, so not saved; a replay rebuilds it.
    #[serde(skip)]
    pub orders: BTreeMap<u32, BTreeMap<String, u32>>,
}

/// A reason: a line about a subject, written where a rule acted on it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub turn: u32,
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    Pregame,
    Choose,
    Main,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    EndTurn,
    /// Lead a civ, or `None` to watch all three on the AI. Only at the pick.
    ChooseCiv {
        civ: Option<u32>,
    },
    /// An order to the civ's units, which the player leads: checked against the world and applied
    /// by `World::order`, then logged.
    Order {
        civ: u32,
        order: Order,
    },
    /// Set the food the civ keeps in its store, or `None` for the default (its camp's pack).
    SetKeep {
        food: Option<f64>,
    },
    /// Set the most food the civ gives a turn to the unaligned clans at its market.
    SetGift {
        food: f64,
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
    Step { name: "lean", run: leaning::lean },
    Step { name: "wear", run: land::wear },
    Step { name: "civs", run: civs::upkeep },
    Step { name: "ai", run: units::ai },
    Step { name: "units", run: units::step },
    Step { name: "hold", run: land::tend },
    Step { name: "markets", run: markets::form_and_fade },
    Step { name: "end", run: pregame::check_end },
];

/// The save format's version. Any change that moves a world's digest bumps it with the constants
/// of `the_save_version_pins_the_rules`, so a save from before the change is refused, not replayed
/// into a different world.
pub const SAVE_VERSION: u32 = 2;

/// What a save holds: enough to replay the world. A save without `version` reads as 0.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Save {
    #[serde(default)]
    pub version: u32,
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
            people_seen: Vec::new(),
            next_id: 1,
            end: None,
            phase: Phase::Pregame,
            player: None,
            civs: BTreeMap::new(),
            units: BTreeMap::new(),
            log: Vec::new(),
            events: Vec::new(),
            centres_seen: Vec::new(),
            notes: BTreeMap::new(),
            candidates: Vec::new(),
            orders: BTreeMap::new(),
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
            Action::Order { civ, order } => {
                if units::leader(self)? != *civ {
                    return Err("that civ is not yours".to_string());
                }
                self.check_order(*civ, order)
            }
            Action::SetKeep { food: Some(f) } if !f.is_finite() || *f < 0.0 => {
                Err("keep a number of food, 0 or more".to_string())
            }
            Action::SetKeep { .. } => units::leader(self).map(|_| ()),
            Action::SetGift { food } if !food.is_finite() || *food < 0.0 || *food > CIV.store_cap => {
                Err(format!("give between 0 and {} food a turn", CIV.store_cap))
            }
            Action::SetGift { .. } => units::leader(self).map(|_| ()),
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
            Action::Order { civ, order } => {
                self.order(*civ, order.clone()).expect("validated: the order checks");
            }
            Action::SetKeep { food } => {
                let civ = self.player.expect("validated: the player leads a civ");
                self.count(civ, "setKeep");
                let text = match food {
                    Some(f) => format!("the player set it to keep {f:.1} food in its store"),
                    None => "the player set it to keep its camp's full pack".to_string(),
                };
                self.civs.get_mut(&civ).unwrap().keep = *food;
                self.note(civ, text);
            }
            Action::SetGift { food } => {
                let civ = self.player.expect("validated: the player leads a civ");
                self.count(civ, "setGift");
                self.civs.get_mut(&civ).unwrap().gift = *food;
                self.note(civ, format!("the player set its gift to {food:.1} food a turn"));
            }
        }
        self.log.push(action);
        Ok(())
    }

    /// Counts one accepted order or action of `kind` for `civ` (`orders`).
    pub fn count(&mut self, civ: u32, kind: &str) {
        *self.orders.entry(civ).or_default().entry(kind.to_string()).or_insert(0) += 1;
    }

    /// A tile as the notes name it: its hex, `(q, r)`.
    pub fn at(&self, tile: usize) -> String {
        let h = self.tiles[tile].hex();
        format!("({}, {})", h.q, h.r)
    }

    /// Records why a rule acted on subject `id` (a clan, market, civ or unit) this turn. A text the
    /// subject noted last turn or this turn only moves that note on to now (and so to the newest
    /// place); past `NOTES.kept` the oldest goes. Writes nothing a digest reads.
    pub fn note(&mut self, id: u32, text: String) {
        let turn = self.turn;
        let list = self.notes.entry(id).or_default();
        if let Some(i) = list.iter().rposition(|n| n.text == text && n.turn + 1 >= turn) {
            let mut n = list.remove(i);
            n.turn = turn;
            list.push(n);
            return;
        }
        list.push(Note { turn, text });
        if list.len() > NOTES.kept {
            list.remove(0);
        }
    }

    fn end_turn(&mut self) {
        let from = self.events.len();
        for step in TURN {
            (step.run)(self);
        }
        self.sweep_notes();
        self.flag_alerts(from);
        self.turn += 1;
    }

    /// Drops the notes of subjects that are gone (a clan merged or died, a market faded, a camp or
    /// scout disbanded).
    fn sweep_notes(&mut self) {
        let (clans, markets, civs, units) = (&self.clans, &self.markets, &self.civs, &self.units);
        self.notes.retain(|id, _| {
            clans.contains_key(id) || markets.contains_key(id) || civs.contains_key(id) || units.contains_key(id)
        });
    }

    /// The alert key of an event this turn: its kind and subject, when it is trouble the led civ
    /// must hear of (the camp's or scout's hunger and cut lines, a rival's clan joining, people
    /// leaving the led civ's clans). `None` when watching.
    fn alert_key(&self, e: &Event) -> Option<(EventKind, u32)> {
        let me = self.player?;
        let mine = match e.kind {
            EventKind::CampHungry
            | EventKind::WarriorsStarved
            | EventKind::CampCutOff
            | EventKind::LineCut
            | EventKind::ScoutHungry
            | EventKind::ScoutStarved
            | EventKind::PeopleLeft => e.civ == Some(me),
            EventKind::ClanJoined => e.civ.is_some_and(|c| c != me),
            _ => false,
        };
        if !mine {
            return None;
        }
        Some((e.kind, e.unit.or(e.clan).or(e.civ)?))
    }

    /// Marks the alerts among the events from index `from`: a candidate (`alert_key`) is an alert
    /// unless the same kind about the same subject was a candidate last turn, so a trouble that
    /// starts is one alert and not one a turn. Keeps this turn's candidates for the next.
    fn flag_alerts(&mut self, from: usize) {
        let before = std::mem::take(&mut self.candidates);
        let mut now = Vec::new();
        for i in from..self.events.len() {
            let Some(key) = self.alert_key(&self.events[i]) else { continue };
            self.events[i].alert = !before.contains(&key);
            now.push(key);
        }
        self.candidates = now;
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
        Save { version: SAVE_VERSION, config: self.config, log: self.log.clone() }
    }

    /// Rebuilds the world a save describes; `Err` names why it can't (another version, or the first
    /// action that no longer applies).
    pub fn replay(save: &Save) -> Result<World, String> {
        if save.version != SAVE_VERSION {
            return Err(if save.version == 0 {
                format!("this save is from an older build (no version); this build reads version {SAVE_VERSION}")
            } else {
                format!("this save is version {}; this build reads version {SAVE_VERSION}", save.version)
            });
        }
        let mut w = World::new(save.config);
        for (i, a) in save.log.iter().enumerate() {
            w.act(a.clone()).map_err(|e| format!("action {i} ({a:?}): {e}"))?;
        }
        Ok(w)
    }
}
