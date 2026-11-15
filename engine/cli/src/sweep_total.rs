//! The sweep's totals over the judged maps, in the groups `sweep --judge` reports: the `Total` structs and
//! the `total_*` functions that fill them. `sweep_judge` builds the report from them.

use std::collections::BTreeMap;

use engine::data::JUDGE;
use engine::judge::{CivLand, EndScores};
use engine::judge_ring::SupplyCiv;
use engine::stats::Trails;
use serde::Serialize;

use crate::sweep_fields::TotalFields;
use crate::sweep_judge::{civ_means, mean, mean_some, MapJudged, SoloGain};
use crate::sweep_towns::TotalTowns;

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
    pub(crate) fields_vs_control: TotalFields,
    pub(crate) towns_vs_control: TotalTowns,
    pub(crate) led: crate::sweep_led::TotalLed,
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
/// each civ's land at +200 (mean tiles per civ, over the maps).
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

pub(crate) fn total_supply(maps: &[MapJudged]) -> TotalSupply {
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

pub(crate) fn total_economy(maps: &[MapJudged]) -> TotalEconomy {
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

pub(crate) fn total_land(maps: &[MapJudged]) -> TotalLand {
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
    let claimed_200 = (!civs.is_empty()).then(|| civ_means(&civs));
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
        claimed_200,
    }
}

/// The end group over the maps: the games that ended before the limit, the mean turns to the end and
/// the capitals taken, each beside the no-fight control's.
pub(crate) fn total_end(maps: &[MapJudged]) -> TotalEnd {
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

pub(crate) fn total_solo(maps: &[MapJudged]) -> Option<TotalSolo> {
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
