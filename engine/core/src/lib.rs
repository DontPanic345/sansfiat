//! Sans Fiat engine: a world that runs turn by turn, changed only by logged actions.
//! The market library finds barter exchange ratios with no money.

pub mod data;
pub mod hex;
pub mod map;
pub mod market;
pub mod rng;
pub mod world;

pub use data::{Config, GOODS, N, TERRAIN};
pub use map::Tile;
pub use market::{clear, utility, Basket, Clearing, Params};
pub use world::{Action, Save, World, TURN};
