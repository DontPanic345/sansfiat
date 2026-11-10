//! Clans: the first actors. A clan lives on a home in a connected territory of its own claims, gathers
//! only the tiles it claims, divides its gathering evenly over them (each tile's share paying the walk
//! there first), eats a varied diet, grows on what it harvests beyond its need and starves when short.

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::actors::{self, Actor, Route, Walks};
use crate::civs;
use crate::data::*;
use crate::dividing;
use crate::events::{Event, EventKind};
use crate::fields;
use crate::homes;
use crate::land::{neighbours, passable};
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
    /// Turns in a row it has gone to its current festival; 0 with none.
    #[serde(default)]
    pub market_turns: u32,
    /// Turns in a row its strongest leaning is to a rival civ it is drawn to (`LEANING.leave_at`
    /// and above its own civ's); 0 otherwise. At `LEANING.leave_run` it leaves (`leaning.rs`).
    #[serde(default)]
    pub drawn: u32,
    /// The clan it divided from or sent out as a daughter; its daughters leave with it.
    #[serde(default)]
    pub parent: Option<u32>,
    /// The share of its days left after its routes that go to fields (`fields.rs`); a daughter starts
    /// at her mother's, and a clan that moves on at 0.
    #[serde(default)]
    pub fields: f64,
    /// Its field days this turn per tile, sorted by tile, only tiles with days (`harvest`). `fields::room`
    /// reads it next turn, to leave the other clans their share of the ground.
    #[serde(default)]
    pub sown: Vec<(usize, f64)>,
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
        homes::claim_land(w, id, &[home]);
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
        market_turns: 0,
        drawn: 0,
        parent: None,
        fields: 0.0,
        sown: Vec::new(),
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

/// What a clan's labour reaps from its tiles in a turn: gathered food per tile, the field grain per tile
/// and the field days per tile (`reap`).
pub struct Reaped {
    /// Gathered food per tile, taken from its stock: what `harvest` takes and what `wear` sees.
    pub takes: Vec<Basket>,
    /// Field grain per tile, drawn from no stock.
    pub grain: Vec<f64>,
    /// Field days per tile, never above the room the tile leaves the clan (`fields::room`).
    pub days: Vec<f64>,
}

impl Reaped {
    /// Nothing reaped from `n` tiles.
    pub fn empty(n: usize) -> Reaped {
        Reaped { takes: vec![[0.0; N]; n], grain: vec![0.0; n], days: vec![0.0; n] }
    }

    /// The whole harvest: gathered food and field grain.
    pub fn food(&self) -> Basket {
        let mut food = sum(&self.takes);
        food[GRAIN] += self.grain.iter().sum::<f64>();
        food
    }
}

/// What clan `id` with `people` and `store`, paying `routes`, would reap from each tile of `tiles` this
/// turn with `yearly` walks and `fields` of their days on the field job (`fields::jobs`): their gathering
/// labour divided over the tiles that `gathers` marks, each tile's take taking its foods in proportion to
/// stock at `harvest_labour` a unit, up to `take_rate` of the stock; and each field tile's days
/// (`fields::sow`, capped at the room `fields::room` leaves `id`) taking their grain. Then gathering and
/// field grain together come to at most `surplus_target × people` less the store, by one factor on every
/// tile. Fields count only once `fields::open`.
#[allow(clippy::too_many_arguments)]
pub fn reap(
    w: &World,
    id: u32,
    people: f64,
    store: &Basket,
    routes: &[Route],
    tiles: &[usize],
    yearly: &[f64],
    fields: f64,
    gathers: &[bool],
) -> Reaped {
    let fields = if fields::open(w) { fields } else { 0.0 };
    let (work, field, fk) = fields::jobs(w, people, routes, tiles, yearly, fields, gathers);
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
    let field_tiles: Vec<usize> = fk.iter().map(|&k| tiles[k]).collect();
    let days_on = fields::sow(w, id, &field_tiles, &field);
    let mut days = vec![0.0; tiles.len()];
    let mut grain = vec![0.0; tiles.len()];
    for ((&k, &t), &d) in fk.iter().zip(&field_tiles).zip(&days_on) {
        days[k] = d;
        grain[k] = d * fields::yield_a_day(w, t);
    }
    total += grain.iter().sum::<f64>();
    let budget = (CLAN.surplus_target * people - store.iter().sum::<f64>()).max(0.0);
    if total > budget {
        let scale = budget / total;
        for t in &mut takes {
            *t = t.map(|x| x * scale);
        }
        for g in grain.iter_mut().chain(days.iter_mut()) {
            *g *= scale;
        }
    }
    Reaped { takes, grain, days }
}

