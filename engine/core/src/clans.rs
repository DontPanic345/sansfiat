//! Clans: small groups living off a round of tiles, paying labour to walk it and to harvest,
//! eating a varied diet, growing when fed and starving when not.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::map;
use crate::market::Basket;
use crate::rng::rand_index;
use crate::round;
use crate::world::World;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Clan {
    pub id: u32,
    pub home: usize,
    /// Distinct tiles, `round[0] == home`.
    pub round: Vec<usize>,
    pub pop: f64,
    pub store: Basket,
    pub harvest: Basket,
    /// What it has to eat this turn: last turn's store and this turn's harvest.
    pub basket: Basket,
    pub fed_ratio: f64,
    pub fed: bool,
    pub starving: bool,
    /// Starving turns in a row.
    pub hungry: u32,
    /// Turns since it last moved.
    pub since: u32,
    /// The round's cost this turn.
    pub walk: f64,
    pub market: Option<u32>,
    /// Labour for the walk to market.
    pub trip: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub take: Vec<(usize, Basket)>,
    pub food: Basket,
    pub labour: f64,
}

const EPS: f64 = 1e-9;

/// Seed clans: one on a random land tile with food in each landmass of `min_landmass`+ tiles,
/// then more on random free land tiles with food until there are `seeds`.
pub fn seed(w: &mut World) {
    let has_food = |w: &World, i: usize| w.tiles[i].cap.iter().any(|&c| c > 0.0);
    let mut homes: Vec<usize> = Vec::new();
    for mass in map::landmasses(&w.tiles, w.config.radius) {
        if mass.len() < CLAN.min_landmass {
            continue;
        }
        let fed: Vec<usize> = mass.into_iter().filter(|&i| has_food(w, i)).collect();
        if !fed.is_empty() {
            homes.push(fed[rand_index(&mut w.rng, fed.len())]);
        }
    }
    while homes.len() < CLAN.seeds {
        let free: Vec<usize> = (0..w.tiles.len())
            .filter(|&i| TERRAIN[w.tiles[i].terrain].land && has_food(w, i) && !homes.contains(&i))
            .collect();
        if free.is_empty() {
            break;
        }
        homes.push(free[rand_index(&mut w.rng, free.len())]);
    }
    for home in homes {
        let id = w.next_id;
        w.next_id += 1;
        w.tiles[home].claim = Some(id);
        w.clans.insert(id, new_clan(id, home, CLAN.start_pop));
    }
}

pub fn new_clan(id: u32, home: usize, pop: f64) -> Clan {
    Clan {
        id,
        home,
        round: vec![home],
        pop,
        store: [0.0; N],
        harvest: [0.0; N],
        basket: [0.0; N],
        fed_ratio: 1.0,
        fed: true,
        starving: false,
        hungry: 0,
        since: 0,
        walk: 0.0,
        market: None,
        trip: 0.0,
    }
}

/// What a basket is worth to `need` people: each food in full up to `diet_share × need`, at
/// `excess_value` beyond it.
pub fn nutrition(b: &Basket, need: f64) -> f64 {
    let k = CLAN.diet_share * need;
    b.iter().map(|&v| v.min(k)).sum::<f64>() + CLAN.excess_value * b.iter().map(|&v| (v - k).max(0.0)).sum::<f64>()
}

