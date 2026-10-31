//! The judge: a game scored turn by turn. A `Watch` is made at the pick and turned after each turn;
//! it keeps every civ's Standing, the leader, the spread, and what changes hands. It reads the world
//! and changes nothing, and it draws nothing from `World.rng`.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::civs::tile_civ;
use crate::data::{JUDGE, TERRAIN};
use crate::stats;
use crate::world::World;

/// The civ with the most Standing, the lowest id on a tie.
fn leader_of(standing: &BTreeMap<u32, f64>) -> Option<u32> {
    let mut best: Option<(u32, f64)> = None;
    for (&id, &share) in standing {
        if best.is_none_or(|(_, b)| share > b) {
            best = Some((id, share));
        }
    }
    best.map(|(id, _)| id)
}

/// The spread: the most Standing minus the least; 0 with no civs.
fn spread(standing: &BTreeMap<u32, f64>) -> f64 {
    let max = standing.values().copied().fold(f64::NEG_INFINITY, f64::max);
    let min = standing.values().copied().fold(f64::INFINITY, f64::min);
    if standing.is_empty() {
        0.0
    } else {
        max - min
    }
}

/// Each clan's civ, by clan id.
fn clan_civs(w: &World) -> BTreeMap<u32, Option<u32>> {
    w.clans.iter().map(|(&id, k)| (id, k.civ)).collect()
}

/// The civ that holds each tile's land (`tile_civ`), by tile index; water is always `None`.
fn land_civs(w: &World) -> Vec<Option<u32>> {
    (0..w.tiles.len()).map(|i| if TERRAIN[w.tiles[i].terrain].land { tile_civ(w, i) } else { None }).collect()
}

/// A game watched turn by turn from its pick.
pub struct Watch {
    pick: u32,
    /// Every civ's Standing at the pick and after each turn: index `k` is `k` turns on.
    standing: Vec<BTreeMap<u32, f64>>,
    /// The spread at each turn, as `standing`.
    spreads: Vec<f64>,
    /// The leader at each turn, as `standing`.
    leaders: Vec<Option<u32>>,
    lead_changes: u32,
    clan_civ: BTreeMap<u32, Option<u32>>,
    land: Vec<Option<u32>>,
    clans_changed: u32,
    people_changed: f64,
    land_changed: u32,
}

impl Watch {
    /// Watches from the world as it is now, the pick: its civs' Standing and leader are the start.
    pub fn new(w: &World) -> Watch {
        let standing = stats::standing(w);
        Watch {
            pick: w.turn,
            spreads: vec![spread(&standing)],
            leaders: vec![leader_of(&standing)],
            standing: vec![standing],
            lead_changes: 0,
            clan_civ: clan_civs(w),
            land: land_civs(w),
            clans_changed: 0,
            people_changed: 0.0,
            land_changed: 0,
        }
    }

    /// Scores the turn that has just ended.
    pub fn turn(&mut self, w: &World) {
        let standing = stats::standing(w);
        let cur = *self.leaders.last().expect("a watch has its pick");
        let next = match (cur, leader_of(&standing)) {
            (Some(c), Some(b))
                if b != c && standing[&b] > standing.get(&c).copied().unwrap_or(0.0) + JUDGE.lead_margin =>
            {
                self.lead_changes += 1;
                Some(b)
            }
            (None, b) => b,
            (c, _) => c,
        };
        self.leaders.push(next);
        self.spreads.push(spread(&standing));
        self.standing.push(standing);

        for (id, civ) in clan_civs(w) {
            if let Some(before) = self.clan_civ.get(&id) {
                if *before != civ {
                    self.clans_changed += 1;
                    self.people_changed += w.clans[&id].people;
                }
            }
        }
        self.clan_civ = clan_civs(w);

        let land = land_civs(w);
        self.land_changed += self.land.iter().zip(&land).filter(|(a, b)| a != b).count() as u32;
        self.land = land;
    }

    /// The turn the watch began at.
    pub fn pick(&self) -> u32 {
        self.pick
    }

    /// Turns after the pick that have been scored.
    pub fn turns(&self) -> u32 {
        (self.standing.len() - 1) as u32
    }

