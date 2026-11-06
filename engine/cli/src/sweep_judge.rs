//! `sweep --judge`: the retro maps played with the army twice, the `--no-army` control, the no-claims
//! control and, with `--solo`, each civ claiming alone; every civ on the AI, scored by `engine::judge`:
//! a report, with no verdict.

use std::any::Any;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

use engine::data::{Claims, JUDGE};
use engine::judge::{CivLand, Scores, Watch};
use engine::stats::Trails;
use engine::{stats, Action, Config, World};
use serde::Serialize;

use crate::retro::{kept_camp, retro_maps, to_pick};

/// The turns past the pick each map is run for.
const AFTER: u32 = 200;

/// One map's health: its pre-game and the digests of its two army runs. These are bugs whatever
/// the design says.
#[derive(Clone, Debug, Serialize)]
struct Health {
    pregame_by_centres: u32,
    digest_mismatches: u32,
}

/// One map's armies: its camps, what they cost their side, and the world's people beside the
/// `--no-army` control. These are measures, not bugs: the design decides what they should be.
#[derive(Clone, Debug, Serialize)]
struct Armies {
    camps_kept: u32,
    camps_of: u32,
    warriors_starved: u32,
    camps_lost: u32,
    people_at_200: f64,
    control_people_at_200: f64,
}

/// One civ's gain from claiming alone on a map (`--solo`): its Standing at +200 when only it claims,
/// less its Standing at +200 in the no-claims control. `clans` and `land` are the two parts of that
/// gain: its clans' and units' people, and the unaligned people on its held land.
#[derive(Clone, Debug, Serialize)]
struct SoloGain {
    civ: u32,
    gain: f64,
    clans: f64,
    land: f64,
}

/// One map that was judged.
#[derive(Clone, Debug, Serialize)]
struct MapJudged {
    size: String,
    seed: u32,
    pick: u32,
    #[serde(flatten)]
    scores: Scores,
    /// The same map with no civ claiming: the control the claims are read against.
    no_claims: Scores,
    /// The same map with `Config.fight` off: the control the fight group is read against.
    no_fight: Scores,
    /// With `--solo`: each civ's gain from claiming alone, in id order.
    solo: Option<Vec<SoloGain>>,
    armies: Armies,
    health: Health,
}

/// A map whose run panicked: its map and the message.
#[derive(Clone, Debug, Serialize)]
struct Panicked {
    size: String,
    seed: u32,
    message: String,
}

/// The sweep's totals, in groups.
#[derive(Clone, Debug, Serialize)]
struct Total {
    standing: TotalStanding,
    doubt: TotalDoubt,
    decisions: TotalDecisions,
    motion: TotalMotion,
    land: TotalLand,
    solo: Option<TotalSolo>,
    armies: TotalArmies,
    fight: TotalFight,
    health: TotalHealth,
}

/// The fight group over the maps, beside the no-fight control's (which has none by construction).
#[derive(Clone, Debug, Serialize)]
struct TotalFight {
    fallen: f64,
    taken: u32,
    surrendered: u32,
    control_fallen: f64,
    control_taken: u32,
    control_surrendered: u32,
}

