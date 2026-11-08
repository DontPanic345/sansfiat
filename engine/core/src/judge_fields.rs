//! Fields in the judge: per civ and in total, the tiles with fields and the people farming at +100 and
//! +200, the turns a civ's store held a pack, and how settled farmers and gatherers move and starve
//! turn by turn. `Watch` reads these after each turn and reports them as the `fields` group; the world
//! is only read.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::civs::alive;
use crate::data::UNIT;
use crate::events::EventKind;
use crate::fields;
use crate::units;
use crate::world::World;

/// The farmer turns in a row, before a turn, from which a clan counts as a settled farmer on it. A
/// line the judge draws, not a rule.
pub const FIELD_SETTLED: u32 = 10;

/// The field counts of one civ's clans and tiles, or of every clan and tile (`FieldScores.total`).
/// A clan-turn counts under the clan's civ at the turn's end; the total counts unaligned clans too.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct FieldCiv {
    /// Tiles with fields worked (`Tile.field > 0`) at +100 and +200, and their field days at +200.
    pub field_tiles_100: usize,
    pub field_tiles_200: usize,
    pub field_days_200: f64,
    /// The clans' people at +200, and their people × fields.
    pub people_200: f64,
    pub farming_people_200: f64,
    /// `farming_people_200 ÷ people_200`: the share of the people's days on fields.
    pub farming_share_200: f64,
    /// Clans that are farmers (`fields::farmer`) at +200.
    pub farmers_200: u32,
    /// Turns the civ's store held `units::pack_cap(UNIT.first)` or more, and its turns alive.
    pub store_turns_at_pack: u32,
    pub turns: u32,
    /// Clan-turns of settled farmers (farmers `FIELD_SETTLED` turns in a row before the turn), their
    /// home moves, their moves on (`ClanMoved`) and their starving turns.
    pub settled_turns: u32,
    pub settled_home_moves: u32,
    pub settled_moved_on: u32,
    pub settled_starving: u32,
    /// As `settled_*`, for gatherers: clans with no fields before the turn.
    pub gatherer_turns: u32,
    pub gatherer_home_moves: u32,
    pub gatherer_moved_on: u32,
    pub gatherer_starving: u32,
    /// Every clan-turn, and the starving ones.
    pub clan_turns: u32,
    pub starving: u32,
}

impl FieldCiv {
    /// Adds `o`'s counts to these, and works the farming share out again.
    pub fn add(&mut self, o: &FieldCiv) {
        self.field_tiles_100 += o.field_tiles_100;
        self.field_tiles_200 += o.field_tiles_200;
        self.field_days_200 += o.field_days_200;
        self.people_200 += o.people_200;
        self.farming_people_200 += o.farming_people_200;
        self.farmers_200 += o.farmers_200;
        self.store_turns_at_pack += o.store_turns_at_pack;
        self.turns += o.turns;
        self.settled_turns += o.settled_turns;
        self.settled_home_moves += o.settled_home_moves;
        self.settled_moved_on += o.settled_moved_on;
        self.settled_starving += o.settled_starving;
        self.gatherer_turns += o.gatherer_turns;
        self.gatherer_home_moves += o.gatherer_home_moves;
        self.gatherer_moved_on += o.gatherer_moved_on;
        self.gatherer_starving += o.gatherer_starving;
        self.clan_turns += o.clan_turns;
        self.starving += o.starving;
        self.share();
    }

    fn share(&mut self) {
        self.farming_share_200 = if self.people_200 > 0.0 { self.farming_people_200 / self.people_200 } else { 0.0 };
    }
}

/// The fields group: per civ and over every clan and tile, and the civs whose store never held a pack.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct FieldScores {
    pub civs: BTreeMap<u32, FieldCiv>,
    pub total: FieldCiv,
    pub civs_never_at_pack: u32,
}

/// What the watch remembers of a clan from the turn before: its home, its fields and its farmer turns
/// in a row.
#[derive(Clone, Copy, Debug)]
struct Before {
    home: usize,
    fields: f64,
    run: u32,
}

/// The judge's field counts as the watch carries them.
#[derive(Clone, Debug, Default)]
pub(crate) struct Fields {
    /// Counts by the clan's civ at the turn's end; `None` holds unaligned clans and unclaimed tiles.
    by: BTreeMap<Option<u32>, FieldCiv>,
    /// The civs at the pick or founded since.
    civs: Vec<u32>,
    clans: BTreeMap<u32, Before>,
    event_mark: usize,
    turns: u32,
}

