//! Settling in the sweep: each map's settled clans and people at +100 and +200, by civ and unaligned,
//! beside its towns and capital towns at +200 and how often its clans change either way, and the seven
//! maps' totals. The control is main's sweep: settling is a reading, so the run is main's game.

use engine::judge_settling::SettledCiv;
use serde::Serialize;

use crate::sweep_judge::{MapJudged, Report};

/// The seven maps' settling: the civs' clans together, the unaligned clans, the total, the clans that
/// changed and changed more than once, and the towns and capital towns at +200 of the capitals alive.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct TotalSettling {
    pub(crate) civs: SettledCiv,
    pub(crate) unaligned: SettledCiv,
    pub(crate) total: SettledCiv,
    pub(crate) changes_per_100: f64,
    pub(crate) clans_changed: u32,
    pub(crate) clans_changed_twice: u32,
    pub(crate) towns_200: u32,
    pub(crate) capital_towns_200: u32,
    pub(crate) capitals_200: u32,
}

pub(crate) fn total_settling(maps: &[MapJudged]) -> TotalSettling {
    let mut t = TotalSettling::default();
    for m in maps {
        let s = &m.scores.settling;
        for c in s.civs.values() {
            t.civs.add(c);
        }
        t.unaligned.add(&s.unaligned);
        t.total.add(&s.total);
        t.clans_changed += s.clans_changed;
        t.clans_changed_twice += s.clans_changed_twice;
        t.towns_200 += m.scores.towns.total.towns_200;
        t.capital_towns_200 += m.scores.towns.total.capital_towns_200;
        t.capitals_200 += m.scores.towns.capitals_200;
    }
    t.changes_per_100 = t.total.changes_per_100();
    t
}

/// `settled clans/clans (people/people)` at +100 and +200.
fn row(c: &SettledCiv) -> String {
    format!(
        "+100 {}/{} clans ({:.0}/{:.0} people), +200 {}/{} clans ({:.0}/{:.0} people), settles {} back {}",
        c.settled_100,
        c.clans_100,
        c.settled_people_100,
        c.people_100,
        c.settled_200,
        c.clans_200,
        c.settled_people_200,
        c.people_200,
        c.settles,
        c.goes_back
    )
}

pub(crate) fn print_settling(r: &Report) {
    println!("settling (settled clans of all clans; the control is main's sweep):");
    for m in &r.maps {
        let s = &m.scores.settling;
        let mut civs = SettledCiv::default();
        for c in s.civs.values() {
            civs.add(c);
        }
        println!(
            "  {:<6} seed {:>3}  civs {}; unaligned {}; towns at +200 {}, capital towns {} of {}; changes {:.3} per 100 clan-turns, clans changed {} (twice or more {})",
            m.size,
            m.seed,
            row(&civs),
            row(&s.unaligned),
            m.scores.towns.total.towns_200,
            m.scores.towns.total.capital_towns_200,
            m.scores.towns.capitals_200,
            s.total.changes_per_100(),
            s.clans_changed,
            s.clans_changed_twice
        );
    }
    let t = &r.total.settling;
    println!(
        "total settling: civs {}; unaligned {}; towns at +200 {}, capital towns {} of {}; changes {:.3} per 100 clan-turns, clans changed {} (twice or more {})",
        row(&t.civs),
        row(&t.unaligned),
        t.towns_200,
        t.capital_towns_200,
        t.capitals_200,
        t.changes_per_100,
        t.clans_changed,
        t.clans_changed_twice
    );
}
