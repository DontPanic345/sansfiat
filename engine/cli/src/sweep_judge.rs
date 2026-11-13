//! `sweep --judge`: the retro maps played with the army twice, the `--no-army` control, the no-claims,
//! no-fight and no-fields controls and, with `--solo`, each civ claiming alone; every civ on the AI, scored by `engine::judge`:
//! a report, with no verdict.

use std::any::Any;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

use engine::data::{Claims, END};
use engine::judge::{CivLand, Scores, Watch};
use engine::rng;
use engine::{stats, Action, Config, Phase, World};
use serde::Serialize;

use crate::compare::flatten;
use crate::retro::{kept_camp, retro_maps, to_pick};
use crate::sweep_fields::{map_fields, total_fields, MapFields};
use crate::sweep_led::{map_led, total_led, LedGame};
use crate::sweep_text::print_text;
use crate::sweep_total::*;
use crate::sweep_towns::{map_towns, settle, total_towns, MapTowns};

/// The turns past the pick each map is run for, at least; a run goes on until it has ended.
const AFTER: u32 = 200;

/// The turns past the largest limit a run may go before it has ended: a game that does not end is a
/// failure, so the sweep panics (a `Panicked` map) rather than run on.
const BACKSTOP: u32 = 100;

/// The nulls `--floor` runs when `--nulls` is not given: the sweep once more with one harmless
/// difference at the pick.
pub(crate) const NULLS: u32 = 1;

/// One map's health: its pre-game and the digests of its two army runs. These are bugs whatever
/// the design says.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Health {
    pub(crate) pregame_by_centres: u32,
    pub(crate) digest_mismatches: u32,
}

/// One map's armies: its camps, what they cost their side, and the world's people beside the
/// `--no-army` control. These are measures, not bugs: the design decides what they should be.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Armies {
    pub(crate) camps_kept: u32,
    pub(crate) camps_of: u32,
    pub(crate) warriors_starved: u32,
    pub(crate) camps_lost: u32,
    pub(crate) people_at_200: f64,
    pub(crate) control_people_at_200: f64,
}

/// One civ's gain from claiming alone on a map (`--solo`): its Standing at +200 when only it claims,
/// less its Standing at +200 in the no-claims control. `clans` and `land` are the two parts of that
/// gain: its clans' and units' people, and the unaligned people on its land.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct SoloGain {
    pub(crate) civ: u32,
    pub(crate) gain: f64,
    pub(crate) clans: f64,
    pub(crate) land: f64,
}

/// One map that was judged.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct MapJudged {
    pub(crate) size: String,
    pub(crate) seed: u32,
    pub(crate) pick: u32,
    #[serde(flatten)]
    pub(crate) scores: Scores,
    /// The same map with no civ claiming: the control the claims are read against.
    pub(crate) no_claims: Scores,
    /// The same map with `Config.fight` off: the control the fight group is read against.
    pub(crate) no_fight: Scores,
    /// The same map with `Config.fields` off, and the run read against it (`sweep_fields`).
    pub(crate) no_fields: Scores,
    pub(crate) fields_vs_control: MapFields,
    /// The same map with `Config.towns` off, and the run read against it (`sweep_towns`).
    pub(crate) no_towns: Scores,
    pub(crate) towns_vs_control: MapTowns,
    /// Each civ led by the planner and led idle, to the game's end (`sweep_led`).
    pub(crate) led: Vec<LedGame>,
    /// Each civ's name, for the text (the ids are not the names' places in `CIVS`).
    #[serde(skip)]
    pub(crate) names: BTreeMap<u32, String>,
    /// With `--solo`: each civ's gain from claiming alone, in id order.
    pub(crate) solo: Option<Vec<SoloGain>>,
    pub(crate) armies: Armies,
    pub(crate) health: Health,
}

/// A map whose run panicked: its map and the message.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Panicked {
    pub(crate) size: String,
    pub(crate) seed: u32,
    pub(crate) message: String,
}

/// The whole report, as `--json` prints it. `floor` is `--floor`'s: per flattened path, the largest
/// change any null made to it.
#[derive(Serialize)]
pub(crate) struct Report {
    pub(crate) maps: Vec<MapJudged>,
    pub(crate) panics: Vec<Panicked>,
    pub(crate) total: Total,
    pub(crate) seconds: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) floor: Option<BTreeMap<String, f64>>,
}

/// One run to the end: the world, and its watch. It plays until the game is `Over` and at least
/// `AFTER` turns past the pick. `draws` extra rolls of the world's rng are taken after the pick (the
/// null perturbation; 0 is the base run).
fn played(config: Config, draws: u32) -> (World, Watch) {
    let mut w = to_pick(config);
    w.act(Action::ChooseCiv { civ: None }).expect("watching is always allowed at the pick");
    for _ in 0..draws {
        rng::rand(&mut w.rng);
    }
    let mut watch = Watch::new(&w);
    let backstop = END.limit.iter().map(|l| l.1).max().unwrap_or(0) + BACKSTOP;
    while watch.turns() < AFTER || !matches!(w.phase, Phase::Over(_)) {
        assert!(watch.turns() < backstop, "a game never ended: not over by {backstop} turns past the pick");
        w.run(1);
        watch.turn(&w);
    }
    (w, watch)
}

