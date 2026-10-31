//! `sweep --judge`: the retro maps played twice with the army and once without, each civ on the AI,
//! scored by `engine::judge` and read against the bands in `judge.toml`.

use std::any::Any;
use std::collections::BTreeMap;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::time::Instant;

use engine::data::JUDGE;
use engine::judge::{Scores, Watch};
use engine::world::SAVE_VERSION;
use engine::{stats, Action, Config, Save, World};
use serde::{Deserialize, Serialize};

use crate::play::{read_entries, Entry};
use crate::retro::{kept_camp, retro_maps, to_pick};
use crate::USAGE;

/// The turns past the pick each map is run for.
const AFTER: u32 = 200;

/// One map's health: its pre-game, its camps, and the digests of its two army runs.
#[derive(Clone, Debug, Serialize)]
struct Health {
    pregame_by_centres: u32,
    camps_kept: u32,
    camps_of: u32,
    warriors_starved: u32,
    camps_lost: u32,
    people_at_200: f64,
    control_people_at_200: f64,
    digest_mismatches: u32,
}

/// One map that was judged.
#[derive(Clone, Debug, Serialize)]
struct MapJudged {
    size: String,
    seed: u32,
    pick: u32,
    #[serde(flatten)]
    scores: Scores,
    health: Health,
}

/// A map whose run panicked: its map and the message.
#[derive(Clone, Debug, Serialize)]
struct Panicked {
    size: String,
    seed: u32,
    message: String,
}

/// The sweep's totals, in the five groups. Each name is a band's name: `group.metric`.
#[derive(Clone, Debug, Serialize)]
struct Total {
    standing: TotalStanding,
    doubt: TotalDoubt,
    decisions: TotalDecisions,
    motion: TotalMotion,
    health: TotalHealth,
}

#[derive(Clone, Debug, Serialize)]
struct TotalStanding {
    top_200: f64,
    aligned_200: f64,
}

#[derive(Clone, Debug, Serialize)]
struct TotalDoubt {
    lead_changes: u32,
    maps_with_lead_change: u32,
    spread_100: f64,
    spread_200: f64,
    swings: u32,
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
}

#[derive(Clone, Debug, Serialize)]
struct TotalHealth {
    pregame_by_centres: u32,
    camps_kept: u32,
    camps_of: u32,
    warriors_starved: u32,
    camps_lost: u32,
    people_vs_control: f64,
    panics: u32,
    digest_mismatches: u32,
}

/// One band of `judge.toml`: its bounds (inclusive) and what it protects.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Band {
    fail_min: Option<f64>,
    fail_max: Option<f64>,
    warn_min: Option<f64>,
    warn_max: Option<f64>,
    note: String,
}

/// A band's outcome against a metric.
#[derive(Clone, Debug, Serialize)]
struct Checked {
    name: String,
    value: Option<f64>,
    status: &'static str,
    bounds: String,
    note: String,
}

/// The whole report, as `--json` prints it.
#[derive(Serialize)]
struct Report {
    maps: Vec<MapJudged>,
    panics: Vec<Panicked>,
    total: Total,
    bands: Vec<Checked>,
    seconds: f64,
}

/// One run to the end: the world, and its watch.
fn played(radius: i32, seed: u32, army: bool) -> (World, Watch) {
    let mut w = to_pick(Config { seed, radius, army, gifts: true, zone: true });
    w.act(Action::ChooseCiv { civ: None }).expect("watching is always allowed at the pick");
    let mut watch = Watch::new(&w);
    for _ in 0..AFTER {
        w.run(1);
        watch.turn(&w);
    }
    (w, watch)
}

