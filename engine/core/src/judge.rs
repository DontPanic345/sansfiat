//! The judge: a game scored turn by turn. A `Watch` is made at the pick and turned after each turn;
//! it keeps every civ's Standing, the leader, the spread, and what changes hands. It reads the world
//! and changes nothing, and it draws nothing from `World.rng`.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::civs::{alive, tile_civ};
use crate::data::{JUDGE, TERRAIN, UNIT};
use crate::events::EventKind;
use crate::judge_fields::Fields;
use crate::judge_ring::Supply;
pub use crate::judge_scores::*;
use crate::judge_towns::Towns;
use crate::over::Why;
use crate::stats;
use crate::units::{self, UnitKind};
use crate::world::{Phase, World};

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

/// The place of civ `led` in Standing `s`: 1 for the largest share; civs with equal shares share the
/// better place.
pub fn rank_of(s: &BTreeMap<u32, f64>, led: u32) -> usize {
    let mine = s.get(&led).copied().unwrap_or(0.0);
    1 + s.values().filter(|&&v| v > mine).count()
}

/// How often civ `led`'s place in Standing changed over `standing` (one entry a turn, the pick
/// first). A rival passes it, or is passed, only once it is ahead or behind by more than
/// `JUDGE.lead_margin`, so two civs neck and neck don't trade places on noise.
pub fn rank_changes(standing: &[BTreeMap<u32, f64>], led: u32) -> u32 {
    let Some(first) = standing.first() else { return 0 };
    let at = |s: &BTreeMap<u32, f64>, id: u32| s.get(&id).copied().unwrap_or(0.0);
    let mut ahead: BTreeMap<u32, bool> =
        first.keys().filter(|&&o| o != led).map(|&o| (o, at(first, o) > at(first, led))).collect();
    let mut changes = 0;
    for s in &standing[1..] {
        let before = ahead.values().filter(|&&a| a).count();
        for (&o, a) in ahead.iter_mut() {
            let (theirs, mine) = (at(s, o), at(s, led));
            if theirs > mine + JUDGE.lead_margin {
                *a = true;
            } else if mine > theirs + JUDGE.lead_margin {
                *a = false;
            }
        }
        if ahead.values().filter(|&&a| a).count() != before {
            changes += 1;
        }
    }
    changes
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

/// A clan's festival now, and the festivals it left within the flip window with the turn it left each.
type Festivals = (Option<u32>, Vec<(Option<u32>, u32)>);

/// A civ's land at a turn: the tiles it claims.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CivLand {
    pub id: u32,
    pub claimed: usize,
}

/// Every civ's land, in id order.
pub fn civ_lands(w: &World) -> Vec<CivLand> {
    w.civs.keys().map(|&id| CivLand { id, claimed: w.civs[&id].claim.len() }).collect()
}

/// The economy's tallies over the civ-turns read so far (see `EconomyScores`).
#[derive(Clone, Debug, Default)]
struct Economy {
    civ_turns: u32,
    store_sum: f64,
    store_max: f64,
    turns_at_pack: u32,
    levy_sum: f64,
    largest_army: f64,
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
    trails_pick: stats::Trails,
    trails_200: Option<stats::Trails>,
    /// Each clan's festival now, and the festivals it left within the flip window: (festival, turn
    /// it left).
    festival: BTreeMap<u32, Festivals>,
    clan_turns: u32,
    festival_changes: u32,
    flips: u32,
    /// The index of the first event not yet scored.
    event_mark: usize,
    clans_left: u32,
    clans_joined: u32,
    /// Tiles of claims lost to a rival's unit (`ClaimTaken`).
    claim_taken: u32,
    /// Capitals taken after the pick (`CapitalFell`).
    capitals_fallen: u32,
    /// Each civ's Standing in its parts at +200.
    parts_200: Option<BTreeMap<u32, stats::Parts>>,
    /// Each civ's land at +200.
    civs_200: Vec<CivLand>,
    economy: Economy,
    /// The ring and the units inside and outside it (`judge_ring`).
    supply: Supply,
    /// Fields, farmers and the store's packs (`judge_fields`).
    fields: Fields,
    /// Towns and the clans in them (`judge_towns`).
    towns: Towns,
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
            trails_pick: stats::trails(w),
            trails_200: None,
            festival: w.clans.iter().map(|(&id, c)| (id, (c.market, Vec::new()))).collect(),
            clan_turns: 0,
            festival_changes: 0,
            flips: 0,
            event_mark: w.events.len(),
            clans_left: 0,
            clans_joined: 0,
            claim_taken: 0,
            capitals_fallen: 0,
            parts_200: None,
            civs_200: Vec::new(),
            economy: Economy::default(),
            supply: Supply::pick(w),
            fields: Fields::pick(w),
            towns: Towns::pick(w),
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

        for e in &w.events[self.event_mark..] {
            match e.kind {
                EventKind::ClanLeft => self.clans_left += 1,
                EventKind::ClanJoined => self.clans_joined += 1,
                EventKind::ClaimTaken => self.claim_taken += e.amount.unwrap_or(0.0) as u32,
                EventKind::CapitalFell => self.capitals_fallen += 1,
                _ => {}
            }
        }
        self.event_mark = w.events.len();

