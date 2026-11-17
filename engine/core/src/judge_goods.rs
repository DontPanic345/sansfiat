//! The `goods` group: how far salt and obsidian move. The share of festivals, from the pick to +200,
//! where either changed hands; and at +200, for each, who holds it and how far from its nearest source.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::data::*;
use crate::sources;
use crate::world::World;

/// Less than this of a good in a store is not counted as holding it.
const HELD: f64 = 0.01;

/// One good that isn't eaten.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GoodScores {
    pub good: String,
    /// Its sources on the map, and those a clan holds at +200.
    pub sources: u32,
    pub sources_held_200: u32,
    /// Festival-turns it changed hands at, from the pick to +200.
    pub festivals: u32,
    /// What all stores hold of it at +200, the stores holding any, and those of them that hold no
    /// source of it: stores it reached by trade or the levy.
    pub held_200: f64,
    pub stores_200: u32,
    pub stores_away_200: u32,
    /// Hex steps from a store holding it to its nearest source at +200: the mean over the stores, and
    /// the furthest.
    pub mean_steps_200: f64,
    pub max_steps_200: i32,
    /// Civ stores (capitals and settlements) holding it at +200, and the most any civ store held over
    /// the watch.
    pub civ_stores_200: u32,
    pub civ_most: f64,
}

/// The goods group of one game.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GoodsScores {
    pub goods: Vec<GoodScores>,
    /// Festival-turns with two or more clans, from the pick to +200, and those where salt or obsidian
    /// changed hands.
    pub festivals: u32,
    pub festivals_wares: u32,
    /// Armies that ever had warriors armed with obsidian.
    pub armies_armed: u32,
}

/// The watch's goods counts.
#[derive(Clone, Debug)]
pub struct Goods {
    pick: u32,
    armed: BTreeSet<u32>,
    scores: GoodsScores,
}

fn wares() -> impl Iterator<Item = usize> {
    (0..N).filter(|&g| !GOOD[g].eaten)
}

impl Goods {
    pub fn pick(w: &World) -> Goods {
        let goods = wares()
            .map(|g| GoodScores {
                good: GOODS[g].to_string(),
                sources: sources::of(w, g).len() as u32,
                ..GoodScores::default()
            })
            .collect();
        Goods { pick: w.turn, armed: BTreeSet::new(), scores: GoodsScores { goods, ..GoodsScores::default() } }
    }

    /// Reads the turn that has just ended.
    pub fn turn(&mut self, w: &World) {
        let after = w.turn - self.pick;
        if after > 200 {
            return;
        }
        for m in w.markets.values().filter(|m| m.traders >= 2) {
            self.scores.festivals += 1;
            self.scores.festivals_wares += (m.wares > 0.0) as u32;
            for (k, g) in wares().enumerate() {
                self.scores.goods[k].festivals += (m.ware[g] > 0.0) as u32;
            }
        }
        self.armed.extend(w.units.values().filter(|u| u.camp().is_some_and(|c| c.armed > 0.0)).map(|u| u.id));
        self.scores.armies_armed = self.armed.len() as u32;
        let civ_stores = || {
            let capitals = w.civs.values().filter(|c| !c.is_out()).map(|c| (c.capital, &c.store));
            capitals.chain(w.settlements.values().map(|s| (s.tile, &s.store)))
        };
        for (k, g) in wares().enumerate() {
            let s = &mut self.scores.goods[k];
            for (_, store) in civ_stores() {
                s.civ_most = s.civ_most.max(store[g]);
            }
            if after != 200 {
                continue;
            }
            let from = sources::of(w, g);
            s.sources_held_200 = from.iter().filter(|&&t| w.tiles[t].claim.is_some()).count() as u32;
            let clans = w.clans.values().map(|c| (c.home, &c.store, c.territory.iter().any(|t| from.contains(t))));
            let mut steps = 0;
            for (tile, store, holds_source) in clans.chain(civ_stores().map(|(t, s)| (t, s, false))) {
                if store[g] < HELD {
                    continue;
                }
                s.held_200 += store[g];
                s.stores_200 += 1;
                s.stores_away_200 += !holds_source as u32;
                let d = sources::steps_from(w, g, tile).unwrap_or(0);
                steps += d;
                s.max_steps_200 = s.max_steps_200.max(d);
            }
            s.mean_steps_200 = if s.stores_200 > 0 { f64::from(steps) / f64::from(s.stores_200) } else { 0.0 };
            s.civ_stores_200 = civ_stores().filter(|(_, store)| store[g] >= HELD).count() as u32;
        }
    }

    pub fn scores(&self) -> GoodsScores {
        self.scores.clone()
    }
}
