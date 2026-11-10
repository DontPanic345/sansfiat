//! Where clans live. A clan's home is a tile of its territory, not a claim: a home may be held by
//! another clan or by nobody, and many clans may live on one tile. A clan gathers only the tiles it
//! claims (`Tile.claim`), and writes its claim only on tiles that are unheld or its own. `homes` is
//! the index of home tiles, derived from `World.clans` each time it is asked and never saved.

use std::collections::BTreeMap;

use crate::actors::{self, walk_cost, Route};
use crate::clans::{self, Clan};
use crate::data::{CLAN, TERRAIN, TOWN};
use crate::fields;
use crate::land::{neighbours, passable, steps};
use crate::markets::{festival_costs, walkers};
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

/// Whether clan `c`'s home step is weighed by `home_step`: towns are on, fields are open, and it is a
/// farmer. Other clans step as before.
pub fn weighs(w: &World, c: &Clan) -> bool {
    w.config.towns && fields::open(w) && fields::farmer(c)
}

/// A farmer's one home step this turn, out of `changed`, its territory after this turn's land change.
/// It weighs staying, the step toward its busiest field (`toward`, `fields::home_toward`), and the step
/// a tile toward its festival's tile: onto the lowest passable land neighbour nearer it, whoever
/// claims it, with that tile joining its territory. Each is valued (`clans::value_from`) from its home
/// with its territory trimmed to the radius and the festival route priced from that home. A step is
/// taken when it feeds the clan better than staying by `TOWN.margin`, the better step if both do; a
/// farmer on its festival's tile prices its route at nothing.
pub fn home_step(
    w: &World,
    s: &mut Scratch,
    c: &Clan,
    changed: &[usize],
    toward: Option<usize>,
) -> (usize, Vec<usize>) {
    let r = CLAN.territory_radius;
    let stay = (c.home, actors::trim(w, c.home, changed, r));
    let Some((mid, t)) = c.market.and_then(|m| w.markets.get(&m)).map(|m| (m.id, m.tile)) else {
        // No festival: the field step, if it feeds the clan better than staying.
        let Some(target) = toward else { return stay };
        let field = actors::step_toward(w, c.home, target, changed, r);
        let here = c.routes.clone();
        let better = clans::value_from(w, c, field.0, &here, &field.1)
            > clans::value_from(w, c, c.home, &here, &stay.1) + TOWN.margin;
        return if better { field } else { stay };
    };
    let mut route = |h: usize| route_from(w, s, c, mid, t, h);
    let mut options: Vec<(usize, Vec<usize>)> = Vec::new();
    if let Some(target) = toward {
        let field = actors::step_toward(w, c.home, target, changed, r);
        let away = steps(w, field.0, t) > steps(w, c.home, t);
        if field.0 != c.home && !away {
            options.push(field);
        }
    }
    let d = steps(w, c.home, t);
    if t != c.home {
        if let Some(n) = neighbours(w, c.home)
            .filter(|&n| TERRAIN[w.tiles[n].terrain].land && passable(w, n) && steps(w, n, t) < d)
            .min()
        {
            let mut cand = changed.to_vec();
            if let Err(k) = cand.binary_search(&n) {
                cand.insert(k, n);
            }
            options.push((n, actors::trim(w, n, &cand, r)));
        }
    }
    if options.is_empty() {
        return stay;
    }
    let Some(here) = route(c.home) else { return stay };
    let mut best = clans::value_from(w, c, stay.0, &here, &stay.1) + TOWN.margin;
    let mut pick = stay;
    for o in options {
        let Some(routes) = route(o.0) else { continue };
        let v = clans::value_from(w, c, o.0, &routes, &o.1);
        if v > best {
            best = v;
            pick = o;
        }
    }
    pick
}

/// Clan `c`'s route to festival `mid` on tile `t` from home `h`, priced as `markets::go` prices it:
/// nothing from the festival's own tile, `None` when the festival is out of reach.
fn route_from(w: &World, s: &mut Scratch, c: &Clan, mid: u32, t: usize, h: usize) -> Option<Vec<Route>> {
    if h == t {
        return Some(vec![Route { to: t, path: Vec::new(), cost: 0.0 }]);
    }
    let f = festival_costs(w, s, h).into_iter().find(|f| f.market == mid)?;
    Some(vec![Route { to: t, path: f.path, cost: walk_cost(f.cost, 1.0) * walkers(c) }])
}

/// The home step `plain` (toward the fields, or to the middle of the land) of a clan `home_step`
/// does not weigh. With `Config.towns` on and fields open, a clan living on its festival's tile takes
/// it only when it feeds the clan better than staying by `TOWN.margin`, its route priced from the new
/// home: a clan does not walk away from the festival it lives at for the middle of its land alone.
pub fn keep(
    w: &World,
    s: &mut Scratch,
    c: &Clan,
    changed: &[usize],
    plain: (usize, Vec<usize>),
) -> (usize, Vec<usize>) {
    if plain.0 == c.home || !w.config.towns || !fields::open(w) {
        return plain;
    }
    let Some((mid, t)) = c.market.and_then(|m| w.markets.get(&m)).map(|m| (m.id, m.tile)) else {
        return plain;
    };
    if t != c.home {
        return plain;
    }
    let stay = (c.home, actors::trim(w, c.home, changed, CLAN.territory_radius));
    let here = [Route { to: t, path: Vec::new(), cost: 0.0 }];
    let was = clans::value_from(w, c, stay.0, &here, &stay.1);
    match route_from(w, s, c, mid, t, plain.0) {
        Some(routes) if clans::value_from(w, c, plain.0, &routes, &plain.1) > was + TOWN.margin => plain,
        _ => stay,
    }
}
