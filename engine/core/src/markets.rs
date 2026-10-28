//! Markets: where clans meet to trade, and in the pre-game a festival. A market forms where
//! clans have lived a long time with enough of them in reach; each clan walks one route to the
//! one worth most to it, if any is worth the walk; it clears their baskets by barter, and it
//! fades when nobody trades there.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::actors::{walk_cost, Route};
use crate::clans::{nutrition, surplus, Clan};
use crate::data::*;
use crate::land::{cost_map, passable, path};
use crate::market::{clear, demand, Basket, Params};
use crate::world::{EventKind, World};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Market {
    pub id: u32,
    pub tile: usize,
    /// The turn it formed.
    pub founded: u32,
    /// Last clearing's exchange ratios.
    pub prices: Basket,
    /// The goods last traded there: what clans expect to find.
    pub goods: Vec<usize>,
    /// Clans that came, smoothed over turns.
    pub activity: f64,
    /// Last turn's volume, summed over the goods.
    pub volume: f64,
    /// Clans that came last turn.
    pub traders: usize,
    /// People moved between its clans this turn.
    pub moved: f64,
}

/// Path cost from the market's tile over passable tiles, out to `MARKET.reach`: the land around
/// it, for a centre's hearths and a civ's clans.
pub fn reach(w: &World, tile: usize) -> BTreeMap<usize, f64> {
    cost_map(w, &[tile], MARKET.reach, |i| passable(w, i))
}

/// What a clan bringing `b` would gain in food from trading at `m`'s last prices, over its own
/// foods and the goods the market last traded, by nutrition for `pop` people.
pub fn gain(m: &Market, b: &Basket, pop: f64) -> f64 {
    let goods: Vec<usize> = (0..N).filter(|&f| b[f] > 0.0 || m.goods.contains(&f)).collect();
    let d = demand(b, &m.prices, &goods);
    (nutrition(&d, pop) - nutrition(b, pop)).max(0.0)
}

/// What a clan wants from going to `m` besides trade, in labour: `MARKET.desire`, plus
/// `company` for each clan that came last turn up to `company_cap`, plus `gather_bonus` with a
/// surplus.
pub fn desire(m: &Market, surplus: bool) -> f64 {
    let company = MARKET.company * m.traders.min(MARKET.company_cap) as f64;
    MARKET.desire + company + if surplus { MARKET.gather_bonus } else { 0.0 }
}

/// What `clan` would make of going to each festival whose route from its home costs at most
/// `MARKET.reach_walk`, in id order: the festival's id, its worth (`gain × harvest_labour +
/// desire − route cost`) and the route's cost.
pub fn worths(w: &World, clan: &Clan) -> Vec<(u32, f64, f64)> {
    let most = MARKET.reach_walk / 2.0;
    let home = w.tiles[clan.home].hex();
    // Every step costs at least 1, so a festival further than the path can go is out of reach.
    if !w.markets.values().any(|m| f64::from(w.tiles[m.tile].hex().distance(home)) <= most) {
        return Vec::new();
    }
    let near = cost_map(w, &[clan.home], most, |i| passable(w, i));
    let b: Basket = std::array::from_fn(|f| clan.store[f] + clan.harvest[f]);
    let surplus = clan.store.iter().sum::<f64>() >= MARKET.surplus_at * clan.people;
    w.markets
        .values()
        .filter_map(|m| {
            let cost = walk_cost(*near.get(&m.tile)?, 1.0);
            Some((m.id, gain(m, &b, clan.people) * CLAN.harvest_labour + desire(m, surplus) - cost, cost))
        })
        .collect()
}

/// The festival a clan goes to, from `worths`: the best worth above 0 (ties to the lower id), but
/// its current one while that's still above 0 and no other beats it by `MARKET.switch_margin`.
pub fn choose(worths: &[(u32, f64, f64)], current: Option<u32>) -> Option<u32> {
    let best = worths.iter().filter(|w| w.1 > 0.0).fold(None, |b: Option<&(u32, f64, f64)>, w| match b {
        Some(b) if b.1 >= w.1 => Some(b),
        _ => Some(w),
    })?;
    match worths.iter().find(|w| Some(w.0) == current) {
        Some(cur) if cur.1 > 0.0 && best.1 <= cur.1 + MARKET.switch_margin => Some(cur.0),
        _ => Some(best.0),
    }
}

