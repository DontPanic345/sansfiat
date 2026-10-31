//! The text view of the led civ: what a player of the page can see, in text, read at every decision
//! by `play`. Every number is read off the world; nothing here changes it.

use engine::data::{CIV, LEANING, UNIT};
use engine::units::{self, Unit, UnitKind};
use engine::{civs, markets, stats, Action, Clan, Event, Order, World, FOODS};

/// The largest clans the view names.
const CLANS_NAMED: usize = 5;
/// The unaligned clans in reach it names, the strongest leaning first.
const REACH_NAMED: usize = 8;
/// The events about the led civ the feed shows, the newest first.
const FEED_SHOWN: usize = 12;

/// A tile as the view names it: its id and its hex.
fn tile(w: &World, i: usize) -> String {
    format!("{i} {}", w.at(i))
}

/// Tiles as the view names them, comma-separated; `none` for an empty list.
fn tiles(w: &World, list: &[usize]) -> String {
    if list.is_empty() {
        return "none".to_string();
    }
    list.iter().map(|&i| tile(w, i)).collect::<Vec<_>>().join(", ")
}

/// The civs by Standing, the largest first, ties by id.
fn ranked(w: &World) -> Vec<(u32, f64)> {
    let mut rows: Vec<(u32, f64)> = stats::standing(w).into_iter().collect();
    rows.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    rows
}

/// Whether event `e` is about civ `me`: its civ, one of its units, or one of its clans.
fn about(w: &World, me: u32, e: &Event) -> bool {
    e.civ == Some(me)
        || e.unit.is_some_and(|u| w.units.get(&u).is_some_and(|x| x.civ == me))
        || e.clan.is_some_and(|k| w.clans.get(&k).is_some_and(|c| c.civ == Some(me)))
}

/// The view of the world for the led civ, the feed being the events from index `since` on. A
/// watched world (no civ led) has no view.
pub(crate) fn text(w: &World, since: usize) -> String {
    let Some(me) = w.player else { return "you lead no civ\n".to_string() };
    let mut out = String::new();
    let pick = w.picked.unwrap_or(w.turn);
    out += &format!(
        "turn {}, {} turns since the pick at {pick}, phase main, leading civ {me}\n",
        w.turn,
        w.turn.saturating_sub(pick)
    );

    out += "standing, share of the people:\n";
    for (rank, (id, share)) in ranked(w).into_iter().enumerate() {
        let lead = match (rank, id == me) {
            (0, true) => " leader, you",
            (0, false) => " leader",
            (_, true) => " you",
            _ => "",
        };
        out += &format!("  civ {id} {} {:.1}%, rank {}{lead}\n", w.civs[&id].name, share * 100.0, rank + 1);
    }

    let c = &w.civs[&me];
    let stock: Vec<String> = FOODS.iter().zip(c.store.iter()).map(|(f, v)| format!("{f} {v:.1}")).collect();
    out += &format!("store {:.1}: {}\n", c.store.iter().sum::<f64>(), stock.join(", "));
    let set = match c.keep {
        Some(k) => format!("set to {k:.1}"),
        None => "the default, the camp's full pack".to_string(),
    };
    out += &format!(
        "keep {:.1} in force ({set}); gift {:.1} a turn, the most {:.0}\n",
        civs::keep(w, me),
        c.gift,
        CIV.store_cap
    );
    out += &format!(
        "last turn posted {:.1}, gave {:.1}; since founding gave {:.1}, clans joined {}\n",
        c.posted, c.given, c.gifted, c.joined
    );

    let units: Vec<&Unit> = w.units.values().filter(|u| u.civ == me).collect();
    if units.is_empty() {
        out += "units: none\n";
    }
    for u in &units {
        out += &unit_line(w, u);
    }

    let mut clans: Vec<&Clan> = w.clans.values().filter(|k| k.civ == Some(me)).collect();
    clans.sort_by(|a, b| b.people.total_cmp(&a.people).then(a.id.cmp(&b.id)));
    let people: f64 = clans.iter().map(|k| k.people).sum();
    out += &format!("clans: {} with {people:.1} people; the largest:\n", clans.len());
    for k in clans.iter().take(CLANS_NAMED) {
        out += &format!("  clan {} at {}: {:.1} people, fed {:.2}\n", k.id, tile(w, k.home), k.people, k.fed_ratio);
    }

    out += &format!("unaligned clans in reach of your market, leaning to each civ (join at {:.1}):\n", LEANING.join);
    let reach = w.markets.get(&c.market).map(|m| markets::in_reach(w, m.tile).0).unwrap_or_default();
    let mut loose: Vec<(f64, u32)> = reach
        .iter()
        .filter_map(|id| {
            let k = &w.clans[id];
            let best = k.leaning.values().copied().fold(f64::NEG_INFINITY, f64::max);
            (k.civ.is_none() && !k.leaning.is_empty()).then_some((best, *id))
        })
        .collect();
    loose.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    if loose.is_empty() {
        out += "  none\n";
    }
    for &(_, id) in loose.iter().take(REACH_NAMED) {
        let k = &w.clans[&id];
        let legs: Vec<String> =
            w.civs.keys().map(|c| format!("civ {c} {:.2}", k.leaning.get(c).copied().unwrap_or(0.0))).collect();
        out += &format!("  clan {id} at {}, {:.1} people: {}\n", tile(w, k.home), k.people, legs.join(", "));
    }
    if loose.len() > REACH_NAMED {
        out += &format!("  and {} more\n", loose.len() - REACH_NAMED);
    }

    let capitals: Vec<String> = w.civs.values().map(|c| format!("civ {} {}", c.id, tile(w, c.capital))).collect();
    out += &format!("places: capitals {}\n", capitals.join(", "));
    let homes: Vec<usize> = clans.iter().map(|k| k.home).collect();
    out += &format!("  your clans' homes: {}\n", tiles(w, &homes));
    if units::camp(w, me).is_some() {
        out += &format!("  the AI would post your camp at {}\n", tile(w, units::post(w, me)));
    }

    out += &feed(w, me, since);
    out += &legal(w, me);
    out
}

