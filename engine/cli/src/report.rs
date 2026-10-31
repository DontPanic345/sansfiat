use engine::{civs, land, pregame, stats, World, TERRAIN};

use crate::run::Output;

/// One summary line.
pub(crate) fn line(s: &stats::Summary) -> String {
    format!(
        "turn {:>4}  clans {:>3}  people {:>6.1}  fed {:.2}  held {:>3}  trails {:>3}  hearths {:>3}  markets {:>2}  trading {:>2}  centres {}  cover {:.2}",
        s.turn, s.clans, s.people, s.mean_fed, s.held, s.trails, s.hearths, s.markets, s.trading, s.centres.len(), s.cover
    )
}

/// The centres' report: one line each, then the distances between them.
pub(crate) fn centres_text(w: &World, ids: &[u32]) -> String {
    let mut out = String::new();
    for c in stats::centre_report(w, ids) {
        out += &format!(
            "  centre market {} at ({}, {}): {} traders, age {}, {} hearths in reach, fertility near {:.2} vs land {:.2}\n",
            c.market, c.q, c.r, c.traders, c.age, c.hearths_in_reach, c.fertility_near, c.fertility_land
        );
    }
    let d = stats::distances(w, ids);
    if !d.is_empty() {
        out +=
            &format!("  steps between centres: {}\n", d.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(", "));
    }
    out
}

pub(crate) fn report(w: &World, out: &Output) {
    if out.events_json {
        let events: Vec<_> = w.events.iter().map(|e| e.json(w)).collect();
        println!("{}", serde_json::json!({ "events": events }));
        return;
    }
    if out.json {
        println!("{}", serde_json::to_string(w).unwrap());
        return;
    }
    let land = w.tiles.iter().filter(|t| TERRAIN[t.terrain].land).count();
    println!(
        "seed {} radius {}: turn {}, {} tiles ({} land), {} actions logged",
        w.config.seed,
        w.config.radius,
        w.turn,
        w.tiles.len(),
        land,
        w.log.len()
    );
    let people: f64 = w.clans.values().map(|c| c.people).sum();
    let fed = w.clans.values().map(|c| c.fed_ratio).sum::<f64>() / w.clans.len().max(1) as f64;
    let held = w.tiles.iter().filter(|t| t.claim.is_some()).count();
    let n = w.clans.len().max(1) as f64;
    let tiles = w.clans.values().map(|c| c.territory.len() as f64).sum::<f64>() / n;
    let walk = w.clans.values().map(|c| c.walk).sum::<f64>() / n;
    let trip = w.clans.values().map(|c| c.trip()).sum::<f64>() / n;
    println!(
        "{} clans, {people:.1} people, mean fed ratio {fed:.2}, {held} tiles held, mean territory {tiles:.1} tiles walked for {walk:.1}, mean route {trip:.1}",
        w.clans.len()
    );
    let trading = w.clans.values().filter(|c| c.market.is_some()).count();
    let hearths = w.tiles.iter().filter(|t| land::is_hearth(t)).count();
    let trails = w.tiles.iter().filter(|t| t.trail).count();
    let cover = stats::summary(w).cover;
    let end = match &w.end {
        Some(e) => {
            let tiles: Vec<String> = e
                .centres
                .iter()
                .map(|id| w.markets.get(id).map_or("gone".to_string(), |m| format!("{}", m.tile)))
                .collect();
            let how = if e.by_limit { "by the turn limit" } else { "by its centres" };
            let up = match e.topped_up.len() {
                0 => String::new(),
                n => format!(
                    ", {n} {} topped up (market {})",
                    if n == 1 { "centre" } else { "centres" },
                    e.topped_up.iter().map(u32::to_string).collect::<Vec<_>>().join(", market ")
                ),
            };
            format!("ended turn {} {how}{up}, centres at tiles [{}]", e.turn, tiles.join(", "))
        }
        None => "not ended".to_string(),
    };
    println!(
        "{} markets, {trading} clans trading, {hearths} hearths, {trails} trails, cover {cover:.2}, {end}",
        w.markets.len()
    );
    let (ids, what) = match &w.end {
        Some(e) if e.by_limit => (e.centres.clone(), "turn limit reached, centres (topped up included)"),
        Some(e) => (e.centres.clone(), "centres"),
        None => (
            pregame::centres(w).iter().filter_map(|id| w.clans[id].market).collect(),
            "not ended, centre clans so far, by their festivals",
        ),
    };
    println!("{what}: {}", ids.len());
    print!("{}", centres_text(w, &ids));
    print!("{}", civs_text(w));
}

/// One line per civ: name, capital, market, clans, people, store, hearths on its land and units.
pub(crate) fn civs_text(w: &World) -> String {
    let mut out = String::new();
    for c in civs::cards(w) {
        let civ = &w.civs[&c.civ];
        let store: f64 = civ.store.iter().sum();
        out += &format!(
            "  civ {} at ({}, {}), market {}: {} clans, {:.1} people, store {store:.1}, {} hearths on its land, camp {}\n",
            c.name,
            c.q,
            c.r,
            civ.market,
            c.clans,
            c.people,
            c.hearths,
            engine::units::camp(w, c.civ).map_or("none".to_string(), |u| format!("{:.0} warriors", u.people))
        );
    }
    out
}
