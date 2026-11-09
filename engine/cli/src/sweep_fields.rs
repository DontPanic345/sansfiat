//! `sweep --judge`'s fields against the no-fields control (`Config.fields` off): per map and over the
//! maps, the Standing farming civs gain, the store's packs, and how settled farmers and gatherers move
//! and starve. Every number is in the JSON, so `--floor` gives each its floor; the text reads the
//! spec's bounds (`relay/specs/155/spec.md`) on them.

use engine::judge::Scores;
use engine::judge_fields::FieldCiv;
use serde::Serialize;

use crate::sweep_judge::{MapJudged, Report};

/// The farming share at +200 from which a civ farms.
const FARMS: f64 = 0.1;

/// One civ on one map: its farming share at +200, whether it farms, its Standing at +200 less the
/// control's, and its store's turns at a pack of its turns.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct CivFields {
    pub(crate) id: u32,
    pub(crate) share: f64,
    pub(crate) farms: bool,
    pub(crate) gain: f64,
    pub(crate) store_turns_at_pack: u32,
    pub(crate) turns: u32,
}

/// One map's fields against its no-fields control.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct MapFields {
    pub(crate) civs: Vec<CivFields>,
    pub(crate) farming_civs: u32,
    /// The farming civs' mean gain, and how many gain (`None`/0 with none farming).
    pub(crate) farming_gain: Option<f64>,
    pub(crate) farming_gaining: u32,
    /// The mean gain of the civs that don't farm.
    pub(crate) others_gain: Option<f64>,
    /// The civ with the largest farming share, and whether its store held a pack on at least half
    /// its turns (1 or 0).
    pub(crate) top_farmer: Option<u32>,
    pub(crate) top_farmer_at_pack: u32,
    /// Starving clan-turns as a share of clan-turns, the control's, and the run's over the control's.
    pub(crate) starving_share: f64,
    pub(crate) control_starving_share: f64,
    pub(crate) starving_ratio: f64,
    pub(crate) settled_starving_share: f64,
    pub(crate) gatherer_starving_share: f64,
}

/// A share, 0 of nothing.
fn share(n: u32, of: u32) -> f64 {
    if of == 0 {
        0.0
    } else {
        f64::from(n) / f64::from(of)
    }
}

/// A ratio, 0 over nothing.
fn ratio(a: f64, b: f64) -> f64 {
    if b > 0.0 {
        a / b
    } else {
        0.0
    }
}

fn mean(v: &[f64]) -> Option<f64> {
    (!v.is_empty()).then(|| v.iter().sum::<f64>() / v.len() as f64)
}

fn standing_200(s: &Scores, civ: u32) -> f64 {
    s.standing.civs.iter().find(|c| c.id == civ).and_then(|c| c.at_200).unwrap_or(0.0)
}

/// One map's run read against its no-fields control.
pub(crate) fn map_fields(run: &Scores, control: &Scores) -> MapFields {
    let civs: Vec<CivFields> = run
        .fields
        .civs
        .iter()
        .map(|(&id, f)| CivFields {
            id,
            share: f.farming_share_200,
            farms: f.farming_share_200 >= FARMS,
            gain: standing_200(run, id) - standing_200(control, id),
            store_turns_at_pack: f.store_turns_at_pack,
            turns: f.turns,
        })
        .collect();
    let farming: Vec<f64> = civs.iter().filter(|c| c.farms).map(|c| c.gain).collect();
    let others: Vec<f64> = civs.iter().filter(|c| !c.farms).map(|c| c.gain).collect();
    let top = civs.iter().filter(|c| c.share > 0.0).fold(None::<&CivFields>, |b, c| match b {
        Some(b) if b.share >= c.share => Some(b),
        _ => Some(c),
    });
    let (t, ct) = (&run.fields.total, &control.fields.total);
    let starving_share = share(t.starving, t.clan_turns);
    let control_starving_share = share(ct.starving, ct.clan_turns);
    MapFields {
        farming_civs: farming.len() as u32,
        farming_gain: mean(&farming),
        farming_gaining: farming.iter().filter(|&&g| g > 0.0).count() as u32,
        others_gain: mean(&others),
        top_farmer: top.map(|c| c.id),
        top_farmer_at_pack: top.is_some_and(|c| 2 * c.store_turns_at_pack >= c.turns.max(1)) as u32,
        starving_share,
        control_starving_share,
        starving_ratio: ratio(starving_share, control_starving_share),
        settled_starving_share: share(t.settled_starving, t.settled_turns),
        gatherer_starving_share: share(t.gatherer_starving, t.gatherer_turns),
        civs,
    }
}

