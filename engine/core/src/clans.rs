//! Clans: the first actors. A clan holds a connected territory around its home, divides its
//! gathering evenly over it (each tile's share paying the walk there first), eats a varied diet,
//! grows on what it harvests beyond its need and starves when short.

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::{self, Actor, Route, Walks};
use crate::civs;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::{neighbours, passable, steps};
use crate::map::{self, Country};
use crate::market::{sum, Basket};
use crate::rng::{rand, rand_index};
use crate::world::World;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Clan {
    pub id: u32,
    /// Its people, home, store, territory, festival route and market; a clan reads them as its own.
    #[serde(flatten)]
    pub actor: Actor,
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
    /// What its walks inside its territory cost this turn.
    pub walk: f64,
    /// The civ it belongs to, once founding has made civs; its daughters belong too.
    pub civ: Option<u32>,
    /// How far an unaligned clan leans towards each civ (civ id to leaning); empty once it
    /// belongs to one.
    #[serde(default)]
    pub leaning: BTreeMap<u32, f64>,
}

impl Deref for Clan {
    type Target = Actor;
    fn deref(&self) -> &Actor {
        &self.actor
    }
}

impl DerefMut for Clan {
    fn deref_mut(&mut self) -> &mut Actor {
        &mut self.actor
    }
}

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
    let has_food =
        |w: &World, i: usize| w.tiles[i].cap.iter().any(|&c| c > 0.0) && w.tiles[i].country != Country::Sealed;
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

pub fn new_clan(id: u32, home: usize, people: f64) -> Clan {
    Clan {
        id,
        actor: Actor::new(home, people),
        harvest: [0.0; N],
        basket: [0.0; N],
        fed_ratio: 1.0,
        fed: true,
        starving: false,
        hungry: 0,
        since: 0,
        walk: 0.0,
        civ: None,
        leaning: BTreeMap::new(),
    }
}

/// What a basket is worth to `need` people: each food in full up to `diet_share × need`, at
/// `excess_value` beyond it.
pub fn nutrition(b: &Basket, need: f64) -> f64 {
    let k = CLAN.diet_share * need;
    b.iter().map(|&v| v.min(k)).sum::<f64>() + CLAN.excess_value * b.iter().map(|&v| (v - k).max(0.0)).sum::<f64>()
}

/// The walks from `home` to each tile of `territory` (sorted), and each one's yearly cost at
/// `CLAN.inside`.
pub fn walks(w: &World, home: usize, territory: &[usize]) -> (Walks, Vec<f64>) {
    let walks = Walks::new(w, home, territory);
    let yearly = walks.yearly(CLAN.inside);
    (walks, yearly)
}

/// What `people` with `store`, paying `routes`, would take from each tile of `tiles` this turn with
/// `yearly` walks: their labour divided over the tiles, each tile's work taking its foods in
/// proportion to stock at `harvest_labour` a unit, up to `take_rate` of the stock; then every
/// tile's take scaled by one factor so the whole comes to at most `surplus_target × people` less
/// the store.
pub fn reap(w: &World, people: f64, store: &Basket, routes: &[Route], tiles: &[usize], yearly: &[f64]) -> Vec<Basket> {
    let work = actors::divide(people * CLAN.work, routes, yearly);
    let mut takes: Vec<Basket> = Vec::with_capacity(tiles.len());
    let mut total = 0.0;
    for (&t, &labour) in tiles.iter().zip(&work) {
        let stock = &w.tiles[t].stock;
        let held: f64 = stock.iter().sum();
        if held <= 0.0 || labour <= 0.0 || w.tiles[t].fallow > 0 {
            takes.push([0.0; N]);
            continue;
        }
        let amount = (labour / CLAN.harvest_labour).min(CLAN.take_rate * held);
        total += amount;
        takes.push(stock.map(|s| amount * s / held));
    }
    let budget = (CLAN.surplus_target * people - store.iter().sum::<f64>()).max(0.0);
    if total > budget {
        let scale = budget / total;
        for t in &mut takes {
            *t = t.map(|x| x * scale);
        }
    }
    takes
}

