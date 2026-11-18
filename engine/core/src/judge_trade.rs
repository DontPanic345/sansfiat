//! The `trade` group: what trades with whom, from the pick to +200. A trade is one row's part in a clearing
//! in which any good moved for it; a good is in a trade if that row gave or took it. Each good's share by count
//! is the trades it is in over the sum of every good's; by worth, its volume times the price it cleared at over
//! the sum. Also: the army-turns that bought food at a market that is not their civ's own (its capital market or
//! a settlement's), with the food they bought beside what those armies needed.

use serde::Serialize;

use crate::data::*;
use crate::settle;
use crate::units::UnitKind;
use crate::world::World;

/// Less than this moved is no part of a trade.
const MOVED: f64 = 1e-9;

/// One good's shares of the trades.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct TradeGood {
    pub good: String,
    /// Its share of the trades it is in, by count and by worth, at +200 and over the watch.
    pub count_200: f64,
    pub worth_200: f64,
    pub count_watch: f64,
    pub worth_watch: f64,
}

/// The trade group of one game.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct TradeScores {
    pub goods: Vec<TradeGood>,
    /// Trades from the pick to +200, and those at +200.
    pub trades: u32,
    pub trades_200: u32,
    /// The largest share any one good has at +200, by count and by worth, and which good.
    pub largest_count_200: f64,
    pub largest_count_good: String,
    pub largest_worth_200: f64,
    pub largest_worth_good: String,
    /// Army-turns from the pick to +200 that bought food at a market not their civ's own, the food those
    /// trades bought, and what the armies making them needed (their people's food, `UNIT.food` a person).
    pub army_turns_foreign: u32,
    pub food_foreign: f64,
    pub need_foreign: f64,
    /// Army-turns that bought food at their civ's own markets.
    pub army_turns_own: u32,
    /// Unit-turns (armies and scouts) that bought food at any market.
    pub unit_turns: u32,
    /// Fill orders given from the pick to +200 (the armies' pack salt, by the planner or the player).
    pub fills: u32,
}

/// The watch's trade counts.
#[derive(Clone, Debug)]
pub struct Trade {
    pick: u32,
    /// The world's Fill orders at the last turn read.
    fills: u32,
    /// Per good: the trades it is in, and its worth moved, over the watch and at +200.
    count: [f64; N],
    worth: [f64; N],
    count_200: [f64; N],
    worth_200: [f64; N],
    scores: TradeScores,
}

impl Trade {
    pub fn pick(w: &World) -> Trade {
        Trade {
            pick: w.turn,
            fills: fills(w),
            count: [0.0; N],
            worth: [0.0; N],
            count_200: [0.0; N],
            worth_200: [0.0; N],
            scores: TradeScores::default(),
        }
    }

    /// Reads the turn that has just ended.
    pub fn turn(&mut self, w: &World) {
        let after = w.turn - self.pick;
        if after > 200 {
            return;
        }
        let at_200 = after == 200;
        for m in w.markets.values() {
            for c in &m.cleared {
                let mut volume = [0.0; N];
                for net in &c.nets {
                    if !(0..N).any(|g| net[g].abs() > MOVED) {
                        continue;
                    }
                    self.scores.trades += 1;
                    self.scores.trades_200 += at_200 as u32;
                    for g in (0..N).filter(|&g| net[g].abs() > MOVED) {
                        self.count[g] += 1.0;
                        self.count_200[g] += at_200 as u32 as f64;
                        volume[g] += net[g].max(0.0);
                    }
                }
                for g in 0..N {
                    let worth = volume[g] * c.prices[g];
                    self.worth[g] += worth;
                    self.worth_200[g] += worth * at_200 as u32 as f64;
                }
            }
        }
        let now = fills(w);
        self.scores.fills += now - self.fills;
        self.fills = now;
        for u in w.units.values() {
            let Some(t) = u.traded.as_ref().filter(|t| t.food > 1e-6) else { continue };
            self.scores.unit_turns += 1;
            if u.kind() != UnitKind::Camp {
                continue;
            }
            let own = w.civs.get(&u.civ).map(|c| c.market) == Some(t.market)
                || settle::of(w, u.civ).any(|s| s.market == t.market);
            if own {
                self.scores.army_turns_own += 1;
            } else {
                self.scores.army_turns_foreign += 1;
                self.scores.food_foreign += t.food;
                self.scores.need_foreign += u.people * UNIT.food;
            }
        }
    }

    pub fn scores(&self) -> TradeScores {
        let mut s = self.scores.clone();
        s.goods = (0..N)
            .map(|g| TradeGood {
                good: GOODS[g].to_string(),
                count_200: share(&self.count_200, g),
                worth_200: share(&self.worth_200, g),
                count_watch: share(&self.count, g),
                worth_watch: share(&self.worth, g),
            })
            .collect();
        let largest = |f: fn(&TradeGood) -> f64| {
            s.goods.iter().max_by(|a, b| f(a).total_cmp(&f(b))).map(|g| (f(g), g.good.clone())).unwrap_or_default()
        };
        (s.largest_count_200, s.largest_count_good) = largest(|g| g.count_200);
        (s.largest_worth_200, s.largest_worth_good) = largest(|g| g.worth_200);
        s
    }
}

/// The Fill orders the world has been given.
fn fills(w: &World) -> u32 {
    w.orders.values().map(|k| k.get("fill").copied().unwrap_or(0)).sum()
}

/// Good `g`'s share of the total of `xs`; 0 with none.
fn share(xs: &[f64; N], g: usize) -> f64 {
    let total: f64 = xs.iter().sum();
    if total > 0.0 {
        xs[g] / total
    } else {
        0.0
    }
}
