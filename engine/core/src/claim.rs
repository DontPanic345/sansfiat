//! The land: a civ's claim, its guard, the land its guard holds, and rival units taking paper.
//!
//! A civ's claim (`Civ.claim`) is the tiles it paints out from its capital: land, connected, holding
//! the capital. Its guard (`units::guard`) stands at the capital and divides `GUARD.guarding` a
//! person over the claim as a camp divides over its zone, each tile's share paying its walk from the
//! capital (`divide`, which sets `Civ.guard`); the camps of the civ that hold a claimed tile add their
//! share to it. A claimed tile whose guard reaches `CLAIM.floor` is held, and the capital always is:
//! a civ's held tiles are its land (`held`, `tile_civ`). The rest of the claim is paper, which a rival
//! unit takes by ending its turn on it (`step`).

use crate::actors;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::steps;
use crate::map;
use crate::units::{self, EPS};
use crate::world::World;

/// Whether `civ` may claim land: `Config.claims` lets it.
pub fn may_claim(w: &World, civ: u32) -> bool {
    match w.config.claims {
        Claims::All => true,
        Claims::None => false,
        Claims::Only(k) => w.civs.keys().nth(k as usize) == Some(&civ),
    }
}

/// `tiles` (sorted) with `capital` added, cut to the tiles a walk over them reaches from the
/// capital: the claim's tiles still joined to it, sorted.
pub fn connected_part(w: &World, capital: usize, tiles: &[usize]) -> Vec<usize> {
    let mut all = tiles.to_vec();
    all.push(capital);
    all.sort_unstable();
    all.dedup();
    let walks = actors::Walks::new(w, capital, &all);
    all.iter().zip(&walks.cost).filter(|(_, c)| c.is_finite()).map(|(&t, _)| t).collect()
}

/// Checks a claim of `tiles` for `civ` (its capital may be left out): `civ` may claim, the tiles are
/// on the map and land, none is another civ's capital, and they are connected to the capital. Returns
/// the claim, sorted, with the capital.
pub fn check(w: &World, civ: u32, tiles: &[usize]) -> Result<Vec<usize>, String> {
    let c = w.civs.get(&civ).ok_or_else(|| format!("{civ} is not a civ"))?;
    if !may_claim(w, civ) {
        return Err("this civ may not claim land: the claims control is off".to_string());
    }
    if let Some(&t) = tiles.iter().find(|&&t| t >= w.tiles.len()) {
        return Err(format!("{t} is not a tile"));
    }
    if tiles.iter().any(|&t| !TERRAIN[w.tiles[t].terrain].land) {
        return Err("a claim can only cover land".to_string());
    }
    if tiles.iter().any(|&t| t != c.capital && w.civs.values().any(|o| o.capital == t)) {
        return Err("a claim can't cover another civ's capital".to_string());
    }
    let mut all = tiles.to_vec();
    all.push(c.capital);
    all.sort_unstable();
    all.dedup();
    if connected_part(w, c.capital, &all).len() != all.len() {
        return Err("a claim must be connected to the capital".to_string());
    }
    Ok(all)
}

/// The people of `civ`'s guard (none before it is raised).
pub fn guard_people(w: &World, civ: u32) -> f64 {
    units::guard_of(w, civ).map_or(0.0, |u| u.people)
}

/// The guard share of `tile` for `civ`: `Civ.guard` at the tile, 0 when it isn't claimed.
pub fn guard_on(w: &World, civ: u32, tile: usize) -> f64 {
    let Some(c) = w.civs.get(&civ) else { return 0.0 };
    c.claim.binary_search(&tile).ok().and_then(|k| c.guard.get(k).copied()).unwrap_or(0.0)
}

/// The tiles of `civ`'s claim it holds (sorted): the capital, and each claimed tile whose guard share
/// reaches `CLAIM.floor`. The rest of the claim is paper.
pub fn held(w: &World, civ: u32) -> Vec<usize> {
    let Some(c) = w.civs.get(&civ) else { return Vec::new() };
    c.claim
        .iter()
        .enumerate()
        .filter(|&(k, &t)| t == c.capital || c.guard.get(k).copied().unwrap_or(0.0) + EPS >= CLAIM.floor)
        .map(|(_, &t)| t)
        .collect()
}

/// The civ whose land tile `i` is: a capital's own civ; else the civ holding it (`held`) with the
/// thicker guard there, a tie to the lower id; else none. Clans' territories make no land.
pub fn tile_civ(w: &World, i: usize) -> Option<u32> {
    if let Some(c) = w.civs.values().find(|c| c.capital == i) {
        return Some(c.id);
    }
    let mut best: Option<(u32, f64)> = None;
    for c in w.civs.values() {
        let Ok(k) = c.claim.binary_search(&i) else { continue };
        let g = c.guard.get(k).copied().unwrap_or(0.0);
        if g + EPS < CLAIM.floor {
            continue;
        }
        // Civs come in id order, so only a strictly thicker guard replaces.
        if best.is_none_or(|(_, b)| g > b) {
            best = Some((c.id, g));
        }
    }
    best.map(|(id, _)| id)
}