/// One unit of the led civ: a camp's warriors, pack, line, zone and hunger; a scout's route.
fn unit_line(w: &World, u: &Unit) -> String {
    match u.kind() {
        UnitKind::Camp => {
            let cs = u.camp().expect("a camp");
            let pack: f64 = u.store.iter().sum();
            let need = u.people * UNIT.food;
            let feeds = if need > 0.0 { pack / need } else { 0.0 };
            let line = match units::line(w, u.id) {
                None => "no line".to_string(),
                Some((_, _, carry)) if carry.is_infinite() => "at the capital, no limit".to_string(),
                Some((_, cost, carry)) => format!("line cost {cost:.1}, carries {carry:.1} a turn"),
            };
            let cut = if units::cut_off(w, u.id) { ", cut off" } else { "" };
            let cut_at = units::cut_at(w, u.id)
                .map(|(_, at, by)| format!(", cut at {} by civ {by}", tile(w, at)))
                .unwrap_or_default();
            let dest = cs.dest.map_or("none".to_string(), |t| tile(w, t));
            format!(
                "  camp {} at {}: {:.1} warriors, pack {pack:.1} of {:.1} ({feeds:.1} turns), dest {dest}, {line}{cut}{cut_at}, hungry {} turns, zone {}, held {}\n",
                u.id,
                tile(w, u.tile()),
                u.people,
                units::pack_cap(u.people),
                u.hungry,
                tiles(w, &u.territory),
                tiles(w, &cs.held)
            )
        }
        UnitKind::Scout => {
            let route = match u.scout().and_then(|s| s.route) {
                Some((from, to)) => {
                    let plan = units::route_plan(w, u.civ, from, to);
                    let cost = |p: &Option<(Vec<usize>, f64)>| {
                        p.as_ref().map_or("none".to_string(), |(_, c)| format!("{c:.1}"))
                    };
                    format!(
                        "route {} to {}, known cost {}, possible cost {}, {} tiles to search",
                        tile(w, from),
                        tile(w, to),
                        cost(&plan.known),
                        cost(&plan.possible),
                        plan.candidates.len()
                    )
                }
                None => "no route".to_string(),
            };
            format!("  scout {} at {}: {route}, hungry {} turns\n", u.id, tile(w, u.tile()), u.hungry)
        }
    }
}

