//! Barter-market engine: clans harvest goods from the land around them and trade at markets.
//! There is no money; prices are exchange ratios found fresh at every clearing.

pub mod data;
pub mod hex;
pub mod market;
pub mod rng;
pub mod world;

pub use data::{Config, GOODS, N, TERRAIN};
pub use market::{clear, utility, Basket, Clearing, Params};
pub use world::World;
