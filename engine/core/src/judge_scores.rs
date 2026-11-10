//! The score structs of the judge: what `Watch::scores` returns, grouped as `docs/process.md` groups
//! them. `judge` re-exports them; the groups' counts are built in their own modules.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::judge::CivLand;
use crate::judge_fields::FieldScores;
use crate::judge_ring::SupplyScores;
use crate::judge_towns::TownScores;
use crate::stats;
use crate::world::World;

/// One game's scores, grouped as `docs/process.md` groups them.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Scores {
    pub standing: StandingScores,
    pub doubt: DoubtScores,
    pub decisions: DecisionScores,
    pub motion: MotionScores,
    pub land: LandScores,
    /// What fighting cost and moved, read from the civs' counters at the end of the watch.
    pub fight: FightScores,
    pub economy: EconomyScores,
    /// The ring and the units inside and outside it, and the clans' territory.
    pub supply: SupplyScores,
    /// Fields and farmers, the store's packs, and how settled farmers and gatherers move and starve.
    pub fields: FieldScores,
    /// Towns that form, and what town clans, farmers outside towns and all clans walk, trade and starve at.
    pub towns: TownScores,
    pub end: EndScores,
}

/// How the game ended, if it has: the end's `why` (`conquest`, `limit` or `fell`), its winner and the
/// turns since the pick it came at.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EndOver {
    pub why: String,
    pub winner: u32,
    pub turn: u32,
}

/// The end group: how a game ended and where each civ stood in Standing at the end.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EndScores {
    /// The end, `None` while the game goes on.
    pub over: Option<EndOver>,
    /// Over before the limit: by conquest or by the led civ's fall.
    pub early: bool,
    /// Capitals taken after the pick.
    pub capitals_fallen: u32,
    /// Each civ's rank in Standing (1 the largest) at the end, or at the watch's last turn.
    pub ranks: BTreeMap<u32, usize>,
    /// The led civ's rank, `None` when no civ is led (watching).
    pub led_rank: Option<usize>,
}

/// The economy group: the civs' stores, the levy that fills them, the camps and the guards, read
/// after each turn over the civs that are alive. The means are over civ-turns.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EconomyScores {
    /// The food in a civ's store, mean over civ-turns.
    pub store_mean: f64,
    /// The most food any civ's store held after a turn.
    pub store_max: f64,
    /// The civ-turns on which a store held at least a pack's worth, `units::pack_cap(UNIT.first)`.
    pub turns_at_pack: u32,
    /// The food the market levy paid a civ in its turn, mean over civ-turns (0 on a turn with none).
    pub levy_per_turn: f64,
    /// The most people in one camp after any turn.
    pub largest_army: f64,
    /// The people of the civ's guard, mean over civ-turns.
    pub guard_mean: f64,
}

/// The fight group: per civ and in total, the people fallen in fights (armies, guards and militia),
/// the tiles taken by fighting and the armies surrendered, from `Civ.army` at the end of the watch.
/// The counters run from the start of the game, and fighting begins at the pick, so they are the
/// watch's.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct FightScores {
    pub civs: BTreeMap<u32, FightCivScores>,
    pub fallen: f64,
    pub taken: u32,
    pub surrendered: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct FightCivScores {
    pub fallen: f64,
    pub taken: u32,
    pub surrendered: u32,
}

impl FightScores {
    /// The fight counts of every civ in `w`.
    pub fn of(w: &World) -> FightScores {
        let civs: BTreeMap<u32, FightCivScores> = w
            .civs
            .values()
            .map(|c| {
                let a = &c.army;
                (c.id, FightCivScores { fallen: a.fallen, taken: a.taken, surrendered: a.surrendered })
            })
            .collect();
        FightScores {
            fallen: civs.values().map(|c| c.fallen).sum(),
            taken: civs.values().map(|c| c.taken).sum(),
            surrendered: civs.values().map(|c| c.surrendered).sum(),
            civs,
        }
    }
}

/// The land group: trail shares and festivals at the pick and +200, and how steady the festivals
/// are after the pick. A change is a clan's festival differing from last turn's; a flip is a change
/// back to one (or none) it had within `JUDGE.flip_window` turns.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LandScores {
    pub at_pick: stats::Trails,
    pub at_200: Option<stats::Trails>,
    pub clan_turns: u32,
    pub festival_changes: u32,
    pub flips: u32,
    pub festival_changes_per_100: f64,
    pub flips_per_100: f64,
    /// Each civ's land at +200 (empty before +200).
    pub civs_200: Vec<CivLand>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StandingScores {
    /// Each civ's Standing at +100 and +200 turns (`None` past the game's end).
    pub civs: Vec<CivStanding>,
    /// The most Standing any civ has at +200.
    pub top_200: Option<f64>,
    /// The civs' Standing summed at +200: the share of the world in any civ.
    pub aligned_200: Option<f64>,
    /// The share of the world in unaligned clans on some civ's held land at +200.
    pub on_land_200: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CivStanding {
    pub id: u32,
    pub at_100: Option<f64>,
    pub at_200: Option<f64>,
    /// At +200, the part of `at_200` from the civ's clans and units, and from unaligned clans on its
    /// held land.
    pub clans_200: Option<f64>,
    pub land_200: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DoubtScores {
    pub lead_changes: u32,
    pub spread_100: Option<f64>,
    pub spread_200: Option<f64>,
    pub spread_min: f64,
    pub spread_max: f64,
    pub swings: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DecisionScores {
    /// Accepted orders and actions of the counted civs, per civ per hundred turns.
    pub orders_per_100: f64,
    /// As `orders_per_100`, by kind.
    pub by_kind: BTreeMap<String, f64>,
    /// The mean per civ of the kinds given at least twice.
    pub levers_repeated: f64,
}

impl DecisionScores {
    /// The decisions of `civs` over `turns` turns, from `World::orders`.
    pub fn of(w: &World, civs: &[u32], turns: u32) -> DecisionScores {
        let per = civs.len() as f64 * f64::from(turns) / 100.0;
        let mut by_kind: BTreeMap<String, f64> = BTreeMap::new();
        let mut repeated = 0usize;
        for civ in civs {
            for (kind, &n) in w.orders.get(civ).into_iter().flatten() {
                *by_kind.entry(kind.clone()).or_insert(0.0) += f64::from(n);
                if n >= 2 {
                    repeated += 1;
                }
            }
        }
        let total: f64 = by_kind.values().sum();
        let rate = |n: f64| if per > 0.0 { n / per } else { 0.0 };
        DecisionScores {
            orders_per_100: rate(total),
            by_kind: by_kind.into_iter().map(|(k, n)| (k, rate(n))).collect(),
            levers_repeated: if civs.is_empty() { 0.0 } else { repeated as f64 / civs.len() as f64 },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MotionScores {
    pub clans_changed: u32,
    pub people_changed: f64,
    pub land_changed: u32,
    /// Civ clans that left a civ for another (`ClanLeft`, daughters included).
    pub clans_left: u32,
    /// Unaligned clans that joined a civ (`ClanJoined`).
    pub clans_joined: u32,
    /// Tiles of claims a rival's unit took (`ClaimTaken`).
    pub claim_taken: u32,
}
