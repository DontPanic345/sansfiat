//! Settlements in the sweep: each map's run read against its no-signposts control
//! (`Config.signposts` off): each civ's settlements at +100 and +200 and its Standing beside the same
//! civ's with no signposts, and the seven maps' totals.

use engine::judge::Scores;
use serde::Serialize;

use crate::sweep_judge::{MapJudged, Report};

/// One civ of one map: its settlements and its Standing, beside the control's Standing.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct CivSettle {
    pub(crate) civ: u32,
    pub(crate) posted: u32,
    pub(crate) at_100: Option<u32>,
    pub(crate) at_200: Option<u32>,
    pub(crate) clans_200: u32,
    pub(crate) people_200: f64,
    pub(crate) store_200: f64,
    pub(crate) standing_100: Option<f64>,
    pub(crate) standing_200: Option<f64>,
    pub(crate) control_standing_100: Option<f64>,
    pub(crate) control_standing_200: Option<f64>,
}

/// One map's settlements read against its control.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct MapSettle {
    pub(crate) civs: Vec<CivSettle>,
    pub(crate) founded: u32,
    pub(crate) taken: u32,
    pub(crate) first: Option<u32>,
    /// The people of the world at the end of the watch, and the control's.
    pub(crate) people: f64,
    pub(crate) control_people: f64,
}

/// The seven maps' settlements.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct TotalSettle {
    pub(crate) posted: u32,
    pub(crate) founded: u32,
    pub(crate) taken: u32,
    pub(crate) at_100: u32,
    pub(crate) at_200: u32,
    /// Civs with a settlement at +200, of the civs scored at +200.
    pub(crate) civs_settled_200: u32,
    pub(crate) civs_of: u32,
    /// Maps with a settlement at +200.
    pub(crate) maps_settled_200: u32,
    /// The mean over those civs of Standing at +200, the control's, and the mean change; and the same
    /// over only the civs with a settlement at +200.
    pub(crate) standing_200: f64,
    pub(crate) control_standing_200: f64,
    pub(crate) settled_standing_200: f64,
    pub(crate) settled_control_standing_200: f64,
    /// Civs whose Standing at +200 is above the control's, and below it.
    pub(crate) civs_up: u32,
    pub(crate) civs_down: u32,
    /// The people of the seven worlds over the control's.
    pub(crate) people_vs_control: f64,
}

/// One map's settlements read against its control; `people` are the worlds' at the watch's end.
pub(crate) fn map_settle(run: &Scores, control: &Scores, people: f64, control_people: f64) -> MapSettle {
    let standing = |s: &Scores, civ: u32| s.standing.civs.iter().find(|c| c.id == civ).map(|c| (c.at_100, c.at_200));
    let civs = run
        .settle
        .civs
        .iter()
        .map(|c| {
            let (s100, s200) = standing(run, c.id).unwrap_or((None, None));
            let (c100, c200) = standing(control, c.id).unwrap_or((None, None));
            CivSettle {
                civ: c.id,
                posted: c.posted,
                at_100: c.at_100,
                at_200: c.at_200,
                clans_200: c.clans_200,
                people_200: c.people_200,
                store_200: c.store_200,
                standing_100: s100,
                standing_200: s200,
                control_standing_100: c100,
                control_standing_200: c200,
            }
        })
        .collect();
    MapSettle {
        civs,
        founded: run.settle.founded,
        taken: run.settle.taken,
        first: run.settle.first,
        people,
        control_people,
    }
}

/// The settle totals over the judged maps.
pub(crate) fn total_settle(maps: &[MapJudged]) -> TotalSettle {
    let mut t = TotalSettle::default();
    let (mut people, mut control) = (0.0, 0.0);
    let (mut settled, mut sum, mut csum, mut ssum, mut scsum) = (0u32, 0.0, 0.0, 0.0, 0.0);
    for m in maps {
        let s = &m.settle_vs_control;
        t.founded += s.founded;
        t.taken += s.taken;
        people += s.people;
        control += s.control_people;
        t.maps_settled_200 += s.civs.iter().any(|c| c.at_200.unwrap_or(0) > 0) as u32;
        for c in &s.civs {
            t.posted += c.posted;
            t.at_100 += c.at_100.unwrap_or(0);
            t.at_200 += c.at_200.unwrap_or(0);
            let (Some(a), Some(b)) = (c.standing_200, c.control_standing_200) else { continue };
            t.civs_of += 1;
            sum += a;
            csum += b;
            t.civs_up += (a > b) as u32;
            t.civs_down += (a < b) as u32;
            if c.at_200.unwrap_or(0) > 0 {
                settled += 1;
                ssum += a;
                scsum += b;
            }
        }
    }
    let mean = |v: f64, n: u32| if n > 0 { v / f64::from(n) } else { 0.0 };
    t.civs_settled_200 = settled;
    t.standing_200 = mean(sum, t.civs_of);
    t.control_standing_200 = mean(csum, t.civs_of);
    t.settled_standing_200 = mean(ssum, settled);
    t.settled_control_standing_200 = mean(scsum, settled);
    t.people_vs_control = if control > 0.0 { people / control } else { 0.0 };
    t
}

/// The settle section of the text report: a line a civ, and the totals.
pub(crate) fn print_settle(r: &Report) {
    let pct = |v: Option<f64>| v.map_or("-".to_string(), |v| format!("{:.1}%", v * 100.0));
    let n = |v: Option<u32>| v.map_or("-".to_string(), |v| v.to_string());
    println!("settlements (control: the same map with no signposts)");
    for m in &r.maps {
        let s = &m.settle_vs_control;
        println!(
            "  {} {}: founded {}, taken {}, first at +{}, people {:.0} ({:.0})",
            m.size,
            m.seed,
            s.founded,
            s.taken,
            s.first.map_or("-".to_string(), |t| t.to_string()),
            s.people,
            s.control_people
        );
        for c in &s.civs {
            let name = m.names.get(&c.civ).cloned().unwrap_or_default();
            println!(
                "    {name}: posted {}, settlements +100 {} +200 {} ({} clans, {:.0} people, store {:.0}); Standing +100 {} ({}), +200 {} ({})",
                c.posted,
                n(c.at_100),
                n(c.at_200),
                c.clans_200,
                c.people_200,
                c.store_200,
                pct(c.standing_100),
                pct(c.control_standing_100),
                pct(c.standing_200),
                pct(c.control_standing_200)
            );
        }
    }
    let t = &r.total.settle_vs_control;
    println!(
        "total settle: posted {}, founded {}, taken {}, standing +100 {} +200 {}; civs with one at +200 {} of {}, maps {} of {}",
        t.posted, t.founded, t.taken, t.at_100, t.at_200, t.civs_settled_200, t.civs_of, t.maps_settled_200, r.maps.len()
    );
    println!(
        "total settle Standing +200 (control): all civs {:.1}% ({:.1}%), civs with a settlement {:.1}% ({:.1}%); civs up {} down {}; people over control {:.3}",
        t.standing_200 * 100.0,
        t.control_standing_200 * 100.0,
        t.settled_standing_200 * 100.0,
        t.settled_control_standing_200 * 100.0,
        t.civs_up,
        t.civs_down,
        t.people_vs_control
    );
}