/// Whether `territory` would harvest the clan's whole target this turn (`reap` scaled down to
/// it, or nothing left to harvest for): a clan short of it wants more land.
pub fn at_target(w: &World, clan: &Clan, territory: &[usize]) -> bool {
    let budget = (CLAN.surplus_target * clan.people - clan.store.iter().sum::<f64>()).max(0.0);
    let (_, yearly) = walks(w, clan.home, territory);
    let gathers = reckoned(w, clan.id, territory);
    let food =
        reap(w, clan.id, clan.people, &clan.store, &clan.routes, territory, &yearly, clan.fields, &gathers).food();
    food.iter().sum::<f64>() >= budget - 1e-9
}

/// Which of `tiles` clan `id` reckons it will claim: those held by nobody or by it. Planning counts a
/// tile it would add as its own; the harvest counts only the claims it holds (`homes::gathers`).
fn reckoned(w: &World, id: u32, tiles: &[usize]) -> Vec<bool> {
    tiles.iter().map(|&t| w.tiles[t].claim.is_none_or(|held| held == id)).collect()
}

/// How well `territory` (sorted) from `home` would feed clan `id` with `people` and `store`, paying
/// `routes`, this turn from the current stock: the nutrition per person of the store and the harvest.
#[allow(clippy::too_many_arguments)]
fn worth(
    w: &World,
    id: u32,
    people: f64,
    store: &Basket,
    routes: &[Route],
    home: usize,
    territory: &[usize],
    fields: f64,
) -> f64 {
    let (_, yearly) = walks(w, home, territory);
    let gathers = reckoned(w, id, territory);
    let food = reap(w, id, people, store, routes, territory, &yearly, fields, &gathers).food();
    nutrition(&std::array::from_fn(|f| store[f] + food[f]), people) / people
}

/// How well `territory` would feed the clan this turn, its days split at `fields`. Walking counts
/// against food, so a tile is worth adding only if its food pays for the walk to it and the labour it
/// takes from the others.
pub fn value_at(w: &World, clan: &Clan, territory: &[usize], fields: f64) -> f64 {
    worth(w, clan.id, clan.people, &clan.store, &clan.routes, clan.home, territory, fields)
}

/// How well `territory` would feed the clan this turn at its own split (`Clan.fields`).
pub fn value(w: &World, clan: &Clan, territory: &[usize]) -> f64 {
    value_at(w, clan, territory, clan.fields)
}

/// The value of territory `[site]` for clan `id` with `people` people and `store`, with no route and no
/// fields: a site it would claim.
pub fn site_value(w: &World, id: u32, site: usize, people: f64, store: &Basket) -> f64 {
    worth(w, id, people, store, &[], site, &[site], 0.0)
}