/// The land group over the maps: festivals summed, shares averaged over the maps that have one, and
/// each civ's land at +200 (mean tiles and guard people per civ, over the maps).
#[derive(Clone, Debug, Serialize)]
struct TotalLand {
    markets_pick: usize,
    markets_200: usize,
    trading_pick: f64,
    trading_200: f64,
    homes_trailed: (Option<f64>, Option<f64>),
    worked_trailed: (Option<f64>, Option<f64>),
    route_trailed: (Option<f64>, Option<f64>),
    festival_changes_per_100: f64,
    flips_per_100: f64,
    claimed_200: Option<f64>,
    held_200: Option<f64>,
    guard_200: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
struct TotalStanding {
    top_200: f64,
    aligned_200: f64,
    on_land_200: f64,
}

#[derive(Clone, Debug, Serialize)]
struct TotalDoubt {
    lead_changes: u32,
    maps_with_lead_change: u32,
    spread_100: f64,
    spread_200: f64,
    swings: u32,
    no_claims_lead_changes: u32,
    no_claims_maps_with_lead_change: u32,
}

#[derive(Clone, Debug, Serialize)]
struct TotalDecisions {
    ai_orders_per_100: f64,
    by_kind: BTreeMap<String, f64>,
    levers_repeated: f64,
}

#[derive(Clone, Debug, Serialize)]
struct TotalMotion {
    clans_changed: u32,
    people_changed: f64,
    land_changed: u32,
    clans_left: u32,
    clans_joined: u32,
    claim_taken: u32,
}

/// The solo gains over every civ of every map: the mean gain and its two parts, and the civs whose
/// gain is above `JUDGE.solo_above`.
#[derive(Clone, Debug, Serialize)]
struct TotalSolo {
    civs: usize,
    gain: f64,
    clans: f64,
    land: f64,
    above: u32,
}

#[derive(Clone, Debug, Serialize)]
struct TotalArmies {
    camps_kept: u32,
    camps_of: u32,
    warriors_starved: u32,
    camps_lost: u32,
    people_vs_control: f64,
}

#[derive(Clone, Debug, Serialize)]
struct TotalHealth {
    pregame_by_centres: u32,
    panics: u32,
    digest_mismatches: u32,
}

/// The whole report, as `--json` prints it.
#[derive(Serialize)]
struct Report {
    maps: Vec<MapJudged>,
    panics: Vec<Panicked>,
    total: Total,
    seconds: f64,
}

/// One run to the end: the world, and its watch.
fn played(config: Config) -> (World, Watch) {
    let mut w = to_pick(config);
    w.act(Action::ChooseCiv { civ: None }).expect("watching is always allowed at the pick");
    let mut watch = Watch::new(&w);
    for _ in 0..AFTER {
        w.run(1);
        watch.turn(&w);
    }
    (w, watch)
}

/// The runs of `configs`, all at once, in order.
fn played_all(configs: &[Config]) -> Vec<(World, Watch)> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = configs.iter().map(|&c| scope.spawn(move || played(c))).collect();
        handles.into_iter().map(|h| h.join().unwrap_or_else(|e| std::panic::resume_unwind(e))).collect()
    })
}

/// A civ's Standing at +200, and its clans' and its land's parts of it, from a run's scores.
fn at_200(s: &Scores, civ: u32) -> (f64, f64, f64) {
    s.standing
        .civs
        .iter()
        .find(|c| c.id == civ)
        .map_or((0.0, 0.0, 0.0), |c| (c.at_200.unwrap_or(0.0), c.clans_200.unwrap_or(0.0), c.land_200.unwrap_or(0.0)))
}

/// Judges one map: the army run twice (their digests must agree), the `--no-army` control, the
/// no-claims control, the no-fight control and, with `solo`, each civ claiming alone (civ `k` in id order, `Only(k)`).
fn judge_map(name: &str, radius: i32, seed: u32, solo: bool) -> MapJudged {
    let base = Config { seed, radius, ..Config::default() };
    let configs = [
        base,
        base,
        Config { army: false, ..base },
        Config { claims: Claims::None, ..base },
        Config { fight: false, ..base },
    ];
    let mut runs = played_all(&configs).into_iter();
    let (w, watch) = runs.next().expect("the army run");
    let (again, _) = runs.next().expect("the army run again");
    let (control, _) = runs.next().expect("the control with no armies");
    let (none, none_watch) = runs.next().expect("the control with no claims");
    let (calm, calm_watch) = runs.next().expect("the control with no fighting");
    let civs: Vec<u32> = w.civs.keys().copied().collect();
    let kept = civs.iter().filter(|&&c| kept_camp(&w, c)).count() as u32;
    let people = |w: &World| stats::summary(w).people;
    let armies = Armies {
        camps_kept: kept,
        camps_of: civs.len() as u32,
        warriors_starved: civs.iter().map(|c| w.civs[c].army.starved).sum(),
        camps_lost: civs.iter().map(|c| w.civs[c].army.lost).sum(),
        people_at_200: people(&w),
        control_people_at_200: people(&control),
    };
    let health = Health {
        pregame_by_centres: w.end.as_ref().is_some_and(|e| !e.by_limit) as u32,
        digest_mismatches: (stats::digest(&w) != stats::digest(&again)) as u32,
    };
    let no_claims = none_watch.scores(&none, &civs);
    let solo = solo.then(|| {
        let configs: Vec<Config> = (0..civs.len() as u32).map(|k| Config { claims: Claims::Only(k), ..base }).collect();
        civs.iter()
            .zip(played_all(&configs))
            .map(|(&civ, (sw, swatch))| {
                let (g, cl, la) = at_200(&swatch.scores(&sw, &civs), civ);
                let (n, ncl, nla) = at_200(&no_claims, civ);
                SoloGain { civ, gain: g - n, clans: cl - ncl, land: la - nla }
            })
            .collect()
    });
    MapJudged {
        size: name.to_string(),
        seed,
        pick: watch.pick(),
        scores: watch.scores(&w, &civs),
        no_claims,
        no_fight: calm_watch.scores(&calm, &civs),
        solo,
        armies,
        health,
    }
}