/// Whether `territory` would harvest the clan's whole target this turn (`reap` scaled down to
/// it, or nothing left to harvest for): a clan short of it wants more land.
pub fn at_target(w: &World, clan: &Clan, territory: &[usize]) -> bool {
    let budget = (CLAN.surplus_target * clan.people - clan.store.iter().sum::<f64>()).max(0.0);
    let (_, yearly) = walks(w, clan.home, territory);
    let food = sum(&reap(w, clan.people, &clan.store, &clan.routes, territory, &yearly));
    food.iter().sum::<f64>() >= budget - 1e-9
}

/// How well `territory` (sorted) from `home` would feed `people` with `store`, paying `routes`,
/// this turn from the current stock: the nutrition per person of the store and the harvest.
fn worth(w: &World, people: f64, store: &Basket, routes: &[Route], home: usize, territory: &[usize]) -> f64 {
    let (_, yearly) = walks(w, home, territory);
    let food = sum(&reap(w, people, store, routes, territory, &yearly));
    nutrition(&std::array::from_fn(|f| store[f] + food[f]), people) / people
}

/// How well `territory` would feed the clan this turn. Walking counts against food, so a tile is
/// worth adding only if its food pays for the walk to it and the labour it takes from the others.
pub fn value(w: &World, clan: &Clan, territory: &[usize]) -> f64 {
    worth(w, clan.people, &clan.store, &clan.routes, clan.home, territory)
}

/// The value of territory `[site]` for `people` people with `store`, with no route.
pub fn site_value(w: &World, site: usize, people: f64, store: &Basket) -> f64 {
    worth(w, people, store, &[], site, &[site])
}

/// The `harvest` step: each clan in a shuffled order walks to every tile of its territory and
/// harvests what its labour divided over them reaps, first come first served. Every tile walked
/// this turn, inside a territory or on a route, is marked `walked`; each tile's take is recorded
/// for `land::wear`.
pub fn harvest(w: &mut World) {
    for t in &mut w.tiles {
        t.walked = false;
        t.take = [0.0; N];
    }
    for id in shuffled(w) {
        let c = &w.clans[&id];
        let (walks, yearly) = walks(w, c.home, &c.territory);
        let takes = reap(w, c.people, &c.store, &c.routes, &c.territory, &yearly);
        let routes = c.routes.clone();
        for (&i, t) in walks.tiles.iter().zip(&takes) {
            let tile = &mut w.tiles[i];
            for (f, x) in t.iter().enumerate() {
                tile.stock[f] = (tile.stock[f] - x).max(0.0);
                tile.take[f] += x;
            }
        }
        actors::mark_walked(w, &walks, &routes);
        let food = sum(&takes);
        let c = w.clans.get_mut(&id).unwrap();
        c.walk = yearly.iter().sum();
        c.harvest = food;
        c.basket = std::array::from_fn(|f| c.store[f] + food[f]);
        c.store = [0.0; N];
    }
}

/// What `people` harvested beyond their need this turn, by nutrition.
pub fn surplus(harvest: &Basket, people: f64) -> f64 {
    (nutrition(harvest, people) - people).max(0.0)
}

