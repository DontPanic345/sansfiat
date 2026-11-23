//! Exchange: an army's line may end at a market that isn't a rival's, as at a store of its civ. Its civ pays it
//! in wares carried along its line home (`pay`), which weigh less than the food they buy, for the market nearest
//! it, where it keeps a stall (`Stall`). It posts them in that market's clearing (`unit_rows`), the food it buys
//! waits at its stall, and its line ends there when that feeds it better than its store (`units::line`). Only
//! with `Config.exchange`; off is the control.

use serde::{Deserialize, Serialize};

use crate::actors::{keeps_back, walk_cost, Actor, Carrier};
use crate::civs;
use crate::clans::Clan;
use crate::data::*;
use crate::land::cost_map;
use crate::lots::{self, Lots};
use crate::market::{add, total, wares, Basket};
use crate::markets::Market;
use crate::settle;
use crate::units::{cap_of, line, Unit, UnitKind};
use crate::world::World;

/// What a unit's pack did at a market last turn: the market, the food it bought and the wares it sold.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnitTrade {
    pub market: u32,
    pub food: f64,
    pub sold: Basket,
}

/// The market a unit is paid to buy at, and the food it bought there that its line hasn't brought yet. The food
/// spoils as a store does, and a unit paid for another market leaves it behind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stall {
    pub market: u32,
    pub food: Basket,
}

/// A unit in a clearing: what its pack brings, and what stays in it.
pub struct UnitRow {
    pub id: u32,
    pub brought: Basket,
    pub kept: Basket,
}

/// Whose lots a clearing's row holds.
#[derive(Clone, Copy, Debug)]
pub enum Owner {
    Clan(u32),
    Unit(u32),
    Civ(u32),
}

/// What army `id`'s civ pays it this turn (`pay_plan`): the market it pays for, the store it pays from, the
/// path cost of the line home to that store, and the wares.
pub struct PayPlan {
    pub market: u32,
    pub store: usize,
    pub cost: f64,
    pub wares: Basket,
}

/// The lots of civ `civ`'s store on `tile`: the capital's or a settlement's of its own there.
pub fn store_lots(w: &World, civ: u32, tile: usize) -> Option<&Lots> {
    let c = w.civs.get(&civ)?;
    if c.capital == tile {
        return Some(&c.actor.lots);
    }
    settle::of(w, civ).find(|s| s.tile == tile).map(|s| &s.lots)
}

/// The start of the `trade` step: no unit has traded yet this turn.
pub fn begin(w: &mut World) {
    for u in w.units.values_mut() {
        u.traded = None;
    }
}

/// Whether market `mid` is a rival's to `civ`: another living civ's capital market or settlement market.
pub fn rival(w: &World, civ: u32, mid: u32) -> bool {
    civs::alive(w).any(|c| c.market == mid && c.id != civ)
        || settle::of_market(w, mid).is_some_and(|s| s.civ != civ && w.civs.get(&s.civ).is_some_and(|c| !c.is_out()))
}

/// The food a unit of each good buys at `m`'s last prices: its price over the mean price of the foods
/// traded there, 1 with no prices yet.
fn food_price(m: &Market, g: usize) -> f64 {
    let foods: Vec<f64> = m.goods.iter().filter(|&&f| GOOD[f].eaten).map(|&f| m.prices[f]).collect();
    if foods.is_empty() {
        return 1.0;
    }
    m.prices[g] / (foods.iter().sum::<f64>() / foods.len() as f64)
}

/// What `held`'s wares buy at `m`'s last prices, in food: nothing for a good `m` didn't trade.
fn worth_at(m: &Market, held: &Basket) -> f64 {
    (0..N).filter(|&g| !GOOD[g].eaten && m.goods.contains(&g)).map(|g| held[g] * food_price(m, g)).sum()
}

/// The food in unit `u`'s stall at market `mid`: none when its stall is at another market.
pub fn stall_at(u: &Unit, mid: u32) -> f64 {
    u.stall.as_ref().filter(|s| s.market == mid).map_or(0.0, |s| total(&s.food))
}

/// Whether market `m` may end unit `u`'s line: exchange is on, `u` is an army, and the market is neither a
/// rival's nor on a store of its civ, where the line ends at the store.
fn may_end(w: &World, u: &Unit, m: &Market) -> bool {
    w.config.exchange
        && u.kind() == UnitKind::Camp
        && !rival(w, u.civ, m.id)
        && !settle::roots(w, u.civ).contains(&m.tile)
}

/// The market that may end army `u`'s line this turn, by tile, with the food waiting at its stall there: none when
/// its stall holds no food or `may_end` refuses it.
pub fn market_end(w: &World, u: &Unit) -> Option<(usize, f64)> {
    let s = u.stall.as_ref()?;
    let m = w.markets.get(&s.market).filter(|m| may_end(w, u, m))?;
    let food = total(&s.food);
    (food > 1e-9).then_some((m.tile, food))
}