fn message(e: Box<dyn Any + Send>) -> String {
    e.downcast_ref::<String>()
        .cloned()
        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "a panic with no message".to_string())
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

fn mean_some(v: &[Option<f64>]) -> Option<f64> {
    let v: Vec<f64> = v.iter().flatten().copied().collect();
    if v.is_empty() {
        None
    } else {
        Some(mean(&v))
    }
}

/// Mean claimed tiles, held tiles and guard people per civ over `civs`.
fn civ_means(civs: &[CivLand]) -> (f64, f64, f64) {
    let n = civs.len().max(1) as f64;
    let sum = |f: fn(&CivLand) -> f64| civs.iter().map(f).sum::<f64>() / n;
    (sum(|c| c.claimed as f64), sum(|c| c.held as f64), sum(|c| c.guard))
}

fn total_land(maps: &[MapJudged]) -> TotalLand {
    let pick = |f: fn(&Trails) -> Option<f64>| {
        let at_200: Vec<Option<f64>> = maps.iter().map(|m| m.scores.land.at_200.as_ref().and_then(f)).collect();
        let at_pick: Vec<Option<f64>> = maps.iter().map(|m| f(&m.scores.land.at_pick)).collect();
        (mean_some(&at_pick), mean_some(&at_200))
    };
    let clan_turns: u32 = maps.iter().map(|m| m.scores.land.clan_turns).sum();
    let changes: u32 = maps.iter().map(|m| m.scores.land.festival_changes).sum();
    let flips: u32 = maps.iter().map(|m| m.scores.land.flips).sum();
    let n = maps.len().max(1) as f64;
    let civs: Vec<CivLand> = maps.iter().flat_map(|m| m.scores.land.civs_200.clone()).collect();
    let mean_civs = |f: fn(&(f64, f64, f64)) -> f64| (!civs.is_empty()).then(|| f(&civ_means(&civs)));
    TotalLand {
        markets_pick: maps.iter().map(|m| m.scores.land.at_pick.markets).sum(),
        markets_200: maps.iter().filter_map(|m| m.scores.land.at_200.as_ref()).map(|t| t.markets).sum(),
        trading_pick: maps.iter().map(|m| m.scores.land.at_pick.trading).sum::<f64>() / n,
        trading_200: maps.iter().filter_map(|m| m.scores.land.at_200.as_ref()).map(|t| t.trading).sum::<f64>() / n,
        homes_trailed: pick(|t| t.homes_trailed),
        worked_trailed: pick(|t| t.worked_trailed),
        route_trailed: pick(|t| t.route_trailed),
        festival_changes_per_100: engine::judge::per_100(changes, clan_turns),
        flips_per_100: engine::judge::per_100(flips, clan_turns),
        claimed_200: mean_civs(|m| m.0),
        held_200: mean_civs(|m| m.1),
        guard_200: mean_civs(|m| m.2),
    }
}

