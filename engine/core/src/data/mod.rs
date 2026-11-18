//! The numbers every rule reads, one file a system (`engine/CLAUDE.md`). Each item's doc says its
//! unit, and a rule's reason is one `note` beside it. Every file's items are re-exported here, so
//! `crate::data::X` names the same item whatever file it is in.

mod civs;
mod clans;
mod game;
mod goods;
mod land;
mod map;
mod units;

pub use self::civs::*;
pub use self::clans::*;
pub use self::game::*;
pub use self::goods::*;
pub use self::land::*;
pub use self::map::*;
pub use self::units::*;

// A day is one person's labour for one day; a turn is a year of `CLAN.work` days, the days a person
// gives (`labour.md`). Every number's doc says its unit.

/// The days a bare plains step takes, so a trail on flat land (`TRAIL.flat` of it) takes one day.
/// Every cost and labour is written as its old number times `DAY`, every food a day over `DAY`.
pub const DAY: f64 = 4.0 / 3.0;
