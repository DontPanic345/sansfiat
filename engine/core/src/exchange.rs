//! Exchange: goods that pass between markets. A unit on a market's tile joins its clearing as a row with
//! its pack's wares to sell and a want for food (`unit_rows`); a clan whose wares its own festival didn't
//! take walks a second route to a festival that did (`carry_route`) and trades them there, after the first
//! clearing (`carriers`). Both only with `Config.exchange`; off is the control.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::actors::{carriers as walkers_of, keeps_back, walk_cost, Carrier, Route};
use crate::civs;
use crate::clans::Clan;
use crate::data::*;
use crate::lots::{self, Lots};
use crate::market::{total, wares, weight, Basket};
use crate::markets::{Festival, Market};
use crate::settle;
use crate::units::{cap_of, Unit};
use crate::world::World;

/// What a unit's pack did at a market last turn: the market, the food it bought and the wares it sold.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnitTrade {
    pub market: u32,
    pub food: f64,
    pub sold: Basket,
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

/// The salt a civ's store keeps back from its keeping for its packs: `KEEPING.packs` with exchange on, else 0.
pub fn packs(w: &World) -> f64 {
    if w.config.exchange {
        KEEPING.packs
    } else {
        0.0
    }
}

/// The lots of civ `civ`'s store on `tile`: the capital's or a settlement's of its own there.
pub fn store_lots(w: &World, civ: u32, tile: usize) -> Option<&Lots> {
    let c = w.civs.get(&civ)?;
    if c.capital == tile {
        return Some(&c.actor.lots);
    }
    settle::of(w, civ).find(|s| s.tile == tile).map(|s| &s.lots)
}

/// The salt a unit's pack has room for: `UNIT.salt` a person, less what it holds.
pub fn salt_room(u: &Unit) -> f64 {
    (UNIT.salt * u.people - u.store[SALT]).max(0.0)
}

/// Checks `Order::Fill` for unit `u` of `civ`: exchange is on, it stands on a store of its civ with salt in
/// it, and its pack has room for salt.
pub fn check_fill(w: &World, civ: u32, u: &Unit) -> Result<(), String> {
    if !w.config.exchange {
        return Err("exchange is off".to_string());
    }
    let Some(store) = settle::store(w, civ, u.tile()) else {
        return Err("it stands off a store: a unit fills its pack at its civ's capital or a settlement".to_string());
    };
    if store[SALT] <= 1e-9 {
        return Err("there is no salt in that store".to_string());
    }
    if salt_room(u) <= 1e-9 {
        return Err("its pack's salt is full".to_string());
    }
    Ok(())
}