impl Fields {
    /// The clans and civs at the pick.
    pub(crate) fn pick(w: &World) -> Fields {
        let mut f = Fields { event_mark: w.events.len(), civs: w.civs.keys().copied().collect(), ..Default::default() };
        f.remember(w);
        f
    }

    fn remember(&mut self, w: &World) {
        let old = std::mem::take(&mut self.clans);
        for (&id, c) in &w.clans {
            let run = if fields::farmer(c) { old.get(&id).map_or(0, |b| b.run) + 1 } else { 0 };
            self.clans.insert(id, Before { home: c.home, fields: c.fields, run });
        }
    }

    /// The tiles with fields at this moment, by the civ of the clan that claims each.
    fn tiles(w: &World) -> BTreeMap<Option<u32>, (usize, f64)> {
        let mut out: BTreeMap<Option<u32>, (usize, f64)> = BTreeMap::new();
        for t in w.tiles.iter().filter(|t| t.field > 0.0) {
            let civ = t.claim.and_then(|id| w.clans.get(&id)).and_then(|c| c.civ);
            let e = out.entry(civ).or_default();
            e.0 += 1;
            e.1 += t.field;
        }
        out
    }

    /// Scores the turn that has just ended.
    pub(crate) fn turn(&mut self, w: &World) {
        self.turns += 1;
        let mut moved: BTreeMap<u32, u32> = BTreeMap::new();
        for e in &w.events[self.event_mark..] {
            if let (EventKind::ClanMoved, Some(id)) = (e.kind, e.clan) {
                *moved.entry(id).or_default() += 1;
            }
        }
        self.event_mark = w.events.len();

        for c in w.civs.keys() {
            if !self.civs.contains(c) {
                self.civs.push(*c);
            }
        }
        let pack = units::pack_cap(UNIT.first as f64);
        for c in alive(w) {
            let f = self.by.entry(Some(c.id)).or_default();
            f.turns += 1;
            f.store_turns_at_pack += (c.store.iter().sum::<f64>() >= pack) as u32;
        }

        for (&id, c) in &w.clans {
            let Some(b) = self.clans.get(&id).copied() else { continue };
            let f = self.by.entry(c.civ).or_default();
            let home = (b.home != c.home) as u32;
            let on = moved.get(&id).copied().unwrap_or(0);
            let starving = c.starving as u32;
            f.clan_turns += 1;
            f.starving += starving;
            if b.run >= FIELD_SETTLED {
                f.settled_turns += 1;
                f.settled_home_moves += home;
                f.settled_moved_on += on;
                f.settled_starving += starving;
            } else if b.fields <= 0.0 {
                f.gatherer_turns += 1;
                f.gatherer_home_moves += home;
                f.gatherer_moved_on += on;
                f.gatherer_starving += starving;
            }
        }
        self.remember(w);

        if self.turns == 100 {
            for (civ, (n, _)) in Fields::tiles(w) {
                self.by.entry(civ).or_default().field_tiles_100 = n;
            }
        }
        if self.turns == 200 {
            for (civ, (n, days)) in Fields::tiles(w) {
                let f = self.by.entry(civ).or_default();
                f.field_tiles_200 = n;
                f.field_days_200 = days;
            }
            for c in w.clans.values() {
                let f = self.by.entry(c.civ).or_default();
                f.people_200 += c.people;
                f.farming_people_200 += c.people * c.fields;
                f.farmers_200 += fields::farmer(c) as u32;
            }
            for f in self.by.values_mut() {
                f.share();
            }
        }
    }

    /// The group's scores: per civ (every civ at the pick or founded since), the total over every clan
    /// and tile, and the civs never at a pack.
    pub(crate) fn scores(&self) -> FieldScores {
        let civs: BTreeMap<u32, FieldCiv> =
            self.civs.iter().map(|&id| (id, self.by.get(&Some(id)).cloned().unwrap_or_default())).collect();
        let mut total = FieldCiv::default();
        for f in self.by.values() {
            total.add(f);
        }
        let civs_never_at_pack = civs.values().filter(|f| f.store_turns_at_pack == 0).count() as u32;
        FieldScores { civs, total, civs_never_at_pack }
    }
}
