//! The game's switches (`Config`), its end, its reasons and the judge's numbers.

use serde::{Deserialize, Serialize};

/// Which civs may claim land (`claim.rs`): `All` (the default), `None` (the control: no civ claims
/// past its capital), or `Only(k)` (the k-th civ in id order, 0-based, claims as
/// under `All`; the others as under `None`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Claims {
    #[default]
    All,
    None,
    Only(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub seed: u32,
    pub radius: i32,
    /// Whether civs have armies: off is the control (no camp at founding, the AI raises none, no
    /// levy). Old saves load with it on.
    #[serde(default = "yes")]
    pub army: bool,
    /// Whether civs give gifts: off is the control (no civ gives, whatever its `gift`). Old saves
    /// load with it on.
    #[serde(default = "yes")]
    pub gifts: bool,
    /// Whether AI camps paint a zone over their line: off is the control (the sweep's `--no-zone`).
    /// Old saves load with it on.
    #[serde(default = "yes")]
    pub zone: bool,
    /// Which civs may claim land: see `Claims`. Old saves load with `All`.
    #[serde(default)]
    pub claims: Claims,
    /// Whether people of two civs on a tile fight (`units/fight.rs`): off is the control (no losses,
    /// no retreats, no surrender, no fall, no pressed capital and no siege; holding is
    /// the strength comparison alone). Old saves load with it on.
    #[serde(default = "yes")]
    pub fight: bool,
    /// The turn limit in turns after the pick, which replaces the size's (`END`); `None` is the size's.
    /// A stage, a test or a script sets a short one. Old saves load with `None`.
    #[serde(default)]
    pub limit: Option<u32>,
    /// Whether clans farm, once civs are founded (`fields.rs`): off is the control, in which no clan
    /// farms. Old saves load with it on.
    #[serde(default = "yes")]
    pub fields: bool,
    /// Whether a farmer's home may step onto a tile another clan holds, where living feeds it better.
    /// Off is the control: a farmer's home never steps onto a tile another clan holds, so no two clans
    /// share a home. Old saves load with it on.
    #[serde(default = "yes")]
    pub towns: bool,
    /// Whether civs post signposts (`settle.rs`): off is the control, in which the planner posts none
    /// and the order is refused, so no settlement is founded. Old saves load with it on.
    #[serde(default = "yes")]
    pub signposts: bool,
    /// Whether the map has sources of salt and obsidian (`SOURCE`): off is the control, in which no
    /// tile has one, so neither good exists. Old saves load with it on.
    #[serde(default = "yes")]
    pub goods: bool,
    /// Whether units trade and goods pass between markets (`exchange.rs`), and whether stores record where
    /// their goods came from (`lots.rs`): off is the control, in which no unit trades, no good passes, no lot
    /// is written and the world is the one before exchange. Old saves load with it on. Last, and written
    /// only when off (`stats::digest` leaves it out, so the control's digest is the world before exchange's).
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    pub exchange: bool,
}

fn is_true(b: &bool) -> bool {
    *b
}

fn yes() -> bool {
    true
}

impl Default for Config {
    /// Radius 16: three centres 10 steps apart need the room.
    fn default() -> Self {
        Config {
            seed: 1,
            radius: 16,
            army: true,
            gifts: true,
            zone: true,
            claims: Claims::All,
            fight: true,
            limit: None,
            fields: true,
            towns: true,
            signposts: true,
            goods: true,
            exchange: true,
        }
    }
}

/// How a game ends (`over.rs`): its turn limit, in turns after the pick, by map radius as `SIZES`. A
/// radius between sizes takes the next size up's; one past the largest takes the largest's.
pub struct EndRules {
    pub limit: [(i32, u32); 3],
    pub note: &'static str,
}

pub const END: EndRules = EndRules {
    limit: [(16, 200), (23, 250), (33, 300)],
    note: "Measured on the seven retro maps (relay/specs/152/limit.md). In time to matter: all on the AI to \
           +600, the judge's leader at the limit is the leader at +600 on six of seven maps at these \
           limits (the AI's lead settles by +73 on six maps; small 9's top two swap to +348, so only a \
           limit of 150 or 300 and up matches there, and it is a coin flip). Room: the conquest story's \
           best armies reach a rival capital by +21 on small and a capital falls by +76; on medium and \
           large no army reaches one, and the farthest is out by +87 and +37; each limit is 2.6 times \
           the latest of these or more.",
};

/// How many reasons (`World::note`) a subject keeps, and how many the selection panel shows.
pub struct NotesRules {
    /// The most reasons a subject keeps; the oldest go.
    pub kept: usize,
    /// The most reasons the panel shows, newest first.
    pub shown: usize,
}

pub const NOTES: NotesRules = NotesRules { kept: 8, shown: 5 };

/// The judge's numbers (`judge.rs`): how far a civ must lead to take the lead, and the turns between
/// the samples the swings are counted on.
pub struct JudgeRules {
    /// Standing a civ must be ahead by to take the lead from the civ that has it, and a difference
    /// between samples of the spread must exceed to count as up or down. Two civs within half a point
    /// of each other swap places on noise: the medium maps count 7 and 8 such changes at 0 (probe.md).
    pub lead_margin: f64,
    /// Turns between the samples the swings are counted on.
    pub sample: u32,
    /// A clan's festival change back to one it had within this many turns is a flip. A fixed window,
    /// so a build with no `MARKET.keep_turns` counts as one with it.
    pub flip_window: u32,
    /// A civ's Standing gain from claiming alone (`sweep --judge --solo`) counts as a gain above this.
    /// It is the lead margin's size: a gain of less is noise in the Standing the judge reads.
    pub solo_above: f64,
}

pub const JUDGE: JudgeRules = JudgeRules { lead_margin: 0.005, sample: 10, flip_window: 10, solo_above: 0.005 };