/// Judges one map: the army run twice (their digests must agree) and the control once.
fn judge_map(name: &str, radius: i32, seed: u32) -> MapJudged {
    let (w, watch) = played(radius, seed, true);
    let (again, _) = played(radius, seed, true);
    let (control, _) = played(radius, seed, false);
    let civs: Vec<u32> = w.civs.keys().copied().collect();
    let kept = civs.iter().filter(|&&c| kept_camp(&w, c)).count() as u32;
    let people = |w: &World| stats::summary(w).people;
    let health = Health {
        pregame_by_centres: w.end.as_ref().is_some_and(|e| !e.by_limit) as u32,
        camps_kept: kept,
        camps_of: civs.len() as u32,
        warriors_starved: civs.iter().map(|c| w.civs[c].army.starved).sum(),
        camps_lost: civs.iter().map(|c| w.civs[c].army.lost).sum(),
        people_at_200: people(&w),
        control_people_at_200: people(&control),
        digest_mismatches: (stats::digest(&w) != stats::digest(&again)) as u32,
    };
    MapJudged { size: name.to_string(), seed, pick: watch.pick(), scores: watch.scores(&w, &civs), health }
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
    let people: f64 = maps.iter().map(|m| m.health.people_at_200).sum();
    let control: f64 = maps.iter().map(|m| m.health.control_people_at_200).sum();
    Total {
        standing: TotalStanding {
            top_200: mean(&at(|m| m.scores.standing.top_200)),
            aligned_200: mean(&at(|m| m.scores.standing.aligned_200)),
        },
        doubt: TotalDoubt {
            lead_changes: maps.iter().map(|m| m.scores.doubt.lead_changes).sum(),
            maps_with_lead_change: maps.iter().filter(|m| m.scores.doubt.lead_changes > 0).count() as u32,
            spread_100: mean(&at(|m| m.scores.doubt.spread_100)),
            spread_200: mean(&at(|m| m.scores.doubt.spread_200)),
            swings: maps.iter().map(|m| m.scores.doubt.swings).sum(),
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
        },
        health: TotalHealth {
            pregame_by_centres: sum(|m| m.health.pregame_by_centres),
            camps_kept: sum(|m| m.health.camps_kept),
            camps_of: sum(|m| m.health.camps_of),
            warriors_starved: sum(|m| m.health.warriors_starved),
            camps_lost: sum(|m| m.health.camps_lost),
            people_vs_control: if control > 0.0 { people / control } else { 0.0 },
            panics: panics as u32,
            digest_mismatches: sum(|m| m.health.digest_mismatches),
        },
    }
}

/// Every number of the total under its `group.metric` name.
fn metrics(total: &Total) -> BTreeMap<String, f64> {
    fn flat(prefix: &str, v: &serde_json::Value, out: &mut BTreeMap<String, f64>) {
        match v {
            serde_json::Value::Object(m) => {
                for (k, x) in m {
                    let name = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    flat(&name, x, out);
                }
            }
            serde_json::Value::Number(n) => {
                out.insert(prefix.to_string(), n.as_f64().unwrap_or(0.0));
            }
            _ => {}
        }
    }
    let mut out = BTreeMap::new();
    flat("", &serde_json::to_value(total).expect("the total serialises"), &mut out);
    out
}

/// Reads `judge.toml`: a table per group, a table per metric, each a `Band`. Every band needs a
/// bound, and names are `group.metric`.
fn bands(text: &str) -> Result<BTreeMap<String, Band>, String> {
    let groups: BTreeMap<String, BTreeMap<String, Band>> = toml::from_str(text).map_err(|e| e.to_string())?;
    let mut out = BTreeMap::new();
    for (group, metrics) in groups {
        for (metric, band) in metrics {
            let bounded = [band.fail_min, band.fail_max, band.warn_min, band.warn_max].iter().any(Option::is_some);
            if !bounded {
                return Err(format!("band {group}.{metric} has no bound"));
            }
            out.insert(format!("{group}.{metric}"), band);
        }
    }
    Ok(out)
}

