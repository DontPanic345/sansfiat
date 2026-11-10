//! Towns in the sweep: each map's run read against its no-towns control (`Config.towns` off), and the
//! seven maps' totals with every bound of `spec.md`'s towns table, each marked pass or miss. The
//! control's towns are the same map with no home shared, so its towns are none; the run's are read
//! as they stand. Build-3 shares no home, so every map reads no towns, and the bounds that need one miss.

use engine::judge::{per_100, Scores};
use engine::judge_towns::{TownCiv, TownGroup, TownMoves};
use serde::Serialize;

use crate::sweep_fields::map_fields;
use crate::sweep_judge::{MapJudged, Report};

/// One map's towns: the run's totals over the clan-turns after +100 and at +200, its home moves over
/// every turn past the pick, and the control's clans' walk, trade and starving.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct MapTowns {
    pub(crate) run: TownCiv,
    pub(crate) moves: TownMoves,
    pub(crate) control_all: TownGroup,
    pub(crate) formed: u32,
    pub(crate) first_town: Option<u32>,
    pub(crate) most_homes_200: u32,
    /// The control's and the run's top farmer at a pack (`sweep_fields`), and the control's farmers.
    pub(crate) top_farmer_at_pack: u32,
    pub(crate) control_top_farmer_at_pack: u32,
    pub(crate) control_civs_never_at_pack: u32,
}

/// One bound of `spec.md`'s towns table: its name, the value and the bar, and whether it holds.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Bound {
    pub(crate) name: String,
    pub(crate) detail: String,
    pub(crate) pass: bool,
}

/// The seven maps' towns: the sums and rates over the maps, the control's beside them, and the bounds.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct TotalTowns {
    pub(crate) maps_with_town: u32,
    pub(crate) run: TownCiv,
    pub(crate) moves: TownMoves,
    pub(crate) control_all: TownGroup,
    pub(crate) formed: u32,
    pub(crate) most_homes_200: u32,
    pub(crate) gatherer_home_moves_per_100: f64,
    pub(crate) top_farmer_at_pack: u32,
    pub(crate) control_top_farmer_at_pack: u32,
    pub(crate) civs_never_at_pack: u32,
    pub(crate) control_civs_never_at_pack: u32,
    pub(crate) clans_changed: u32,
    pub(crate) control_clans_changed: u32,
    pub(crate) people_changed: f64,
    pub(crate) control_people_changed: f64,
    pub(crate) land_changed: u32,
    pub(crate) control_land_changed: u32,
    pub(crate) aligned_200: f64,
    pub(crate) control_aligned_200: f64,
    /// Each bound, set by `settle` once the report's floors are known.
    pub(crate) bounds: Vec<Bound>,
}

/// One map's towns read against its control.
pub(crate) fn map_towns(run: &Scores, control: &Scores) -> MapTowns {
    let top = |s: &Scores| map_fields(s, s).top_farmer_at_pack;
    MapTowns {
        run: run.towns.total.clone(),
        moves: run.towns.moves.clone(),
        control_all: control.towns.total.all.clone(),
        formed: run.towns.formed,
        first_town: run.towns.first_town,
        most_homes_200: run.towns.most_homes_200,
        top_farmer_at_pack: map_fields(run, control).top_farmer_at_pack,
        control_top_farmer_at_pack: top(control),
        control_civs_never_at_pack: control.fields.civs_never_at_pack,
    }
}