/// The runs of `configs`, all at once, in order, each with `draws` extra rolls.
fn played_all(configs: &[Config], draws: u32) -> Vec<(World, Watch)> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = configs.iter().map(|&c| scope.spawn(move || played(c, draws))).collect();
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
/// no-claims control, the no-fight control, the no-fields control and, with `solo`, each civ claiming alone (civ `k` in id
/// order, `Only(k)`). Every run takes `draws` extra rolls after the pick.
fn judge_map(name: &str, radius: i32, seed: u32, solo: bool, draws: u32) -> MapJudged {
    let base = Config { seed, radius, ..Config::default() };
    let configs = [
        base,
        base,
        Config { army: false, ..base },
        Config { claims: Claims::None, ..base },
        Config { fight: false, ..base },
        Config { fields: false, ..base },
        Config { towns: false, ..base },
    ];
    let mut runs = played_all(&configs, draws).into_iter();
    let (w, watch) = runs.next().expect("the army run");
    let (again, _) = runs.next().expect("the army run again");
    let (control, _) = runs.next().expect("the control with no armies");
    let (none, none_watch) = runs.next().expect("the control with no claims");
    let (calm, calm_watch) = runs.next().expect("the control with no fighting");
    let (bare, bare_watch) = runs.next().expect("the control with no fields");
    let (towns_off, towns_off_watch) = runs.next().expect("the control with no towns");
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
            .zip(played_all(&configs, draws))
            .map(|(&civ, (sw, swatch))| {
                let (g, cl, la) = at_200(&swatch.scores(&sw, &civs), civ);
                let (n, ncl, nla) = at_200(&no_claims, civ);
                SoloGain { civ, gain: g - n, clans: cl - ncl, land: la - nla }
            })
            .collect()
    });
    let (scores, no_fields) = (watch.scores(&w, &civs), bare_watch.scores(&bare, &civs));
    let no_towns = towns_off_watch.scores(&towns_off, &civs);
    let towns_vs_control = map_towns(&scores, &no_towns);
    MapJudged {
        led: map_led(base, &civs, draws),
        size: name.to_string(),
        seed,
        pick: watch.pick(),
        fields_vs_control: map_fields(&scores, &no_fields),
        scores,
        no_claims,
        no_fight: calm_watch.scores(&calm, &civs),
        no_fields,
        no_towns,
        towns_vs_control,
        names: w.civs.iter().map(|(&id, c)| (id, c.name.clone())).collect(),
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

pub(crate) fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

pub(crate) fn mean_some(v: &[Option<f64>]) -> Option<f64> {
    let v: Vec<f64> = v.iter().flatten().copied().collect();
    if v.is_empty() {
        None
    } else {
        Some(mean(&v))
    }
}

/// Mean claimed tiles per civ over `civs`.
pub(crate) fn civ_means(civs: &[CivLand]) -> f64 {
    civs.iter().map(|c| c.claimed as f64).sum::<f64>() / civs.len().max(1) as f64
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
        economy: total_economy(maps),
        supply: total_supply(maps),
        solo: total_solo(maps),
        end: total_end(maps),
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
        fields_vs_control: total_fields(maps),
        towns_vs_control: total_towns(maps),
        led: total_led(maps),
    }
}

/// Every retro map judged at `draws` (see `played`), its panics apart.
fn judged(maps: &[(String, i32, u32)], solo: bool, draws: u32) -> (Vec<MapJudged>, Vec<Panicked>) {
    let results: Vec<Result<MapJudged, Panicked>> = std::thread::scope(|scope| {
        let handles: Vec<_> = maps
            .iter()
            .map(|(name, radius, seed)| {
                scope.spawn(move || {
                    catch_unwind(AssertUnwindSafe(|| judge_map(name, *radius, *seed, solo, draws)))
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
    (judged, panics)
}

/// The retro maps judged and the report printed. With `floor`, the sweep runs that many more times,
/// each null one extra roll further on, and the report carries each path's largest change.
pub(crate) fn judge(json: bool, solo: bool, floor: Option<u32>) -> Result<(), String> {
    let start = Instant::now();
    let maps = retro_maps()?;
    let (judged_maps, panics) = judged(&maps, solo, 0);
    let totals = total(&judged_maps, panics.len());
    let mut report =
        Report { maps: judged_maps, panics, total: totals, seconds: start.elapsed().as_secs_f64(), floor: None };
    if let Some(nulls) = floor {
        let base = flatten(&serde_json::to_value(&report).expect("the report serialises"));
        let mut floors: BTreeMap<String, f64> = base.keys().map(|p| (p.clone(), 0.0)).collect();
        for k in 1..=nulls {
            let (m, p) = judged(&maps, solo, k);
            let t = total(&m, p.len());
            let null = Report { maps: m, panics: p, total: t, seconds: 0.0, floor: None };
            for (path, v) in flatten(&serde_json::to_value(&null).expect("the report serialises")) {
                if let (Some(f), Some(b)) = (floors.get_mut(&path), base.get(&path)) {
                    *f = f.max((v - b).abs());
                }
            }
        }
        report.floor = Some(floors);
    }
    settle(&mut report);
    if json {
        println!("{}", serde_json::to_string_pretty(&report).expect("the report serialises"));
    } else {
        print_text(&report);
    }
    Ok(())
}
