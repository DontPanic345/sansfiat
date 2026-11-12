//! The land: a civ's claim, which is its land, and rival armies taking it.
//!
//! A civ's claim (`Civ.claim`) is the tiles it paints out from its capital: land, connected, holding
//! the capital, no tile of it a rival's. Every tile of it is the civ's land (`held`, `tile_civ`), with
//! nobody standing on it. A rival army that ends its turn on a tile of it, where no army of the civ
//! holds, takes the tile (`step`): it leaves the claim with the tiles no longer joined to the
//! capital, and joins the taker's claim where it touches it.

use crate::actors;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::{neighbours, steps};
use crate::map;
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
/// on the map and land, none is a rival's land, and they are connected to the capital. Returns
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
    if tiles.iter().any(|&t| t != c.capital && crate::civs::alive(w).any(|o| o.capital == t)) {
        return Err("a claim can't cover another civ's capital".to_string());
    }
    if tiles.iter().any(|&t| tile_civ(w, t).is_some_and(|o| o != civ)) {
        return Err("a claim can't cover a rival's land: an army that ends its turn on it takes it".to_string());
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

/// The tiles of `civ`'s land (sorted): its claim.
pub fn held(w: &World, civ: u32) -> Vec<usize> {
    w.civs.get(&civ).map_or_else(Vec::new, |c| c.claim.clone())
}

/// The civ whose land tile `i` is: the standing civ that claims it. Claims never overlap (`check`),
/// and clans' territories make no land.
pub fn tile_civ(w: &World, i: usize) -> Option<u32> {
    crate::civs::alive(w).find(|c| c.claim.binary_search(&i).is_ok()).map(|c| c.id)
}

/// What a claim order does, in words: the tiles it adds to `was` and the tiles it drops, each named
/// (`World::at`), and the land's size after it.
pub fn change(w: &World, was: &[usize], now: &[usize]) -> String {
    let names = |a: &[usize], b: &[usize]| -> Vec<String> {
        a.iter().filter(|t| b.binary_search(t).is_err()).map(|&t| w.at(t)).collect()
    };
    let (adds, drops) = (names(now, was), names(was, now));
    let mut parts = Vec::new();
    if !adds.is_empty() {
        parts.push(format!("adds {} ({})", adds.len(), adds.join(", ")));
    }
    if !drops.is_empty() {
        parts.push(format!("drops {} ({})", drops.len(), drops.join(", ")));
    }
    let what = if parts.is_empty() { "changes nothing".to_string() } else { parts.join(" and ") };
    format!("the claim {what}: its land is {} tile(s)", now.len())
}

/// Sets `civ`'s claim (sorted, with its capital).
pub fn set(w: &mut World, civ: u32, claim: Vec<usize>) {
    w.civs.get_mut(&civ).unwrap().claim = claim;
}

/// The claim a civ is founded with: the land tiles within `CLAIM.ring` steps of the capital or of
/// any clan's home (its own, unaligned or a rival's) within `CLAIM.reach` steps of it, not a rival's
/// land and, with `Config.fight` on, not a tile a rival army holds or stands on, kept to the part
/// connected to the capital. Sorted, with the capital. The planner widens it from there
/// (`units::claim_plan`); a led civ's is the player's to repaint.
pub fn first(w: &World, civ: u32) -> Vec<usize> {
    let c = &w.civs[&civ];
    let seeds: Vec<usize> = std::iter::once(c.capital)
        .chain(w.clans.values().filter(|k| steps(w, c.capital, k.home) <= CLAIM.reach).map(|k| k.home))
        .collect();
    let mut tiles: Vec<usize> = seeds
        .iter()
        .flat_map(|&s| w.tiles[s].hex().within(CLAIM.ring).into_iter().filter_map(|h| map::index(w.config.radius, h)))
        .filter(|&t| TERRAIN[w.tiles[t].terrain].land && tile_civ(w, t).is_none_or(|o| o == civ))
        .filter(|&t| !crate::civs::alive(w).any(|o| o.id != civ && o.capital == t))
        .collect();
    let rival: Vec<usize> = w
        .units
        .values()
        .filter(|u| w.config.fight && u.civ != civ && u.camp().is_some())
        .flat_map(|u| u.held().iter().copied().chain(std::iter::once(u.home)))
        .collect();
    tiles.retain(|t| !rival.contains(t));
    tiles.sort_unstable();
    tiles.dedup();
    connected_part(w, c.capital, &tiles)
}

/// The `claim` step, after `units`: each army (never a scout) that ends its turn on a tile of a
/// rival's claim, not its capital, where no army of that rival holds, takes it (`take`).
pub fn step(w: &mut World) {
    let standing: Vec<(u32, u32, usize)> = w
        .units
        .values()
        .filter(|u| u.camp().is_some() && u.people > crate::units::EPS)
        .map(|u| (u.id, u.civ, u.home))
        .collect();
    for (unit, by, tile) in standing {
        let Some(owner) = tile_civ(w, tile).filter(|&o| o != by) else { continue };
        if w.civs[&owner].capital == tile || w.civs.get(&by).is_none_or(|c| c.is_out()) {
            continue;
        }
        if w.units.values().any(|u| u.civ == owner && u.camp().is_some() && (u.home == tile || u.held().contains(&tile)))
        {
            continue;
        }
        take(w, owner, unit, by, tile);
    }
}

/// Takes `tile` out of `owner`'s claim, with the claim's tiles cut off from its capital. The tile
/// joins `by`'s claim when it touches it and `by` may claim; the tiles cut off are nobody's. Emits
/// `ClaimTaken` with the tiles lost, and notes it on both civs.
pub(crate) fn take(w: &mut World, owner: u32, unit: u32, by: u32, tile: usize) {
    let (capital, claim) = (w.civs[&owner].capital, w.civs[&owner].claim.clone());
    let without: Vec<usize> = claim.iter().copied().filter(|&t| t != tile).collect();
    let kept = connected_part(w, capital, &without);
    let lost = claim.len() - kept.len();
    set(w, owner, kept);
    let mine = w.civs[&by].claim.clone();
    let joins = may_claim(w, by) && neighbours(w, tile).any(|n| mine.binary_search(&n).is_ok());
    if joins {
        let mut claim = mine;
        claim.push(tile);
        claim.sort_unstable();
        set(w, by, claim);
    }
    w.emit(Event::new(EventKind::ClaimTaken, tile).civ(owner).other(by).unit(unit).amount(lost as f64));
    let at = w.at(tile);
    let (taker, loser) = (name(w, by), name(w, owner));
    w.note(owner, format!("lost {lost} tile(s) of its land at {at}: {taker}'s army stands on it"));
    let joined = if joins { ": the tile joins its own" } else { "" };
    w.note(by, format!("its army took {lost} tile(s) of {loser}'s land at {at}{joined}"));
}

/// The name of civ `id`, for a note.
fn name(w: &World, id: u32) -> String {
    w.civs.get(&id).map_or_else(|| "a rival".to_string(), |c| c.name.clone())
}