/// The `eat` step: each clan eats for its diet, stores what's left less spoilage, and grows on its
/// surplus, holds or starves. A clan under one person dies and frees its tiles.
pub fn eat(w: &mut World) {
    let mut dead = Vec::new();
    let mut starving = Vec::new();
    for c in w.clans.values_mut() {
        let was_starving = c.starving;
        let need = c.people;
        c.store = c.basket;
        let got = c.eat_by_diet(need);
        c.spoil();
        c.fed_ratio = if need > 0.0 { got / need } else { 1.0 };
        c.fed = c.fed_ratio >= 0.999;
        c.starving = c.fed_ratio < CLAN.stable_at;
        if c.starving && !was_starving {
            // The food it has least of, which it is short of.
            let short = (0..N).min_by(|&a, &b| c.basket[a].total_cmp(&c.basket[b])).unwrap_or(0);
            starving.push((c.id, c.home, c.fed_ratio, short));
        }
        c.hungry = if c.starving { c.hungry + 1 } else { 0 };
        c.since += 1;
        if c.fed {
            let full = (surplus(&c.harvest, need) / (CLAN.grow_full * need)).min(1.0);
            c.people *= 1.0 + growth(c.since) * full;
        } else if c.starving {
            c.people *= 1.0 - CLAN.starve * (1.0 - c.fed_ratio / CLAN.stable_at);
        }
        if c.people < 1.0 {
            dead.push(c.id);
        }
    }
    for (id, home, fed, short) in starving {
        w.emit(Event::new(EventKind::ClanStarving, home).clan(id));
        w.note(id, format!("began starving: fed {:.0}% of its need, short of {}", fed * 100.0, GOODS[short]));
    }
    for id in dead {
        let c = w.clans.remove(&id).unwrap();
        free(w, id, &c.territory);
    }
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let c = &w.clans[&id];
        if c.people >= CLAN.split_at && c.territory.len() >= 2 {
            divide_land(w, id);
        } else if c.people >= CLAN.split_at || (c.starving && c.people >= CLAN.hungry_split) {
            split(w, id);
        }
    }
}

/// Clears `id`'s claim on `tiles`.
fn free(w: &mut World, id: u32, tiles: &[usize]) {
    for &i in tiles {
        if w.tiles[i].claim == Some(id) {
            w.tiles[i].claim = None;
        }
    }
}

/// The most a fed clan grows a turn, from just moved to settled over `settle_turns`.
pub fn growth(since: u32) -> f64 {
    CLAN.growth.0 + (CLAN.growth.1 - CLAN.growth.0) * (since as f64 / CLAN.settle_turns as f64).min(1.0)
}

/// Tiles within `steps` steps of home over passable land, held by nobody, in tile order.
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

/// Moving on: after `patience` starving turns, a clan settles a site within `move_range` worth
/// more than its territory by `move_margin`. It frees its territory; home and territory become
/// the site.
fn move_on(w: &mut World, c: &Clan, now: f64) -> bool {
    let scored: Vec<(usize, f64)> =
        sites(w, c, CLAN.move_range).into_iter().map(|s| (s, site_value(w, s, c.people, &c.store))).collect();
    let min = now + CLAN.move_margin;
    let Some(site) = pick_site(w, &scored, min) else { return false };
    let worth = scored.iter().find(|s| s.0 == site).map_or(0.0, |s| s.1);
    let (from, to) = (w.at(c.home), w.at(site));
    free(w, c.id, &c.territory);
    w.tiles[site].claim = Some(c.id);
    let m = w.clans.get_mut(&c.id).unwrap();
    m.home = site;
    m.territory = vec![site];
    m.since = 0;
    m.hungry = 0;
    w.emit(Event::new(EventKind::ClanMoved, site).clan(c.id));
    w.note(
        c.id,
        format!("left {from} for {to} after starving {} turns: worth {worth:.1} there against {now:.1}", c.hungry),
    );
    true
}

/// The `move` step: each clan in a shuffled order moves on, or changes its territory by at most
/// one tile (`actors::change`: short of its harvest target it adds or swaps; at it, it drops);
/// then home steps towards the territory's middle and the territory keeps to the radius and to
/// what's connected.
pub fn adjust(w: &mut World) {
    for id in shuffled(w) {
        let c = w.clans[&id].clone();
        let pinned = civs::is_civ_clan(w, id);
        if !pinned && c.hungry >= CLAN.patience {
            if move_on(w, &c, value(w, &c, &c.territory)) {
                continue;
            }
            if c.hungry == CLAN.patience {
                let range = CLAN.move_range;
                w.note(id, format!("starving {} turns but no site within {range} steps is better", c.hungry));
            }
        }
        let changed = actors::change(
            w,
            c.home,
            &c.territory,
            !at_target(w, &c, &c.territory),
            CLAN.territory_radius,
            CLAN.land_margin,
            |t| value(w, &c, t),
        );
        // The civ's own clan stays on the capital: it adjusts its land around it.
        let (home, territory) = if pinned {
            (c.home, actors::trim(w, c.home, &changed, CLAN.territory_radius))
        } else {
            actors::recentre(w, c.home, &changed, CLAN.territory_radius)
        };
        let dropped = c.territory.iter().filter(|t| !territory.contains(t)).count();
        if dropped > 0 {
            let (was, now) = (value(w, &c, &c.territory), value(w, &c, &territory));
            w.note(id, format!("let go of {dropped} tile(s): land worth {now:.2} now, {was:.2} before"));
        }
        if home != c.home {
            w.note(id, "moved its home to the middle of its land".to_string());
        }
        free(w, id, &c.territory);
        for &t in &territory {
            w.tiles[t].claim = Some(id);
        }
        let m = w.clans.get_mut(&id).unwrap();
        m.home = home;
        m.territory = territory;
    }
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
    for part in &mut parts {
        part.sort_unstable();
    }
    parts
}

