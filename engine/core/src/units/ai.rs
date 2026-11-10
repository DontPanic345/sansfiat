//! The planner: how a civ plays for Standing with the player's own levers. Once a turn it reads a
//! civ's needs in order (feed the armies, hold the land, grow Standing, hurt the nearest rival, take
//! a capital it outweighs) and gives the orders that serve them. Every order goes through a sink
//! (`Give`): the AI's is `World::order` and `World::set_gift`, not logged, so a replay re-derives
//! them; the automatic player's (`script::Auto`) is `World::act`, logged as a player's are. The
//! planner has no order the player lacks.

use std::collections::BTreeMap;

use super::line::{line_by, path_between, ring, turns};
use super::{camp, cut_off, givers, guard_of, pack_cap, room, zone_cap, zone_of, zone_ok, Unit, UnitKind, EPS};
use crate::civs;
use crate::claim;
use crate::data::*;
use crate::land::{neighbours, steps};
use crate::market::total;
use crate::orders::Order;
use crate::world::World;

/// What the planner gives: an order to a unit or the claim, or the civ's gift a turn.
#[derive(Clone, Debug, PartialEq)]
pub enum Decision {
    Order(Order),
    Gift(f64),
}

/// Where the planner's decisions go: `Err` when the world refuses one, which the planner ignores.
pub type Give<'a> = &'a mut dyn FnMut(&mut World, u32, Decision) -> Result<(), String>;

/// The paths `post` reads, by their ends, as `path_between` finds them. `ai` keeps one for its turn.
type Paths = BTreeMap<(usize, usize), Option<Vec<usize>>>;

/// `path_between`'s tiles from `from` to `to`, found once in `paths`.
fn path_cached(w: &World, paths: &mut Paths, from: usize, to: usize) -> Option<Vec<usize>> {
    paths.entry((from, to)).or_insert_with(|| path_between(w, from, to).map(|(p, _)| p)).clone()
}

/// Where an idle army of `civ` is posted: up to `UNIT.post` steps along the way from the capital to
/// the nearest rival capital reachable by land, or with none, to its farthest clan, keeping to its
/// own land and on land (not on a built pass). The capital when there is nowhere to go.
pub fn post(w: &World, civ: u32) -> usize {
    post_by(w, civ, &mut Paths::new())
}

fn post_by(w: &World, civ: u32, paths: &mut Paths) -> usize {
    let c = &w.civs[&civ];
    let to_rival = civs::alive(w)
        .filter(|o| o.id != civ)
        .filter_map(|o| {
            path_cached(w, paths, c.capital, o.capital).map(|p| ((steps(w, c.capital, o.capital), o.id), p))
        })
        .min_by_key(|(k, _)| *k)
        .map(|(_, p)| p);
    let way = to_rival.or_else(|| {
        let far = w
            .clans
            .values()
            .filter(|k| k.civ == Some(civ))
            .max_by_key(|k| (steps(w, c.capital, k.home), std::cmp::Reverse(k.id)))?;
        path_cached(w, paths, c.capital, far.home)
    });
    let mut at = c.capital;
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
    for civ in ids {
        plan(w, civ, &mut |w, civ, d| match d {
            Decision::Order(order) => w.order(civ, order),
            Decision::Gift(food) => w.set_gift(civ, food),
        });
    }
}

/// One civ's turn of planning, its decisions given to `give` as they are made, each read against
/// the world the last one left. In order: its gift, its claim and guard, its army's size, and each
/// army's orders.
pub fn plan(w: &mut World, civ: u32, give: Give) {
    if w.civs.get(&civ).is_none_or(|c| c.is_out()) {
        return;
    }
    if w.config.gifts {
        gift(w, civ, give);
    }
    land(w, civ, give);
    if !w.config.army {
        return;
    }
    muster(w, civ, give);
    let armies: Vec<u32> = w.units.values().filter(|u| u.civ == civ && u.camp().is_some()).map(|u| u.id).collect();
    let mut paths = Paths::new();
    for id in armies {
        direct(w, civ, id, &mut paths, give);
    }
}

/// The food `civ`'s mouths eat a turn: its guard's and its armies' people.
fn mouths(w: &World, civ: u32) -> f64 {
    w.units.values().filter(|u| u.civ == civ && u.scout().is_none()).map(|u| u.people).sum::<f64>() * UNIT.food
}