/// Picks what to harvest from `options` (tile, setup labour) with `labour`: food first by diet
/// share, then by how much it can take, each time from the tile that gives the most food per
/// labour, up to `surplus_target × pop` less the store.
pub fn plan_harvest(w: &World, pop: f64, store: &Basket, labour: f64, options: &[(usize, f64)]) -> Plan {
    let mut budget = (CLAN.surplus_target * pop - store.iter().sum::<f64>()).max(0.0);
    let mut left = labour;
    let mut offer: Vec<Basket> = options.iter().map(|&(i, _)| w.tiles[i].stock.map(|s| CLAN.take_rate * s)).collect();
    let mut opened = vec![false; options.len()];
    let mut planned: Basket = [0.0; N];
    let mut take: Vec<Basket> = vec![[0.0; N]; options.len()];

    for phase in 0..2 {
        loop {
            if left <= EPS || budget <= EPS {
                break;
            }
            let wanted: Basket = std::array::from_fn(|f| {
                if phase == 0 {
                    (CLAN.diet_share * pop - store[f] - planned[f]).max(0.0)
                } else {
                    f64::INFINITY
                }
            });
            // The best tile: most food per labour, ties to the lower tile.
            let mut best: Option<(usize, Basket, f64, f64)> = None;
            for (o, &(tile, setup)) in options.iter().enumerate() {
                let setup = if opened[o] { 0.0 } else { setup };
                if setup >= left {
                    continue;
                }
                let t: Basket = std::array::from_fn(|f| offer[o][f].min(wanted[f]));
                let amount: f64 = t.iter().sum();
                if amount <= EPS {
                    continue;
                }
                let rate = amount / (setup + CLAN.harvest_labour * amount);
                let better = match &best {
                    None => true,
                    Some((b, _, _, r)) => rate > *r || (rate == *r && tile < options[*b].0),
                };
                if better {
                    best = Some((o, t, setup, rate));
                }
            }
            let Some((o, t, setup, _)) = best else { break };
            let amount: f64 = t.iter().sum();
            let scale = 1f64.min((left - setup) / (CLAN.harvest_labour * amount)).min(budget / amount);
            opened[o] = true;
            left -= setup;
            for f in 0..N {
                let x = t[f] * scale;
                offer[o][f] = (offer[o][f] - x).max(0.0);
                take[o][f] += x;
                planned[f] += x;
            }
            left = (left - CLAN.harvest_labour * amount * scale).max(0.0);
            budget = (budget - amount * scale).max(0.0);
            if scale < 1.0 {
                break;
            }
        }
    }

    let take: Vec<(usize, Basket)> =
        options.iter().zip(take).filter(|(_, t)| t.iter().any(|&x| x > 0.0)).map(|(&(i, _), t)| (i, t)).collect();
    Plan { take, food: planned, labour: labour - left }
}

/// The `harvest` step: each clan in a shuffled order walks its round, then harvests what its
/// labour plans, first come first served.
pub fn harvest(w: &mut World) {
    let mut ids: Vec<u32> = w.clans.keys().copied().collect();
    for i in (1..ids.len()).rev() {
        let j = rand_index(&mut w.rng, i + 1);
        ids.swap(i, j);
    }
    for id in ids {
        let c = &w.clans[&id];
        let walk = round::cost(w, &c.round);
        let labour = (c.pop * CLAN.work - walk - c.trip).max(0.0);
        let options = round::options(w, id, &c.round);
        let plan = plan_harvest(w, c.pop, &c.store, labour, &options);
        for (i, t) in &plan.take {
            for (s, x) in w.tiles[*i].stock.iter_mut().zip(t) {
                *s = (*s - x).max(0.0);
            }
        }
        let c = w.clans.get_mut(&id).unwrap();
        c.walk = walk;
        c.harvest = plan.food;
        c.basket = std::array::from_fn(|f| c.store[f] + plan.food[f]);
        c.store = [0.0; N];
    }
}

/// The `eat` step: each clan eats for its diet, stores what's left less spoilage, and grows,
/// holds or starves. A clan under one person dies and frees its tiles.
pub fn eat(w: &mut World) {
    let mut dead = Vec::new();
    for c in w.clans.values_mut() {
        let need = c.pop;
        let k = CLAN.diet_share * need;
        let mut got: f64 = c.basket.iter().map(|&v| v.min(k)).sum();
        let left: Basket = c.basket.map(|v| (v - k).max(0.0));
        let total: f64 = left.iter().sum();
        let eaten = total.min((need - got).max(0.0) / CLAN.excess_value);
        got += eaten * CLAN.excess_value;
        let keep = if total > 0.0 { 1.0 - eaten / total } else { 0.0 };
        c.store = left.map(|v| v * keep * (1.0 - CLAN.spoilage));
        c.fed_ratio = if need > 0.0 { got / need } else { 1.0 };
        c.fed = c.fed_ratio >= 0.999;
        c.starving = c.fed_ratio < CLAN.stable_at;
        c.hungry = if c.starving { c.hungry + 1 } else { 0 };
        c.since += 1;
        if c.fed {
            c.pop *= 1.0 + growth(c.since);
        } else if c.starving {
            c.pop *= 1.0 - CLAN.starve * (1.0 - c.fed_ratio / CLAN.stable_at);
        }
        if c.pop < 1.0 {
            dead.push(c.id);
        }
    }
    for id in dead {
        let c = w.clans.remove(&id).unwrap();
        for i in c.round {
            if w.tiles[i].claim == Some(id) {
                w.tiles[i].claim = None;
            }
        }
    }
}

/// Growth when fed, from just moved to settled over `settle_turns`.
pub fn growth(since: u32) -> f64 {
    CLAN.growth.0 + (CLAN.growth.1 - CLAN.growth.0) * (since as f64 / CLAN.settle_turns as f64).min(1.0)
}
