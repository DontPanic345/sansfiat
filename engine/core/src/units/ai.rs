//! The planner: how a civ plays for Standing with the player's own levers. Once a turn it reads a
//! civ's needs in order (feed the armies, hold the land with them, grow Standing, hurt the nearest rival, take
//! a capital it outweighs) and gives the orders that serve them. Every order goes through a sink
//! (`Give`): the AI's is `World::order` and `World::set_gift`, not logged, so a replay re-derives
//! them; the automatic player's (`script::Auto`) is `World::act`, logged as a player's are. The
//! planner has no order the player lacks.

use super::line::{line_by, path_between, ring, store_of, turns};
use super::paths::{path_cached, reaches, within_march, Paths};
use super::{camp, cut_off, givers_at, pack_cap, room, zone_cap, zone_of, zone_ok, UnitKind, EPS};
use crate::civs;
use crate::claim;
use crate::data::*;
use crate::land::{neighbours, steps};
use crate::market::total;
use crate::orders::Order;
use crate::settle;
use crate::world::World;

/// What the planner gives: an order to a unit or the claim, or the civ's gift a turn.
#[derive(Clone, Debug, PartialEq)]
pub enum Decision {
    Order(Order),
    Gift(f64),
}

/// Where the planner's decisions go: `Err` when the world refuses one, which the planner ignores.
pub type Give<'a> = &'a mut dyn FnMut(&mut World, u32, Decision) -> Result<(), String>;

/// `civ`'s front: of its stores (its capital and settlements) and the rival capitals reachable by
/// land, the pair fewest steps apart (ties to the capital, then the lower rival), as the store's
/// tile and the way from it to that capital.
fn front(w: &World, civ: u32, paths: &mut Paths) -> Option<(usize, Vec<usize>)> {
    let stores = settle::roots(w, civ);
    let mut best: Option<(i32, usize, u32)> = None;
    let mut way = None;
    for o in civs::alive(w).filter(|o| o.id != civ) {
        for (k, &s) in stores.iter().enumerate() {
            let key = (steps(w, s, o.capital), k, o.id);
            if best.is_some_and(|b| b <= key) {
                continue;
            }
            if let Some(p) = path_cached(w, paths, s, o.capital) {
                best = Some(key);
                way = Some((s, p));
            }
        }
    }
    way
}

/// Where an idle army of `civ` is posted: up to `UNIT.post` steps along the way from its front store
/// (`front`) to the rival capital nearest it, or with none reachable by land, from the capital to its
/// farthest clan, keeping to its own land and on land (not on a built pass). The store when there is
/// nowhere to go.
pub fn post(w: &World, civ: u32) -> usize {
    post_by(w, civ, &mut Paths::new())
}

fn post_by(w: &World, civ: u32, paths: &mut Paths) -> usize {
    let c = &w.civs[&civ];
    let to_rival = front(w, civ, paths);
    let from = to_rival.as_ref().map_or(c.capital, |f| f.0);
    let way = to_rival.map(|f| f.1).or_else(|| {
        let far = w
            .clans
            .values()
            .filter(|k| k.civ == Some(civ))
            .max_by_key(|k| (steps(w, c.capital, k.home), std::cmp::Reverse(k.id)))?;
        path_cached(w, paths, c.capital, far.home)
    });
    let mut at = from;
    for t in way.unwrap_or_default().into_iter().take(UNIT.post) {
        // A camp stands only on land (`check_move`), so a built pass over a mountain ends the way.
        if civs::tile_civ(w, t) != Some(civ) || !TERRAIN[w.tiles[t].terrain].land {
            break;
        }
        at = t;
    }
    at
}

/// The `ai` step, before `units`: the planner's turn for each civ the player doesn't lead. Draws no
/// random numbers.
pub fn ai(w: &mut World) {
    if !w.phase.playing() {
        return;
    }
    let ids: Vec<u32> = civs::alive(w).map(|c| c.id).filter(|&c| Some(c) != w.player).collect();
    let mut paths = Paths::new();
    for civ in ids {
        plan_with(w, civ, &mut paths, &mut |w, civ, d| match d {
            Decision::Order(order) => w.order(civ, order),
            Decision::Gift(food) => w.set_gift(civ, food),
        });
    }
}