/// The band's outcome for `value`: `fail` past a fail bound, else `warn` past a warn bound, else `ok`.
fn check(name: &str, band: &Band, value: Option<f64>) -> Checked {
    let bounds: Vec<String> = [
        band.fail_min.map(|b| format!("fail_min {b}")),
        band.fail_max.map(|b| format!("fail_max {b}")),
        band.warn_min.map(|b| format!("warn_min {b}")),
        band.warn_max.map(|b| format!("warn_max {b}")),
    ]
    .into_iter()
    .flatten()
    .collect();
    let Some(v) = value else {
        return Checked {
            name: name.into(),
            value,
            status: "fail",
            bounds: "no such metric".into(),
            note: band.note.clone(),
        };
    };
    let outside = [
        (true, band.fail_min.filter(|&b| v < b).map(|b| format!("below {b}"))),
        (true, band.fail_max.filter(|&b| v > b).map(|b| format!("above {b}"))),
        (false, band.warn_min.filter(|&b| v < b).map(|b| format!("below {b}"))),
        (false, band.warn_max.filter(|&b| v > b).map(|b| format!("above {b}"))),
    ]
    .into_iter()
    .find_map(|(fail, why)| why.map(|w| (fail, w)));
    let (status, bound) = match outside {
        Some((true, why)) => ("fail", why),
        Some((false, why)) => ("warn", why),
        None => ("ok", bounds.join(", ")),
    };
    Checked { name: name.into(), value, status, bounds: bound, note: band.note.clone() }
}

