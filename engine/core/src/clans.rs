//! Clans: small groups living off a round of tiles, paying labour to walk it and to harvest,
//! eating a varied diet, growing when fed and starving when not.

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::land::{neighbours, passable};
use crate::map::{self, Zone};
use crate::market::Basket;
use crate::rng::{rand, rand_index};
use crate::round::{self, Legs};
use crate::world::{EventKind, World};

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
    /// The civ it belongs to, once founding has made civs; its daughters belong too.
    pub civ: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub take: Vec<(usize, Basket)>,
    pub food: Basket,
    pub labour: f64,
}

const EPS: f64 = 1e-9;

/// The clans' ids in a Fisher–Yates shuffle.
fn shuffled(w: &mut World) -> Vec<u32> {
    let mut ids: Vec<u32> = w.clans.keys().copied().collect();
    for i in (1..ids.len()).rev() {
        let j = rand_index(&mut w.rng, i + 1);
        ids.swap(i, j);
    }
    ids
}

/// Seed clans: one on a random land tile with food in each landmass of `min_landmass`+ tiles,
/// then more on random free land tiles with food until there are `seeds`; never in a sealed valley.
pub fn seed(w: &mut World) {
    // Nobody starts in a sealed valley: it stays empty until a scout bridges its wall.
    let has_food = |w: &World, i: usize| w.tiles[i].cap.iter().any(|&c| c > 0.0) && w.tiles[i].zone != Zone::Sealed;
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
        civ: None,
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
    for id in shuffled(w) {
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
    let mut starving = Vec::new();
    for c in w.clans.values_mut() {
        let was_starving = c.starving;
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
        if c.starving && !was_starving {
            starving.push((c.id, c.home));
        }
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
    for (id, home) in starving {
        w.event(EventKind::ClanStarving, home, Some(id), None, None);
    }
    for id in dead {
        let c = w.clans.remove(&id).unwrap();
        for i in c.round {
            if w.tiles[i].claim == Some(id) {
                w.tiles[i].claim = None;
            }
        }
    }
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let c = &w.clans[&id];
        if c.pop >= CLAN.split_at && c.round.len() >= 2 {
            divide(w, id);
        } else if c.pop >= CLAN.split_at || (c.starving && c.pop >= CLAN.hungry_split) {
            split(w, id);
        }
    }
}

/// Growth when fed, from just moved to settled over `settle_turns`.
pub fn growth(since: u32) -> f64 {
    CLAN.growth.0 + (CLAN.growth.1 - CLAN.growth.0) * (since as f64 / CLAN.settle_turns as f64).min(1.0)
}

/// How well `round` would feed `pop` people with `store` this turn, from the current stock, after
/// walking it and `trip`: the nutrition of the store and the plan's food, per person.
fn worth(w: &World, legs: &Legs, id: u32, round: &[usize], pop: f64, store: &Basket, trip: f64) -> f64 {
    let labour = (pop * CLAN.work - legs.cost(round) - trip).max(0.0);
    let plan = plan_harvest(w, pop, store, labour, &round::options(w, id, round));
    nutrition(&std::array::from_fn(|f| store[f] + plan.food[f]), pop) / pop
}

/// How well `round` would feed the clan this turn. Walking counts against food, so a tile is
/// worth adding only if what it opens up pays for the extra walk.
pub fn value(w: &World, clan: &Clan, round: &[usize]) -> f64 {
    worth(w, &Legs::new(w, round[0]), clan.id, round, clan.pop, &clan.store, clan.trip)
}

/// The value of round `[site]` for `pop` people with `store`, with no market trip.
pub fn site_value(w: &World, clan: &Clan, site: usize, pop: f64, store: &Basket) -> f64 {
    worth(w, &Legs::new(w, site), clan.id, &[site], pop, store, 0.0)
}

/// Tiles within `steps` steps of home over passable land, on no round, in tile order.
fn sites(w: &World, clan: &Clan, steps: i32) -> Vec<usize> {
    let passable = |i: usize| TERRAIN[w.tiles[i].terrain].land && passable(w, i);
    let mut seen = vec![clan.home];
    let mut frontier = vec![clan.home];
    for _ in 0..steps {
        let mut next = Vec::new();
        for &i in &frontier {
            for j in neighbours(w, i) {
                if passable(j) && !seen.contains(&j) {
                    seen.push(j);
                    next.push(j);
                }
            }
        }
        frontier = next;
    }
    seen.retain(|&i| w.tiles[i].claim.is_none());
    seen.sort_unstable();
    seen
}

/// Among sites scoring above `min`, a weighted draw with weights `exp((score − best) /
/// choice_spread)`: one `rand` call. `None` if none score above `min`.
pub fn pick_site(w: &mut World, scored: &[(usize, f64)], min: f64) -> Option<usize> {
    let ok: Vec<(usize, f64)> = scored.iter().copied().filter(|&(_, v)| v > min).collect();
    let best = ok.iter().map(|&(_, v)| v).max_by(f64::total_cmp)?;
    let weights: Vec<f64> = ok.iter().map(|&(_, v)| libm::exp((v - best) / CLAN.choice_spread)).collect();
    let mut r = rand(&mut w.rng) * weights.iter().sum::<f64>();
    for (&(site, _), wt) in ok.iter().zip(&weights) {
        if r < *wt {
            return Some(site);
        }
        r -= wt;
    }
    ok.last().map(|&(site, _)| site)
}

fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// The round tile (not home, not `keep`) whose loss leaves the most value, ties to the lower
/// tile: the round without it, and that value.
fn best_drop(w: &World, legs: &Legs, clan: &Clan, round: &[usize], keep: usize) -> Option<(Vec<usize>, f64)> {
    let mut best: Option<(usize, Vec<usize>, f64)> = None;
    for k in 1..round.len() {
        let t = round[k];
        if t == keep {
            continue;
        }
        let mut r = round.to_vec();
        r.remove(k);
        let v = worth(w, legs, clan.id, &r, clan.pop, &clan.store, clan.trip);
        if best.as_ref().is_none_or(|&(bt, _, bv)| v > bv || (v == bv && t < bt)) {
            best = Some((t, r, v));
        }
    }
    best.map(|(_, r, v)| (r, v))
}

/// Moving on: after `patience` starving turns, a clan settles a site within `move_range` worth
/// more than its round by `move_margin`. It frees its round; home and round become the site.
fn move_on(w: &mut World, c: &Clan, now: f64) -> bool {
    let scored: Vec<(usize, f64)> =
        sites(w, c, CLAN.move_range).into_iter().map(|s| (s, site_value(w, c, s, c.pop, &c.store))).collect();
    let min = now + CLAN.move_margin;
    let Some(site) = pick_site(w, &scored, min) else { return false };
    for &i in &c.round {
        w.tiles[i].claim = None;
    }
    w.tiles[site].claim = Some(c.id);
    let m = w.clans.get_mut(&c.id).unwrap();
    m.home = site;
    m.round = vec![site];
    m.since = 0;
    m.hungry = 0;
    w.event(EventKind::ClanMoved, site, Some(c.id), None, None);
    true
}

/// The `move` step: each clan in a shuffled order moves on, or changes its round by at most one
/// tile; then home steps towards the territory's middle and the round keeps to the limit.
pub fn adjust(w: &mut World) {
    for id in shuffled(w) {
        let c = w.clans[&id].clone();
        if c.hungry >= CLAN.patience && move_on(w, &c, value(w, &c, &c.round)) {
            continue;
        }
        let round = change(w, &c);
        let (home, round) = recentre(w, c.home, round);
        for &t in &c.round {
            if !round.contains(&t) {
                w.tiles[t].claim = None;
            }
        }
        for &t in &round {
            w.tiles[t].claim = Some(id);
        }
        let m = w.clans.get_mut(&id).unwrap();
        m.home = home;
        m.round = round;
    }
}

/// The clan's round after its one change, if any: not fully fed, it adds the edge tile that
/// raises its value most, or failing that swaps it for its worst tile; fed, it drops a tile it
/// doesn't need.
fn change(w: &World, c: &Clan) -> Vec<usize> {
    let legs = Legs::new(w, c.home);
    let worth = |r: &[usize]| worth(w, &legs, c.id, r, c.pop, &c.store, c.trip);
    let now = worth(&c.round);
    if c.fed {
        return match best_drop(w, &legs, c, &c.round, c.home) {
            Some((dropped, v)) if v >= now => dropped,
            _ => c.round.clone(),
        };
    }
    let mut best: Option<(Vec<usize>, usize, f64)> = None;
    for t in round::free_edge(w, c) {
        let Some((k, _)) = legs.insertion(&c.round, t) else { continue };
        let mut r = c.round.clone();
        r.insert(k, t);
        let v = worth(&r);
        if best.as_ref().is_none_or(|&(_, _, bv)| v > bv) {
            best = Some((r, t, v));
        }
    }
    let Some((added, t, v)) = best else { return c.round.clone() };
    if v > now + CLAN.land_margin {
        return added;
    }
    match best_drop(w, &legs, c, &added, t) {
        Some((swapped, sv)) if sv > now + CLAN.land_margin => swapped,
        _ => c.round.clone(),
    }
}

/// Home steps to a neighbouring round tile nearer the territory's middle, and the round turns to
/// start there; then tiles too far from home, or with no leg from the tile before, drop.
fn recentre(w: &World, home: usize, mut round: Vec<usize>) -> (usize, Vec<usize>) {
    let mut home = home;
    let m = round::middle(w, &round::territory(w, &round), home);
    if m != home {
        let d = steps(w, home, m);
        if let Some(n) = neighbours(w, home).filter(|&n| round.contains(&n) && steps(w, n, m) < d).min() {
            home = n;
            let k = round.iter().position(|&t| t == n).unwrap();
            round.rotate_left(k);
        }
    }
    round.retain(|&t| steps(w, home, t) <= CLAN.territory_radius);
    let legs = Legs::new(w, home);
    loop {
        let n = round.len();
        let gap = (1..n)
            .find(|&k| legs.get(round[k - 1], round[k]).is_none())
            .or_else(|| (n > 1 && legs.get(round[n - 1], home).is_none()).then_some(n - 1));
        match gap {
            Some(k) => {
                round.remove(k);
            }
            None => break,
        }
    }
    (home, round)
}

/// Splits `tiles` in two halves grown from its two farthest tiles: each half in turn takes the
/// unassigned tile next to it that's nearest its seed; leftovers go to the half with the nearer
/// seed. Ties go to the lower tile.
fn halve(w: &World, tiles: &[usize]) -> [Vec<usize>; 2] {
    let mut seeds = [tiles[0], tiles[0]];
    for &a in tiles {
        for &b in tiles {
            if steps(w, a, b) > steps(w, seeds[0], seeds[1]) {
                seeds = [a, b];
            }
        }
    }
    let mut parts = [vec![seeds[0]], vec![seeds[1]]];
    let mut left: Vec<usize> = tiles.iter().copied().filter(|t| !seeds.contains(t)).collect();
    left.sort_unstable();
    loop {
        let mut grew = false;
        for (p, part) in parts.iter_mut().enumerate() {
            let next = left
                .iter()
                .copied()
                .filter(|&k| part.iter().any(|&q| steps(w, q, k) == 1))
                .min_by_key(|&k| (steps(w, k, seeds[p]), k));
            if let Some(k) = next {
                part.push(k);
                left.retain(|&x| x != k);
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }
    for k in left {
        let p = usize::from(steps(w, k, seeds[1]) < steps(w, k, seeds[0]));
        parts[p].push(k);
    }
    parts
}

/// A big clan divides like a cell: the daughter takes the half of the round away from home, with
/// half the people and half the store.
fn divide(w: &mut World, id: u32) {
    let c = w.clans[&id].clone();
    let [a, b] = halve(w, &c.round);
    let (keep, give) = if a.contains(&c.home) { (a, b) } else { (b, a) };
    // No preference among ties but the lowest tile.
    let home = round::middle(w, &give, usize::MAX);
    let mut rest: Vec<usize> = give.iter().copied().filter(|&t| t != home).collect();
    rest.sort_by_key(|&t| (steps(w, home, t), t));
    let mut round = vec![home];
    let mut freed = Vec::new();
    let legs = Legs::new(w, home);
    for t in rest {
        let at = if steps(w, home, t) <= CLAN.territory_radius { legs.insertion(&round, t) } else { None };
        match at {
            Some((k, _)) => round.insert(k, t),
            None => freed.push(t),
        }
    }
    for t in freed {
        w.tiles[t].claim = None;
    }

    let did = w.next_id;
    w.next_id += 1;
    for &t in &round {
        w.tiles[t].claim = Some(did);
    }
    let mut d = new_clan(did, home, c.pop / 2.0);
    d.store = c.store.map(|v| v / 2.0);
    d.since = c.since;
    d.fed_ratio = c.fed_ratio;
    d.civ = c.civ;
    d.round = round;
    let m = w.clans.get_mut(&id).unwrap();
    m.round.retain(|t| keep.contains(t));
    m.pop -= d.pop;
    m.store = std::array::from_fn(|f| m.store[f] - d.store[f]);
    w.clans.insert(did, d);
    w.event(EventKind::ClanDivided, home, Some(id), None, Some(did));
}

/// A clan with no land to divide, or a starving one, sends a daughter of `min(pop, split_at) / 2`
/// people to a site within `scout_range` that would feed her at least `settle_fed`.
fn split(w: &mut World, id: u32) {
    let c = w.clans[&id].clone();
    let pop = c.pop.min(CLAN.split_at) / 2.0;
    let share = pop / c.pop;
    let mut d = new_clan(w.next_id, c.home, pop);
    d.civ = c.civ;
    d.store = c.store.map(|v| v * share);
    let scored: Vec<(usize, f64)> =
        sites(w, &c, CLAN.scout_range).into_iter().map(|s| (s, site_value(w, &d, s, pop, &d.store))).collect();
    let Some(site) = pick_site(w, &scored, CLAN.settle_fed) else { return };
    w.next_id += 1;
    d.home = site;
    d.round = vec![site];
    w.tiles[site].claim = Some(d.id);
    let m = w.clans.get_mut(&id).unwrap();
    m.pop -= pop;
    m.store = std::array::from_fn(|f| m.store[f] - d.store[f]);
    m.hungry = 0;
    w.event(EventKind::ClanDivided, site, Some(id), None, Some(d.id));
    w.clans.insert(d.id, d);
}