/// One civ's turn of planning, its decisions given to `give` as they are made, each read against
/// the world the last one left. In order: its gift, its claim, its army's size, and each
/// army's orders. It finds its own paths; `ai` shares one set over the turn.
pub fn plan(w: &mut World, civ: u32, give: Give) {
    plan_with(w, civ, &mut Paths::new(), give);
}

/// `plan`, with the turn's `paths`.
fn plan_with(w: &mut World, civ: u32, paths: &mut Paths, give: Give) {
    if w.civs.get(&civ).is_none_or(|c| c.is_out()) {
        return;
    }
    if w.config.gifts {
        gift(w, civ, give);
    }
    land(w, civ, give);
    signposts(w, civ, give);
    if !w.config.army {
        return;
    }
    muster(w, civ, paths, give);
    let armies: Vec<u32> = w.units.values().filter(|u| u.civ == civ && u.camp().is_some()).map(|u| u.id).collect();
    for id in armies {
        direct(w, civ, id, paths, give);
    }
}

/// The food `civ`'s mouths eat a turn: its armies' people.
fn mouths(w: &World, civ: u32) -> f64 {
    w.units.values().filter(|u| u.civ == civ && u.scout().is_none()).map(|u| u.people).sum::<f64>() * UNIT.food
}

/// The levy `civ`'s stores took this turn: the capital's and its settlements'.
fn levied(w: &World, civ: u32) -> f64 {
    w.civs[&civ].levied + settle::of(w, civ).filter_map(|s| w.settle_levied.get(&s.id)).sum::<f64>()
}

/// The food in `civ`'s stores: the capital's and its settlements'.
fn stored(w: &World, civ: u32) -> f64 {
    total(&w.civs[&civ].store) + settle::of(w, civ).map(|s| total(&s.store)).sum::<f64>()
}

/// Whether `civ`'s levy feeds its mouths and one more: the levy this turn, at all its stores, covers
/// them `PLAN.feed` times over, or its capital's store is so near its cap (`PLAN.full`) that the levy
/// is cut to the room left.
fn levy_feeds(w: &World, civ: u32) -> bool {
    let c = &w.civs[&civ];
    let eats = mouths(w, civ) + UNIT.food;
    let levied = levied(w, civ);
    levied + EPS >= PLAN.feed * eats || (total(&c.store) + EPS >= PLAN.full * CIV.store_cap && levied + EPS >= eats)
}

/// Whether `civ` can feed one more mouth raised at its store on `store`: its levy feeds it
/// (`levy_feeds`), a clan at that store's market can give a person, and the store holds
/// `PLAN.reserve` turns of the civ's mouths' food beyond `more`.
fn feeds_one_more(w: &World, civ: u32, more: f64, store: usize) -> bool {
    let eats = mouths(w, civ) + UNIT.food;
    levy_feeds(w, civ)
        && settle::store(w, civ, store).map_or(0.0, total) + EPS >= more + PLAN.reserve * eats
        && !givers_at(w, civ, store).is_empty()
}

/// The gift: `PLAN.gift_contested` a turn while a clan at its festival is a rival's or leans to one
/// (a rival gives there too, or holds its home), the default while only unaligned clans come, and
/// nothing when nobody but its own does.
fn gift(w: &mut World, civ: u32, give: Give) {
    let c = &w.civs[&civ];
    let others: Vec<&crate::clans::Clan> =
        w.clans.values().filter(|k| k.market == Some(c.market) && k.civ != Some(civ)).collect();
    let contested = others.iter().any(|k| k.civ.is_some() || k.leaning.keys().any(|&o| o != civ));
    let want = match (others.is_empty(), contested) {
        (true, _) => 0.0,
        (false, true) => PLAN.gift_contested,
        (false, false) => LEANING.gift_default,
    };
    if (c.gift - want).abs() > EPS {
        let _ = give(w, civ, Decision::Gift(want));
    }
}

