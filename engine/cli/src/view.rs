//! The text view of the led civ: what a player of the page can see, in text, read at every decision
//! by `play`. Every number is read off the world; nothing here changes it.

use engine::data::{Fighter, CIV, FIGHT, LEANING, SETTLE, SETTLING, UNIT};
use engine::data::{ARMS, GOOD, KEEPING, N};
use engine::market::total;
use engine::over::{self, Why};
use engine::units::{self, Unit, UnitKind};
use engine::{civs, claim, fields, markets, settle, stats, towns, Action, Clan, Event, Order, Phase, World, GOODS};

use crate::view_land::{claim_lines, claim_orders, settle_lines};

/// The largest clans the view names.
const CLANS_NAMED: usize = 5;
/// The unaligned clans in reach it names, the strongest leaning first.
const REACH_NAMED: usize = 8;
/// The events about the led civ the feed shows, the newest first.
const FEED_SHOWN: usize = 12;

/// A clan's fields this turn for its line, `, fields 3 tiles, 9.4 days, 6.1 grain`; empty for a clan with no share.
fn fields_text(w: &World, k: &Clan) -> String {
    if k.fields <= 0.0 {
        return String::new();
    }
    let plots = fields::plots(w, k);
    let days: f64 = plots.iter().map(|p| p.1).sum();
    let grain: f64 = plots.iter().map(|p| p.2).sum();
    let tiles = if plots.len() == 1 { "tile" } else { "tiles" };
    format!(", fields {} {tiles}, {days:.1} days, {grain:.1} grain", plots.len())
}

/// A tile as the view names it: its id and its hex.
pub(crate) fn tile(w: &World, i: usize) -> String {
    format!("{i} {}", w.at(i))
}

