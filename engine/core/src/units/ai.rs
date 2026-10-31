//! The AI's camps: each civ the player doesn't lead raises its camp, posts it on its way to a
//! rival or its farthest clan, and zones it along its line home. The AI gives its orders through
//! `World::order` (`Raise`, `Move`, `Zone`), not logged: a replay re-derives them.

use super::line::{line_by, path_between};
use super::{camp, zone_cap, zone_of, zone_ok, EPS};
use crate::civs;
use crate::data::*;
use crate::land::steps;
use crate::orders::Order;
use crate::world::{Phase, World};

/// Where the AI posts `civ`'s camp: up to `UNIT.post` steps along the way from the capital to the
/// nearest rival capital reachable by land, or with none, to its farthest clan, keeping to its
/// own land and on land (not on a built pass). The capital when there is nowhere to go.
pub fn post(w: &World, civ: u32) -> usize {
    let c = &w.civs[&civ];
    let to_rival = w
        .civs
        .values()
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

/// The `ai` step, before `units`: each civ the player doesn't lead raises its camp if it has none
/// and can, and sends it to its post if it is idle somewhere else. Draws no random numbers.
pub fn ai(w: &mut World) {
    if w.phase != Phase::Main || !w.config.army {
        return;
    }
    let ids: Vec<u32> = w.civs.keys().copied().filter(|&c| Some(c) != w.player).collect();
    for civ in ids {
        if camp(w, civ).is_none() {
            let _ = w.order(civ, Order::Raise { n: UNIT.start });
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
