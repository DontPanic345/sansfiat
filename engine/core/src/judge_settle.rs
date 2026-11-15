//! The `settle` group: signposts posted, settlements founded and taken, and what each civ has at +100
//! and +200: its settlements, the clans living on them and the food in their stores.

use serde::Serialize;

use crate::events::EventKind;
use crate::market::total;
use crate::world::World;

/// One civ's settlements.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SettleCiv {
    pub id: u32,
    /// Signpost orders it gave.
    pub posted: u32,
    /// Its settlements at +100 and +200.
    pub at_100: Option<u32>,
    pub at_200: Option<u32>,
    /// The clans living on its settlements' tiles at +200, and their people.
    pub clans_200: u32,
    pub people_200: f64,
    /// The food in its settlements' stores at +200.
    pub store_200: f64,
}

/// The settle group of one game.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SettleScores {
    pub civs: Vec<SettleCiv>,
    /// Settlements founded and taken after the pick.
    pub founded: u32,
    pub taken: u32,
    /// Turns after the pick the first was founded.
    pub first: Option<u32>,
    /// Settlements standing at +100 and +200, over the civs.
    pub at_100: u32,
    pub at_200: u32,
}

/// The watch's settle counts.
#[derive(Clone, Debug)]
pub struct Settle {
    pick: u32,
    mark: usize,
    scores: SettleScores,
}

impl Settle {
    pub fn pick(w: &World) -> Settle {
        let civs = w.civs.keys().map(|&id| SettleCiv { id, ..SettleCiv::default() }).collect();
        Settle { pick: w.turn, mark: w.events.len(), scores: SettleScores { civs, ..SettleScores::default() } }
    }

    /// Reads the turn that has just ended.
    pub fn turn(&mut self, w: &World) {
        let after = w.turn - self.pick;
        for e in &w.events[self.mark..] {
            match e.kind {
                EventKind::SettlementFounded => {
                    self.scores.founded += 1;
                    self.scores.first.get_or_insert(after);
                }
                EventKind::SettlementTaken => self.scores.taken += 1,
                _ => {}
            }
        }
        self.mark = w.events.len();
        let count = |civ: u32| crate::settle::of(w, civ).count() as u32;
        if after == 100 {
            for c in &mut self.scores.civs {
                c.at_100 = Some(count(c.id));
            }
            self.scores.at_100 = w.settlements.len() as u32;
        }
        if after == 200 {
            for c in &mut self.scores.civs {
                c.at_200 = Some(count(c.id));
                for s in crate::settle::of(w, c.id) {
                    c.store_200 += total(&s.store);
                    for k in w.clans.values().filter(|k| k.home == s.tile) {
                        c.clans_200 += 1;
                        c.people_200 += k.people;
                    }
                }
            }
            self.scores.at_200 = w.settlements.len() as u32;
        }
        for c in &mut self.scores.civs {
            c.posted = w.orders.get(&c.id).and_then(|o| o.get("signpost")).copied().unwrap_or(0);
        }
    }

    pub fn scores(&self) -> SettleScores {
        self.scores.clone()
    }
}