fn total_solo(maps: &[MapJudged]) -> Option<TotalSolo> {
    if maps.iter().all(|m| m.solo.is_none()) {
        return None;
    }
    let gains: Vec<&SoloGain> = maps.iter().filter_map(|m| m.solo.as_ref()).flatten().collect();
    Some(TotalSolo {
        civs: gains.len(),
        gain: mean(&gains.iter().map(|g| g.gain).collect::<Vec<_>>()),
        clans: mean(&gains.iter().map(|g| g.clans).collect::<Vec<_>>()),
        land: mean(&gains.iter().map(|g| g.land).collect::<Vec<_>>()),
        above: gains.iter().filter(|g| g.gain > JUDGE.solo_above).count() as u32,
    })
}

/// The totals over the maps that judged (panicked maps are not in them).
fn total(maps: &[MapJudged], panics: usize) -> Total {
    let n = maps.len().max(1) as f64;
    let sum = |f: fn(&MapJudged) -> u32| maps.iter().map(f).sum::<u32>();
    let at = |f: fn(&MapJudged) -> Option<f64>| maps.iter().filter_map(f).collect::<Vec<f64>>();
    let mut by_kind: BTreeMap<String, f64> = BTreeMap::new();
    for m in maps {
        for (kind, &rate) in &m.scores.decisions.by_kind {
            *by_kind.entry(kind.clone()).or_insert(0.0) += rate / n;
        }
    }
    let people: f64 = maps.iter().map(|m| m.armies.people_at_200).sum();
    let control: f64 = maps.iter().map(|m| m.armies.control_people_at_200).sum();
    Total {
        standing: TotalStanding {
            top_200: mean(&at(|m| m.scores.standing.top_200)),
            aligned_200: mean(&at(|m| m.scores.standing.aligned_200)),
            on_land_200: mean(&at(|m| m.scores.standing.on_land_200)),
        },
        doubt: TotalDoubt {
            lead_changes: maps.iter().map(|m| m.scores.doubt.lead_changes).sum(),
            maps_with_lead_change: maps.iter().filter(|m| m.scores.doubt.lead_changes > 0).count() as u32,
            spread_100: mean(&at(|m| m.scores.doubt.spread_100)),
            spread_200: mean(&at(|m| m.scores.doubt.spread_200)),
            swings: maps.iter().map(|m| m.scores.doubt.swings).sum(),
            no_claims_lead_changes: maps.iter().map(|m| m.no_claims.doubt.lead_changes).sum(),
            no_claims_maps_with_lead_change: maps.iter().filter(|m| m.no_claims.doubt.lead_changes > 0).count() as u32,
        },
        decisions: TotalDecisions {
            ai_orders_per_100: mean(&maps.iter().map(|m| m.scores.decisions.orders_per_100).collect::<Vec<_>>()),
            by_kind,
            levers_repeated: mean(&maps.iter().map(|m| m.scores.decisions.levers_repeated).collect::<Vec<_>>()),
        },
        motion: TotalMotion {
            clans_changed: maps.iter().map(|m| m.scores.motion.clans_changed).sum(),
            people_changed: maps.iter().map(|m| m.scores.motion.people_changed).sum(),
            land_changed: maps.iter().map(|m| m.scores.motion.land_changed).sum(),
            clans_left: maps.iter().map(|m| m.scores.motion.clans_left).sum(),
            clans_joined: maps.iter().map(|m| m.scores.motion.clans_joined).sum(),
            claim_taken: maps.iter().map(|m| m.scores.motion.claim_taken).sum(),
        },
        land: total_land(maps),
        solo: total_solo(maps),
        armies: TotalArmies {
            camps_kept: sum(|m| m.armies.camps_kept),
            camps_of: sum(|m| m.armies.camps_of),
            warriors_starved: sum(|m| m.armies.warriors_starved),
            camps_lost: sum(|m| m.armies.camps_lost),
            people_vs_control: if control > 0.0 { people / control } else { 0.0 },
        },
        fight: TotalFight {
            fallen: maps.iter().map(|m| m.scores.fight.fallen).sum(),
            taken: sum(|m| m.scores.fight.taken),
            surrendered: sum(|m| m.scores.fight.surrendered),
            control_fallen: maps.iter().map(|m| m.no_fight.fight.fallen).sum(),
            control_taken: sum(|m| m.no_fight.fight.taken),
            control_surrendered: sum(|m| m.no_fight.fight.surrendered),
        },
        health: TotalHealth {
            pregame_by_centres: sum(|m| m.health.pregame_by_centres),
            panics: panics as u32,
            digest_mismatches: sum(|m| m.health.digest_mismatches),
        },
    }
}