    /// Every civ's Standing `offset` turns after the pick; `None` past the last turn scored.
    pub fn standing_at(&self, offset: u32) -> Option<&BTreeMap<u32, f64>> {
        self.standing.get(offset as usize)
    }

    /// The leader `offset` turns after the pick.
    pub fn leader_at(&self, offset: u32) -> Option<u32> {
        self.leaders.get(offset as usize).copied().flatten()
    }

    /// The spread `offset` turns after the pick.
    pub fn spread_at(&self, offset: u32) -> Option<f64> {
        self.spreads.get(offset as usize).copied()
    }

    /// The least and the most spread over the whole watch.
    pub fn spread_range(&self) -> (f64, f64) {
        let min = self.spreads.iter().copied().fold(f64::INFINITY, f64::min);
        let max = self.spreads.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    }

    /// Times the lead passed to another civ by more than `JUDGE.lead_margin`.
    pub fn lead_changes(&self) -> u32 {
        self.lead_changes
    }

    /// How often the spread opens and closes: sampled every `JUDGE.sample` turns, each change of
    /// more than `JUDGE.lead_margin` is up or down, and a swing is one whose direction is the
    /// opposite of the last such change.
    pub fn swings(&self) -> u32 {
        let samples: Vec<f64> = self.spreads.iter().step_by(JUDGE.sample as usize).copied().collect();
        let mut last: Option<bool> = None;
        let mut swings = 0;
        for pair in samples.windows(2) {
            let d = pair[1] - pair[0];
            if d.abs() > JUDGE.lead_margin {
                let up = d > 0.0;
                if last.is_some_and(|l| l != up) {
                    swings += 1;
                }
                last = Some(up);
            }
        }
        swings
    }

    /// Clans whose civ changed (a clan gone or new doesn't count).
    pub fn clans_changed(&self) -> u32 {
        self.clans_changed
    }

    /// The people of the clans that changed civ, at the change.
    pub fn people_changed(&self) -> f64 {
        self.people_changed
    }

    /// Land tiles whose civ changed.
    pub fn land_changed(&self) -> u32 {
        self.land_changed
    }

    /// The watch's scores for one game. `civs` are the civs whose orders are counted in
    /// `decisions`: every civ for a sweep, the led civ for a played game.
    pub fn scores(&self, w: &World, civs: &[u32]) -> Scores {
        let at = |offset: u32| self.standing_at(offset);
        let share = |offset: u32| at(offset).map(|s| s.values().sum::<f64>());
        let top = |offset: u32| at(offset).map(|s| s.values().copied().fold(0.0, f64::max));
        let (lowest, highest) = self.spread_range();
        Scores {
            standing: StandingScores {
                civs: at(0)
                    .map(|s| {
                        s.keys()
                            .map(|&id| CivStanding {
                                id,
                                at_100: at(100).and_then(|m| m.get(&id).copied()),
                                at_200: at(200).and_then(|m| m.get(&id).copied()),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                top_200: top(200),
                aligned_200: share(200),
            },
            doubt: DoubtScores {
                lead_changes: self.lead_changes,
                spread_100: self.spread_at(100),
                spread_200: self.spread_at(200),
                spread_min: lowest,
                spread_max: highest,
                swings: self.swings(),
            },
            decisions: DecisionScores::of(w, civs, self.turns()),
            motion: MotionScores {
                clans_changed: self.clans_changed,
                people_changed: self.people_changed,
                land_changed: self.land_changed,
            },
        }
    }
}

/// One game's scores, grouped as `docs/process.md` groups them.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Scores {
    pub standing: StandingScores,
    pub doubt: DoubtScores,
    pub decisions: DecisionScores,
    pub motion: MotionScores,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StandingScores {
    /// Each civ's Standing at +100 and +200 turns (`None` past the game's end).
    pub civs: Vec<CivStanding>,
    /// The most Standing any civ has at +200.
    pub top_200: Option<f64>,
    /// The civs' Standing summed at +200: the share of the world in any civ.
    pub aligned_200: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CivStanding {
    pub id: u32,
    pub at_100: Option<f64>,
    pub at_200: Option<f64>,
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
}