/// The towns totals over the judged maps. `sum` reads each map's run or control.
pub(crate) fn total_towns(maps: &[MapJudged]) -> TotalTowns {
    let mut t = TotalTowns::default();
    for m in maps {
        let mt = &m.towns_vs_control;
        t.maps_with_town += (mt.run.towns_200 > 0) as u32;
        t.run.add(&mt.run);
        t.moves.settled_turns += mt.moves.settled_turns;
        t.moves.settled_town_steps += mt.moves.settled_town_steps;
        t.moves.settled_other_moves += mt.moves.settled_other_moves;
        t.moves.town_turns += mt.moves.town_turns;
        t.moves.town_moves_off += mt.moves.town_moves_off;
        t.control_all.clan_turns += mt.control_all.clan_turns;
        t.control_all.traded += mt.control_all.traded;
        t.control_all.starving += mt.control_all.starving;
        t.control_all.walk += mt.control_all.walk;
        t.formed += mt.formed;
        t.most_homes_200 = t.most_homes_200.max(mt.most_homes_200);
        t.top_farmer_at_pack += mt.top_farmer_at_pack;
        t.control_top_farmer_at_pack += mt.control_top_farmer_at_pack;
        t.civs_never_at_pack += m.scores.fields.civs_never_at_pack;
        t.control_civs_never_at_pack += mt.control_civs_never_at_pack;
        let (r, c) = (&m.scores.motion, &m.no_towns.motion);
        t.clans_changed += r.clans_changed;
        t.control_clans_changed += c.clans_changed;
        t.people_changed += r.people_changed;
        t.control_people_changed += c.people_changed;
        t.land_changed += r.land_changed;
        t.control_land_changed += c.land_changed;
        t.aligned_200 += m.scores.standing.aligned_200.unwrap_or(0.0) / maps.len().max(1) as f64;
        t.control_aligned_200 += m.no_towns.standing.aligned_200.unwrap_or(0.0) / maps.len().max(1) as f64;
    }
    t.run.share();
    t.control_all.share();
    let gatherers: u32 = maps.iter().map(|m| m.scores.fields.total.gatherer_turns).sum();
    let moves: u32 = maps.iter().map(|m| m.scores.fields.total.gatherer_home_moves).sum();
    t.gatherer_home_moves_per_100 = per_100(moves, gatherers);
    t
}

/// The bounds of the towns table, each with its detail. `floor` is the floor of a path of the
/// report's `total.towns_vs_control` block, 0 without a floor.
pub(crate) fn bounds(t: &TotalTowns, floor: &dyn Fn(&str) -> f64) -> Vec<Bound> {
    let run = &t.run;
    let (town, out, all) = (&run.town, &run.farmers_outside, &run.all);
    let mut b = Vec::new();
    let mut add = |name: &str, detail: String, pass: bool| b.push(Bound { name: name.to_string(), detail, pass });

    add(
        "towns form",
        format!("{} towns at +200 over the seven (≥ 10), on {} of 7 maps (≥ 5)", run.towns_200, t.maps_with_town),
        run.towns_200 >= 10 && t.maps_with_town >= 5,
    );
    add(
        "farmers meet the festival",
        format!(
            "town clans at +200 {}, of them farmers {} (≥ ¾), towns working fields {} of {}",
            run.town_clans_200, run.town_farmers_200, run.towns_fields_200, run.towns_200
        ),
        run.town_clans_200 > 0
            && 4 * run.town_farmers_200 >= 3 * run.town_clans_200
            && run.towns_fields_200 == run.towns_200,
    );
    add(
        "town clans walk less",
        format!("walk per turn {:.3} vs farmers outside {:.3} (under half)", town.walk_per_turn, out.walk_per_turn),
        town.clan_turns > 0 && town.walk_per_turn < 0.5 * out.walk_per_turn,
    );
    let (trade_t, trade_o) = (town.traded_per_turn, out.traded_per_turn);
    add(
        "town clans trade more",
        format!("traded per person a turn {trade_t:.4} vs farmers outside {trade_o:.4} (≥ 1.1 times)"),
        town.clan_turns > 0 && out.clan_turns > 0 && trade_t >= 1.1 * trade_o,
    );
    let fl = floor("run.all.traded_per_turn");
    add(
        "traded over all clans",
        format!(
            "{:.4} vs the control's {:.4}, less the floor {fl:.4}",
            all.traded_per_turn, t.control_all.traded_per_turn
        ),
        all.traded_per_turn >= t.control_all.traded_per_turn - fl,
    );
    let hands = |name: &str, v: f64, c: f64| -> (String, bool) {
        let f = floor(name);
        (format!("{v:.1} vs {c:.1}, less the floor {f:.1}"), v >= c - f)
    };
    let (d, ok) = hands("clans_changed", f64::from(t.clans_changed), f64::from(t.control_clans_changed));
    add("clans change hands", d, ok);
    let (d, ok) = hands("people_changed", t.people_changed, t.control_people_changed);
    add("people change hands", d, ok);
    let (d, ok) = hands("land_changed", f64::from(t.land_changed), f64::from(t.control_land_changed));
    add("land changes hands", d, ok);
    let settled_other = per_100(t.moves.settled_other_moves, t.moves.settled_turns);
    let off = per_100(t.moves.town_moves_off, t.moves.town_turns);
    let g = t.gatherer_home_moves_per_100;
    add(
        "settled farmers stay",
        format!(
            "settled home moves less town steps {settled_other:.3} per 100 (town steps {:.3}); town clans' moves off {off:.3}; gatherers {g:.3} (each under half)",
            per_100(t.moves.settled_town_steps, t.moves.settled_turns)
        ),
        t.moves.settled_turns > 0 && t.moves.town_turns > 0 && settled_other < g / 2.0 && off < g / 2.0,
    );
    add(
        "the store",
        format!(
            "maps with the top farmer at a pack {} vs control {} (no worse by more than 1); civs never at a pack {} vs {} (no worse by more than 1)",
            t.top_farmer_at_pack, t.control_top_farmer_at_pack, t.civs_never_at_pack, t.control_civs_never_at_pack
        ),
        i64::from(t.top_farmer_at_pack) >= i64::from(t.control_top_farmer_at_pack) - 1
            && i64::from(t.civs_never_at_pack) <= i64::from(t.control_civs_never_at_pack) + 1,
    );
    b
}