/// Applies `Order::Fill` for unit `id`, checked: it takes what salt the store has, up to its room, with the
/// store's lots (`Lots::mix`), and the store keeps the rest.
pub fn fill(w: &mut World, civ: u32, id: u32) {
    let (tile, room) = {
        let u = &w.units[&id];
        (u.tile(), salt_room(u))
    };
    let have = settle::store(w, civ, tile).map_or(0.0, |s| s[SALT]);
    let got = room.min(have);
    if got <= 0.0 {
        return;
    }
    let from = store_lots(w, civ, tile).cloned().unwrap_or_default();
    settle::store_mut(w, civ, tile).expect("checked: a store")[SALT] -= got;
    let u = w.units.get_mut(&id).expect("checked: a unit");
    let held = u.store[SALT];
    u.actor.store[SALT] += got;
    u.actor.lots.mix(SALT, held, got, &from);
    let name = settle::store_name(w, civ, tile);
    w.note(id, format!("filled its pack with {got:.2} salt from {name}"));
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

/// The units that join market `mid`'s clearing: those on its tile with wares in their pack, where the market
/// is no rival's, in id order. A unit brings its wares, scaled down together to what buys its pack's room
/// at the market's last prices (all of them when the market has traded nothing yet), so the food it takes
/// fits its pack. It keeps its food and the rest.
pub fn unit_rows(w: &World, mid: u32) -> Vec<UnitRow> {
    let m = &w.markets[&mid];
    w.units
        .values()
        .filter(|u| u.tile() == m.tile && !rival(w, u.civ, mid))
        .filter_map(|u| {
            let held: Basket = std::array::from_fn(|g| if GOOD[g].eaten { 0.0 } else { u.store[g] });
            if wares(&held) <= 1e-9 {
                return None;
            }
            let room = (cap_of(u) - total(&u.store)).max(0.0);
            if room <= 1e-9 {
                return None;
            }
            let value: f64 = (0..N).map(|g| held[g] * food_price(m, g)).sum();
            let scale = if m.goods.is_empty() || value <= room { 1.0 } else { room / value };
            let brought: Basket = held.map(|v| v * scale);
            Some(UnitRow { id: u.id, brought, kept: std::array::from_fn(|g| u.store[g] - brought[g]) })
        })
        .collect()
}

/// Puts each unit's part of a clearing in its pack: what it kept and what it has after, and notes the trade.
pub fn apply_units(w: &mut World, mid: u32, rows: &[UnitRow], after: &[Basket]) {
    let at = w.at(w.markets[&mid].tile);
    for (row, after) in rows.iter().zip(after) {
        let food = total(after);
        let sold: Basket = std::array::from_fn(|g| (row.brought[g] - after[g]).max(0.0));
        let u = w.units.get_mut(&row.id).unwrap();
        u.actor.store = std::array::from_fn(|g| row.kept[g] + after[g]);
        u.traded = Some(UnitTrade { market: mid, food, sold });
        if food > 1e-9 {
            let what: Vec<&str> = (0..N).filter(|&g| sold[g] > 1e-9).map(|g| GOODS[g]).collect();
            w.note(row.id, format!("bought {food:.1} food at the festival at {at} with {}", what.join(" and ")));
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
/// people` of weight leaves (`GOOD`'s weights); with `wares_only` (a carrier at its second festival) no food.
pub fn bring(c: &Clan, wares_only: bool) -> (Basket, Basket) {
    let back = keeps_back(&c.basket, c.people);
    let light: f64 = (0..N).filter(|&g| !GOOD[g].eaten).map(|g| (c.basket[g] - back[g]) * GOOD[g].weight).sum();
    let total = total(&c.basket);
    let most = (MARKET.carry * c.people - light).max(0.0);
    let share = if wares_only {
        0.0
    } else if total > most {
        most / total
    } else {
        1.0
    };
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

/// The wares a clan would carry from `store`: each ware its own festival's `goods` lacks (so it goes home
/// unsold), that market `to` last traded if one is given, less the salt its own store will use.
fn carried(own: &Market, store: &Basket, people: f64, to: Option<&Market>) -> Basket {
    let back = keeps_back(store, people);
    std::array::from_fn(|g| {
        if GOOD[g].eaten || own.goods.contains(&g) || to.is_some_and(|t| !t.goods.contains(&g)) {
            0.0
        } else {
            (store[g] - back[g]).max(0.0)
        }
    })
}

/// The second route a clan walks, if its wares are worth it: from `festivals` (the festivals its home
/// reaches this turn) the nearest other than its own where its unsold wares last traded, if the wares'
/// worth there at the last prices, in labour, covers `PASS.cover` of the walk's cost. The walk is
/// costed on the wares' weight, as every route is. A clan that wasn't fully fed last turn walks none.
pub fn carry_route(w: &World, c: &Clan, festivals: &[Festival]) -> Option<Route> {
    // A clan that went hungry last turn has no days to spare.
    if !c.fed {
        return None;
    }
    let own = w.markets.get(&c.market?)?;
    let wares: Vec<usize> =
        (0..N).filter(|&g| !GOOD[g].eaten && c.store[g] > 1e-9 && !own.goods.contains(&g)).collect();
    if wares.is_empty() {
        return None;
    }
    let mut best: Option<(&Festival, Basket)> = None;
    for f in festivals.iter().filter(|f| f.market != own.id && !f.path.is_empty()) {
        let carry = carried(own, &c.store, c.people, Some(&w.markets[&f.market]));
        if wares.iter().any(|&g| carry[g] > 1e-9) && best.as_ref().is_none_or(|b| f.cost < b.0.cost) {
            best = Some((f, carry));
        }
    }
    let (f, carry) = best?;
    let to = &w.markets[&f.market];
    let worth: f64 = (0..N).map(|g| carry[g] * food_price(to, g)).sum::<f64>() * CLAN.harvest_labour;
    let cost = walk_cost(f.cost, 1.0) * walkers_of(weight(&carry).min(MARKET.carry * c.people), Carrier::Clan);
    (worth > PASS.cover * cost).then(|| Route { to: to.tile, path: f.path.clone(), cost })
}

/// The clans that walk a second route this turn, by the market it ends at, in id order: those whose route
/// list has a second route to a market's tile and that hold wares their own festival left unsold.
pub fn carriers(w: &World) -> BTreeMap<u32, Vec<u32>> {
    let mut out: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    if !w.config.exchange {
        return out;
    }
    for c in w.clans.values().filter(|c| c.routes.len() > 1) {
        let Some(m) = w.markets.values().find(|m| m.tile == c.routes[1].to) else { continue };
        if Some(m.id) != c.market && wares(&carried_basket(w, c)) > 1e-9 {
            out.entry(m.id).or_default().push(c.id);
        }
    }
    out
}

/// What clan `c` brings to its second market: the wares in its basket its own festival didn't take. The
/// second market's goods are the route's reason, read when the route was chosen (`carry_route`).
pub fn carried_basket(w: &World, c: &Clan) -> Basket {
    match c.market.and_then(|m| w.markets.get(&m)) {
        Some(own) => carried(own, &c.basket, c.people, None),
        None => [0.0; N],
    }
}

/// The unit's last trade, for a unit that traded at all.
pub fn last(u: &Unit) -> Option<&UnitTrade> {
    u.traded.as_ref()
}