/// The `harvest` step: each clan in a shuffled order reaps its territory (`reap`: it gathers the tiles it
/// claims, and sows the field ground its room allows), then the field days on a tile that the clans
/// together overfill scale to its ground, and the takes are applied. Every tile walked this turn, inside
/// a territory or on a route, carries the clan's people as traffic (`TRAIL.keep` of last turn's stays);
/// each tile's take is recorded for `land::wear`.
pub fn harvest(w: &mut World) {
    for t in &mut w.tiles {
        t.traffic *= TRAIL.keep;
        t.used = false;
        t.take = [0.0; N];
    }
    // Pass one reaps every clan before any take or field total is written: each reads last turn's
    // `Tile.field` (the room `fields::room` leaves it) and the stock, and a clan gathers only its own
    // claims, so no two clans read one stock.
    let mut reaps: Vec<(u32, Walks, Vec<f64>, Reaped)> = Vec::new();
    for id in shuffled(w) {
        let c = &w.clans[&id];
        let (walks, yearly) = walks(w, c.home, &c.territory);
        // A civ besieged at its capital: its own clans there harvest nothing (`fight::posture`).
        let besieged = c.civ.and_then(|k| w.civs.get(&k)).is_some_and(|k| k.besieged && k.capital == c.home);
        let reaped = if besieged {
            Reaped::empty(walks.tiles.len())
        } else {
            let gathers: Vec<bool> = walks.tiles.iter().map(|&t| homes::gathers(w, id, t)).collect();
            reap(w, id, c.people, &c.store, &c.routes, &walks.tiles, &yearly, c.fields, &gathers)
        };
        reaps.push((id, walks, yearly, reaped));
    }
    // Shared ground: where the clans' field days on a tile add to more than its ground, each clan's days
    // and grain there scale by ground over the total days.
    let mut total = vec![0.0; w.tiles.len()];
    for (_, walks, _, reaped) in &reaps {
        for (&t, &d) in walks.tiles.iter().zip(&reaped.days) {
            total[t] += d;
        }
    }
    for (_, walks, _, reaped) in &mut reaps {
        for (k, &t) in walks.tiles.iter().enumerate() {
            if total[t] > fields::ground(w, t) {
                let scale = fields::ground(w, t) / total[t];
                reaped.days[k] *= scale;
                reaped.grain[k] *= scale;
            }
        }
    }
    // Pass two applies the takes, this turn's field total per tile, the traffic and the baskets, in
    // the shuffled order.
    for t in &mut w.tiles {
        t.field = 0.0;
    }
    for (id, walks, yearly, reaped) in reaps {
        for (&i, t) in walks.tiles.iter().zip(&reaped.takes) {
            let tile = &mut w.tiles[i];
            for (f, x) in t.iter().enumerate() {
                tile.stock[f] = (tile.stock[f] - x).max(0.0);
                tile.take[f] += x;
            }
        }
        let mut sown = Vec::new();
        for (&i, &d) in walks.tiles.iter().zip(&reaped.days) {
            w.tiles[i].field += d;
            if d > 0.0 {
                sown.push((i, d));
            }
        }
        let c = &w.clans[&id];
        let (routes, home, people) = (c.routes.clone(), c.home, c.people);
        actors::add_traffic(w, home, people, &walks, &routes);
        let food = reaped.food();
        let c = w.clans.get_mut(&id).unwrap();
        c.walk = yearly.iter().sum();
        c.harvest = food;
        c.basket = std::array::from_fn(|f| c.store[f] + food[f]);
        c.store = [0.0; N];
        c.sown = sown;
    }
    w.sowers = w.clans.values().filter(|c| !c.sown.is_empty()).map(|c| c.id).collect();
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
            dividing::divide_land(w, id);
        } else if c.people >= CLAN.split_at || (c.starving && c.people >= CLAN.hungry_split) {
            dividing::split(w, id);
        }
    }
}

/// Clears `id`'s claim on `tiles`: only the claims it holds, so a home it lives on and another holds stays.
pub(crate) fn free(w: &mut World, id: u32, tiles: &[usize]) {
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

/// Tiles within `steps` steps of home over passable land, held by nobody and not a home, in tile order.
/// A daughter or a clan moving on never settles on another clan's home.
pub(crate) fn sites(w: &World, clan: &Clan, steps: i32) -> Vec<usize> {
    let passable = |i: usize| TERRAIN[w.tiles[i].terrain].land && passable(w, i);
    let mut seen = vec![clan.home];
    let mut visited = vec![false; w.tiles.len()];
    visited[clan.home] = true;
    let mut frontier = vec![clan.home];
    for _ in 0..steps {
        let mut next = Vec::new();
        for &i in &frontier {
            for j in neighbours(w, i) {
                if passable(j) && !visited[j] {
                    visited[j] = true;
                    seen.push(j);
                    next.push(j);
                }
            }
        }
        frontier = next;
    }
    // A home is a tile some clan lives on, held or not, and a site is never one.
    let mut homed = vec![false; w.tiles.len()];
    for c in w.clans.values() {
        homed[c.home] = true;
    }
    seen.retain(|&i| w.tiles[i].claim.is_none() && !homed[i]);
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
        sites(w, c, CLAN.move_range).into_iter().map(|s| (s, site_value(w, c.id, s, c.people, &c.store))).collect();
    let min = now + CLAN.move_margin;
    let Some(site) = pick_site(w, &scored, min) else { return false };
    let worth = scored.iter().find(|s| s.0 == site).map_or(0.0, |s| s.1);
    let (from, to) = (w.at(c.home), w.at(site));
    free(w, c.id, &c.territory);
    homes::claim_land(w, c.id, &[site]);
    let m = w.clans.get_mut(&c.id).unwrap();
    m.home = site;
    m.territory = vec![site];
    m.since = 0;
    m.hungry = 0;
    m.fields = 0.0;
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
        // The civ's own clan stays on the capital: it adjusts its land around it. A farmer's home
        // steps toward its fields instead of the middle.
        let toward = if pinned { None } else { fields::home_toward(w, &c, &changed) };
        let (home, territory) = if pinned {
            (c.home, actors::trim(w, c.home, &changed, CLAN.territory_radius))
        } else if let Some(target) = toward {
            actors::step_toward(w, c.home, target, &changed, CLAN.territory_radius)
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
        homes::claim_land(w, id, &territory);
        let m = w.clans.get_mut(&id).unwrap();
        m.home = home;
        m.territory = territory;
        fields::notch(w, id);
    }
}