/// The land: the claim (`claim_plan`) is sent when it differs from the civ's, where `Config.claims`
/// lets it claim.
fn land(w: &mut World, civ: u32, give: Give) {
    if claim::may_claim(w, civ) {
        let want = claim_plan(w, civ);
        if want != w.civs[&civ].claim {
            let _ = give(w, civ, Decision::Order(Order::Claim { tiles: want }));
        }
    }
}

/// The signposts: a signpost nobody has settled in `SETTLE.plan_stale` turns is removed; and every
/// `SETTLE.plan_every` turns, while the civ keeps fewer than `SETTLE.signposts`, one is posted on the
/// site the planner likes best (`signpost_plan`).
fn signposts(w: &mut World, civ: u32, give: Give) {
    if !settle::open(w) {
        return;
    }
    let stale: Vec<usize> =
        w.civs[&civ].signposts.iter().filter(|s| s.turn + SETTLE.plan_stale <= w.turn).map(|s| s.tile).collect();
    for tile in stale {
        let _ = give(w, civ, Decision::Order(Order::Unpost { tile }));
    }
    if !(w.turn + civ).is_multiple_of(SETTLE.plan_every) || w.civs[&civ].signposts.len() >= SETTLE.signposts {
        return;
    }
    if let Some(tile) = signpost_plan(w, civ) {
        let _ = give(w, civ, Decision::Order(Order::Signpost { tile }));
    }
}

/// The sites the planner weighs for `civ`'s next signpost, best first: the tiles a signpost may stand
/// on within `SETTLE.plan_range` steps of its capital or a settlement, at least `SETTLE.spacing` steps
/// from its other signposts, that nobody holds and that a clan can walk to from one of those seats
/// within `SETTLE.reach`, each with its worth as a site (`settle::worth`, above `SETTLE.fed`); ties to
/// the nearer a seat and then the lower tile.
pub fn signpost_sites(w: &World, civ: u32) -> Vec<(f64, usize)> {
    let c = &w.civs[&civ];
    let roots = settle::roots(w, civ);
    let near = crate::land::cost_map(w, &roots, SETTLE.reach, |i| crate::land::passable(w, i));
    let mut sites: Vec<(f64, usize)> = near
        .keys()
        .copied()
        .filter(|&t| w.tiles[t].claim.is_none())
        .filter(|&t| roots.iter().any(|&r| steps(w, r, t) <= SETTLE.plan_range))
        .filter(|&t| c.signposts.iter().all(|s| steps(w, s.tile, t) >= SETTLE.spacing))
        .filter(|&t| settle::check_post(w, civ, t).is_ok())
        .map(|t| (settle::worth(w, t), t))
        .filter(|&(v, _)| v > SETTLE.fed)
        .collect();
    let d = |t: usize| roots.iter().map(|&r| steps(w, r, t)).min();
    sites.sort_by(|a, b| b.0.total_cmp(&a.0).then(d(a.1).cmp(&d(b.1))).then(a.1.cmp(&b.1)));
    sites
}

/// The tile the planner would post `civ`'s next signpost on: the best of `signpost_sites`.
pub fn signpost_plan(w: &World, civ: u32) -> Option<usize> {
    signpost_sites(w, civ).first().map(|&(_, t)| t)
}

/// The people of `civ`'s armies.
fn armed(w: &World, civ: u32) -> f64 {
    w.units.values().filter(|u| u.civ == civ && u.camp().is_some()).map(|u| u.people).sum()
}