/// The fields over the maps, beside the no-fields control's where it has one.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalFields {
    pub(crate) civs: u32,
    pub(crate) farming_civs: u32,
    /// The mean gain over every farming civ of every map, how many gain, and the others' mean.
    pub(crate) farming_gain: f64,
    pub(crate) farming_gaining: u32,
    pub(crate) others_gain: f64,
    /// Farming civs' mean gain less the others'.
    pub(crate) farming_over_others: f64,
    /// Aligned Standing at +200, mean over the maps, the control's, and the run's less the control's.
    pub(crate) aligned_200: f64,
    pub(crate) control_aligned_200: f64,
    pub(crate) aligned_gain: f64,
    /// The maps whose top farmer held a pack on at least half its turns, of the maps with one.
    pub(crate) maps_top_farmer_at_pack: u32,
    pub(crate) maps_with_top_farmer: u32,
    pub(crate) civs_never_at_pack: u32,
    pub(crate) control_civs_never_at_pack: u32,
    /// Home moves and moves on per 100 clan-turns, summed over the maps, for settled farmers and
    /// gatherers.
    pub(crate) settled_home_moves_per_100: f64,
    pub(crate) gatherer_home_moves_per_100: f64,
    pub(crate) settled_moved_on_per_100: f64,
    pub(crate) gatherer_moved_on_per_100: f64,
    /// Starving clan-turns over clan-turns summed over the maps, the control's, their ratio, and the
    /// largest map's ratio.
    pub(crate) starving_share: f64,
    pub(crate) control_starving_share: f64,
    pub(crate) starving_ratio: f64,
    pub(crate) worst_map_starving_ratio: f64,
    pub(crate) settled_starving_share: f64,
    pub(crate) gatherer_starving_share: f64,
    /// Motion beside the control's: clans, people and land that changed civ, and clans joined.
    pub(crate) clans_changed: u32,
    pub(crate) control_clans_changed: u32,
    pub(crate) people_changed: f64,
    pub(crate) control_people_changed: f64,
    pub(crate) land_changed: u32,
    pub(crate) control_land_changed: u32,
    pub(crate) clans_joined: u32,
    pub(crate) control_clans_joined: u32,
    /// AI orders per 100 turns, mean over the maps, and the control's.
    pub(crate) ai_orders_per_100: f64,
    pub(crate) control_ai_orders_per_100: f64,
    /// Lead changes and the spread at +200 (mean), beside the control's: reported, not bound.
    pub(crate) lead_changes: u32,
    pub(crate) control_lead_changes: u32,
    pub(crate) spread_200: f64,
    pub(crate) control_spread_200: f64,
}

fn run(m: &MapJudged) -> &Scores {
    &m.scores
}

fn ctl(m: &MapJudged) -> &Scores {
    &m.no_fields
}

