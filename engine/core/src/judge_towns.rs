//! Towns in the judge: the towns that form, the farmers and town clans in them at +100 and +200, and over
//! the clan-turns after +100 what town clans, farmers outside towns and all clans walk, trade and starve
//! at. Settled farmers' home moves are split into town steps (a home that comes a tile nearer its
//! festival) and the rest, and town clans' moves off their town are counted, every turn past the pick.
//! `Watch` reads these after each turn and reports them as the `towns` group; the world is only read.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::data::CLAN;
use crate::events::EventKind;
use crate::fields;
use crate::homes;
use crate::judge_fields::FIELD_SETTLED;
use crate::towns::{towns, Town};
use crate::world::World;

/// The clan-turns of one group after +100: its walk, the food it traded and its starving turns.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct TownGroup {
    pub clan_turns: u32,
    /// Σ route cost ÷ (people × `CLAN.work`) over the clan-turns: the festival walk.
    pub walk: f64,
    /// `walk ÷ clan_turns`.
    pub walk_per_turn: f64,
    /// Σ food traded per person over the clan-turns (`World.traded`).
    pub traded: f64,
    /// `traded ÷ clan_turns`: food traded per person a turn.
    pub traded_per_turn: f64,
    pub starving: u32,
}

impl TownGroup {
    pub fn add(&mut self, o: &TownGroup) {
        self.clan_turns += o.clan_turns;
        self.walk += o.walk;
        self.traded += o.traded;
        self.starving += o.starving;
    }

    /// Starving clan-turns over clan-turns.
    pub fn starving_share(&self) -> f64 {
        if self.clan_turns == 0 {
            0.0
        } else {
            f64::from(self.starving) / f64::from(self.clan_turns)
        }
    }

    pub fn share(&mut self) {
        let n = f64::from(self.clan_turns);
        self.walk_per_turn = if n > 0.0 { self.walk / n } else { 0.0 };
        self.traded_per_turn = if n > 0.0 { self.traded / n } else { 0.0 };
    }
}

/// The town counts of one civ's clans and towns, or of every one (`TownScores.total`). A clan counts
/// under its civ at the turn's end; a town under the civ of its lowest-id clan that has one.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct TownCiv {
    pub towns_100: u32,
    pub towns_200: u32,
    /// Clans living in a town at +200, of them the farmers, and the towns whose clans work fields.
    pub town_clans_200: u32,
    pub town_farmers_200: u32,
    pub towns_fields_200: u32,
    /// Town clans, farmers outside towns and all clans, over the clan-turns after +100.
    pub town: TownGroup,
    pub farmers_outside: TownGroup,
    pub all: TownGroup,
}

impl TownCiv {
    pub fn add(&mut self, o: &TownCiv) {
        self.towns_100 += o.towns_100;
        self.towns_200 += o.towns_200;
        self.town_clans_200 += o.town_clans_200;
        self.town_farmers_200 += o.town_farmers_200;
        self.towns_fields_200 += o.towns_fields_200;
        self.town.add(&o.town);
        self.farmers_outside.add(&o.farmers_outside);
        self.all.add(&o.all);
    }

    pub fn share(&mut self) {
        self.town.share();
        self.farmers_outside.share();
        self.all.share();
    }
}

/// Home moves over every turn past the pick (as `FieldCiv`'s settled counts): settled farmers' town
/// steps and other home moves, and town clans' moves off their town's tile, with their clan-turns.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct TownMoves {
    pub settled_turns: u32,
    pub settled_town_steps: u32,
    pub settled_other_moves: u32,
    pub town_turns: u32,
    pub town_moves_off: u32,
}

/// The towns group: per civ and in total, and the moves, the most homes on one tile at +200, the turn
/// (after the pick) the first town formed, and how many formed.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct TownScores {
    pub civs: BTreeMap<u32, TownCiv>,
    pub total: TownCiv,
    pub moves: TownMoves,
    pub most_homes_200: u32,
    pub first_town: Option<u32>,
    pub formed: u32,
}

/// What the watch remembers of a clan from the turn before.
#[derive(Clone, Copy, Debug)]
struct Before {
    home: usize,
    run: u32,
    in_town: bool,
}

/// The judge's town counts as the watch carries them.
#[derive(Clone, Debug, Default)]
pub(crate) struct Towns {
    by: BTreeMap<Option<u32>, TownCiv>,
    /// The civs at the pick or founded since.
    civs: Vec<u32>,
    clans: BTreeMap<u32, Before>,
    moves: TownMoves,
    most_homes_200: u32,
    first_town: Option<u32>,
    formed: u32,
    event_mark: usize,
    turns: u32,
}