/// Tiles as the view names them, comma-separated; `none` for an empty list.
pub(crate) fn tiles(w: &World, list: &[usize]) -> String {
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

/// What a civ's clan paid in levy last turn, or why it paid nothing.
fn levy_text(w: &World, k: &Clan) -> String {
    let capital = k.civ.and_then(|c| w.civs.get(&c)).map(|c| c.market);
    match w.levied.get(&k.id) {
        Some(paid) => format!("paid {paid:.2} food in levy"),
        None if k.market.is_some() && k.market == capital => "paid no levy".to_string(),
        None => "pays no levy: it doesn't trade at the capital's market".to_string(),
    }
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
    out += &match w.phase {
        Phase::Over(o) => {
            let winner = w.civs[&o.winner].name.as_str();
            let why = match o.why {
                Why::Conquest => "the last capital standing".to_string(),
                Why::Limit => "the highest Standing at the turn limit".to_string(),
                Why::Fell { by } => format!("the highest Standing once your capital fell to {}", w.civs[&by].name),
            };
            format!("turn {}, the game is over: {winner} won, {why}, turn +{}, leading civ {me}\n", w.turn, o.turn)
        }
        _ => format!(
            "turn {}, {} of {} turns since the pick at {pick}, phase main, leading civ {me}\n",
            w.turn,
            w.turn.saturating_sub(pick),
            over::limit(w)
        ),
    };

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
    out += &format!("store {:.1} food: {}\n", total(&c.store), crate::view_goods::stock(&c.store));
    out += &crate::view_goods::store_chains(w, me, &c.store, &c.actor.lots);
    out += &sources(w, me);
    let set = match c.keep {
        Some(k) => format!("set to {k:.1}"),
        None => format!("not set: the default, a full pack for its camps, {:.1} now", civs::keep(w, me)),
    };
    let reach = civs::gift_reach(w, me);
    let reaches = match reach {
        0 => "reaches no clan at your market".to_string(),
        1 => "reaches 1 clan".to_string(),
        n => format!("reaches {n} clans"),
    };
    out += &format!(
        "keep {:.1} in force ({set}); gift {:.1} a turn, the most {:.0}, {reaches}: gave {:.1}, left {:.1}; last turn \
         posted {:.1}; since founding gave {:.1}, clans joined {}; ",
        civs::keep(w, me),
        c.gift,
        CIV.store_cap,
        c.given,
        (c.gift - c.given).max(0.0),
        c.posted,
        c.gifted,
        c.joined
    );

    out +=
        &format!("reach: {}; capital: {}\n", engine::outlook::reach_line(w, me), engine::outlook::capital_line(w, me));

    let units: Vec<&Unit> = w.units.values().filter(|u| u.civ == me).collect();
    if units.is_empty() {
        out += "units: none\n";
    }
    for u in &units {
        out += &unit_line(w, u);
    }
    out += &claim_lines(w, me);
    out += &settle_lines(w, me);

    let mut clans: Vec<&Clan> = w.clans.values().filter(|k| k.civ == Some(me)).collect();
    clans.sort_by(|a, b| b.people.total_cmp(&a.people).then(a.id.cmp(&b.id)));
    let people: f64 = clans.iter().map(|k| k.people).sum();
    let on_land = clans.iter().filter(|k| claim::tile_civ(w, k.home) == Some(me)).count();
    let levy = clans.iter().filter_map(|k| w.levied.get(&k.id)).fold(0.0, |a, p| a + p);
    let l = engine::settling::living(w, me);
    out += &format!(
        "clans: {} with {people:.1} people, {on_land} with their home on your land (a home on your land makes a clan yours); they paid {levy:.1} food in levy last turn; living: gathering {:.1} people, farming {:.1} in {} settled clan(s), in the army {:.1} (a clan settles after {} turns with {:.0}% or more of its days on fields, and goes back after {} under {:.0}%); the largest:\n",
        clans.len(),
        l.gathering,
        l.farming,
        l.settled,
        l.army,
        SETTLING.settle_after,
        SETTLING.farms * 100.0,
        SETTLING.leave_after,
        SETTLING.gathers * 100.0
    );
    for k in clans.iter().take(CLANS_NAMED) {
        let home = if claim::tile_civ(w, k.home) == Some(me) { "on your land" } else { "off your land" };
        out += &format!(
            "  clan {} at {}: {:.1} people, {}, fed {:.2}{}, {home}, {}\n",
            k.id,
            tile(w, k.home),
            k.people,
            if k.settled { "settled" } else { "gathering" },
            k.fed_ratio,
            fields_text(w, k),
            levy_text(w, k)
        );
    }

    out += &town_lines(w, me);
    out += &format!("unaligned clans in reach of your market, leaning to each civ (join at {:.1}, or at once when your claim covers their home; a gift reaches only those that trade there):\n", LEANING.join);
    let reach = w.markets.get(&c.market).map(|m| markets::in_reach(w, m.tile).0).unwrap_or_default();
    let mut loose: Vec<(f64, u32)> = reach
        .iter()
        .filter_map(|id| {
            let k = &w.clans[id];
            let best = k.leaning.values().copied().fold(0.0, f64::max);
            k.civ.is_none().then_some((best, *id))
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
        let trades = match k.market {
            Some(m) if m == c.market => "trades at your market, so your gift reaches it".to_string(),
            Some(m) => format!("trades at market {m}, so your gift doesn't reach it"),
            None => "trades at no market, so your gift doesn't reach it".to_string(),
        };
        out += &format!(
            "  clan {id} at {}, {:.1} people{}: {}; {trades}\n",
            tile(w, k.home),
            k.people,
            fields_text(w, k),
            legs.join(", ")
        );
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

/// One line per town on the led civ's land, or with one of its clans living on it: the tile, its
/// clans and how many of them farm.
fn town_lines(w: &World, me: u32) -> String {
    let mut out = String::new();
    for t in towns::towns(w) {
        let civ_of = |id: u32| w.clans.get(&id).and_then(|c| c.civ);
        let mine = t.clans.iter().any(|&id| civ_of(id) == Some(me));
        let claimed = w.tiles[t.tile].claim.and_then(civ_of) == Some(me);
        if mine || claimed {
            let farmers = t.clans.iter().filter(|id| fields::farmer(&w.clans[id])).count();
            out += &format!("town at {}: {} clans, {farmers} farming\n", tile(w, t.tile), t.clans.len());
        }
    }
    out
}

/// What a unit draws on: the store its line ends at, and the line's days when it has one, or that it
/// has no line.
fn draws_on(w: &World, u: &Unit) -> String {
    match units::store_line(w, u.id) {
        Some((path, cost, _)) => {
            let at = path.last().copied().unwrap_or(u.home);
            let days = if path.is_empty() { String::new() } else { format!(", {cost:.1} days") };
            format!("draws on {}{days}", settle::store_name(w, u.civ, at))
        }
        None => "draws on no store".to_string(),
    }
}

/// The map's sources of salt and obsidian, good by good: each source's tile and who gathers it, and
/// what the goods do.
fn sources(w: &World, me: u32) -> String {
    let mut out = String::new();
    for g in (0..N).filter(|&g| !GOOD[g].eaten) {
        let at: Vec<String> = engine::sources::of(w, g)
            .into_iter()
            .map(|t| {
                let who = match w.tiles[t].claim.and_then(|id| w.clans.get(&id)) {
                    None => "nobody".to_string(),
                    Some(k) if k.civ == Some(me) => format!("your clan {}", k.id),
                    Some(k) => match k.civ {
                        Some(c) => format!("civ {c}'s clan {}", k.id),
                        None => format!("unaligned clan {}", k.id),
                    },
                };
                format!("{} ({who})", tile(w, t))
            })
            .collect();
        if !at.is_empty() {
            out += &format!("{} sources: {}\n", GOODS[g], at.join(", "));
        }
    }
    if !out.is_empty() {
        out += &format!(
            "  a clan that holds a source gathers it and trades it at its festival; your store gets it by the levy and \
             at your market. A salt keeps {:.0} food in a store at half its spoiling for a turn and is used up; a \
             warrior raised takes {:.1} obsidian from the store he is raised from and fights {:.0}% stronger\n",
            KEEPING.keeps,
            ARMS.each,
            (ARMS.edge - 1.0) * 100.0
        );
    }
    out
}

/// One unit of the led civ: an army's stack, pack, line, zone and hunger; a scout's route.
fn unit_line(w: &World, u: &Unit) -> String {
    match u.kind() {
        UnitKind::Camp => {
            let cs = u.camp().expect("a camp");
            let pack: f64 = u.store.iter().sum();
            let need = u.people * UNIT.food;
            let feeds = if need > 0.0 { pack / need } else { 0.0 };
            let line = match units::line(w, u.id) {
                None => "no line".to_string(),
                Some((path, _, carry)) if carry.is_infinite() => {
                    let end = path.last().copied().unwrap_or(u.home);
                    match settle::store(w, u.civ, end) {
                        Some(_) => format!("at {}, no limit", settle::store_name(w, u.civ, end)),
                        None => format!("at the market at {}, no limit", tile(w, end)),
                    }
                }
                Some((_, cost, carry)) => format!("line {cost:.1} days, carries {carry:.1} food a turn"),
            };
            let cut = if units::cut_off(w, u.id) { ", cut off" } else { "" };
            let cut_at = units::cut_at(w, u.id)
                .map(|(_, at, by)| format!(", cut at {} by civ {by}", tile(w, at)))
                .unwrap_or_default();
            let cut_at = match units::cut_by_rival(w, u.id) {
                Some(by) if cs.dest.is_none() => {
                    format!(
                        "{cut_at}, cut off by civ {by}'s land: surrenders hungry {} turns in a row",
                        FIGHT.surrender
                    )
                }
                _ => cut_at,
            };
            let draws = draws_on(w, u);
            let wares = crate::view_goods::pack_wares(u);
            let bows = if cs.bows { ", making bows" } else { "" };
            let bows = format!("{bows}, {:.0}% armed with obsidian", cs.armed * 100.0);
            let dest = cs.dest.map_or("none".to_string(), |t| tile(w, t));
            format!(
                "  army {} at {}: {:.1} warriors, {:.1} archers, pack {pack:.1} of {:.1} ({feeds:.1} turns){wares}, entrenched {} turns{bows}, dest {dest}, {line}, {draws}{cut}{cut_at}, hungry {} turns, zone {}, held {}\n",
                u.id,
                tile(w, u.tile()),
                cs.stack[Fighter::Warriors.index()],
                cs.stack[Fighter::Archers.index()],
                units::pack_cap(u.people),
                cs.still,
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
                        p.as_ref().map_or("none".to_string(), |(_, c)| format!("{c:.1} days"))
                    };
                    format!(
                        "route {} to {}, known route {}, possible route {}, {} tiles to search",
                        tile(w, from),
                        tile(w, to),
                        cost(&plan.known),
                        cost(&plan.possible),
                        plan.candidates.len()
                    )
                }
                None => match u.dest() {
                    Some(dest) => format!("going to {}", tile(w, dest)),
                    None => "no route".to_string(),
                },
            };
            let store = match units::store_walk(w, u.id) {
                Some((at, path, _)) if path.is_empty() => format!("at {}", settle::store_name(w, u.civ, at)),
                Some((at, _, n)) => format!("{n} turns' walk from {}", settle::store_name(w, u.civ, at)),
                None => "no walk to a store".to_string(),
            };
            format!(
                "  scout {} at {}: {}; {route}, pack {:.1} of {:.0} ({:.1} turns), {store}, hungry {} turns\n",
                u.id,
                tile(w, u.tile()),
                units::doing_text(w, u),
                u.store.iter().sum::<f64>(),
                units::cap_of(u),
                units::pack_turns(w, u.id),
                u.hungry
            )
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
    if ok(Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None, at: None }) {
        out += "  {\"type\":\"raise\",\"n\":1}  warriors, into a new army at your capital\n";
    }
    if ok(Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Archers, into: None, at: None }) {
        out += "  {\"type\":\"raise\",\"n\":1,\"fighter\":\"archers\"}  archers, into a new army at your capital\n";
    }
    for s in settle::of(w, me) {
        let at = s.tile;
        let raise = |kind, fighter| Order::Raise { n: 1, kind, fighter, into: None, at: Some(at) };
        if ok(raise(UnitKind::Camp, Fighter::Warriors)) {
            out += &format!(
                "  {{\"type\":\"raise\",\"n\":1,\"at\":{at}}}  warriors, into a new army at your settlement at {}\n",
                tile(w, at)
            );
        }
        if ok(raise(UnitKind::Scout, Fighter::Warriors)) {
            out += &format!("  {{\"type\":\"raise\",\"n\":1,\"kind\":\"scout\",\"at\":{at}}}  a scout, from your settlement at {}\n", tile(w, at));
        }
    }
    for u in w.units.values().filter(|u| u.civ == me && u.kind() == UnitKind::Camp) {
        let id = u.id;
        for f in Fighter::ALL {
            let name = f.rules().name;
            let order = Order::Raise { n: 1, kind: UnitKind::Camp, fighter: f, into: Some(id), at: None };
            if ok(order) {
                let fighter = if f == Fighter::Warriors { String::new() } else { format!(",\"fighter\":\"{name}\"") };
                out += &format!("  {{\"type\":\"raise\",\"n\":1{fighter},\"into\":{id}}}  {name} into army {id}\n");
            }
        }
    }
    let claimed = w.civs[&me].claim.clone();
    if ok(Order::Claim { tiles: claimed }) {
        out += &claim_orders(w, me);
    }
    if settle::open(w) {
        let posts = &w.civs[&me].signposts;
        if posts.len() < SETTLE.signposts {
            let sites: Vec<usize> = units::signpost_sites(w, me).into_iter().take(6).map(|(_, t)| t).collect();
            let good = if sites.is_empty() { "none free nearby".to_string() } else { tiles(w, &sites) };
            out += &format!(
                "  {{\"type\":\"signpost\",\"tile\":<tile>}}  a place for your clans to settle: land nobody lives on, {} \
                 steps or more from every capital and settlement (you keep {} at a time, {} now); good sites: {good}\n",
                SETTLE.spacing,
                SETTLE.signposts,
                posts.len()
            );
        }
        for s in posts {
            out += &format!("  {{\"type\":\"unpost\",\"tile\":{}}}  removes your signpost\n", s.tile);
        }
    }
    for u in w.units.values().filter(|u| u.civ == me) {
        let id = u.id;
        let near: Vec<usize> = engine::land::neighbours(w, u.tile()).collect();
        match u.kind() {
            UnitKind::Camp => {
                if ok(Order::Disband { unit: id, n: None, fighter: None }) {
                    out += &format!("  {{\"type\":\"disband\",\"unit\":{id}}}\n");
                }
                for f in Fighter::ALL {
                    if ok(Order::Disband { unit: id, n: None, fighter: Some(f) }) {
                        out += &format!(
                            "  {{\"type\":\"disband\",\"unit\":{id},\"fighter\":\"{}\"}}  its {}\n",
                            f.rules().name,
                            f.rules().name
                        );
                    }
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
                        "  {{\"type\":\"route\",\"unit\":{id},\"from\":{from},\"to\":<tile>}}  to: any tile it may route to, such as {}\n",
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
    if w.validate(&Action::SetGift { food: 0.0 }).is_ok() {
        out += &format!("  {{\"type\":\"setGift\",\"food\":<0 to {:.0}>}}\n", CIV.store_cap);
    }
    out += "  end [N]: end N turns (1 by default)\n";
    out
}

#[cfg(test)]
mod tests {
    use engine::clans::new_clan;
    use engine::hex::Hex;
    use engine::homes;
    use engine::map::index;
    use engine::stage::Stage;
    use engine::{settle, Order};

    use super::text;

    const RADIUS: i32 = 9;

    fn tile(q: i32, r: i32) -> usize {
        index(RADIUS, Hex { q, r }).unwrap()
    }

    /// Red leads, with a settlement on (−1, 0) and an army of 3 on it: the stores of `stores.rs`.
    #[test]
    fn an_army_on_a_settlement_names_it_and_not_its_store() {
        let mut stage = Stage::flat(RADIUS);
        let ids = stage.civs(&[tile(-6, 0), tile(6, 0)], 5.0);
        let red = ids[0];
        stage.lead(red).unwrap();
        let w = stage.world_mut();
        w.clans.clear();
        for t in &mut w.tiles {
            t.claim = None;
        }
        let t = tile(-1, 0);
        w.order(red, Order::Signpost { tile: t }).unwrap();
        let id = w.next_id;
        w.next_id += 1;
        let mut c = new_clan(id, t, 4.0);
        c.civ = Some(red);
        homes::claim_land(w, id, &[t]);
        w.clans.insert(id, c);
        settle::step(w);
        let sid = settle::at(w, t).expect("a settlement").id;
        w.settlements.get_mut(&sid).unwrap().store = [40.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let army = stage.army(red, t, 3.0);
        let view = text(stage.world(), 0);
        let line = view.lines().find(|l| l.starts_with(&format!("  army {army} "))).expect("the army's line");
        assert!(line.contains("at its settlement at (-1, 0), no limit"), "{line}");
        assert!(!line.contains("at its store"), "{line}");
        assert!(line.contains("draws on its settlement at (-1, 0)"), "{line}");
    }
}
