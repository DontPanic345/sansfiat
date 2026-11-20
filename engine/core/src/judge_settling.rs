//! Settling in the judge: the settled clans and their people at +100 and +200, by civ and unaligned, and
//! over every turn past the pick how often a clan settles and goes back (`ClanSettled`, `ClanGathers`),
//! with the clan-turns they are read against and the clans that changed more than once. The world is
//! only read.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::events::EventKind;
use crate::world::World;

/// One civ's settling, or the unaligned clans', or the total.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SettledCiv {
    /// Settled clans and their people at +100 and +200, beside all clans and people then.
    pub settled_100: u32,
    pub settled_people_100: f64,
    pub clans_100: u32,
    pub people_100: f64,
    pub settled_200: u32,
    pub settled_people_200: f64,
    pub clans_200: u32,
    pub people_200: f64,
    /// Clans that settled, and settled clans that went back, over the turns past the pick, counted under
    /// the clan's civ at the change.
    pub settles: u32,
    pub goes_back: u32,
    /// Clan-turns past the pick, the changes' denominator.
    pub clan_turns: u32,
}

impl SettledCiv {
    pub fn add(&mut self, o: &SettledCiv) {
        self.settled_100 += o.settled_100;
        self.settled_people_100 += o.settled_people_100;
        self.clans_100 += o.clans_100;
        self.people_100 += o.people_100;
        self.settled_200 += o.settled_200;
        self.settled_people_200 += o.settled_people_200;
        self.clans_200 += o.clans_200;
        self.people_200 += o.people_200;
        self.settles += o.settles;
        self.goes_back += o.goes_back;
        self.clan_turns += o.clan_turns;
    }

    /// Changes either way per 100 clan-turns.
    pub fn changes_per_100(&self) -> f64 {
        crate::judge::per_100(self.settles + self.goes_back, self.clan_turns)
    }
}

/// The settling group: per civ, the unaligned clans, the total, and the clans that changed more than
/// once over the watch.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SettlingScores {
    pub civs: BTreeMap<u32, SettledCiv>,
    pub unaligned: SettledCiv,
    pub total: SettledCiv,
    /// Clans that changed at all, and those that changed more than once.
    pub clans_changed: u32,
    pub clans_changed_twice: u32,
}

/// The judge's settling counts as the watch carries them.
#[derive(Clone, Debug, Default)]
pub(crate) struct Settling {
    by: BTreeMap<Option<u32>, SettledCiv>,
    changes: BTreeMap<u32, u32>,
    event_mark: usize,
    turns: u32,
}

impl Settling {
    pub(crate) fn pick(w: &World) -> Settling {
        Settling { event_mark: w.events.len(), ..Default::default() }
    }

    /// Scores the turn that has just ended.
    pub(crate) fn turn(&mut self, w: &World) {
        self.turns += 1;
        for e in &w.events[self.event_mark..] {
            let (EventKind::ClanSettled | EventKind::ClanGathers) = e.kind else { continue };
            let Some(id) = e.clan else { continue };
            let civ = w.clans.get(&id).and_then(|c| c.civ);
            let f = self.by.entry(civ).or_default();
            if e.kind == EventKind::ClanSettled {
                f.settles += 1;
            } else {
                f.goes_back += 1;
            }
            *self.changes.entry(id).or_default() += 1;
        }
        self.event_mark = w.events.len();
        let now = self.turns;
        for c in w.clans.values() {
            let f = self.by.entry(c.civ).or_default();
            f.clan_turns += 1;
            let (s, p) = (c.settled as u32, if c.settled { c.people } else { 0.0 });
            if now == 100 {
                f.settled_100 += s;
                f.settled_people_100 += p;
                f.clans_100 += 1;
                f.people_100 += c.people;
            } else if now == 200 {
                f.settled_200 += s;
                f.settled_people_200 += p;
                f.clans_200 += 1;
                f.people_200 += c.people;
            }
        }
    }

    pub(crate) fn scores(&self) -> SettlingScores {
        let mut s = SettlingScores::default();
        for (civ, f) in &self.by {
            s.total.add(f);
            match civ {
                Some(id) => {
                    s.civs.insert(*id, f.clone());
                }
                None => s.unaligned.add(f),
            }
        }
        s.clans_changed = self.changes.len() as u32;
        s.clans_changed_twice = self.changes.values().filter(|&&n| n > 1).count() as u32;
        s
    }
}