/// The retro maps judged, the bands checked, and the report printed. Exits 1 on a failing band.
pub(crate) fn judge(bands_file: Option<&str>, json: bool) -> Result<(), String> {
    let path = bands_file.unwrap_or("judge.toml");
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let bands = bands(&text).map_err(|e| format!("{path}: {e}"))?;
    let start = Instant::now();
    let maps = retro_maps()?;
    let results: Vec<Result<MapJudged, Panicked>> = std::thread::scope(|scope| {
        let handles: Vec<_> = maps
            .iter()
            .map(|(name, radius, seed)| {
                scope.spawn(move || {
                    catch_unwind(AssertUnwindSafe(|| judge_map(name, *radius, *seed))).map_err(|e| Panicked {
                        size: name.clone(),
                        seed: *seed,
                        message: message(e),
                    })
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
    let metrics = metrics(&total);
    let checked: Vec<Checked> =
        bands.iter().map(|(name, band)| check(name, band, metrics.get(name).copied())).collect();
    let report = Report { maps: judged, panics, total, bands: checked, seconds: start.elapsed().as_secs_f64() };
    if json {
        println!("{}", serde_json::to_string_pretty(&report).expect("the report serialises"));
    } else {
        print_text(&report);
    }
    if report.bands.iter().any(|b| b.status == "fail") {
        std::process::exit(1);
    }
    Ok(())
}

fn print_text(r: &Report) {
    for p in &r.panics {
        println!("PANIC {} seed {}: {}", p.size, p.seed, p.message);
    }
    for m in &r.maps {
        let s = &m.scores;
        let h = &m.health;
        println!(
            "{:<6} seed {:>3} pick {:>3}  standing +200 top {:.3} aligned {:.3}  lead changes {}  spread +100 {:.3} +200 {:.3} swings {}  orders/100 {:.2} levers {:.1}  motion: clans changed {} people {:.1} land {}  centres {} camps kept {}/{} starved {} lost {} people vs control {:.2} digests differ {}",
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
            h.pregame_by_centres,
            h.camps_kept,
            h.camps_of,
            h.warriors_starved,
            h.camps_lost,
            h.people_at_200 / h.control_people_at_200.max(f64::MIN_POSITIVE),
            h.digest_mismatches,
        );
    }
    let t = &r.total;
    println!("total standing: top at +200 {:.3}, aligned at +200 {:.3}", t.standing.top_200, t.standing.aligned_200);
    println!(
        "total doubt: lead changes {} on {} maps, spread +100 {:.3} +200 {:.3}, swings {}",
        t.doubt.lead_changes, t.doubt.maps_with_lead_change, t.doubt.spread_100, t.doubt.spread_200, t.doubt.swings
    );
    let kinds: Vec<String> = t.decisions.by_kind.iter().map(|(k, v)| format!("{k} {v:.2}")).collect();
    println!(
        "total decisions: AI orders per 100 turns {:.2} ({}), levers repeated {:.2}",
        t.decisions.ai_orders_per_100,
        kinds.join(", "),
        t.decisions.levers_repeated
    );
    println!(
        "total motion: clans changed {}, people {:.1}, land tiles {}",
        t.motion.clans_changed, t.motion.people_changed, t.motion.land_changed
    );
    let h = &t.health;
    println!(
        "total health: pre-games by centres {} of {}, camps kept {} of {}, warriors starved {}, camps lost {}, people vs control {:.3}, panics {}, digest mismatches {}",
        h.pregame_by_centres,
        r.maps.len() + r.panics.len(),
        h.camps_kept,
        h.camps_of,
        h.warriors_starved,
        h.camps_lost,
        h.people_vs_control,
        h.panics,
        h.digest_mismatches
    );
    for b in &r.bands {
        let value = b.value.map_or("missing".to_string(), |v| format!("{v}"));
        match b.status {
            "ok" => println!("ok {} {value} ({})", b.name, b.bounds),
            status => println!("{} {} {value} ({}) — {}", status.to_uppercase(), b.name, b.bounds, b.note),
        }
    }
    println!("seconds {:.1}", r.seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn band(fail: (Option<f64>, Option<f64>), warn: (Option<f64>, Option<f64>)) -> Band {
        Band { fail_min: fail.0, fail_max: fail.1, warn_min: warn.0, warn_max: warn.1, note: "n".into() }
    }

    #[test]
    fn a_value_is_ok_warned_or_failed_by_its_bounds() {
        let high = band((None, Some(2.0)), (None, Some(1.0)));
        assert_eq!(check("x.y", &high, Some(0.5)).status, "ok");
        assert_eq!(check("x.y", &high, Some(1.5)).status, "warn");
        assert_eq!(check("x.y", &high, Some(2.5)).status, "fail");
        let low = band((Some(6.0), None), (Some(7.0), None));
        assert_eq!(check("x.y", &low, Some(7.5)).status, "ok");
        assert_eq!(check("x.y", &low, Some(6.5)).status, "warn");
        assert_eq!(check("x.y", &low, Some(5.0)).status, "fail");
    }

    #[test]
    fn a_band_on_a_metric_that_is_not_there_fails() {
        let b = band((None, Some(0.0)), (None, None));
        assert_eq!(check("nope.none", &b, None).status, "fail");
    }

    #[test]
    fn bands_parse_by_group_and_metric_and_need_a_bound() {
        let ok = bands("[health.panics]\nfail_max = 0\nnote = \"a panic\"\n").unwrap();
        assert!(ok.contains_key("health.panics"));
        assert!(bands("[health.panics]\nnote = \"none\"\n").is_err());
        assert!(bands("[health.panics]\nfail_max = 0\nnote = \"a\"\nfail_mx = 1\n").is_err());
    }

    #[test]
    fn the_metrics_are_the_totals_numbers_under_their_names() {
        let t = total(&[], 0);
        let m = metrics(&t);
        assert_eq!(m.get("health.panics"), Some(&0.0));
        assert_eq!(m.get("doubt.spread_100"), Some(&0.0));
    }
}

/// A played game read back: the led civ's Standing beside the control's, the scores of both, and the
/// player's own decisions from the log and the lines file.
#[derive(Serialize)]
struct GameJudged {
    pick: u32,
    turns: u32,
    led: u32,
    game: Scores,
    control: Scores,
    standing: Ending,
    player: Player,
}

/// The led civ's Standing and rank at the end, in the game and in the control.
#[derive(Serialize)]
struct Ending {
    game: f64,
    game_rank: usize,
    control: f64,
    control_rank: usize,
}

/// What the player did: accepted orders and actions by kind (from the log), how often a turn had
/// none, and the refusals and notes of the lines file.
#[derive(Serialize)]
struct Player {
    orders_by_kind: BTreeMap<String, u32>,
    orders_per_100: f64,
    idle_turns: u32,
    longest_idle: u32,
    refused: Vec<Entry>,
    noted: usize,
}

/// The place of civ `led` in Standing `s`: 1 for the largest share.
fn rank_of(s: &BTreeMap<u32, f64>, led: u32) -> usize {
    let mine = s.get(&led).copied().unwrap_or(0.0);
    1 + s.values().filter(|&&v| v > mine).count()
}

/// `judge FILE [--json]`: replays a played game a turn at a time under a `Watch`, plays the same map
/// with every civ on the AI to the same turn, and reads both in the judge's scores.
pub(crate) fn judge_game(args: &[String]) -> Result<(), String> {
    let mut file = None;
    let mut json = false;
    for a in args {
        match a.as_str() {
            "--json" => json = true,
            f if file.is_none() && !f.starts_with("--") => file = Some(f.to_string()),
            _ => return Err(format!("judge takes a game file and --json\n{USAGE}")),
        }
    }
    let file = file.ok_or(format!("judge needs a game file\n{USAGE}"))?;
    let path = Path::new(&file);
    let text = fs::read_to_string(path).map_err(|e| format!("{file}: {e}"))?;
    let save: Save = serde_json::from_str(&text).map_err(|e| format!("{file}: {e}"))?;
    if save.version != SAVE_VERSION {
        return Err(format!("{file}: save version {} is not {SAVE_VERSION}", save.version));
    }

    // The game, action by action: its watch starts at the pick and is turned after each end.
    let mut w = World::new(save.config);
    let mut watch: Option<Watch> = None;
    let (mut idle, mut longest, mut run, mut acted) = (0u32, 0u32, 0u32, false);
    let mut by_kind: BTreeMap<String, u32> = BTreeMap::new();
    for a in &save.log {
        w.act(a.clone()).map_err(|e| format!("{file}: {e}"))?;
        match a {
            Action::ChooseCiv { .. } => watch = Some(Watch::new(&w)),
            Action::EndTurn => {
                let Some(wt) = watch.as_mut() else { continue };
                wt.turn(&w);
                if acted {
                    run = 0;
                } else {
                    idle += 1;
                    run += 1;
                    longest = longest.max(run);
                }
                acted = false;
            }
            Action::Order { order, .. } => {
                acted = true;
                *by_kind.entry(order.kind().to_string()).or_insert(0) += 1;
            }
            Action::SetKeep { .. } | Action::SetGift { .. } => {
                acted = true;
                let kind = if matches!(a, Action::SetKeep { .. }) { "setKeep" } else { "setGift" };
                *by_kind.entry(kind.to_string()).or_insert(0) += 1;
            }
        }
    }
    let wt = watch.ok_or(format!("{file}: the game has no pick"))?;
    let led = w.player.ok_or(format!("{file}: the game leads no civ"))?;
    let n = wt.turns();

    // The control: the same config to the same pick, every civ on the AI, the same number of turns.
    let mut control = to_pick(save.config);
    control.act(Action::ChooseCiv { civ: None })?;
    let mut cwatch = Watch::new(&control);
    if cwatch.pick() != wt.pick() {
        return Err(format!("{file}: the control's pick is {}, the game's {}", cwatch.pick(), wt.pick()));
    }
    for _ in 0..n {
        control.act(Action::EndTurn)?;
        cwatch.turn(&control);
    }

    let game_scores = wt.scores(&w, &[led]);
    let control_scores = cwatch.scores(&control, &[led]);
    let g_end = wt.standing_at(n).expect("scored to the end");
    let c_end = cwatch.standing_at(n).expect("scored to the end");
    let entries = read_entries(path)?;
    let refused: Vec<Entry> = entries.iter().filter(|e| e.result.starts_with("refused")).cloned().collect();
    let noted = entries.iter().filter(|e| e.result == "noted").count();
    let player_orders: u32 = by_kind.values().sum();
    let judged = GameJudged {
        pick: wt.pick(),
        turns: n,
        led,
        game: game_scores,
        control: control_scores,
        standing: Ending {
            game: g_end.get(&led).copied().unwrap_or(0.0),
            game_rank: rank_of(g_end, led),
            control: c_end.get(&led).copied().unwrap_or(0.0),
            control_rank: rank_of(c_end, led),
        },
        player: Player {
            orders_per_100: if n > 0 { f64::from(player_orders) * 100.0 / f64::from(n) } else { 0.0 },
            orders_by_kind: by_kind,
            idle_turns: idle,
            longest_idle: longest,
            refused,
            noted,
        },
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&judged).expect("the judgement serialises"));
    } else {
        print_game(&judged, &wt, &cwatch);
    }
    Ok(())
}

/// The judgement as text: the score line every `JUDGE.sample` turns, the led civ at the end beside
/// the control, the doubt and motion of both, the player's decisions and refusals.
fn print_game(j: &GameJudged, game: &Watch, control: &Watch) {
    let shares = |s: &BTreeMap<u32, f64>| -> String {
        s.iter().map(|(id, v)| format!("civ {id} {:.1}%", v * 100.0)).collect::<Vec<_>>().join(", ")
    };
    for k in (0..=j.turns).step_by(JUDGE.sample as usize) {
        let s = game.standing_at(k).expect("scored to the end");
        let leader = game.leader_at(k).map_or("none".to_string(), |l| format!("civ {l}"));
        println!("turn {} (+{k}): {}; leader {leader}", j.pick + k, shares(s));
    }
    let s = &j.standing;
    println!(
        "led civ {}: Standing at +{} {:.1}% (rank {}) in the game, {:.1}% (rank {}) with every civ on the AI",
        j.led,
        j.turns,
        s.game * 100.0,
        s.game_rank,
        s.control * 100.0,
        s.control_rank
    );
    let (g, c) = (&j.game.doubt, &j.control.doubt);
    println!(
        "doubt (game / control): lead changes {} / {}, spread +{} {:.3} / {:.3}, swings {} / {}",
        g.lead_changes,
        c.lead_changes,
        j.turns,
        game.spread_at(j.turns).unwrap_or(0.0),
        control.spread_at(j.turns).unwrap_or(0.0),
        g.swings,
        c.swings
    );
    let (gm, cm) = (&j.game.motion, &j.control.motion);
    println!(
        "motion (game / control): clans changed {} / {}, people {:.1} / {:.1}, land tiles {} / {}",
        gm.clans_changed, cm.clans_changed, gm.people_changed, cm.people_changed, gm.land_changed, cm.land_changed
    );
    let p = &j.player;
    let kinds: Vec<String> = p.orders_by_kind.iter().map(|(k, v)| format!("{k} {v}")).collect();
    println!(
        "player orders: {} ({}), {:.2} per 100 turns; turns with none {} of {}, longest run {}",
        p.orders_by_kind.values().sum::<u32>(),
        if kinds.is_empty() { "none".to_string() } else { kinds.join(", ") },
        p.orders_per_100,
        p.idle_turns,
        j.turns,
        p.longest_idle
    );
    println!("player lines: {} refused, {} noted", p.refused.len(), p.noted);
    for r in &p.refused {
        println!("  refused at turn {}: {} ({})", r.turn, r.line, r.result);
    }
}