        let pack = units::pack_cap(UNIT.first as f64);
        for c in alive(w) {
            let store: f64 = c.store.iter().sum();
            let e = &mut self.economy;
            e.civ_turns += 1;
            e.store_sum += store;
            e.store_max = e.store_max.max(store);
            e.turns_at_pack += (store >= pack) as u32;
            e.levy_sum += c.levied;
        }
        let camp = w.units.values().filter(|u| u.kind() == UnitKind::Camp).map(|u| u.people).fold(0.0, f64::max);
        self.economy.largest_army = self.economy.largest_army.max(camp);

        self.supply.turn(w);
        self.fields.turn(w);
        self.towns.turn(w);

        let now = self.turns();
        let mut festival = BTreeMap::new();
        for (&id, c) in &w.clans {
            let Some((was, mut left)) = self.festival.remove(&id) else {
                festival.insert(id, (c.market, Vec::new()));
                continue;
            };
            self.clan_turns += 1;
            left.retain(|&(_, t)| now - t < JUDGE.flip_window);
            if c.market != was {
                self.festival_changes += 1;
                if left.iter().any(|&(m, _)| m == c.market) {
                    self.flips += 1;
                }
                left.push((was, now));
            }
            festival.insert(id, (c.market, left));
        }
        self.festival = festival;
        if now == 200 {
            self.trails_200 = Some(stats::trails(w));
            self.parts_200 = Some(stats::standing_parts(w));
            self.civs_200 = civ_lands(w);
            self.supply.at_200(w);
        }
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
        let parts = self.parts_200.as_ref();
        Scores {
            standing: StandingScores {
                civs: at(0)
                    .map(|s| {
                        s.keys()
                            .map(|&id| CivStanding {
                                id,
                                at_100: at(100).and_then(|m| m.get(&id).copied()),
                                at_200: at(200).and_then(|m| m.get(&id).copied()),
                                clans_200: parts.and_then(|p| p.get(&id)).map(|p| p.clans),
                                land_200: parts.and_then(|p| p.get(&id)).map(|p| p.land),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                top_200: top(200),
                aligned_200: share(200),
                on_land_200: parts.map(|p| p.values().map(|p| p.land).sum()),
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
                clans_left: self.clans_left,
                clans_joined: self.clans_joined,
                claim_taken: self.claim_taken,
            },
            land: LandScores {
                at_pick: self.trails_pick.clone(),
                at_200: self.trails_200.clone(),
                clan_turns: self.clan_turns,
                festival_changes: self.festival_changes,
                flips: self.flips,
                festival_changes_per_100: per_100(self.festival_changes, self.clan_turns),
                flips_per_100: per_100(self.flips, self.clan_turns),
                civs_200: self.civs_200.clone(),
            },
            fight: FightScores::of(w),
            economy: self.economy.scores(),
            supply: self.supply.scores(),
            fields: self.fields.scores(),
            towns: self.towns.scores(),
            end: self.end_scores(w),
        }
    }

    /// The end as the world reads it now: the game's `Over` if it has ended, and each civ's rank at
    /// the end (at `Over`'s turn), or at the watch's last turn while the game goes on.
    fn end_scores(&self, w: &World) -> EndScores {
        let over = match w.phase {
            Phase::Over(o) => Some(EndOver {
                why: match o.why {
                    Why::Conquest => "conquest",
                    Why::Limit => "limit",
                    Why::Fell { .. } => "fell",
                }
                .to_string(),
                winner: o.winner,
                turn: o.turn,
            }),
            _ => None,
        };
        let early = matches!(w.phase, Phase::Over(o) if o.why != Why::Limit);
        let at = over.as_ref().map_or(self.turns(), |o| o.turn) as usize;
        let ranks: BTreeMap<u32, usize> =
            w.standing.get(at).map(|s| s.keys().map(|&id| (id, rank_of(s, id))).collect()).unwrap_or_default();
        EndScores {
            led_rank: w.player.and_then(|p| ranks.get(&p).copied()),
            over,
            early,
            capitals_fallen: self.capitals_fallen,
            ranks,
        }
    }
}

/// `n` per 100 clan-turns.
pub fn per_100(n: u32, clan_turns: u32) -> f64 {
    if clan_turns == 0 {
        0.0
    } else {
        f64::from(n) * 100.0 / f64::from(clan_turns)
    }
}

impl Economy {
    fn scores(&self) -> EconomyScores {
        let n = f64::from(self.civ_turns);
        let mean = |sum: f64| if self.civ_turns == 0 { 0.0 } else { sum / n };
        EconomyScores {
            store_mean: mean(self.store_sum),
            store_max: self.store_max,
            turns_at_pack: self.turns_at_pack,
            levy_per_turn: mean(self.levy_sum),
            largest_army: self.largest_army,
        }
    }
}