/// The claim the planner wants for `civ` this turn, from the one it has. It leaves the tiles a rival
/// army holds or stands on (with `Config.fight` on). Its armies hold its land, `PLAN.hold` tiles a
/// person: while they are enough for a wider claim it widens by `PLAN.widen` tiles: the land next
/// to the claim, nobody's, nearest a clan's home it doesn't yet claim. Sorted, with the capital,
/// connected to it.
pub fn claim_plan(w: &World, civ: u32) -> Vec<usize> {
    let c = &w.civs[&civ];
    let rival: Vec<usize> = w
        .units
        .values()
        .filter(|u| w.config.fight && u.civ != civ && u.camp().is_some())
        .flat_map(|u| u.held().iter().copied().chain(std::iter::once(u.home)))
        .collect();
    let mut tiles: Vec<usize> = c.claim.iter().copied().filter(|t| !rival.contains(t)).collect();
    if w.config.army && armed(w, civ) * PLAN.hold + EPS >= (tiles.len() + PLAN.widen) as f64 {
        // The clans' homes the claim doesn't cover, whoever's they are.
        let homes: Vec<usize> = w.clans.values().map(|k| k.home).filter(|h| tiles.binary_search(h).is_err()).collect();
        let near = |t: usize| homes.iter().map(|&h| steps(w, t, h)).min();
        let mut edge: Vec<(i32, i32, usize)> = tiles
            .iter()
            .flat_map(|&t| neighbours(w, t))
            .filter(|&t| {
                tiles.binary_search(&t).is_err()
                    && TERRAIN[w.tiles[t].terrain].land
                    && !rival.contains(&t)
                    && civs::tile_civ(w, t).is_none()
            })
            .filter_map(|t| near(t).filter(|&d| d <= PLAN.widen_reach).map(|d| (d, steps(w, c.capital, t), t)))
            .collect();
        edge.sort_unstable();
        edge.dedup();
        tiles.extend(edge.into_iter().take(PLAN.widen).map(|(_, _, t)| t));
    }
    tiles.sort_unstable();
    tiles.dedup();
    claim::connected_part(w, civ, &tiles)
}

/// The army's size. A hungry army at a store of its civ, or stores that together can't feed the
/// civ's mouths a turn, sends a warrior home. Else, while the civ can feed one more
/// (`feeds_one_more`, with the new warrior's pack) and its army is under its cap, it raises one: into
/// its army where its line isn't cut, from the store the line ends at; or a new army at its front
/// store (`front`) when that can raise one, else at the capital. The cap is `PLAN.army` of its people
/// while a rival's capital is within `PLAN.march` turns' walk of one of its stores or a rival army is
/// on its land, and a party of `PLAN.party`, which can live off the land, while none is.
fn muster(w: &mut World, civ: u32, paths: &mut Paths, give: Give) {
    let c = &w.civs[&civ];
    let army = camp(w, civ).map(|u| (u.id, u.people, u.hungry, u.home));
    let starving = stored(w, civ) + EPS < mouths(w, civ);
    let stores = settle::roots(w, civ);
    if let Some((id, _, hungry, home)) = army {
        if (hungry > 0 && stores.contains(&home)) || starving {
            let _ = give(w, civ, Decision::Order(Order::Disband { unit: id, n: Some(1), fighter: None }));
            return;
        }
    }
    let people: f64 = w.clans.values().filter(|k| k.civ == Some(civ)).map(|k| k.people).sum();
    let armed = army.map_or(0.0, |a| a.1);
    // A rival within a pack's march of a store, or on its land, is worth a full army; else a party.
    let marches: Vec<_> = stores.iter().map(|&s| (s, within_march(w, s))).collect();
    let near =
        civs::alive(w).filter(|o| o.id != civ).any(|o| {
            marches.iter().any(|(s, r)| {
                r.contains_key(&o.capital)
                    && path_between(w, *s, o.capital).is_some_and(|(p, _)| turns(w, &p, *s, 0.0) <= PLAN.march)
            })
        }) || w.units.values().any(|o| o.civ != civ && o.camp().is_some() && c.claim.binary_search(&o.home).is_ok());
    let cap = if near { PLAN.army * (people + armed) } else { PLAN.party.min(PLAN.army * (people + armed)) };
    if armed + 1.0 > cap + EPS {
        return;
    }
    let into = army.map(|a| a.0).filter(|&id| !cut_off(w, id));
    if army.is_some() && into.is_none() {
        return;
    }
    let capital = c.capital;
    let at = match into {
        Some(id) => store_of(w, id),
        None => {
            front(w, civ, paths).map(|f| f.0).filter(|&s| feeds_one_more(w, civ, pack_cap(1.0), s)).or(Some(capital))
        }
    };
    let Some(store) = at.filter(|&s| feeds_one_more(w, civ, pack_cap(1.0), s)) else { return };
    let at = if into.is_some() || store == capital { None } else { Some(store) };
    let order = Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into, at };
    let _ = give(w, civ, Decision::Order(order));
}

