//! Markets: where clans meet to trade, and in the pre-game a festival. A market forms where
//! clans have lived a long time with enough of them in reach; each clan walks one route to the
//! one worth most to it, if any is worth the walk; it clears their baskets by barter, and it
//! fades when nobody trades there.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::actors::{walk_cost, Route};
use crate::clans::{nutrition, Clan};
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
    /// Volume smoothed over turns.
    pub activity: f64,
    /// Last turn's volume, summed over the goods.
    pub volume: f64,
    /// Clans that came last turn.
    pub traders: usize,
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

/// The `trade` step, after harvest: each market clears the baskets its clans brought.
pub fn trade(w: &mut World) {
    let mids: Vec<u32> = w.markets.keys().copied().collect();
    for mid in mids {
        let ids: Vec<u32> = w.clans.values().filter(|c| c.market == Some(mid)).map(|c| c.id).collect();
        let baskets: Vec<Basket> = ids.iter().map(|id| w.clans[id].basket).collect();
        let m = w.markets.get_mut(&mid).unwrap();
        let c = clear(&baskets, &m.prices, &Params::default());
        if c.cleared {
            m.prices = c.prices;
            m.goods = c.goods.clone();
        }
        m.volume = c.volume.iter().sum();
        m.traders = ids.len();
        m.activity = MARKET.fade * m.activity + (1.0 - MARKET.fade) * m.volume;
        for (id, after) in ids.iter().zip(c.after) {
            w.clans.get_mut(id).unwrap().basket = after;
        }
    }
}

/// The `markets` step: markets nobody trades at go, then at most one forms.
pub fn form_and_fade(w: &mut World) {
    fade(w);
    form(w);
}

/// A market whose activity falls under `min_volume` goes, and its clans with it. A centre the
/// pre-game ended on stays, and so does every civ's market.
pub fn fade(w: &mut World) {
    let mut kept = w.end.as_ref().map(|e| e.centres.clone()).unwrap_or_default();
    kept.extend(w.civs.values().map(|c| c.market));
    let gone: Vec<u32> =
        w.markets.values().filter(|m| m.activity < MARKET.min_volume && !kept.contains(&m.id)).map(|m| m.id).collect();
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

/// A market forms on a clan's home held `form_after` turns, `spacing` steps from every market,
/// with `min_clans` clans in reach: the one with the most people in reach, ties to the lower tile.
pub fn form(w: &mut World) {
    let mut homes: Vec<usize> = w.clans.values().map(|c| c.home).collect();
    homes.sort_unstable();
    homes.dedup();
    let mut best: Option<(usize, f64)> = None;
    for tile in homes {
        let h = w.tiles[tile].hex();
        if w.tiles[tile].held < MARKET.form_after
            || w.markets.values().any(|m| w.tiles[m.tile].hex().distance(h) < MARKET.spacing)
        {
            continue;
        }
        let (ids, people) = in_reach(w, tile);
        if ids.len() >= MARKET.min_clans && best.is_none_or(|(_, bp)| people > bp) {
            best = Some((tile, people));
        }
    }
    let Some((tile, _)) = best else { return };
    let id = raise(w, tile);
    w.event(EventKind::MarketFormed, tile, None, Some(id), None);
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
        activity: 2.0 * MARKET.min_volume,
        volume: 0.0,
        traders: 0,
    };
    w.markets.insert(id, m);
    id
}