/// What army `id`'s civ would pay it this turn, before the labour that carries it. An army is paid when its line
/// home can't carry a turn of its food: for the nearest market that may end its line (`may_end`) within the reach
/// of a line that carries a turn of its food, from the store its line home ends at. A ware is pay when that market
/// traded it last turn and a unit of it buys more food there than its own weight in food; the best by that
/// measure goes first, until what the pack has room for, less what its stall and its wares already buy, is met.
pub fn pay_plan(w: &World, id: u32) -> Option<PayPlan> {
    let u = w.units.get(&id)?;
    if !w.config.exchange || u.kind() != UnitKind::Camp {
        return None;
    }
    let eats = u.people * UNIT.food;
    let (path, cost, carry) = line::store_line(w, id)?;
    if carry + 1e-9 >= eats {
        return None;
    }
    let store = path.last().copied().unwrap_or(u.home);
    let held = settle::store(w, u.civ, store)?;
    let reach = u.people * UNIT.labour * Actor::carry(Carrier::Warrior).load / eats / 2.0;
    let near = cost_map(w, &[u.home], reach, |i| line::may_cross(w, u.civ, i));
    let m = w
        .markets
        .values()
        .filter(|m| may_end(w, u, m))
        .filter_map(|m| near.get(&m.tile).map(|&c| (c, m)))
        .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.id.cmp(&b.1.id)))?
        .1;
    let room = (cap_of(u) - total(&u.store)).max(0.0);
    let mut want = room - stall_at(u, m.id) - worth_at(m, &u.store);
    let per_weight = |g: usize| food_price(m, g) / GOOD[g].weight;
    let mut goods: Vec<usize> =
        (0..N).filter(|&g| !GOOD[g].eaten && held[g] > 1e-9 && m.goods.contains(&g) && per_weight(g) > 1.0).collect();
    goods.sort_by(|&a, &b| per_weight(b).total_cmp(&per_weight(a)).then(a.cmp(&b)));
    let mut wares = [0.0; N];
    for g in goods {
        if want <= 1e-9 {
            break;
        }
        wares[g] = held[g].min(want / food_price(m, g));
        want -= wares[g] * food_price(m, g);
    }
    (wares.iter().sum::<f64>() > 1e-9).then_some(PayPlan { market: m.id, store, cost, wares })
}

/// The salt civ `civ`'s store on `tile` keeps back from its keeping this turn: what its armies' pay would take
/// (`pay_plan`), so the pay is drawn before the store's own keeping uses it. Nothing with exchange off.
pub fn pay_back(w: &World, civ: u32, tile: usize) -> f64 {
    if !w.config.exchange {
        return 0.0;
    }
    w.units
        .values()
        .filter(|u| u.civ == civ)
        .filter_map(|u| pay_plan(w, u.id))
        .filter(|p| p.store == tile)
        .map(|p| (0..N).filter(|&g| GOOD[g].uses == Use::Keeps).map(|g| p.wares[g]).sum::<f64>())
        .sum()
}

/// Carries army `id`'s pay (`pay_plan`) from the store along its line home on at most `labour`, each ware at its
/// weight. Returns the labour used.
pub fn pay(w: &mut World, id: u32, labour: f64) -> f64 {
    let Some(plan) = pay_plan(w, id) else { return 0.0 };
    let weight: f64 = (0..N).map(|g| plan.wares[g] * GOOD[g].weight).sum();
    let full = walk_cost(plan.cost, weight) / Actor::carry(Carrier::Warrior).load;
    let share = if full > labour { (labour / full).max(0.0) } else { 1.0 };
    let wares: Basket = plan.wares.map(|v| v * share);
    if wares.iter().sum::<f64>() <= 1e-9 {
        return 0.0;
    }
    let civ = w.units[&id].civ;
    let from = store_lots(w, civ, plan.store).cloned().unwrap_or_default();
    let store = settle::store_mut(w, civ, plan.store).expect("planned: a store");
    for g in 0..N {
        store[g] = (store[g] - wares[g]).max(0.0);
    }
    let u = w.units.get_mut(&id).unwrap();
    let held = u.actor.store;
    u.actor.lots.arrive(&held, &wares, &from);
    add(&mut u.actor.store, &wares);
    if u.stall.as_ref().is_none_or(|s| s.market != plan.market) {
        u.stall = Some(Stall { market: plan.market, food: [0.0; N] });
    }
    let what: Vec<String> =
        (0..N).filter(|&g| wares[g] > 1e-9).map(|g| format!("{:.2} {}", wares[g], GOODS[g])).collect();
    let name = settle::store_name(w, civ, plan.store);
    let at = w.at(w.markets[&plan.market].tile);
    w.note(id, format!("{name} paid it {} to buy food at the festival at {at}", what.join(" and ")));
    full * share
}

