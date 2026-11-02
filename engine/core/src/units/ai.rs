//! The AI's civs: each civ the player doesn't lead claims its stand-in land and keeps its guard
//! (`land`), raises its camp, posts it on its way to a rival or its farthest clan, and zones it along
//! its line home. The AI gives its orders through `World::order` (`Claim`, `Raise`, `Disband`, `Move`,
//! `Zone`), not logged: a replay re-derives them.

use super::line::{line_by, path_between};
use super::{camp, guard_of, zone_cap, zone_of, zone_ok, UnitKind, EPS};
use crate::civs;
use crate::claim;
use crate::data::*;
use crate::land::steps;
use crate::market::total;
use crate::orders::Order;
use crate::world::{Phase, World};

/// Where the AI posts `civ`'s camp: up to `UNIT.post` steps along the way from the capital to the
/// nearest rival capital reachable by land, or with none, to its farthest clan, keeping to its
/// own land and on land (not on a built pass). The capital when there is nowhere to go.
pub fn post(w: &World, civ: u32) -> usize {
    let c = &w.civs[&civ];
    let to_rival = civs::alive(w)
        .filter(|o| o.id != civ)
        .filter_map(|o| path_between(w, c.capital, o.capital).map(|(p, _)| ((steps(w, c.capital, o.capital), o.id), p)))
        .min_by_key(|(k, _)| *k)
        .map(|(_, p)| p);
    let way = to_rival.or_else(|| {
        let far = w
            .clans
            .values()
            .filter(|k| k.civ == Some(civ))
            .max_by_key(|k| (steps(w, c.capital, k.home), std::cmp::Reverse(k.id)))?;
        path_between(w, c.capital, far.home).map(|(p, _)| p)
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

/// The `ai` step, before `units`: each civ the player doesn't lead keeps its land (`land`), raises its
/// camp if it has none and can, and sends it to its post if it is idle somewhere else. Draws no random
/// numbers.
pub fn ai(w: &mut World) {
    if w.phase != Phase::Main {
        return;
    }
    let ids: Vec<u32> = civs::alive(w).map(|c| c.id).filter(|&c| Some(c) != w.player).collect();
    for civ in ids {
        land(w, civ);
        if !w.config.army {
            continue;
        }
        if camp(w, civ).is_none() {
            let _ = w.order(
                civ,
                Order::Raise { n: UNIT.start, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None },
            );
        }
        let Some(u) = camp(w, civ) else { continue };
        if u.camp().is_some_and(|c| c.dest.is_some()) {
            continue;
        }
        let (id, here) = (u.id, u.home);
        let at = post(w, civ);
        if at != here {
            let text = format!("posted at {}, on the way to its nearest rival or its farthest clan", w.at(at));
            w.note(id, text);
            w.order(civ, Order::Move { unit: id, to: at }).expect("the AI's posts pass the camp's move check");
        } else if w.config.zone {
            ai_zone(w, civ, id);
        }
    }
}

/// The AI's land for `civ`: its stand-in claim is sent whenever it differs from the claim, where
/// `Config.claims` lets it claim; then its guard (`guard_order`).
fn land(w: &mut World, civ: u32) {
    if claim::may_claim(w, civ) {
        let want = claim::ai_claim(w, civ);
        if want != w.civs[&civ].claim {
            w.order(civ, Order::Claim { tiles: want }).expect("the stand-in claim passes the claim's check");
        }
    }
    if let Some(order) = guard_order(w, civ) {
        let _ = w.order(civ, order);
    }
}

/// The AI's rule for `civ`'s guard, as the order it would give: a guard that went hungry last turn
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

/// The AI's zone for a posted, idle camp: the first `UNIT.zone` tiles of its line home (no more
/// than its warriors less one, never the capital), when it can afford it: its line, read without
/// held ground, still carries a turn's food on the labour guarding leaves. Otherwise it undeploys.
/// Kept as it is while a rival's ground cuts the line.
fn ai_zone(w: &mut World, civ: u32, id: u32) {
    let u = &w.units[&id];
    let Some((p, _, carry)) = line_by(w, id, false) else { return };
    let afford = carry * (1.0 - UNIT.guarding / UNIT.labour) + EPS >= u.people * UNIT.food;
    let capital = w.civs[&u.civ].capital;
    let k = UNIT.zone.min(zone_cap(u).saturating_sub(1));
    let tiles: Vec<usize> = if afford { p.into_iter().filter(|&t| t != capital).take(k).collect() } else { Vec::new() };
    if zone_of(u.home, &tiles) != u.actor.territory && zone_ok(w, u, &tiles).is_ok() {
        w.order(civ, Order::Zone { unit: id, tiles }).expect("checked: the zone is one the camp can hold");
    }
}