/// The `go` step, before harvest: each clan picks the festival worth most to it (`choose`) and
/// takes the cheapest path there from home as its one route; or stays home.
pub fn go(w: &mut World) {
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let c = &w.clans[&id];
        let choice = choose(&worths(w, c), c.market);
        let route = choice.and_then(|mid| {
            let to = w.markets[&mid].tile;
            let (path, cost) = path(w, c.home, to, |i| passable(w, i))?;
            Some(Route { to, path, cost: walk_cost(cost, 1.0) })
        });
        let c = w.clans.get_mut(&id).unwrap();
        c.market = route.as_ref().and(choice);
        c.routes = route.into_iter().collect();
    }
}

/// The `trade` step, after harvest: each market clears what its clans brought. A clan brings its
/// basket scaled evenly down to at most `MARKET.carry × people` of food; the rest stays in its
/// basket, and the cleared goods come back on top.
pub fn trade(w: &mut World) {
    let mids: Vec<u32> = w.markets.keys().copied().collect();
    for mid in mids {
        let ids: Vec<u32> = w.clans.values().filter(|c| c.market == Some(mid)).map(|c| c.id).collect();
        let (brought, kept): (Vec<Basket>, Vec<Basket>) = ids
            .iter()
            .map(|id| {
                let c = &w.clans[id];
                let total: f64 = c.basket.iter().sum();
                let most = MARKET.carry * c.people;
                let share = if total > most { most / total } else { 1.0 };
                (c.basket.map(|v| v * share), c.basket.map(|v| v - v * share))
            })
            .unzip();
        let m = w.markets.get_mut(&mid).unwrap();
        let c = clear(&brought, &m.prices, &Params::default());
        if c.cleared {
            m.prices = c.prices;
            m.goods = c.goods.clone();
        }
        m.volume = c.volume.iter().sum();
        m.traders = ids.len();
        m.activity = MARKET.fade * m.activity + (1.0 - MARKET.fade) * ids.len() as f64;
        for ((id, after), kept) in ids.iter().zip(c.after).zip(kept) {
            w.clans.get_mut(id).unwrap().basket = std::array::from_fn(|f| after[f] + kept[f]);
        }
    }
}

/// The `people` step, after eating: at each festival, in id order, the starving clans that came
/// offer `MARKET.shed × people × (1 − fed_ratio)` people (never the last one), and the growing
/// clans (a surplus, not starving) have room for their surplus in people. The offers go to the
/// growing clans in proportion to their room, all scaled down to the room if they are more.
pub fn people(w: &mut World) {
    for m in w.markets.values_mut() {
        m.moved = 0.0;
    }
    let mids: Vec<u32> = w.markets.keys().copied().collect();
    for mid in mids {
        let mut offers: Vec<(u32, f64)> = Vec::new();
        let mut rooms: Vec<(u32, f64)> = Vec::new();
        for c in w.clans.values().filter(|c| c.market == Some(mid)) {
            if c.starving {
                let offer = (MARKET.shed * c.people * (1.0 - c.fed_ratio)).min(c.people - 1.0);
                if offer > 0.0 {
                    offers.push((c.id, offer));
                }
            } else if surplus(&c.harvest, c.people) > 0.0 {
                rooms.push((c.id, surplus(&c.harvest, c.people)));
            }
        }
        let offered: f64 = offers.iter().map(|o| o.1).sum();
        let room: f64 = rooms.iter().map(|r| r.1).sum();
        if offered <= 0.0 || room <= 0.0 {
            continue;
        }
        let moved = offered.min(room);
        for (id, offer) in offers {
            w.clans.get_mut(&id).unwrap().people -= offer * moved / offered;
        }
        for (id, r) in rooms {
            w.clans.get_mut(&id).unwrap().people += moved * r / room;
        }
        w.markets.get_mut(&mid).unwrap().moved = moved;
        w.moved += moved;
    }
    merge(w);
}

