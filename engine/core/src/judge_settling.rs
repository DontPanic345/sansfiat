//! Settling in the judge: the settled clans and their people at +100 and +200, by civ and unaligned, and
//! over every turn past the pick how often a clan settles and goes back (`ClanSettled`, `ClanGathers`),
//! with the clan-turns they are read against and the clans that changed more than once; and the people
//! born to settled and to gathering clans a person a turn, and the people crowding killed. The world is
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
    /// Over the turns past the pick, the people born (`Clan.born`) to settled and to gathering clans,
    /// beside their person-turns (people before the turn's births and crowding deaths).
    pub settled_born: f64,
    pub settled_person_turns: f64,
    pub gathering_born: f64,
    pub gathering_person_turns: f64,
    /// People lost to crowding (`Clan.crowded`) over the turns past the pick: not starving.
    pub crowded: f64,
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
        self.settled_born += o.settled_born;
        self.settled_person_turns += o.settled_person_turns;
        self.gathering_born += o.gathering_born;
        self.gathering_person_turns += o.gathering_person_turns;
        self.crowded += o.crowded;
    }

    /// People born a person a turn, to settled and to gathering clans.
    pub fn growth(&self) -> (f64, f64) {
        let per = |b: f64, p: f64| if p > 0.0 { b / p } else { 0.0 };
        (per(self.settled_born, self.settled_person_turns), per(self.gathering_born, self.gathering_person_turns))
    }

    /// People lost to crowding a person a turn.
    pub fn crowded_share(&self) -> f64 {
        let p = self.settled_person_turns + self.gathering_person_turns;
        if p > 0.0 { self.crowded / p } else { 0.0 }
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
            let before = c.people - c.born + c.crowded;
            if c.settled {
                f.settled_born += c.born;
                f.settled_person_turns += before;
            } else {
                f.gathering_born += c.born;
                f.gathering_person_turns += before;
            }
            f.crowded += c.crowded;
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
