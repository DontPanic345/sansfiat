//! Sans Fiat engine: a world that runs turn by turn, changed only by logged actions.
//! Clans live off food on the land and trade at markets that form where they live; the market
//! library finds barter exchange ratios with no money. The pre-game ends with centres far apart.

pub mod actors;
pub mod civs;
pub mod clans;
pub mod data;
pub mod events;
pub mod hex;
pub mod land;
pub mod leaning;
pub mod map;
pub mod market;
pub mod markets;
pub mod orders;
pub mod pregame;
pub mod rng;
pub mod stats;
pub mod units;
pub mod world;

pub use actors::{Actor, Route};
pub use civs::Civ;
pub use clans::Clan;
pub use data::{Config, CLAN, FOODS, GOODS, N, TERRAIN};
pub use events::{Event, EventKind};
pub use map::Tile;
pub use market::{clear, utility, Basket, Clearing, Params};
pub use markets::Market;
pub use orders::Order;
pub use pregame::End;
pub use units::{Counters, Unit, UnitKind};
pub use world::{Action, Phase, Save, World, TURN};
