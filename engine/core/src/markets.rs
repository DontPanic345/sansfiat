//! Markets: where clans meet to trade, and in the pre-game a festival. A market forms where
//! clans have lived a long time with enough of them in reach, clans go when it's worth the walk,
//! it clears their baskets by barter, and it fades when nobody trades there.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::clans::nutrition;
use crate::data::*;
use crate::land::{cost_map, passable};
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

/// Path cost from the market's tile over passable tiles, out to `MARKET.reach`.
pub fn reach(w: &World, tile: usize) -> BTreeMap<usize, f64> {
    cost_map(w, &[tile], MARKET.reach, |i| passable(w, i))
}

/// For each clan with a round tile in some market's reach: that market's id and the least path
/// cost, the nearest market, ties to the lower id. A tile's claim is the clan whose round it's on.
fn nearest(w: &World) -> BTreeMap<u32, (u32, f64)> {
    let mut best: BTreeMap<u32, (u32, f64)> = BTreeMap::new();
    for m in w.markets.values() {
        for (t, c) in reach(w, m.tile) {
            let Some(clan) = w.tiles[t].claim else { continue };
            // Markets come in id order, so only a strictly cheaper one replaces.
            if best.get(&clan).is_none_or(|&(_, bc)| c < bc) {
                best.insert(clan, (m.id, c));
            }
        }
    }
    best
}

/// What a clan bringing `b` would gain in food from trading at `m`'s last prices, over its own
/// foods and the goods the market last traded, by nutrition for `pop` people.
pub fn gain(m: &Market, b: &Basket, pop: f64) -> f64 {
    let goods: Vec<usize> = (0..N).filter(|&f| b[f] > 0.0 || m.goods.contains(&f)).collect();
    let d = demand(b, &m.prices, &goods);
    (nutrition(&d, pop) - nutrition(b, pop)).max(0.0)
}

/// The `go` step, before harvest: each clan goes to the nearest market in reach of its round if
/// what it would gain there, in labour, plus its desire to gather, beats the walk there and back.
pub fn go(w: &mut World) {
    let near = nearest(w);
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let c = &w.clans[&id];
        let choice = near.get(&id).and_then(|&(mid, cost)| {
            let m = &w.markets[&mid];
            let b: Basket = std::array::from_fn(|f| c.store[f] + c.harvest[f]);
            let surplus = c.store.iter().sum::<f64>() >= MARKET.surplus_at * c.pop;
            let bonus = if surplus { MARKET.gather_bonus } else { 0.0 };
            let trip = 2.0 * cost;
            (gain(m, &b, c.pop) * CLAN.harvest_labour + bonus > trip).then_some((mid, trip))
        });
        let c = w.clans.get_mut(&id).unwrap();
        c.market = choice.map(|(mid, _)| mid);
        c.trip = choice.map_or(0.0, |(_, trip)| trip);
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
                c.trip = 0.0;
            }
        }
    }
}

/// The clans with a round tile in `tile`'s reach, in id order, and their people.
pub fn in_reach(w: &World, tile: usize) -> (Vec<u32>, f64) {
    let mut ids: Vec<u32> = reach(w, tile).keys().filter_map(|&t| w.tiles[t].claim).collect();
    ids.sort_unstable();
    ids.dedup();
    let people = ids.iter().map(|id| w.clans[id].pop).sum();
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
