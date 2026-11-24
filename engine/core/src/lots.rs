//! Lots: where the salt and obsidian in a store came from. A store (a clan's, a civ's, a settlement's, a
//! unit's pack) holds each ware as a few lots: shares of the good, each with the tile it was dug at and the
//! markets it changed hands at since, oldest first. Shares, not amounts: spoiling, eating, arming and any
//! leaving take a good in proportion, so they leave the shares as they are. Only an arrival writes
//! (`mix`): a dig, a clearing, a levy, a merge, a daughter's share, a disband.
//!
//! With `Config.exchange` off no lot is written.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::market::Basket;

/// A share of one good in a store, and where it came from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lot {
    pub good: usize,
    /// The tile the good was dug at.
    pub source: usize,
    /// The markets it changed hands at, oldest first (at most `LOTS.chain`).
    pub markets: Vec<u32>,
    /// Its share of the good in the store, over the good's lots: they sum to 1.
    pub share: f64,
}

/// A store's lots: its goods' lots, ordered by good, then by share, largest first.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Lots(pub Vec<Lot>);

impl Lots {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// `good`'s lots.
    pub fn of(&self, good: usize) -> impl Iterator<Item = &Lot> {
        self.0.iter().filter(move |l| l.good == good)
    }

    /// The lots of a good dug at `source`: one lot of the whole.
    pub fn dug(good: usize, source: usize) -> Lots {
        Lots(vec![Lot { good, source, markets: Vec::new(), share: 1.0 }])
    }

    /// A copy of `good`'s lots with `market` added to each chain; a chain over `LOTS.chain` loses its oldest
    /// market.
    pub fn traded_at(&self, good: usize, market: u32) -> Lots {
        Lots(
            self.of(good)
                .map(|l| {
                    let mut markets = l.markets.clone();
                    markets.push(market);
                    if markets.len() > LOTS.chain {
                        markets.remove(0);
                    }
                    Lot { markets, ..l.clone() }
                })
                .collect(),
        )
    }

    /// Mixes `amount_in` of `good` arriving with `lots_in` into this store, which held `held` of it: each
    /// lot's share is reweighted by amount, equal chains merge, and past `LOTS.most` the smallest are dropped.
    /// A good arriving with no lots (nothing known of it) adds none, and the shares are renormalised.
    pub fn mix(&mut self, good: usize, held: f64, amount_in: f64, lots_in: &Lots) {
        if amount_in <= 0.0 {
            return;
        }
        let mut mixed: Vec<Lot> = Vec::new();
        let mut add = |lot: &Lot, weight: f64| {
            let share = lot.share * weight;
            if share <= 0.0 {
                return;
            }
            match mixed.iter_mut().find(|m| m.source == lot.source && m.markets == lot.markets) {
                Some(m) => m.share += share,
                None => mixed.push(Lot { share, ..lot.clone() }),
            }
        };
        for lot in self.of(good) {
            add(lot, held.max(0.0));
        }
        for lot in lots_in.of(good) {
            add(lot, amount_in);
        }
        let all: f64 = mixed.iter().map(|l| l.share).sum();
        if all <= 0.0 {
            return;
        }
        mixed.sort_by(|a, b| b.share.total_cmp(&a.share).then(a.source.cmp(&b.source)).then(a.markets.cmp(&b.markets)));
        mixed.truncate(LOTS.most);
        let kept: f64 = mixed.iter().map(|l| l.share).sum();
        for l in &mut mixed {
            l.share /= kept;
        }
        self.0.retain(|l| l.good != good);
        self.0.extend(mixed);
        self.0.sort_by(|a, b| a.good.cmp(&b.good).then(b.share.total_cmp(&a.share)));
    }

    /// Mixes in every ware of `got` arriving with `from`'s lots, into a store that held `held`: the one call
    /// for a basket of wares moving from one store to another.
    pub fn arrive(&mut self, held: &Basket, got: &Basket, from: &Lots) {
        for g in (0..N).filter(|&g| !GOOD[g].eaten && got[g] > 0.0) {
            self.mix(g, held[g], got[g], from);
        }
    }
}

/// Each row's lots after a clearing at `market`. Row `i` brought `brought[i]` and kept `kept[i]` at home,
/// and has `after[i]` of what it brought. Each ware's sellers (rows with less after than they brought) are
/// pooled by what each sold, the pool takes `market` on its chain, and every buyer mixes it in by what it
/// bought into what it still holds. Rows that bought no ware keep their lots.
pub fn cleared(rows: &[Lots], kept: &[Basket], brought: &[Basket], after: &[Basket], market: u32) -> Vec<Lots> {
    let mut out = rows.to_vec();
    for g in (0..N).filter(|&g| !GOOD[g].eaten) {
        let mut pool = Lots::default();
        let mut sold = 0.0;
        for (i, row) in rows.iter().enumerate() {
            let s = brought[i][g] - after[i][g];
            if s > 1e-12 {
                pool.mix(g, sold, s, row);
                sold += s;
            }
        }
        if sold <= 0.0 {
            continue;
        }
        let pool = pool.traded_at(g, market);
        for i in 0..rows.len() {
            let bought = after[i][g] - brought[i][g];
            if bought > 1e-12 {
                let held = kept[i][g] + brought[i][g].min(after[i][g]);
                out[i].mix(g, held, bought, &pool);
            }
        }
    }
    out
}