/// The retro maps judged and the report printed.
pub(crate) fn judge(json: bool, solo: bool) -> Result<(), String> {
    let start = Instant::now();
    let maps = retro_maps()?;
    let results: Vec<Result<MapJudged, Panicked>> =
        std::thread::scope(|scope| {
            let handles: Vec<_> =
                maps.iter()
                    .map(|(name, radius, seed)| {
                        scope.spawn(move || {
                            catch_unwind(AssertUnwindSafe(|| judge_map(name, *radius, *seed, solo)))
                                .map_err(|e| Panicked { size: name.clone(), seed: *seed, message: message(e) })
                        })
                    })
                    .collect();
            handles.into_iter().map(|h| h.join().expect("a map's thread joins")).collect()
        });
    let mut judged = Vec::new();
    let mut panics = Vec::new();
    for r in results {
        match r {
            Ok(m) => judged.push(m),
            Err(p) => panics.push(p),
        }
    }
    let total = total(&judged, panics.len());
    let report = Report { maps: judged, panics, total, seconds: start.elapsed().as_secs_f64() };
    if json {
        println!("{}", serde_json::to_string_pretty(&report).expect("the report serialises"));
    } else {
        print_text(&report);
    }
    Ok(())
}

fn print_text(r: &Report) {
    for p in &r.panics {
        println!("PANIC {} seed {}: {}", p.size, p.seed, p.message);
    }
    for m in &r.maps {
        let s = &m.scores;
        let (a, h, l) = (&m.armies, &m.health, &s.land);
        let f = |v: Option<f64>| v.map_or("-".to_string(), |v| format!("{v:.2}"));
        println!(
            "{:<6} seed {:>3} pick {:>3}  standing +200 top {:.3} aligned {:.3}  lead changes {}  spread +100 {:.3} +200 {:.3} swings {}  orders/100 {:.2} levers {:.1}  motion: clans changed {} people {:.1} land {}  armies: camps kept {}/{} starved {} lost {} people vs control {:.2}  land: markets {}/{} trading {:.2}/{:.2} homes {}/{} worked {}/{} route {}/{} changes {:.2} flips {:.2}  fight (control): fallen {:.2} ({:.2}) taken {} ({}) surrendered {} ({})  health: centres {} digests differ {}",
            m.size,
            m.seed,
            m.pick,
            s.standing.top_200.unwrap_or(0.0),
            s.standing.aligned_200.unwrap_or(0.0),
            s.doubt.lead_changes,
            s.doubt.spread_100.unwrap_or(0.0),
            s.doubt.spread_200.unwrap_or(0.0),
            s.doubt.swings,
            s.decisions.orders_per_100,
            s.decisions.levers_repeated,
            s.motion.clans_changed,
            s.motion.people_changed,
            s.motion.land_changed,
            a.camps_kept,
            a.camps_of,
            a.warriors_starved,
            a.camps_lost,
            a.people_at_200 / a.control_people_at_200.max(f64::MIN_POSITIVE),
            l.at_pick.markets,
            l.at_200.as_ref().map_or(0, |t| t.markets),
            l.at_pick.trading,
            l.at_200.as_ref().map_or(0.0, |t| t.trading),
            f(l.at_pick.homes_trailed),
            f(l.at_200.as_ref().and_then(|t| t.homes_trailed)),
            f(l.at_pick.worked_trailed),
            f(l.at_200.as_ref().and_then(|t| t.worked_trailed)),
            f(l.at_pick.route_trailed),
            f(l.at_200.as_ref().and_then(|t| t.route_trailed)),
            l.festival_changes_per_100,
            l.flips_per_100,
            s.fight.fallen,
            m.no_fight.fight.fallen,
            s.fight.taken,
            m.no_fight.fight.taken,
            s.fight.surrendered,
            m.no_fight.fight.surrendered,
            h.pregame_by_centres,
            h.digest_mismatches,
        );
        let (claimed, held, guard) = civ_means(&l.civs_200);
        println!(
            "        claims: on land at +200 {:.3}; per civ at +200 claimed {claimed:.1}, held {held:.1}, guard {guard:.1}; no claims: lead changes {}, spread +200 {:.3}",
            s.standing.on_land_200.unwrap_or(0.0),
            m.no_claims.doubt.lead_changes,
            m.no_claims.doubt.spread_200.unwrap_or(0.0),
        );
        if let Some(gains) = &m.solo {
            let parts: Vec<String> = gains
                .iter()
                .map(|g| format!("civ {} {:+.4} (clans {:+.4}, land {:+.4})", g.civ, g.gain, g.clans, g.land))
                .collect();
            println!("        solo: {}", parts.join("; "));
        }
    }
    let t = &r.total;
    println!(
        "total standing: top at +200 {:.3}, aligned at +200 {:.3}, on land at +200 {:.3}",
        t.standing.top_200, t.standing.aligned_200, t.standing.on_land_200
    );
    println!(
        "total doubt: lead changes {} on {} maps, spread +100 {:.3} +200 {:.3}, swings {}; no claims: lead changes {} on {} maps",
        t.doubt.lead_changes,
        t.doubt.maps_with_lead_change,
        t.doubt.spread_100,
        t.doubt.spread_200,
        t.doubt.swings,
        t.doubt.no_claims_lead_changes,
        t.doubt.no_claims_maps_with_lead_change
    );
    let kinds: Vec<String> = t.decisions.by_kind.iter().map(|(k, v)| format!("{k} {v:.2}")).collect();
    println!(
        "total decisions: AI orders per 100 turns {:.2} ({}), levers repeated {:.2}",
        t.decisions.ai_orders_per_100,
        kinds.join(", "),
        t.decisions.levers_repeated
    );
    println!(
        "total motion: clans changed {}, people {:.1}, land tiles {}; clans left {}, joined {}; claimed tiles taken {}",
        t.motion.clans_changed,
        t.motion.people_changed,
        t.motion.land_changed,
        t.motion.clans_left,
        t.motion.clans_joined,
        t.motion.claim_taken
    );
    let l = &t.land;
    let f = |v: Option<f64>| v.map_or("-".to_string(), |v| format!("{v:.2}"));
    println!(
        "total land: markets {} at the pick, {} at +200; trading {:.2}/{:.2}; homes trailed {}/{}; worked trailed {}/{}; route trailed {}/{} (pick/+200); festival changes {:.2} and flips {:.2} per 100 clan-turns; per civ at +200 claimed {}, held {}, guard {}",
        l.markets_pick,
        l.markets_200,
        l.trading_pick,
        l.trading_200,
        f(l.homes_trailed.0),
        f(l.homes_trailed.1),
        f(l.worked_trailed.0),
        f(l.worked_trailed.1),
        f(l.route_trailed.0),
        f(l.route_trailed.1),
        l.festival_changes_per_100,
        l.flips_per_100,
        f(l.claimed_200),
        f(l.held_200),
        f(l.guard_200)
    );
    if let Some(s) = &t.solo {
        println!(
            "total solo: gain {:+.4} (clans {:+.4}, land {:+.4}) per civ over {} civs; above {} of {} civs by more than {}",
            s.gain, s.clans, s.land, s.civs, s.above, s.civs, JUDGE.solo_above
        );
    }
    let a = &t.armies;
    println!(
        "total armies: camps kept {} of {}, warriors starved {}, camps lost {}, people vs control {:.3}",
        a.camps_kept, a.camps_of, a.warriors_starved, a.camps_lost, a.people_vs_control
    );
    let fi = &t.fight;
    println!(
        "total fight (control, fighting off): people fallen {:.2} ({:.2}), tiles taken {} ({}), armies surrendered {} ({})",
        fi.fallen, fi.control_fallen, fi.taken, fi.control_taken, fi.surrendered, fi.control_surrendered
    );
    let h = &t.health;
    println!(
        "total health: pre-games by centres {} of {}, panics {}, digest mismatches {}",
        h.pregame_by_centres,
        r.maps.len() + r.panics.len(),
        h.panics,
        h.digest_mismatches
    );
    println!("seconds {:.1}", r.seconds);
}