/// Whether army `id` outweighs `rival`'s capital: the fight there played forward by the fight's own
/// rule, its people attacking the armies standing on it. A capital no army stands on falls to
/// whoever walks onto it. It outweighs a held capital when the armies there are gone or beaten off
/// within `PLAN.siege` turns, the army never having to retreat, with `PLAN.left` of its people left.
pub fn outweighs(w: &World, id: u32, rival: u32) -> bool {
    let (Some(u), Some(r)) = (w.units.get(&id), w.civs.get(&rival).filter(|r| !r.is_out())) else { return false };
    let attack = super::fight::might(u).0;
    let mut a = u.people;
    // The defenders: people and strength a person.
    let mut parts: Vec<(f64, f64)> = w
        .units
        .values()
        .filter(|o| o.civ == rival && o.camp().is_some() && (o.home == r.capital || o.held().contains(&r.capital)))
        .map(|o| (o.people, super::fight::might(o).1.max(FIGHTERS[0].defence)))
        .collect();
    for _ in 0..PLAN.siege {
        let strength: f64 = parts.iter().map(|p| p.0 * p.1).sum();
        let people: f64 = parts.iter().map(|p| p.0).sum();
        if people <= EPS {
            return a + EPS >= PLAN.left * u.people;
        }
        let loss = (FIGHT.rate * a * attack).min(people);
        for p in &mut parts {
            p.0 -= loss * p.0 / people.max(EPS);
        }
        a -= (FIGHT.rate * strength).min(a);
        let left: f64 = parts.iter().map(|p| p.0 * p.1).sum();
        if a <= EPS || a * attack < FIGHT.retreat * left {
            return false;
        }
        if left < FIGHT.retreat * a * attack {
            return a + EPS >= PLAN.left * u.people;
        }
    }
    false
}

/// The strength an army of `civ` entering `tile` would fight: the rival armies standing there,
/// each defending.
fn defence_at(w: &World, civ: u32, tile: usize) -> f64 {
    w.units
        .values()
        .filter(|o| o.civ != civ && o.camp().is_some() && (o.home == tile || o.held().contains(&tile)))
        .map(|o| o.people * super::fight::might(o).1)
        .sum()
}