/// Whether a home move from `from` to `to` brings the home a tile nearer its festival's tile: the
/// clan's festival now, and the hex distance from each home to it.
fn town_step(w: &World, from: usize, to: usize, market: Option<u32>) -> bool {
    let Some(m) = market.and_then(|id| w.markets.get(&id)) else { return false };
    let festival = w.tiles[m.tile].hex();
    w.tiles[to].hex().distance(festival) < w.tiles[from].hex().distance(festival)
}

impl Towns {
    /// The clans and civs at the pick.
    pub(crate) fn pick(w: &World) -> Towns {
        let mut t = Towns { event_mark: w.events.len(), civs: w.civs.keys().copied().collect(), ..Default::default() };
        t.remember(w);
        t
    }

    fn remember(&mut self, w: &World) {
        let old = std::mem::take(&mut self.clans);
        let tiles: BTreeSet<usize> = towns(w).iter().map(|t| t.tile).collect();
        for (&id, c) in &w.clans {
            let run = if fields::farmer(c) { old.get(&id).map_or(0, |b| b.run) + 1 } else { 0 };
            self.clans.insert(id, Before { home: c.home, run, in_town: tiles.contains(&c.home) });
        }
    }

    /// The civ a town counts under: that of its lowest-id clan with one.
    fn town_civ(w: &World, t: &Town) -> Option<u32> {
        t.clans.iter().find_map(|id| w.clans.get(id).and_then(|c| c.civ))
    }

    /// Scores the turn that has just ended.
    pub(crate) fn turn(&mut self, w: &World) {
        self.turns += 1;
        let now = self.turns;
        for e in &w.events[self.event_mark..] {
            if e.kind == EventKind::TownFormed {
                self.formed += 1;
                self.first_town.get_or_insert(now);
            }
        }
        self.event_mark = w.events.len();

        for c in w.civs.keys() {
            if !self.civs.contains(c) {
                self.civs.push(*c);
            }
        }

        let tw = towns(w);
        let in_town: BTreeSet<usize> = tw.iter().map(|t| t.tile).collect();
        for (&id, c) in &w.clans {
            let Some(b) = self.clans.get(&id).copied() else { continue };
            let moved = b.home != c.home;
            if b.run >= FIELD_SETTLED {
                self.moves.settled_turns += 1;
                if moved {
                    if town_step(w, b.home, c.home, c.market) {
                        self.moves.settled_town_steps += 1;
                    } else {
                        self.moves.settled_other_moves += 1;
                    }
                }
            }
            if b.in_town {
                self.moves.town_turns += 1;
                self.moves.town_moves_off += moved as u32;
            }
        }

        if now > 100 {
            for (&id, c) in &w.clans {
                let walk: f64 = c.routes.iter().map(|r| r.cost).sum::<f64>() / (c.people * CLAN.work);
                let one = TownGroup {
                    clan_turns: 1,
                    walk,
                    traded: w.traded.get(&id).copied().unwrap_or(0.0),
                    starving: c.starving as u32,
                    ..Default::default()
                };
                let f = self.by.entry(c.civ).or_default();
                let outside = !in_town.contains(&c.home) && fields::farmer(c);
                if in_town.contains(&c.home) {
                    f.town.add(&one);
                }
                if outside {
                    f.farmers_outside.add(&one);
                }
                f.all.add(&one);
            }
        }

        if now == 100 || now == 200 {
            for t in &tw {
                let civ = Self::town_civ(w, t);
                if now == 100 {
                    self.by.entry(civ).or_default().towns_100 += 1;
                    continue;
                }
                let works = t.clans.iter().any(|id| fields::plots(w, &w.clans[id]).iter().any(|p| p.1 > 0.0));
                let f = self.by.entry(civ).or_default();
                f.towns_200 += 1;
                f.towns_fields_200 += works as u32;
                for id in &t.clans {
                    let c = &w.clans[id];
                    let g = self.by.entry(c.civ).or_default();
                    g.town_clans_200 += 1;
                    g.town_farmers_200 += fields::farmer(c) as u32;
                }
            }
            if now == 200 {
                self.most_homes_200 = homes::homes(w).values().map(Vec::len).max().unwrap_or(0) as u32;
            }
        }
        self.remember(w);
    }

    /// The group's scores: per civ (every civ at the pick or founded since), and the total.
    pub(crate) fn scores(&self) -> TownScores {
        let civs: BTreeMap<u32, TownCiv> = self
            .civs
            .iter()
            .map(|&id| {
                let mut f = self.by.get(&Some(id)).cloned().unwrap_or_default();
                f.share();
                (id, f)
            })
            .collect();
        let mut total = TownCiv::default();
        for f in self.by.values() {
            total.add(f);
        }
        total.share();
        TownScores {
            civs,
            total,
            moves: self.moves.clone(),
            most_homes_200: self.most_homes_200,
            first_town: self.first_town,
            formed: self.formed,
        }
    }
}