/// Sets the bounds of the report's towns total, once its floors are known.
pub(crate) fn settle(r: &mut Report) {
    let b = {
        let floor = |k: &str| r.floor.as_ref().and_then(|f| f.get(&format!("total.towns_vs_control.{k}"))).copied();
        bounds(&r.total.towns_vs_control, &|k| floor(k).unwrap_or(0.0))
    };
    r.total.towns_vs_control.bounds = b;
}

fn mark(ok: bool) -> &'static str {
    if ok {
        "pass"
    } else {
        "MISS"
    }
}

/// The `towns` block of the text: a line per map, then the totals with the table's bounds, each marked.
pub(crate) fn print_towns(r: &Report) {
    println!("towns (against the no-towns control, Config.towns off):");
    for m in &r.maps {
        let t = &m.towns_vs_control;
        println!(
            "  {:<6} seed {:>3}  towns at +200 {} (clans {}, farmers {}, fields {}), formed {}, most homes {}; walk {:.3} vs farmers {:.3}; traded {:.4} vs {:.4}; starving {:.4} vs control {:.4}",
            m.size,
            m.seed,
            t.run.towns_200,
            t.run.town_clans_200,
            t.run.town_farmers_200,
            t.run.towns_fields_200,
            t.formed,
            t.most_homes_200,
            t.run.town.walk_per_turn,
            t.run.farmers_outside.walk_per_turn,
            t.run.town.traded_per_turn,
            t.run.farmers_outside.traded_per_turn,
            t.run.all.starving_share(),
            t.control_all.starving_share(),
        );
    }
    let t = &r.total.towns_vs_control;
    for b in &t.bounds {
        println!("  {}: {} {}", b.name, b.detail, mark(b.pass));
    }
    println!(
        "  health: town clans starving {:.4}, farmers outside towns {:.4}, the control's {:.4} (reported, not bound); lead changes and the spread: reported",
        t.run.town.starving_share(),
        t.run.farmers_outside.starving_share(),
        t.control_all.starving_share(),
    );
}
