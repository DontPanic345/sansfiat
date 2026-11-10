//! Where clans live. A clan's home is a tile of its territory, not a claim: a home may be held by
//! another clan or by nobody, and many clans may live on one tile. A clan gathers only the tiles it
//! claims (`Tile.claim`), and writes its claim only on tiles that are unheld or its own. `homes` is
//! the index of home tiles, derived from `World.clans` each time it is asked and never saved.

use std::collections::BTreeMap;

use crate::actors::{self, walk_cost, Route};
use crate::clans::{self, Clan};
use crate::data::*;
use crate::fields;
use crate::land::{neighbours, passable, steps};
use crate::markets::{festival_costs, walkers, Festival};
use crate::reach::Scratch;
use crate::world::World;

/// Each home tile and the clans living on it, ids ascending.
pub fn homes(w: &World) -> BTreeMap<usize, Vec<u32>> {
    let mut index: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    for c in w.clans.values() {
        index.entry(c.home).or_default().push(c.id);
    }
    index
}

/// Writes clan `id`'s claim on each tile of `territory` that is unheld or already its own. A tile
/// another clan holds keeps its claim: a home may stand on it.
pub fn claim_land(w: &mut World, id: u32, territory: &[usize]) {
    for &t in territory {
        let tile = &mut w.tiles[t];
        if tile.claim.is_none_or(|held| held == id) {
            tile.claim = Some(id);
        }
    }
}

/// Whether clan `id` gathers tile `t`: it holds the tile's claim.
pub fn gathers(w: &World, id: u32, t: usize) -> bool {
    w.tiles[t].claim == Some(id)
}

/// The festival routes priced from each tile this turn: one festival search per tile (`festival_costs`),
/// kept for the move step. Terrain and trails do not change within it.
pub struct Prices {
    scratch: Scratch,
    found: BTreeMap<usize, Vec<Festival>>,
    /// Homes on each tile, kept as the move step moves them.
    homes: Vec<u32>,
}

impl Prices {
    pub fn new(w: &World) -> Prices {
        let mut homes = vec![0; w.tiles.len()];
        for c in w.clans.values() {
            homes[c.home] += 1;
        }
        Prices { scratch: Scratch::new(w), found: BTreeMap::new(), homes }
    }

    /// A home moved from `from` to `to`.
    pub fn moved(&mut self, from: usize, to: usize) {
        self.homes[from] -= 1;
        self.homes[to] += 1;
    }

    /// Clan `c`'s festival route if it lived on `home`: its market's route priced from there as `go`
    /// prices it, nothing when the festival stands on `home`, and its current routes when the festival is
    /// out of the search's reach or it has none.
    fn routes(&mut self, w: &World, c: &Clan, home: usize) -> Vec<Route> {
        let Some(m) = c.market.and_then(|m| w.markets.get(&m)) else { return c.routes.clone() };
        let scratch = &mut self.scratch;
        let found = self.found.entry(home).or_insert_with(|| festival_costs(w, scratch, home));
        match found.iter().find(|f| f.market == m.id) {
            Some(f) => vec![Route { to: m.tile, path: f.path.clone(), cost: walk_cost(f.cost, 1.0) * walkers(c) }],
            None => c.routes.clone(),
        }
    }
}

/// A farmer's chosen home and territory, and the note it leaves when it moves.
pub struct Step {
    pub home: usize,
    pub territory: Vec<usize>,
    pub why: &'static str,
}

/// A farmer's home step with `Config.towns` on and fields open, over its changed territory `changed`
/// (sorted): one choice a turn among staying, the step toward its busiest field (`fields::home_toward`)
/// and the step one tile toward its festival's tile, onto any passable land whoever holds it (the lowest
/// such neighbour nearer it). A clan that is not a farmer weighs its step to the middle of its land the
/// same way when it lives on a shared home (another clan's home on its tile, or a tile it does not
/// hold), so a town's clans leave it only when that feeds them better; elsewhere it steps as the move
/// step has it. Each step is valued from its home by what its labour could reap (`clans::could_reap`):
/// the territory with the new home in it, less the tiles another clan holds other than the home, trimmed
/// to the radius, and the festival route priced from there. A step is taken when it feeds the clan
/// better than staying by `TOWN.margin`, the better of the two if both do. `None` when the rule does not apply (towns off, fields closed, neither
/// a farmer nor on a shared home): the move step's own home step then runs.
pub fn step(w: &World, prices: &mut Prices, c: &Clan, changed: &[usize]) -> Option<Step> {
    let farmer = fields::farmer(c);
    let shared = prices.homes[c.home] > 1 || !gathers(w, c.id, c.home);
    if !w.config.towns || !fields::open(w) || !(farmer || shared) {
        return None;
    }
    let value = |prices: &mut Prices, home: usize, territory: &[usize]| {
        let routes = prices.routes(w, c, home);
        clans::could_reap(w, c.id, c.people, &c.store, &routes, home, territory, c.fields)
    };
    // The territory from `home`: the changed territory with `home` in it, less the tiles another clan
    // holds other than the home, trimmed to the radius.
    let land = |home: usize| {
        let mut with: Vec<usize> =
            changed.iter().copied().filter(|&x| x == home || w.tiles[x].claim.is_none_or(|h| h == c.id)).collect();
        if let Err(k) = with.binary_search(&home) {
            with.insert(k, home);
        }
        actors::trim(w, home, &with, CLAN.territory_radius)
    };
    let stay = land(c.home);
    let now = value(prices, c.home, &stay);
    let mut best: Option<(f64, Step)> = None;
    let mut weigh = |prices: &mut Prices, home: usize, territory: Vec<usize>, why: &'static str| {
        if home == c.home {
            return;
        }
        let v = value(prices, home, &territory);
        if v > now + TOWN.margin && best.as_ref().is_none_or(|(b, _)| v > *b) {
            best = Some((v, Step { home, territory, why }));
        }
    };
    if let Some(target) = fields::home_toward(w, c, changed) {
        let (home, _) = actors::step_toward(w, c.home, target, changed, CLAN.territory_radius);
        weigh(prices, home, land(home), "moved its home toward its fields");
    } else if !farmer {
        let (home, _) = actors::recentre(w, c.home, changed, CLAN.territory_radius);
        weigh(prices, home, land(home), "moved its home to the middle of its land");
    }
    let festival = c.market.and_then(|m| w.markets.get(&m)).map(|m| m.tile);
    if let Some(t) = festival.filter(|&t| farmer && t != c.home) {
        let d = steps(w, c.home, t);
        let near = neighbours(w, c.home)
            .filter(|&n| TERRAIN[w.tiles[n].terrain].land && passable(w, n) && steps(w, n, t) < d)
            .min();
        if let Some(n) = near {
            weigh(prices, n, land(n), "moved its home toward its festival");
        }
    }
    Some(match best {
        Some((_, s)) => s,
        None => Step { home: c.home, territory: stay, why: "" },
    })
}