/// What the army is for this turn, the first that applies:
/// 1. **Feed it.** Hungry, or its line cut by a rival: it marches home, to the store its line ends
///    at (the capital with no line), through whoever stands in
///    the way. It leaves its own land only with `PLAN.packed` of its pack; off it with under
///    `PLAN.low` turns in its pack it halts and lives off the tile (foraging, or pillaging a
///    rival's), and marches home once the tile is bare.
/// 2. **Hold the land.** A rival army on its civ's land or beside it: it marches onto it
///    when it outweighs it by `PLAN.edge`, else it stands across that army's line where its own line
///    feeds it, else it stands on its capital.
/// 3. **Take a capital.** A rival's capital it outweighs (`outweighs`), no more than `PLAN.march`
///    turns' walk away: it marches on it, and off again the turn it no longer outweighs it.
/// 4. **Hurt the nearest rival.** Of `PLAN.raid` people or more: a rival's land inside its ring
///    (`PLAN.reach`) whose defenders it outweighs, a clan's home or ground first: it marches there,
///    takes the tile by standing on it and pillages it; and a rival army abroad whose line it can stand across.
/// 5. Else it stands at its post with a zone along its line home.
fn direct(w: &mut World, civ: u32, id: u32, paths: &mut Paths, give: Give) {
    let Some(u) = w.units.get(&id) else { return };
    let (here, dest, people) = (u.home, u.dest(), u.people);
    let capital = w.civs[&civ].capital;
    // Home is the store its line ends at: the capital when it has no line.
    let home = store_of(w, id).unwrap_or(capital);
    let strength = people * super::fight::might(u).0;
    let go = |w: &mut World, give: Give, to: usize, why: String| {
        let at = w.units.get(&id).map(|u| (u.home, u.dest()));
        let Some((home, dest)) = at else { return };
        if dest == Some(to) || (dest.is_none() && home == to) {
            return;
        }
        w.note(id, why);
        let _ = give(w, civ, Decision::Order(Order::Move { unit: id, to }));
    };

    // 1. Feed it.
    let severed = u.camp().is_some_and(|c| c.cut_by.is_some());
    if here != home && severed {
        return go(w, give, home, "a rival cuts its line: it marches home".to_string());
    }
    let ground = civs::tile_civ(w, here);
    let home_ground = ground == Some(civ);
    let low = total(&u.store) < PLAN.low * people * UNIT.food;
    if low && !home_ground && w.tiles[here].fallow == 0 && total(&w.tiles[here].stock) > EPS {
        if dest.is_some() {
            w.note(id, "its pack is low: it halts to live off the land".to_string());
            let _ = give(w, civ, Decision::Order(Order::Move { unit: id, to: here }));
        }
        let order = if ground.is_some() { Order::Pillage { unit: id } } else { Order::Forage { unit: id } };
        let _ = give(w, civ, Decision::Order(order));
        return;
    }
    if here != home && (u.hungry > 0 || (low && !home_ground)) {
        let why = if u.hungry > 0 {
            "hungry: it marches home"
        } else {
            "its pack is low and the land here is bare: it marches home"
        };
        return go(w, give, home, why.to_string());
    }
    // An army sets out with its pack full: on its own land it waits for it.
    let packed = total(&u.store) + EPS >= PLAN.packed * pack_cap(people);
    let party = people <= PLAN.party + EPS;
    if !w.config.fight {
        return stand(w, civ, id, paths, give);
    }

    // 2. Hold the land: the nearest rival army on the claim, beside it or beside the capital.
    let claim = &w.civs[&civ].claim;
    let on_land =
        |t: usize| claim.binary_search(&t).is_ok() || neighbours(w, t).any(|n| claim.binary_search(&n).is_ok());
    let threat = w
        .units
        .values()
        .filter(|o| o.civ != civ && o.camp().is_some() && o.people > EPS)
        .filter(|o| on_land(o.home) || steps(w, o.home, capital) <= 1)
        .min_by_key(|o| (steps(w, o.home, capital), o.id))
        .map(|o| (o.id, o.home, o.people * super::fight::might(o).1));
    if let Some((foe, at, theirs)) = threat {
        if strength > PLAN.edge * theirs {
            return go(w, give, at, format!("marches on the rival army at {}: it outweighs it", w.at(at)));
        }
        if let Some(t) = across(w, civ, id, foe, paths) {
            if here == t && dest.is_none() {
                return cut(w, civ, id, foe, give);
            }
            return go(w, give, t, format!("marches to {} to stand across a rival army's line", w.at(t)));
        }
        return go(w, give, capital, "a rival army it can't beat is on its land: it stands on its capital".to_string());
    }

    // 3. Take a capital it outweighs.
    let prize = civs::alive(w)
        .filter(|o| o.id != civ)
        .filter_map(|o| {
            let p = path_cached(w, paths, here, o.capital)?;
            (party || turns(w, &p, here, 0.0) <= PLAN.march || here == o.capital).then_some((p.len(), o.id, o.capital))
        })
        .filter(|&(_, o, _)| outweighs(w, id, o))
        .min()
        .map(|(_, o, t)| (o, t));
    let fill = |w: &mut World, give: Give| {
        let name = settle::store_name(w, civ, home);
        go(w, give, home, format!("fills its pack at {name} before it sets out"));
    };
    if let Some((rival, t)) = prize {
        if !packed && home_ground && dest != Some(t) {
            return fill(w, give);
        }
        let name = w.civs[&rival].name.clone();
        return go(w, give, t, format!("marches on {name}'s capital at {}: it outweighs it", w.at(t)));
    }
    let on_capital = civs::alive(w).any(|o| o.id != civ && (o.capital == here || Some(o.capital) == dest));
    if on_capital {
        return go(w, give, home, "it no longer outweighs the capital: it pulls back".to_string());
    }

    // 4. Hurt the nearest rival.
    if people + EPS >= PLAN.raid {
        let abroad: Vec<u32> = w
            .units
            .values()
            .filter(|o| o.civ != civ && o.camp().is_some() && civs::tile_civ(w, o.home) != Some(o.civ))
            .map(|o| o.id)
            .collect();
        for foe in abroad {
            if let Some(t) = across(w, civ, id, foe, paths) {
                if here == t && dest.is_none() {
                    return cut(w, civ, id, foe, give);
                }
                return go(w, give, t, format!("marches to {} to stand across a rival army's line", w.at(t)));
            }
        }
        if ground.is_some_and(|c| c != civ) && dest.is_none() && w.tiles[here].fallow == 0 {
            if room(&w.units[&id]) > EPS {
                let _ = give(w, civ, Decision::Order(Order::Pillage { unit: id }));
            }
            return;
        }
        if let Some(t) = raid(w, civ, id, strength, party, paths) {
            if !packed && home_ground && dest != Some(t) {
                return fill(w, give);
            }
            return go(w, give, t, format!("marches to {} to take a rival's land", w.at(t)));
        }
    }

    // 5. Its post.
    stand(w, civ, id, paths, give)
}