/// A big clan divides like a cell: the daughter takes the half of the territory away from home,
/// with half the people and half the store, and settles its middle. Each half keeps only the
/// tiles connected to its home; the rest are freed.
fn divide_land(w: &mut World, id: u32) {
    let c = w.clans[&id].clone();
    let [a, b] = halve(w, &c.territory);
    let (keep, give) = if a.contains(&c.home) { (a, b) } else { (b, a) };
    // No preference among ties but the lowest tile.
    let home = actors::middle(w, &give, usize::MAX);
    let mine = actors::trim(w, c.home, &keep, CLAN.territory_radius);
    let hers = actors::trim(w, home, &give, CLAN.territory_radius);
    free(w, id, &c.territory);

    let did = w.next_id;
    w.next_id += 1;
    for &t in &mine {
        w.tiles[t].claim = Some(id);
    }
    for &t in &hers {
        w.tiles[t].claim = Some(did);
    }
    let mut d = new_clan(did, home, c.people / 2.0);
    d.store = c.store.map(|v| v / 2.0);
    d.since = c.since;
    d.fed_ratio = c.fed_ratio;
    d.civ = c.civ;
    d.leaning = c.leaning.clone();
    let took = hers.len();
    d.territory = hers;
    let m = w.clans.get_mut(&id).unwrap();
    m.territory = mine;
    m.people -= d.people;
    m.store = std::array::from_fn(|f| m.store[f] - d.store[f]);
    w.clans.insert(did, d);
    w.emit(Event::new(EventKind::ClanDivided, home).clan(id).other(did));
    w.note(
        id,
        format!(
            "divided at {:.0} people (it splits at {:.0}); clan {did} took the far {took} tiles",
            c.people, CLAN.split_at
        ),
    );
    w.note(did, format!("born when clan {id} divided its land"));
}

/// A clan with no land to divide, or a starving one, sends a daughter of `min(people, split_at) /
/// 2` people to a site within `scout_range` that would feed her at least `settle_fed`.
fn split(w: &mut World, id: u32) {
    let c = w.clans[&id].clone();
    let people = c.people.min(CLAN.split_at) / 2.0;
    let share = people / c.people;
    let mut d = new_clan(w.next_id, c.home, people);
    d.civ = c.civ;
    d.leaning = c.leaning.clone();
    d.store = c.store.map(|v| v * share);
    let scored: Vec<(usize, f64)> =
        sites(w, &c, CLAN.scout_range).into_iter().map(|s| (s, site_value(w, s, people, &d.store))).collect();
    let Some(site) = pick_site(w, &scored, CLAN.settle_fed) else { return };
    w.next_id += 1;
    d.home = site;
    d.territory = vec![site];
    w.tiles[site].claim = Some(d.id);
    let m = w.clans.get_mut(&id).unwrap();
    m.people -= people;
    m.store = std::array::from_fn(|f| m.store[f] - d.store[f]);
    m.hungry = 0;
    w.emit(Event::new(EventKind::ClanDivided, site).clan(id).other(d.id));
    let why = if c.starving { "it is starving" } else { "it has no land to divide" };
    let at = w.at(site);
    w.note(id, format!("sent clan {} to {at} with {people:.0} people: {why}", d.id));
    w.note(d.id, format!("set out from clan {id} with {people:.0} people"));
    w.clans.insert(d.id, d);
}
