//! `sweep --judge`: the retro maps played with the army twice, the `--no-army` control, the no-claims
//! control and, with `--solo`, each civ claiming alone; every civ on the AI, scored by `engine::judge`:
//! a report, with no verdict.

use std::any::Any;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

use engine::data::{Claims, END, JUDGE};
use engine::judge::{CivLand, EndScores, Scores, Watch};
use engine::judge_ring::SupplyCiv;
use engine::rng;
use engine::stats::Trails;
use engine::{stats, Action, Config, Phase, World};
use serde::Serialize;

use crate::compare::flatten;
use crate::retro::{kept_camp, retro_maps, to_pick};
use crate::sweep_text::print_text;

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
/// gain: its clans' and units' people, and the unaligned people on its held land.
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

/// The sweep's totals, in groups.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Total {
    pub(crate) standing: TotalStanding,
    pub(crate) doubt: TotalDoubt,
    pub(crate) decisions: TotalDecisions,
    pub(crate) motion: TotalMotion,
    pub(crate) land: TotalLand,
    pub(crate) economy: TotalEconomy,
    pub(crate) supply: TotalSupply,
    pub(crate) solo: Option<TotalSolo>,
    pub(crate) armies: TotalArmies,
    pub(crate) fight: TotalFight,
    pub(crate) end: TotalEnd,
    pub(crate) health: TotalHealth,
}

/// The end group over the maps: how many games ended before the limit, the mean turns to the end and
/// the capitals taken after the pick, each beside the no-fight control's.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalEnd {
    pub(crate) early: u32,
    pub(crate) of: u32,
    pub(crate) no_fight_early: u32,
    pub(crate) turns_mean: f64,
    pub(crate) no_fight_turns_mean: f64,
    pub(crate) capitals_fallen: u32,
    pub(crate) no_fight_capitals_fallen: u32,
}

/// The economy group over the maps: means over the maps of each map's means, and the largest over the
/// maps of the two maxima.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalEconomy {
    pub(crate) store_mean: f64,
    pub(crate) store_max: f64,
    pub(crate) turns_at_pack: f64,
    pub(crate) levy_per_turn: f64,
    pub(crate) largest_army: f64,
    pub(crate) guard_mean: f64,
}

/// The fight group over the maps, beside the no-fight control's (which has none by construction).
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalFight {
    pub(crate) fallen: f64,
    pub(crate) taken: u32,
    pub(crate) surrendered: u32,
    pub(crate) control_fallen: f64,
    pub(crate) control_taken: u32,
    pub(crate) control_surrendered: u32,
}