/// The rival land army `id` would take: a tile of a rival's land, not a capital and not the
/// tile it stands on, whose defenders it outweighs by `PLAN.edge`; inside `civ`'s ring where its
/// line takes no more than `PLAN.reach` of its days, or anywhere it can walk when it is a `party`,
/// small enough to live off the land. The nearest, a rival clan's home counting two steps nearer
/// than its ground and that two nearer than bare land.
fn raid(w: &World, civ: u32, id: u32, strength: f64, party: bool, paths: &mut Paths) -> Option<usize> {
    let from = w.units[&id].home;
    let grounds: Vec<(usize, bool)> = w
        .clans
        .values()
        .filter(|k| k.civ.is_some_and(|c| c != civ))
        .flat_map(|k| k.territory.iter().map(move |&t| (t, t == k.home)))
        .collect();
    let ring = ring(w, civ);
    let mut tiles: Vec<(i32, usize)> = civs::alive(w)
        .filter(|o| o.id != civ)
        .flat_map(|o| claim::held(w, o.id).into_iter().filter(move |&t| t != o.capital))
        .filter(|&t| t != from && TERRAIN[w.tiles[t].terrain].land && w.tiles[t].fallow == 0)
        .filter(|&t| civs::tile_civ(w, t).is_some_and(|c| c != civ))
        .filter(|t| party || ring.get(t).is_some_and(|&share| share <= PLAN.reach))
        .map(|t| {
            let kind = match (grounds.iter().any(|&(g, home)| g == t && home), grounds.iter().any(|&(g, _)| g == t)) {
                (true, _) => 0,
                (false, true) => 2,
                _ => 4,
            };
            (steps(w, from, t) + kind, t)
        })
        .collect();
    tiles.sort_unstable();
    tiles
        .into_iter()
        .take(PLAN.looks)
        .find(|&(_, t)| strength > PLAN.edge * defence_at(w, civ, t) && reaches(w, paths, from, t))
        .map(|(_, t)| t)
}

