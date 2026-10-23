//! Sans Fiat engine: a world that runs turn by turn, changed only by logged actions.
//! Clans live off food on the land; the market library finds barter exchange ratios with no money.

pub mod clans;
pub mod data;
pub mod hex;
pub mod land;
pub mod map;
pub mod market;
pub mod rng;
pub mod round;
pub mod world;

pub use clans::{Clan, Plan};
pub use data::{Config, CLAN, FOODS, GOODS, N, TERRAIN};
pub use map::Tile;
pub use market::{clear, utility, Basket, Clearing, Params};
pub use world::{Action, Save, World, TURN};
