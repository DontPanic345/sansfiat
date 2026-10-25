//! Sans Fiat engine: a world that runs turn by turn, changed only by logged actions.
//! Clans live off food on the land and trade at markets that form where they live; the market
//! library finds barter exchange ratios with no money. The pre-game ends with centres far apart.

pub mod civs;
pub mod clans;
pub mod data;
pub mod hex;
pub mod land;
pub mod map;
pub mod market;
pub mod markets;
pub mod pregame;
pub mod rng;
pub mod round;
pub mod stats;
pub mod world;

pub use civs::Civ;
pub use clans::{Clan, Plan};
pub use data::{Config, CLAN, FOODS, GOODS, N, TERRAIN};
pub use map::Tile;
pub use market::{clear, utility, Basket, Clearing, Params};
pub use markets::Market;
pub use pregame::End;
pub use world::{Action, Event, EventKind, Phase, Save, World, TURN};