/// The tile where army `id` would stand across rival army `foe`'s line: the first tile of that line
/// past the rival's own that is no capital and not the rival's land, that it can walk to without
/// passing that army, where nobody of a rival's stands, and where its own line would carry its
/// food. `None` when the rival has no line or no such tile.
fn across(w: &World, civ: u32, id: u32, foe: u32, paths: &mut Paths) -> Option<usize> {
    let o = w.units.get(&foe)?;
    let line = &o.actor.routes.first()?.path;
    let from = w.units[&id].home;
    line.iter().copied().find(|&t| {
        t != o.home
            && !civs::alive(w).any(|c| c.capital == t)
            && TERRAIN[w.tiles[t].terrain].land
            && civs::tile_civ(w, t) != Some(o.civ)
            && !super::past_reach(w, id, t)
            && (t == from || path_cached(w, paths, from, t).is_some_and(|p| !p.contains(&o.home)))
            && defence_at(w, civ, t) <= EPS
    })
}

/// Army `id`, standing on rival army `foe`'s line, paints its zone across it: its own tile and the
/// line's tiles beside it, as many as its people hold. It keeps the zone it has.
fn cut(w: &mut World, civ: u32, id: u32, foe: u32, give: Give) {
    if !w.config.zone {
        return;
    }
    let u = &w.units[&id];
    let Some(line) = w.units.get(&foe).and_then(|o| o.actor.routes.first()).map(|r| r.path.clone()) else { return };
    let beside: Vec<usize> = neighbours(w, u.home).filter(|t| line.contains(t)).collect();
    let tiles: Vec<usize> = beside.into_iter().take(zone_cap(u).saturating_sub(1)).collect();
    if zone_of(u.home, &tiles) != u.actor.territory && zone_ok(w, u, &tiles).is_ok() {
        w.note(id, "stands across a rival army's line".to_string());
        let _ = give(w, civ, Decision::Order(Order::Zone { unit: id, tiles }));
    }
}

/// An idle army goes to its post (`post`), and there paints its zone along its line home.
fn stand(w: &mut World, civ: u32, id: u32, paths: &mut Paths, give: Give) {
    let u = &w.units[&id];
    if u.camp().is_some_and(|c| c.dest.is_some()) {
        return;
    }
    let here = u.home;
    let at = post_by(w, civ, paths);
    if at != here {
        let text = format!("posted at {}, on the way to its nearest rival or its farthest clan", w.at(at));
        w.note(id, text);
        let _ = give(w, civ, Decision::Order(Order::Move { unit: id, to: at }));
    } else if w.config.zone {
        post_zone(w, civ, id, give);
    }
}

/// The zone of a posted, idle army: the first `UNIT.zone` tiles of its line home (no more than its
/// warriors less one, never the capital), when it can afford it: its line, read without held ground,
/// still carries a turn's food on the labour guarding leaves. Otherwise it undeploys. Kept as it is
/// while a rival's ground cuts the line.
fn post_zone(w: &mut World, civ: u32, id: u32, give: Give) {
    let u = &w.units[&id];
    let Some((p, _, carry)) = line_by(w, id, false) else { return };
    let afford = carry * (1.0 - UNIT.guarding / UNIT.labour) + EPS >= u.people * UNIT.food;
    let stores = settle::roots(w, u.civ);
    let k = UNIT.zone.min(zone_cap(u).saturating_sub(1));
    let tiles: Vec<usize> =
        if afford { p.into_iter().filter(|t| !stores.contains(t)).take(k).collect() } else { Vec::new() };
    if zone_of(u.home, &tiles) != u.actor.territory && zone_ok(w, u, &tiles).is_ok() {
        let _ = give(w, civ, Decision::Order(Order::Zone { unit: id, tiles }));
    }
}