/// The fields totals over the judged maps.
pub(crate) fn total_fields(maps: &[MapJudged]) -> TotalFields {
    let civs: Vec<&CivFields> = maps.iter().flat_map(|m| &m.fields_vs_control.civs).collect();
    let farming: Vec<f64> = civs.iter().filter(|c| c.farms).map(|c| c.gain).collect();
    let others: Vec<f64> = civs.iter().filter(|c| !c.farms).map(|c| c.gain).collect();
    let sum = |f: fn(&Scores) -> &FieldCiv, which: fn(&MapJudged) -> &Scores| {
        let mut t = FieldCiv::default();
        for m in maps {
            t.add(f(which(m)));
        }
        t
    };
    let t = sum(|s| &s.fields.total, run);
    let ct = sum(|s| &s.fields.total, ctl);
    let n = maps.len().max(1) as f64;
    let mean_of =
        |f: fn(&Scores) -> f64, which: fn(&MapJudged) -> &Scores| maps.iter().map(|m| f(which(m))).sum::<f64>() / n;
    let count = |f: fn(&Scores) -> u32, which: fn(&MapJudged) -> &Scores| maps.iter().map(|m| f(which(m))).sum();
    let aligned = |s: &Scores| s.standing.aligned_200.unwrap_or(0.0);
    let starving_share = share(t.starving, t.clan_turns);
    let control_starving_share = share(ct.starving, ct.clan_turns);
    let (fg, og) = (mean(&farming).unwrap_or(0.0), mean(&others).unwrap_or(0.0));
    let per_100 = engine::judge::per_100;
    TotalFields {
        civs: civs.len() as u32,
        farming_civs: farming.len() as u32,
        farming_gain: fg,
        farming_gaining: farming.iter().filter(|&&g| g > 0.0).count() as u32,
        others_gain: og,
        farming_over_others: fg - og,
        aligned_200: mean_of(aligned, run),
        control_aligned_200: mean_of(aligned, ctl),
        aligned_gain: mean_of(aligned, run) - mean_of(aligned, ctl),
        maps_top_farmer_at_pack: maps.iter().map(|m| m.fields_vs_control.top_farmer_at_pack).sum(),
        maps_with_top_farmer: maps.iter().filter(|m| m.fields_vs_control.top_farmer.is_some()).count() as u32,
        civs_never_at_pack: count(|s| s.fields.civs_never_at_pack, run),
        control_civs_never_at_pack: count(|s| s.fields.civs_never_at_pack, ctl),
        settled_home_moves_per_100: per_100(t.settled_home_moves, t.settled_turns),
        gatherer_home_moves_per_100: per_100(t.gatherer_home_moves, t.gatherer_turns),
        settled_moved_on_per_100: per_100(t.settled_moved_on, t.settled_turns),
        gatherer_moved_on_per_100: per_100(t.gatherer_moved_on, t.gatherer_turns),
        starving_share,
        control_starving_share,
        starving_ratio: ratio(starving_share, control_starving_share),
        worst_map_starving_ratio: maps.iter().map(|m| m.fields_vs_control.starving_ratio).fold(0.0, f64::max),
        settled_starving_share: share(t.settled_starving, t.settled_turns),
        gatherer_starving_share: share(t.gatherer_starving, t.gatherer_turns),
        clans_changed: count(|s| s.motion.clans_changed, run),
        control_clans_changed: count(|s| s.motion.clans_changed, ctl),
        people_changed: mean_of(|s| s.motion.people_changed, run) * n,
        control_people_changed: mean_of(|s| s.motion.people_changed, ctl) * n,
        land_changed: count(|s| s.motion.land_changed, run),
        control_land_changed: count(|s| s.motion.land_changed, ctl),
        clans_joined: count(|s| s.motion.clans_joined, run),
        control_clans_joined: count(|s| s.motion.clans_joined, ctl),
        ai_orders_per_100: mean_of(|s| s.decisions.orders_per_100, run),
        control_ai_orders_per_100: mean_of(|s| s.decisions.orders_per_100, ctl),
        lead_changes: count(|s| s.doubt.lead_changes, run),
        control_lead_changes: count(|s| s.doubt.lead_changes, ctl),
        spread_200: mean_of(|s| s.doubt.spread_200.unwrap_or(0.0), run),
        control_spread_200: mean_of(|s| s.doubt.spread_200.unwrap_or(0.0), ctl),
    }
}

/// `pass` or `miss`.
fn mark(ok: bool) -> &'static str {
    if ok {
        "pass"
    } else {
        "MISS"
    }
}