/// The feed: the events since `since` about the led civ, the newest `FEED_SHOWN`, alerts marked `!`.
fn feed(w: &World, me: u32, since: usize) -> String {
    let rows = &w.events[since.min(w.events.len())..];
    let mine: Vec<&Event> = rows.iter().filter(|e| about(w, me, e)).collect();
    let mut out = format!("feed since the last call: {} about you, {} others\n", mine.len(), rows.len() - mine.len());
    let hidden = mine.len().saturating_sub(FEED_SHOWN);
    if hidden > 0 {
        out += &format!("  {hidden} earlier not shown\n");
    }
    for e in &mine[hidden..] {
        let mark = if e.alert { "!" } else { " " };
        out += &format!("  {:>5} {mark} {}\n", e.turn, e.text(w));
    }
    out
}

/// The legal orders and actions, each as its JSON with what it needs filled in: a tile order with a
/// placeholder tile and the tiles it would accept, and only where `validate` passes it now.
fn legal(w: &World, me: u32) -> String {
    let ok = |o: Order| w.validate(&Action::Order { civ: me, order: o }).is_ok();
    let mut out = String::from("legal orders:\n");
    if ok(Order::Raise { n: 1 }) {
        out += "  {\"type\":\"raise\",\"n\":1}\n";
    }
    for u in w.units.values().filter(|u| u.civ == me) {
        let id = u.id;
        let near: Vec<usize> = engine::land::neighbours(w, u.tile()).collect();
        match u.kind() {
            UnitKind::Camp => {
                if ok(Order::Disband { unit: id, n: None }) {
                    out += &format!("  {{\"type\":\"disband\",\"unit\":{id}}}\n");
                }
                let mut to: Vec<usize> = near.clone();
                to.push(units::post(w, me));
                let places: Vec<usize> = to.into_iter().filter(|&t| ok(Order::Move { unit: id, to: t })).collect();
                if !places.is_empty() {
                    out += &format!("  {{\"type\":\"move\",\"unit\":{id},\"to\":<tile>}}  to: {}\n", tiles(w, &places));
                }
                let zone: Vec<usize> =
                    near.iter().copied().filter(|&t| ok(Order::Zone { unit: id, tiles: vec![t] })).collect();
                if !zone.is_empty() {
                    out += &format!(
                        "  {{\"type\":\"zone\",\"unit\":{id},\"tiles\":[<tile>]}}  tiles: {}\n",
                        tiles(w, &zone)
                    );
                }
                if ok(Order::Zone { unit: id, tiles: Vec::new() }) {
                    out += &format!("  {{\"type\":\"zone\",\"unit\":{id},\"tiles\":[]}}  undeploys\n");
                }
                out += &format!(
                    "  {{\"type\":\"switches\",\"unit\":{id},\"forage\":<true or false>,\"supply\":<true or false>}}\n"
                );
                for (kind, o) in [
                    ("resupply", Order::Resupply { unit: id }),
                    ("forage", Order::Forage { unit: id }),
                    ("pillage", Order::Pillage { unit: id }),
                ] {
                    if ok(o) {
                        out += &format!("  {{\"type\":\"{kind}\",\"unit\":{id}}}\n");
                    }
                }
            }
            UnitKind::Scout => {
                let from = u.tile();
                let places: Vec<usize> =
                    near.iter().copied().filter(|&t| ok(Order::Route { unit: id, from, to: t })).collect();
                if !places.is_empty() {
                    out += &format!(
                        "  {{\"type\":\"route\",\"unit\":{id},\"from\":{from},\"to\":<tile>}}  to: {}\n",
                        tiles(w, &places)
                    );
                }
                if u.scout().is_some_and(|s| s.route.is_some()) {
                    out += &format!("  {{\"type\":\"clearRoute\",\"unit\":{id}}}\n");
                }
            }
        }
    }
    let keep = Action::SetKeep { food: Some(0.0) };
    if w.validate(&keep).is_ok() {
        out += "  {\"type\":\"setKeep\",\"food\":<0 or more, or null for the camp's full pack>}\n";
    }
    out += &format!("  {{\"type\":\"setGift\",\"food\":<0 to {:.0}>}}\n", CIV.store_cap);
    out += "  end [N]: end N turns (1 by default)\n";
    out
}