/// Sets `civ`'s claim (sorted, with its capital) and divides its guard over it.
pub fn set(w: &mut World, civ: u32, claim: Vec<usize>) {
    w.civs.get_mut(&civ).unwrap().claim = claim;
    divide(w, civ);
}

/// Divides `civ`'s guard over its claim: `GUARD.guarding` a person of its guard, each tile's share
/// paying its walk from the capital at `GUARD.inside` (`actors::divide`), plus the share of each of
/// its camps that holds a claimed tile. Sets `Civ.guard`, aligned with the claim, and the guard's
/// territory to the claim. Nothing else reads a guard share.
pub fn divide(w: &mut World, civ: u32) {
    let c = &w.civs[&civ];
    let (capital, claim) = (c.capital, c.claim.clone());
    let walks = actors::Walks::new(w, capital, &claim);
    let mut guard = actors::divide(guard_people(w, civ) * GUARD.guarding, &[], &walks.yearly(GUARD.inside));
    for u in w.units.values().filter(|u| u.civ == civ) {
        let Some(camp) = u.camp() else { continue };
        for (k, &t) in camp.held.iter().enumerate() {
            if let (Ok(j), Some(&g)) = (claim.binary_search(&t), camp.guard.get(k)) {
                guard[j] += g;
            }
        }
    }
    w.civs.get_mut(&civ).unwrap().guard = guard;
    if let Some(u) = units::guard_of_mut(w, civ) {
        u.actor.territory = claim;
    }
}

/// The AI's stand-in claim (`spec.md` item 22): the land tiles within `CLAIM.ai_ring` steps of the
/// capital or of any clan's home (its own, unaligned or a rival's) within `CLAIM.ai_reach` steps of it, not another civ's capital, kept to
/// the part connected to the capital. Sorted, with the capital.
pub fn ai_claim(w: &World, civ: u32) -> Vec<usize> {
    let c = &w.civs[&civ];
    let seeds: Vec<usize> = std::iter::once(c.capital)
        .chain(w.clans.values().filter(|k| steps(w, c.capital, k.home) <= CLAIM.ai_reach).map(|k| k.home))
        .collect();
    let mut tiles: Vec<usize> = seeds
        .iter()
        .flat_map(|&s| {
            w.tiles[s].hex().within(CLAIM.ai_ring).into_iter().filter_map(|h| map::index(w.config.radius, h))
        })
        .filter(|&t| TERRAIN[w.tiles[t].terrain].land && !w.civs.values().any(|o| o.id != civ && o.capital == t))
        .collect();
    tiles.sort_unstable();
    tiles.dedup();
    connected_part(w, c.capital, &tiles)
}

/// The `claim` step, after `units`: each civ's guard is divided over its claim, then each unit
/// standing on another civ's paper (a tile of its claim that isn't held, and isn't its capital) takes
/// it: the tile leaves that claim, and so do the claim's tiles no longer joined to the capital.
pub fn step(w: &mut World) {
    let civs: Vec<u32> = w.civs.keys().copied().collect();
    for &c in &civs {
        divide(w, c);
    }
    let standing: Vec<(u32, u32, usize)> = w.units.values().map(|u| (u.id, u.civ, u.home)).collect();
    for (unit, by, tile) in standing {
        for &owner in &civs {
            if owner == by {
                continue;
            }
            let c = &w.civs[&owner];
            if c.capital == tile {
                continue;
            }
            let Ok(k) = c.claim.binary_search(&tile) else { continue };
            if c.guard.get(k).copied().unwrap_or(0.0) + EPS >= CLAIM.floor {
                continue;
            }
            take(w, owner, unit, by, tile);
        }
    }
}

/// Takes `tile` out of `owner`'s claim, with the claim's tiles cut off from its capital, and divides
/// its guard again. Emits `ClaimTaken` and notes it on both civs.
fn take(w: &mut World, owner: u32, unit: u32, by: u32, tile: usize) {
    let (capital, claim) = (w.civs[&owner].capital, w.civs[&owner].claim.clone());
    let without: Vec<usize> = claim.iter().copied().filter(|&t| t != tile).collect();
    let kept = connected_part(w, capital, &without);
    let lost = claim.len() - kept.len();
    set(w, owner, kept);
    w.emit(Event::new(EventKind::ClaimTaken, tile).civ(owner).other(by).unit(unit).amount(lost as f64));
    let at = w.at(tile);
    let (taker, loser) = (name(w, by), name(w, owner));
    w.note(owner, format!("lost {lost} tile(s) of its claim at {at}: {taker}'s unit walked onto its paper"));
    w.note(by, format!("its unit took {lost} tile(s) of {loser}'s claim at {at}"));
}

/// The name of civ `id`, for a note.
fn name(w: &World, id: u32) -> String {
    w.civs.get(&id).map_or_else(|| "a rival".to_string(), |c| c.name.clone())
}