/// The units that join market `mid`'s clearing, in id order: armies with their stall there, or that stand on it,
/// where `may_end` allows it, with wares in their pack. An army brings its wares, scaled down together to what buys
/// its pack's room less what its stall holds at the market's last prices (all of them when the market has traded
/// nothing yet). It keeps its food and the rest.
pub fn unit_rows(w: &World, mid: u32) -> Vec<UnitRow> {
    let m = &w.markets[&mid];
    w.units
        .values()
        .filter(|u| may_end(w, u, m) && (u.home == m.tile || u.stall.as_ref().is_some_and(|s| s.market == mid)))
        .filter_map(|u| {
            let held: Basket = std::array::from_fn(|g| if GOOD[g].eaten { 0.0 } else { u.store[g] });
            if wares(&held) <= 1e-9 {
                return None;
            }
            let want = (cap_of(u) - total(&u.store)).max(0.0) - stall_at(u, mid);
            if want <= 1e-9 {
                return None;
            }
            let value: f64 = (0..N).map(|g| held[g] * food_price(m, g)).sum();
            let scale = if m.goods.is_empty() || value <= want { 1.0 } else { want / value };
            let brought: Basket = held.map(|v| v * scale);
            Some(UnitRow { id: u.id, brought, kept: std::array::from_fn(|g| u.store[g] - brought[g]) })
        })
        .collect()
}

/// Puts each unit's part of a clearing where it goes: its wares, kept and unsold, in its pack, and the food it
/// bought in its stall at the market; and notes the trade.
pub fn apply_units(w: &mut World, mid: u32, rows: &[UnitRow], after: &[Basket]) {
    let at = w.at(w.markets[&mid].tile);
    for (row, after) in rows.iter().zip(after) {
        let food: Basket = std::array::from_fn(|g| if GOOD[g].eaten { after[g] } else { 0.0 });
        let got = total(&food);
        let sold: Basket = std::array::from_fn(|g| (row.brought[g] - after[g]).max(0.0));
        let u = w.units.get_mut(&row.id).unwrap();
        u.actor.store = std::array::from_fn(|g| row.kept[g] + after[g] - food[g]);
        match u.stall.as_mut().filter(|s| s.market == mid) {
            Some(s) => add(&mut s.food, &food),
            None if got > 1e-9 => u.stall = Some(Stall { market: mid, food }),
            None => {}
        }
        u.traded = Some(UnitTrade { market: mid, food: got, sold });
        if got > 1e-9 {
            let what: Vec<&str> = (0..N).filter(|&g| sold[g] > 1e-9).map(|g| GOODS[g]).collect();
            w.note(row.id, format!("bought {got:.1} food at the festival at {at} with {}", what.join(" and ")));
        }
    }
}

/// Writes each row's lots after the clearing at `mid` (`lots::cleared`): `owners` are the rows' holders, in
/// row order, `kept` what each held back (and the civ row's store).
pub fn pass_lots(w: &mut World, mid: u32, owners: &[Owner], kept: &[Basket], brought: &[Basket], after: &[Basket]) {
    let lots_of = |w: &World, o: &Owner| -> Lots {
        match *o {
            Owner::Clan(id) => w.clans[&id].lots.clone(),
            Owner::Unit(id) => w.units[&id].lots.clone(),
            Owner::Civ(id) => w.civs[&id].actor.lots.clone(),
        }
    };
    let rows: Vec<Lots> = owners.iter().map(|o| lots_of(w, o)).collect();
    let out = lots::cleared(&rows, kept, brought, after, mid);
    for ((o, new), old) in owners.iter().zip(out).zip(rows) {
        if new == old {
            continue;
        }
        match *o {
            Owner::Clan(id) => w.clans.get_mut(&id).unwrap().actor.lots = new,
            Owner::Unit(id) => w.units.get_mut(&id).unwrap().actor.lots = new,
            Owner::Civ(id) => w.civs.get_mut(&id).unwrap().actor.lots = new,
        }
    }
}

/// What clan `c` brings to a festival, and what it keeps at home. It brings its salt and obsidian (less the salt
/// its own store will use, `actors::keeps_back`), and its food scaled evenly down to what `MARKET.carry ×
/// people` of weight leaves (`GOOD`'s weights).
pub fn bring(c: &Clan) -> (Basket, Basket) {
    let back = keeps_back(&c.basket, c.people);
    let light: f64 = (0..N).filter(|&g| !GOOD[g].eaten).map(|g| (c.basket[g] - back[g]) * GOOD[g].weight).sum();
    let total = total(&c.basket);
    let most = (MARKET.carry * c.people - light).max(0.0);
    let share = if total > most { most / total } else { 1.0 };
    let part = |g: usize| {
        if GOOD[g].eaten {
            share
        } else if c.basket[g] > 0.0 {
            1.0 - back[g] / c.basket[g]
        } else {
            0.0
        }
    };
    (std::array::from_fn(|g| c.basket[g] * part(g)), std::array::from_fn(|g| c.basket[g] - c.basket[g] * part(g)))
}

/// The unit's last trade, for a unit that traded at all.
pub fn last(u: &Unit) -> Option<&UnitTrade> {
    u.traded.as_ref()
}