/// The `fields` block of the text: a line per map, then the totals with the spec's bounds, each
/// marked. With floors, the starving ratios are read less their floor, and the floor is printed.
pub(crate) fn print_fields(r: &Report) {
    let floors = r.floor.as_ref();
    let floor = |path: &str| floors.and_then(|f| f.get(path).copied());
    let past = |v: f64, path: &str| v - floor(path).unwrap_or(0.0);
    let shown = |path: &str| floor(path).map_or(String::new(), |f| format!(" (floor {f:.3})"));
    println!("fields (against the no-fields control):");
    let mut worst: Option<(f64, String)> = None;
    for m in &r.maps {
        let f = &m.fields_vs_control;
        let civs: Vec<String> = f
            .civs
            .iter()
            .map(|c| {
                let name = m.names.get(&c.id).cloned().unwrap_or_else(|| format!("civ {}", c.id));
                format!("{name} {:.2} {:+.3} {}/{}", c.share, c.gain, c.store_turns_at_pack, c.turns)
            })
            .collect();
        let key = format!("maps.{} {}.fields_vs_control.starving_ratio", m.size, m.seed);
        let read = past(f.starving_ratio, &key);
        if worst.as_ref().is_none_or(|w| read > w.0) {
            worst = Some((read, key.clone()));
        }
        println!(
            "  {:<6} seed {:>3}  civs (share, gain, store turns at a pack): {}  starving {:.4} vs {:.4} = {:.2}{}  settled {:.4} gatherers {:.4}",
            m.size,
            m.seed,
            civs.join("; "),
            f.starving_share,
            f.control_starving_share,
            f.starving_ratio,
            shown(&key),
            f.settled_starving_share,
            f.gatherer_starving_share,
        );
    }
    let t = &r.total.fields_vs_control;
    let p = |k: &str| format!("total.fields_vs_control.{k}");
    let gaining = 4 * t.farming_gaining >= 3 * t.farming_civs.max(1);
    println!(
        "  standing: farming civs {} of {}, gain {:+.4} ({} gaining), others {:+.4}, over others {:+.4}: gain ≥ +0.01 {}, ¾ gaining {}, over others ≥ 0.01 {}",
        t.farming_civs,
        t.civs,
        t.farming_gain,
        t.farming_gaining,
        t.others_gain,
        t.farming_over_others,
        mark(t.farming_gain >= 0.01),
        mark(gaining),
        mark(t.farming_over_others >= 0.01),
    );
    let af = floor(&p("aligned_gain")).unwrap_or(0.0);
    println!(
        "  aligned at +200 {:.4} vs {:.4}, gain {:+.4}{}: past the floor {}",
        t.aligned_200,
        t.control_aligned_200,
        t.aligned_gain,
        shown(&p("aligned_gain")),
        mark(t.aligned_gain > af),
    );
    println!(
        "  store: top farmer at a pack on half its turns on {} of {} maps (≥ 5 {}); civs never at a pack {} (control {}) (≤ 2 {})",
        t.maps_top_farmer_at_pack,
        t.maps_with_top_farmer,
        mark(t.maps_top_farmer_at_pack >= 5),
        t.civs_never_at_pack,
        t.control_civs_never_at_pack,
        mark(t.civs_never_at_pack <= 2),
    );
    println!(
        "  motion: home moves per 100 settled {:.3} gatherers {:.3} (under half {}); moves on settled {:.3} gatherers {:.3} (no more {})",
        t.settled_home_moves_per_100,
        t.gatherer_home_moves_per_100,
        mark(t.settled_home_moves_per_100 < t.gatherer_home_moves_per_100 / 2.0),
        t.settled_moved_on_per_100,
        t.gatherer_moved_on_per_100,
        mark(t.settled_moved_on_per_100 <= t.gatherer_moved_on_per_100),
    );
    let not_fewer = |a: f64, b: f64, k: &str| mark(a >= b - floor(&p(k)).unwrap_or(0.0));
    println!(
        "  changed hands: clans {} vs {} {}, people {:.1} vs {:.1} {}, land {} vs {} {}; joined {} vs {}",
        t.clans_changed,
        t.control_clans_changed,
        not_fewer(f64::from(t.clans_changed), f64::from(t.control_clans_changed), "clans_changed"),
        t.people_changed,
        t.control_people_changed,
        not_fewer(t.people_changed, t.control_people_changed, "people_changed"),
        t.land_changed,
        t.control_land_changed,
        not_fewer(f64::from(t.land_changed), f64::from(t.control_land_changed), "land_changed"),
        t.clans_joined,
        t.control_clans_joined,
    );
    let read = past(t.starving_ratio, &p("starving_ratio"));
    let (wr, wk) = worst.unwrap_or((0.0, String::new()));
    println!(
        "  starving: {:.4} vs {:.4} = {:.3}{}, read {:.3} (≤ 1.25 {}); worst map read {:.3}{} (≤ 1.6 {}); settled {:.4} gatherers {:.4} (lower {})",
        t.starving_share,
        t.control_starving_share,
        t.starving_ratio,
        shown(&p("starving_ratio")),
        read,
        mark(read <= 1.25),
        wr,
        shown(&wk),
        mark(wr <= 1.6),
        t.settled_starving_share,
        t.gatherer_starving_share,
        mark(t.settled_starving_share < t.gatherer_starving_share),
    );
    println!(
        "  AI orders per 100 {:.2} vs {:.2} (≥ half {}); lead changes {} vs {}, spread +200 {:.3} vs {:.3} (not bound)",
        t.ai_orders_per_100,
        t.control_ai_orders_per_100,
        mark(t.ai_orders_per_100 >= t.control_ai_orders_per_100 / 2.0),
        t.lead_changes,
        t.control_lead_changes,
        t.spread_200,
        t.control_spread_200,
    );
}