/// After people move: in id order, a clan under `CLAN.min_pop` at a festival
/// where another clan came joins the clan there whose home is fewest hex steps from its own
/// (ties to the lower id): its people and store go to that clan, its tiles are freed, it goes.
fn merge(w: &mut World) {
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let Some(c) = w.clans.get(&id) else { continue };
        let Some(mid) = c.market.filter(|_| c.people < CLAN.min_pop) else { continue };
        let home = w.tiles[c.home].hex();
        let Some(into) = w
            .clans
            .values()
            .filter(|o| o.id != id && o.market == Some(mid))
            .min_by_key(|o| (w.tiles[o.home].hex().distance(home), o.id))
            .map(|o| o.id)
        else {
            continue;
        };
        let c = w.clans.remove(&id).unwrap();
        for &t in &c.territory {
            if w.tiles[t].claim == Some(id) {
                w.tiles[t].claim = None;
            }
        }
        let o = w.clans.get_mut(&into).unwrap();
        o.people += c.people;
        for f in 0..N {
            o.store[f] += c.store[f];
        }
        w.event(EventKind::ClanMerged, c.home, Some(id), None, Some(into));
    }
}

/// The `markets` step: festivals nobody comes to fade, then clans raise new ones.
pub fn form_and_fade(w: &mut World) {
    fade(w);
    form(w);
}

/// A festival whose activity falls under `min_attendance` goes, and its clans with it. A centre
/// the pre-game ended on stays, and so does every civ's market.
pub fn fade(w: &mut World) {
    let mut kept = w.end.as_ref().map(|e| e.centres.clone()).unwrap_or_default();
    kept.extend(w.civs.values().map(|c| c.market));
    let gone: Vec<u32> = w
        .markets
        .values()
        .filter(|m| m.activity < MARKET.min_attendance && !kept.contains(&m.id))
        .map(|m| m.id)
        .collect();
    for mid in gone {
        let m = w.markets.remove(&mid).unwrap();
        w.event(EventKind::MarketFaded, m.tile, None, Some(mid), None);
        for c in w.clans.values_mut() {
            if c.market == Some(mid) {
                c.market = None;
                c.routes.clear();
            }
        }
    }
}

/// The clans whose route from home to `tile` would cost at most `MARKET.reach_walk`, in id
/// order, and their people. Paths are found out from `tile`.
pub fn in_reach(w: &World, tile: usize) -> (Vec<u32>, f64) {
    let near = cost_map(w, &[tile], MARKET.reach_walk / 2.0, |i| passable(w, i));
    let mut ids: Vec<u32> = near
        .keys()
        .filter_map(|&t| w.tiles[t].claim.filter(|id| w.clans.get(id).is_some_and(|c| c.home == t)))
        .collect();
    ids.sort_unstable();
    ids.dedup();
    let people = ids.iter().map(|id| w.clans[id].people).sum();
    (ids, people)
}

/// Each clan with no festival that has been settled `form_after` turns raises one on its home,
/// if none is within `spacing` hex steps. Clans go biggest first (then lower id), each checked
/// against the festivals raised before it this turn.
pub fn form(w: &mut World) {
    let mut ids: Vec<u32> =
        w.clans.values().filter(|c| c.market.is_none() && c.since >= MARKET.form_after).map(|c| c.id).collect();
    ids.sort_by(|a, b| w.clans[b].people.total_cmp(&w.clans[a].people).then(a.cmp(b)));
    for id in ids {
        let tile = w.clans[&id].home;
        let h = w.tiles[tile].hex();
        if w.markets.values().any(|m| w.tiles[m.tile].hex().distance(h) < MARKET.spacing) {
            continue;
        }
        let mid = raise(w, tile);
        w.event(EventKind::MarketFormed, tile, None, Some(mid), None);
    }
}

/// Opens a market on `tile` this turn, with no traders yet; returns its id.
pub fn raise(w: &mut World, tile: usize) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let m = Market {
        id,
        tile,
        founded: w.turn,
        prices: [1.0; N],
        goods: Vec::new(),
        activity: 2.0 * MARKET.min_attendance,
        volume: 0.0,
        traders: 0,
        moved: 0.0,
    };
    w.markets.insert(id, m);
    id
}
