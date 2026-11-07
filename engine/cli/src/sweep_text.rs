//! `sweep --judge`'s text report: each map's lines and the totals, from the report `sweep_judge`
//! builds.

use engine::data::JUDGE;

use engine::judge::EndScores;

use crate::sweep_judge::{civ_means, Report};

/// An end as the text reads it: its why and turn, or "goes on".
fn ended(e: &EndScores) -> String {
    e.over.as_ref().map_or("goes on".to_string(), |o| format!("{} +{}", o.why, o.turn))
}

pub(crate) fn print_text(r: &Report) {
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
        let winner = s.end.over.as_ref().map_or(String::new(), |o| {
            format!(", winner {}", m.names.get(&o.winner).cloned().unwrap_or_else(|| format!("civ {}", o.winner)))
        });
        println!(
            "        end: {}{winner}; no fight: {}; capitals fallen {} ({})",
            ended(&s.end),
            ended(&m.no_fight.end),
            s.end.capitals_fallen,
            m.no_fight.end.capitals_fallen,
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
    let e = &t.economy;
    println!(
        "total economy: store mean {:.2}, store max {:.2}, at pack {:.1} civ-turns a map, levy per turn {:.2}, largest army {:.1}, guard mean {:.2}",
        e.store_mean, e.store_max, e.turns_at_pack, e.levy_per_turn, e.largest_army, e.guard_mean
    );
    let u = &t.supply;
    let c = &u.counts;
    println!(
        "total supply: ring tiles {} at the pick ({} held), {} at +200 ({} held); army-turns inside {} ({} hungry), outside {} ({} hungry); scout-turns inside {} ({} hungry), outside {} ({} hungry); clan territory {:.2} at the pick, {} at +200",
        c.ring_pick,
        c.held_ring_pick,
        c.ring_200,
        c.held_ring_200,
        c.army_inside,
        c.army_inside_hungry,
        c.army_outside,
        c.army_outside_hungry,
        c.scout_inside,
        c.scout_inside_hungry,
        c.scout_outside,
        c.scout_outside_hungry,
        u.clan_territory_pick,
        u.clan_territory_200.map_or("-".to_string(), |v| format!("{v:.2}")),
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
    let en = &t.end;
    println!(
        "total end: games ended before the limit {} of {} (no fight {} of {}), turns to the end {:.1} ({:.1}), capitals fallen {} ({})",
        en.early, en.of, en.no_fight_early, en.of, en.turns_mean, en.no_fight_turns_mean, en.capitals_fallen, en.no_fight_capitals_fallen
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