/// Whether `civ`'s levy feeds its mouths and one more: the levy this turn covers them `PLAN.feed`
/// times over, or its store is so near its cap (`PLAN.full`) that the levy is cut to the room left.
fn levy_feeds(w: &World, civ: u32) -> bool {
    let c = &w.civs[&civ];
    let eats = mouths(w, civ) + UNIT.food;
    c.levied + EPS >= PLAN.feed * eats || (total(&c.store) + EPS >= PLAN.full * CIV.store_cap && c.levied + EPS >= eats)
}

/// Whether `civ` can feed one more mouth: its levy feeds it (`levy_feeds`), a clan can give a
/// person, and its store holds `PLAN.reserve` turns of its mouths' food beyond `more`.
fn feeds_one_more(w: &World, civ: u32, more: f64) -> bool {
    let eats = mouths(w, civ) + UNIT.food;
    levy_feeds(w, civ) && total(&w.civs[&civ].store) + EPS >= more + PLAN.reserve * eats && !givers(w, civ).is_empty()
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
/// lets it claim; then its guard (`guard_order`).
fn land(w: &mut World, civ: u32, give: Give) {
    if claim::may_claim(w, civ) {
        let want = claim_plan(w, civ);
        if want != w.civs[&civ].claim {
            let _ = give(w, civ, Decision::Order(Order::Claim { tiles: want }));
        }
    }
    if let Some(order) = guard_plan(w, civ) {
        let _ = give(w, civ, Decision::Order(order));
    }
}

/// The planner's guard: `guard_order`, raising only while the levy feeds the civ's mouths and one
/// more (`levy_feeds`) and no rival army presses its capital (a pressed guard is all drawn back, so
/// the paper it leaves is not for want of guards). A guard the levy no longer feeds one more of
/// sends a person home while every tile it holds has `PLAN.thick` floors of guard.
fn guard_plan(w: &World, civ: u32) -> Option<Order> {
    let c = &w.civs[&civ];
    let fed = levy_feeds(w, civ) && !c.pressed;
    let order = guard_order(w, civ).filter(|o| fed || !matches!(o, Order::Raise { .. }));
    if order.is_some() || fed || c.pressed {
        return order;
    }
    let g = guard_of(w, civ).filter(|g| g.people > 1.0 + EPS)?;
    let thick = c
        .claim
        .iter()
        .zip(&c.guard)
        .all(|(&t, &share)| t == c.capital || share + EPS >= PLAN.thick * CLAIM.floor);
    thick.then_some(Order::Disband { unit: g.id, n: Some(1), fighter: None })
}

/// The claim the planner wants for `civ` this turn, from the one it has. It leaves the tiles a rival
/// army holds or stands on (with `Config.fight` on: a tile a stack took would come back under it and
/// be fought for again each turn). While some of it is paper and no guard can be raised, it gives up
/// the paper tile farthest from the capital. While all of it is held and it can feed one more guard,
/// it widens by `PLAN.widen` tiles: the land next to the claim, no rival's, nearest a clan's home it
/// doesn't yet claim. Sorted, with the capital, connected to it.
pub fn claim_plan(w: &World, civ: u32) -> Vec<usize> {
    let c = &w.civs[&civ];
    let rival: Vec<usize> = w
        .units
        .values()
        .filter(|u| w.config.fight && u.civ != civ && u.camp().is_some())
        .flat_map(|u| u.held().iter().copied().chain(std::iter::once(u.home)))
        .collect();
    let mut tiles: Vec<usize> = c.claim.iter().copied().filter(|t| !rival.contains(t)).collect();
    let paper: Vec<usize> = c
        .claim
        .iter()
        .enumerate()
        .filter(|&(k, &t)| t != c.capital && c.guard.get(k).copied().unwrap_or(0.0) + EPS < CLAIM.floor)
        .map(|(_, &t)| t)
        .collect();
    let raising = matches!(guard_plan(w, civ), Some(Order::Raise { .. }));
    if !paper.is_empty() {
        if !raising {
            let far = paper.iter().copied().max_by_key(|&t| (steps(w, c.capital, t), t));
            tiles.retain(|&t| Some(t) != far);
        }
    } else if w.config.army && feeds_one_more(w, civ, civs::keep(w, civ)) {
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
    claim::connected_part(w, c.capital, &tiles)
}

/// The rule for `civ`'s guard, as the order it would give: a guard that went hungry last turn
/// loses a person (`Disband`); else the guard is raised a person a turn while a claimed tile under
/// `CLAIM.floor` has no guard and its store and its clans' stores, after `keep`, hold
/// `GUARD.keep_turns` turns of food for one more guard. `None` when it does neither. A story keeps
/// the led civ's guard by giving this order through `act`.
pub fn guard_order(w: &World, civ: u32) -> Option<Order> {
    if !w.config.army || !claim::may_claim(w, civ) {
        return None;
    }
    if let Some((id, hungry)) = guard_of(w, civ).map(|u| (u.id, u.hungry)) {
        if hungry > 0 {
            return Some(Order::Disband { unit: id, n: Some(1), fighter: None });
        }
    }
    let c = &w.civs[&civ];
    let short = c
        .claim
        .iter()
        .enumerate()
        .any(|(k, &t)| t != c.capital && c.guard.get(k).copied().unwrap_or(0.0) + EPS < CLAIM.floor);
    let people = claim::guard_people(w, civ);
    // The guard eats from the store and then its clans' stores, so both count to feed one more.
    let kin: f64 = w.clans.values().filter(|k| k.civ == Some(civ)).map(|k| total(&k.actor.store)).sum();
    let spare = total(&c.store) + kin - civs::keep(w, civ);
    (short && spare >= GUARD.keep_turns * UNIT.food * (people + 1.0)).then_some(Order::Raise {
        n: 1,
        kind: UnitKind::Guard,
        fighter: Fighter::Warriors,
        into: None,
    })
}

/// The army's size. A hungry army at its capital, or a store that can't feed the civ's mouths a
/// turn, sends a warrior home. Else, while the civ can feed one more (`feeds_one_more`, with the
/// new warrior's pack) and its army is under its cap, it raises one: a new army at the capital, or
/// into its army where its line isn't cut. The cap is `PLAN.army` of its people while a rival's
/// capital is within `PLAN.march` turns' walk or a rival army is on its claim, and a party of
/// `PLAN.party`, which can live off the land, while none is.
fn muster(w: &mut World, civ: u32, give: Give) {
    let c = &w.civs[&civ];
    let army = camp(w, civ).map(|u| (u.id, u.people, u.hungry, u.home));
    let starving = total(&c.store) + EPS < mouths(w, civ);
    if let Some((id, _, hungry, home)) = army {
        if (hungry > 0 && home == c.capital) || starving {
            let _ = give(w, civ, Decision::Order(Order::Disband { unit: id, n: Some(1), fighter: None }));
            return;
        }
    }
    let people: f64 = w.clans.values().filter(|k| k.civ == Some(civ)).map(|k| k.people).sum();
    let armed = army.map_or(0.0, |a| a.1);
    // A rival within a pack's march, or on its land, is worth a full army; else a party.
    let capital = c.capital;
    let near = civs::alive(w).filter(|o| o.id != civ).any(|o| {
        path_between(w, capital, o.capital).is_some_and(|(p, _)| turns(w, &p, capital, 0.0) <= PLAN.march)
    }) || w.units.values().any(|o| o.civ != civ && o.camp().is_some() && c.claim.binary_search(&o.home).is_ok());
    let cap = if near { PLAN.army * (people + armed) } else { PLAN.party.min(PLAN.army * (people + armed)) };
    if armed + 1.0 > cap + EPS || !feeds_one_more(w, civ, pack_cap(1.0)) {
        return;
    }
    let into = army.map(|a| a.0).filter(|&id| !cut_off(w, id));
    if army.is_some() && into.is_none() {
        return;
    }
    let order = Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into };
    let _ = give(w, civ, Decision::Order(order));
}

/// An army's strength a person, attacking and defending as it stands (entrenchment and hunger in).
fn might(u: &Unit) -> (f64, f64) {
    let Some(c) = u.camp() else { return (0.0, 0.0) };
    let sum: f64 = c.stack.iter().sum();
    let per = |f: fn(&FighterRules) -> f64| {
        if sum <= EPS {
            f(&FIGHTERS[0])
        } else {
            Fighter::ALL.iter().map(|k| c.stack[k.index()] * f(k.rules())).sum::<f64>() / sum
        }
    };
    let hungry = if u.hungry > 0 { FIGHT.hungry } else { 1.0 };
    (per(|r| r.attack) * hungry, per(|r| r.defence) * super::fight::entrenched(c.still) * hungry)
}

/// Whether army `id` outweighs `rival`'s capital: the fight there played forward by the fight's own
/// rule, its people attacking the whole guard drawn back and entrenched, the militia of the clans
/// living there and any army standing on it. It outweighs the capital when the capital falls within
/// `PLAN.siege` turns, the army never having to retreat, with `PLAN.left` of its people left.
pub fn outweighs(w: &World, id: u32, rival: u32) -> bool {
    let (Some(u), Some(r)) = (w.units.get(&id), w.civs.get(&rival).filter(|r| !r.is_out())) else { return false };
    let attack = might(u).0;
    let mut a = u.people;
    // The defenders: people and strength a person.
    let guard_def = FIGHTERS[0].defence * super::fight::entrenched(u32::MAX);
    let mut parts: Vec<(f64, f64, bool)> = vec![(claim::guard_people(w, rival), guard_def, false)];
    let militia: f64 = w.clans.values().filter(|k| k.civ == Some(rival) && k.home == r.capital).map(|k| k.people).sum();
    parts.push((militia, FIGHTERS[0].defence * FIGHT.militia, true));
    for o in w.units.values().filter(|o| o.civ == rival && o.camp().is_some() && o.home == r.capital) {
        parts.push((o.people, might(o).1.max(FIGHTERS[0].defence), false));
    }
    for _ in 0..PLAN.siege {
        let strength: f64 = parts.iter().map(|p| p.0 * p.1).sum();
        let people: f64 = parts.iter().map(|p| p.0).sum();
        let loss = (FIGHT.rate * a * attack).min(people);
        for p in &mut parts {
            p.0 -= loss * p.0 / people.max(EPS);
        }
        a -= (FIGHT.rate * strength).min(a);
        let left: f64 = parts.iter().map(|p| p.0 * p.1).sum();
        let fallen = parts.iter().all(|p| if p.2 { p.0 < 1.0 } else { p.0 <= EPS });
        if fallen {
            return a + EPS >= PLAN.left * u.people;
        }
        if a <= EPS || a * attack < FIGHT.retreat * left {
            return false;
        }
    }
    false
}

/// The people of `rival` an army entering `tile` would fight, as strength: its guard's share there,
/// the militia of its clans living there, and its armies standing there, each defending.
fn defence_at(w: &World, civ: u32, tile: usize) -> f64 {
    let mut sum = 0.0;
    for o in civs::alive(w).filter(|o| o.id != civ) {
        let held = claim::held(w, o.id);
        if held.binary_search(&tile).is_ok() {
            let shares = claim::guard_shares(w, o.id);
            let on: f64 = o.claim.iter().zip(&shares).filter(|(t, _)| held.binary_search(t).is_ok()).map(|(_, g)| g).sum();
            let here = o.claim.binary_search(&tile).ok().and_then(|k| shares.get(k).copied()).unwrap_or(0.0);
            if on > EPS {
                sum += claim::guard_people(w, o.id) * here / on
                    * FIGHTERS[0].defence
                    * super::fight::entrenched(u32::MAX);
            }
        }
    }
    for k in w.clans.values().filter(|k| k.home == tile && k.civ.is_some_and(|c| c != civ)) {
        sum += k.people * FIGHTERS[0].defence * FIGHT.militia;
    }
    for o in w.units.values().filter(|o| o.civ != civ && o.camp().is_some() && o.home == tile) {
        sum += o.people * might(o).1;
    }
    sum
}

/// What the army is for this turn, the first that applies:
/// 1. **Feed it.** Hungry, or its line cut by a rival: it marches home, through whoever stands in
///    the way. It leaves its own land for a rival's only with `PLAN.packed` of its pack, and on no
///    one's land with under `PLAN.low` turns in its pack it halts and forages before it goes on.
/// 2. **Hold the land.** A rival army on its civ's claim or beside its capital: it marches onto it
///    when it outweighs it by `PLAN.edge`, else it stands across that army's line where its own line
///    feeds it, else it stands on its capital.
/// 3. **Take a capital.** A rival's capital it outweighs (`outweighs`), no more than `PLAN.march`
///    turns' walk away: it marches on it, and off again the turn it no longer outweighs it.
/// 4. **Hurt the nearest rival.** Of `PLAN.raid` people or more: a rival's land inside its ring
///    (`PLAN.reach`) whose defenders it outweighs, a clan's home or ground first: it marches there
///    and pillages it; and a rival army abroad whose line it can stand across.
/// 5. Else it stands at its post with a zone along its line home.
fn direct(w: &mut World, civ: u32, id: u32, paths: &mut Paths, give: Give) {
    let Some(u) = w.units.get(&id) else { return };
    let (here, dest, people) = (u.home, u.dest(), u.people);
    let capital = w.civs[&civ].capital;
    let strength = people * might(u).0;
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
    if here != capital && severed {
        return go(w, give, capital, "a rival cuts its line: it marches home".to_string());
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
    if here != capital && u.hungry > 0 {
        return go(w, give, capital, "hungry: it marches home".to_string());
    }
    // An army sets out with its pack full: on its own land it waits for it.
    let packed = total(&u.store) + EPS >= PLAN.packed * pack_cap(people);
    let party = people <= PLAN.party + EPS;
    if !w.config.fight {
        return stand(w, civ, id, paths, give);
    }

    // 2. Hold the land: the nearest rival army on the claim or beside the capital.
    let threat = w
        .units
        .values()
        .filter(|o| o.civ != civ && o.camp().is_some() && o.people > EPS)
        .filter(|o| w.civs[&civ].claim.binary_search(&o.home).is_ok() || steps(w, o.home, capital) <= 1)
        .min_by_key(|o| (steps(w, o.home, capital), o.id))
        .map(|o| (o.id, o.home, o.people * might(o).1));
    if let Some((foe, at, theirs)) = threat {
        if strength > PLAN.edge * theirs {
            return go(w, give, at, format!("marches on the rival army at {}: it outweighs it", w.at(at)));
        }
        if let Some(t) = across(w, civ, id, foe) {
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
        go(w, give, capital, "fills its pack at its capital before it sets out".to_string());
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
        return go(w, give, capital, "it no longer outweighs the capital: it pulls back".to_string());
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
            if let Some(t) = across(w, civ, id, foe) {
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
        if let Some(t) = raid(w, civ, id, strength, party) {
            if !packed && home_ground && dest != Some(t) {
                return fill(w, give);
            }
            return go(w, give, t, format!("marches to {} to pillage a rival's land", w.at(t)));
        }
    }

    // 5. Its post.
    stand(w, civ, id, paths, give)
}

/// The rival land army `id` would pillage: a tile of a rival's land, not a capital and not the
/// tile it stands on, whose defenders it outweighs by `PLAN.edge`; inside `civ`'s ring where its
/// line takes no more than `PLAN.reach` of its days, or anywhere it can walk when it is a `party`,
/// small enough to live off the land. The nearest, a rival clan's home counting two steps nearer
/// than its ground and that two nearer than bare land.
fn raid(w: &World, civ: u32, id: u32, strength: f64, party: bool) -> Option<usize> {
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
        .find(|&(_, t)| strength > PLAN.edge * defence_at(w, civ, t) && path_between(w, from, t).is_some())
        .map(|(_, t)| t)
}

/// The tile where army `id` would stand across rival army `foe`'s line: the first tile of that line
/// past the rival's own that is no capital and not the rival's land, that it can walk to, and where
/// its own line would carry its food. `None` when the rival has no line or no such tile.
fn across(w: &World, civ: u32, id: u32, foe: u32) -> Option<usize> {
    let o = w.units.get(&foe)?;
    let line = &o.actor.routes.first()?.path;
    let from = w.units[&id].home;
    line.iter().copied().find(|&t| {
        t != o.home
            && !civs::alive(w).any(|c| c.capital == t)
            && TERRAIN[w.tiles[t].terrain].land
            && civs::tile_civ(w, t) != Some(o.civ)
            && !super::past_reach(w, id, t)
            && (t == from || path_between(w, from, t).is_some())
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
    let capital = w.civs[&u.civ].capital;
    let k = UNIT.zone.min(zone_cap(u).saturating_sub(1));
    let tiles: Vec<usize> = if afford { p.into_iter().filter(|&t| t != capital).take(k).collect() } else { Vec::new() };
    if zone_of(u.home, &tiles) != u.actor.territory && zone_ok(w, u, &tiles).is_ok() {
        let _ = give(w, civ, Decision::Order(Order::Zone { unit: id, tiles }));
    }
}