/// The land group over the maps: festivals summed, shares averaged over the maps that have one, and
/// each civ's land at +200 (mean tiles and guard people per civ, over the maps).
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalLand {
    pub(crate) markets_pick: usize,
    pub(crate) markets_200: usize,
    pub(crate) trading_pick: f64,
    pub(crate) trading_200: f64,
    pub(crate) homes_trailed: (Option<f64>, Option<f64>),
    pub(crate) worked_trailed: (Option<f64>, Option<f64>),
    pub(crate) route_trailed: (Option<f64>, Option<f64>),
    pub(crate) festival_changes_per_100: f64,
    pub(crate) flips_per_100: f64,
    pub(crate) claimed_200: Option<f64>,
    pub(crate) held_200: Option<f64>,
    pub(crate) guard_200: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalStanding {
    pub(crate) top_200: f64,
    pub(crate) aligned_200: f64,
    pub(crate) on_land_200: f64,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalDoubt {
    pub(crate) lead_changes: u32,
    pub(crate) maps_with_lead_change: u32,
    pub(crate) spread_100: f64,
    pub(crate) spread_200: f64,
    pub(crate) swings: u32,
    pub(crate) no_claims_lead_changes: u32,
    pub(crate) no_claims_maps_with_lead_change: u32,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalDecisions {
    pub(crate) ai_orders_per_100: f64,
    pub(crate) by_kind: BTreeMap<String, f64>,
    pub(crate) levers_repeated: f64,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalMotion {
    pub(crate) clans_changed: u32,
    pub(crate) people_changed: f64,
    pub(crate) land_changed: u32,
    pub(crate) clans_left: u32,
    pub(crate) clans_joined: u32,
    pub(crate) claim_taken: u32,
}

/// The supply group over the maps: the ring and the units inside and outside it, summed over the maps,
/// and the mean territory of a clan at the pick and at +200 (mean over the maps).
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalSupply {
    pub(crate) counts: SupplyCiv,
    pub(crate) clan_territory_pick: f64,
    pub(crate) clan_territory_200: Option<f64>,
}

fn total_supply(maps: &[MapJudged]) -> TotalSupply {
    let mut counts = SupplyCiv::default();
    for m in maps {
        counts.add(&m.scores.supply.total);
    }
    TotalSupply {
        counts,
        clan_territory_pick: mean(&maps.iter().map(|m| m.scores.supply.clan_territory_pick).collect::<Vec<_>>()),
        clan_territory_200: mean_some(&maps.iter().map(|m| m.scores.supply.clan_territory_200).collect::<Vec<_>>()),
    }
}

fn total_economy(maps: &[MapJudged]) -> TotalEconomy {
    let of = |f: fn(&engine::judge::EconomyScores) -> f64| {
        mean(&maps.iter().map(|m| f(&m.scores.economy)).collect::<Vec<_>>())
    };
    let largest =
        |f: fn(&engine::judge::EconomyScores) -> f64| maps.iter().map(|m| f(&m.scores.economy)).fold(0.0, f64::max);
    TotalEconomy {
        store_mean: of(|e| e.store_mean),
        store_max: largest(|e| e.store_max),
        turns_at_pack: of(|e| f64::from(e.turns_at_pack)),
        levy_per_turn: of(|e| e.levy_per_turn),
        largest_army: largest(|e| e.largest_army),
        guard_mean: of(|e| e.guard_mean),
    }
}

/// The solo gains over every civ of every map: the mean gain and its two parts, and the civs whose
/// gain is above `JUDGE.solo_above`.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalSolo {
    pub(crate) civs: usize,
    pub(crate) gain: f64,
    pub(crate) clans: f64,
    pub(crate) land: f64,
    pub(crate) above: u32,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalArmies {
    pub(crate) camps_kept: u32,
    pub(crate) camps_of: u32,
    pub(crate) warriors_starved: u32,
    pub(crate) camps_lost: u32,
    pub(crate) people_vs_control: f64,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalHealth {
    pub(crate) pregame_by_centres: u32,
    pub(crate) panics: u32,
    pub(crate) digest_mismatches: u32,
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
/// no-claims control, the no-fight control and, with `solo`, each civ claiming alone (civ `k` in id
/// order, `Only(k)`). Every run takes `draws` extra rolls after the pick.
fn judge_map(name: &str, radius: i32, seed: u32, solo: bool, draws: u32) -> MapJudged {
    let base = Config { seed, radius, ..Config::default() };
    let configs = [
        base,
        base,
        Config { army: false, ..base },
        Config { claims: Claims::None, ..base },
        Config { fight: false, ..base },
    ];
    let mut runs = played_all(&configs, draws).into_iter();
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
            .zip(played_all(&configs, draws))
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
pub(crate) fn civ_means(civs: &[CivLand]) -> (f64, f64, f64) {
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

/// The end group over the maps: the games that ended before the limit, the mean turns to the end and
/// the capitals taken, each beside the no-fight control's.
fn total_end(maps: &[MapJudged]) -> TotalEnd {
    let run: Vec<&EndScores> = maps.iter().map(|m| &m.scores.end).collect();
    let calm: Vec<&EndScores> = maps.iter().map(|m| &m.no_fight.end).collect();
    let turns = |v: &[&EndScores]| {
        mean(&v.iter().filter_map(|e| e.over.as_ref()).map(|o| f64::from(o.turn)).collect::<Vec<_>>())
    };
    let early = |v: &[&EndScores]| v.iter().filter(|e| e.early).count() as u32;
    let capitals = |v: &[&EndScores]| v.iter().map(|e| e.capitals_fallen).sum();
    TotalEnd {
        early: early(&run),
        of: maps.len() as u32,
        no_fight_early: early(&calm),
        turns_mean: turns(&run),
        no_fight_turns_mean: turns(&calm),
        capitals_fallen: capitals(&run),
        no_fight_capitals_fallen: capitals(&calm),
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
    if json {
        println!("{}", serde_json::to_string_pretty(&report).expect("the report serialises"));
    } else {
        print_text(&report);
    }
    Ok(())
}
