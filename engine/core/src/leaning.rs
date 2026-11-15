//! Whose a clan is. A clan whose home is on a civ's land is that civ's (`land`). A clan whose home
//! is on nobody's land leans: a civ gives food to the clans at its market and trades there, each
//! clan leans towards the civ for it (`give`, and `markets::trade` for trade), and the `lean` step
//! fades every leaning and joins a clan to the civ it leans to most once that reaches
//! `LEANING.join`.

use std::collections::BTreeMap;

use crate::civs;
use crate::claim;
use crate::data::*;
use crate::events::{By, Event, EventKind};
use crate::market::{take, Basket};
use crate::world::World;

/// The leaning a gift of `g` food buys a clan of `people`: `v × LEANING.gift × ln(1 + g /
/// (LEANING.gift_scale × people))`, where `v` is `LEANING.starving` for a starving clan, else 1.
/// A log, so twice the food buys less than twice the leaning.
pub fn gift_leaning(g: f64, people: f64, starving: bool) -> f64 {
    let v = if starving { LEANING.starving } else { 1.0 };
    v * LEANING.gift * libm::log(1.0 + g / (LEANING.gift_scale * people))
}

/// `civ` gives `pool` food from its store, in proportion to its goods, to the clans `ids` (the
/// unaligned ones at its market), split in proportion to `v × people`. Each clan's share goes into
/// its entry of `kept` (what it keeps back from the clearing, so it eats it), and it leans
/// towards the civ for it. Returns the food given; nothing with no clan or no pool.
pub fn give(w: &mut World, civ: u32, ids: &[u32], kept: &mut [Basket], pool: f64) -> f64 {
    let weight = |w: &World, id: u32| {
        let c = &w.clans[&id];
        if c.starving {
            LEANING.starving * c.people
        } else {
            c.people
        }
    };
    let sum: f64 = ids.iter().map(|&id| weight(w, id)).sum();
    if pool <= 0.0 || sum <= 0.0 || w.civs[&civ].is_out() {
        return 0.0;
    }
    let taken = take(&mut w.civs.get_mut(&civ).unwrap().store, pool);
    let given: f64 = taken.iter().sum();
    for (i, &id) in ids.iter().enumerate() {
        let share = weight(w, id) / sum;
        for f in 0..N {
            kept[i][f] += taken[f] * share;
        }
        let c = w.clans.get_mut(&id).unwrap();
        let lean = gift_leaning(given * share, c.people, c.starving);
        *c.leaning.entry(civ).or_insert(0.0) += lean;
        *c.gifts.entry(civ).or_insert(0.0) += lean;
    }
    let name = w.civs[&civ].name.clone();
    for &id in ids {
        w.note(id, format!("received a gift from {name}"));
    }
    given
}

/// Whether gifts or trade brought clan `id` to lean to `civ`: gifts when they bought at least half
/// of the leaning.
fn won_by(w: &World, id: u32, civ: u32) -> By {
    let c = &w.clans[&id];
    let (all, gifts) = (c.leaning.get(&civ).copied().unwrap_or(0.0), c.gifts.get(&civ).copied().unwrap_or(0.0));
    if gifts * 2.0 >= all {
        By::Gift
    } else {
        By::Trade
    }
}

/// Clan `id` comes to civ `to` for `by`: its leaning there is at `LEANING.join` and its others are
/// cleared. An unaligned clan joins (`ClanJoined`); a civ's clan leaves it (`ClanLeft`). Notes the
/// clan and each civ. `quiet` (founding) makes no event and counts no joining.
fn come(w: &mut World, id: u32, to: u32, by: By, quiet: bool) {
    let c = w.clans.get_mut(&id).unwrap();
    let (from, home) = (c.civ, c.home);
    c.civ = Some(to);
    c.leaning = BTreeMap::from([(to, LEANING.join)]);
    c.gifts.clear();
    c.drawn = 0;
    let new = w.civs[&to].name.clone();
    let why = by.words(&new);
    if quiet {
        w.note(id, format!("joined {new} at founding: {why}"));
        return;
    }
    w.civs.get_mut(&to).unwrap().joined += 1;
    match from {
        None => {
            w.emit(Event::new(EventKind::ClanJoined, home).civ(to).clan(id).by(by));
            w.note(id, format!("joined {new}: {why}"));
            w.note(to, format!("clan {id} joined it: {why}"));
        }
        Some(from) => {
            w.emit(Event::new(EventKind::ClanLeft, home).civ(from).other(to).clan(id).by(by));
            let old = w.civs[&from].name.clone();
            w.note(id, format!("left {old} for {new}: {why}"));
            w.note(from, format!("clan {id} left it for {new}: {why}"));
            w.note(to, format!("clan {id} left {old} and joined it: {why}"));
        }
    }
}

/// Every clan whose home is on a civ's land is that civ's: an unaligned one joins it and a rival's
/// leaves for it (`come`, by land), all but a civ's own clan. A clan already the civ's keeps its
/// leaning to it at `LEANING.join` or above and is drawn nowhere. Run by the `lean` step and
/// wherever a claim changes, so a clan changes civ with its tile.
pub fn land(w: &mut World, quiet: bool) {
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let Some(civ) = claim::tile_civ(w, w.clans[&id].home) else { continue };
        if w.clans[&id].civ == Some(civ) {
            let c = w.clans.get_mut(&id).unwrap();
            let l = c.leaning.entry(civ).or_insert(0.0);
            *l = l.max(LEANING.join);
            c.drawn = 0;
        } else if !civs::is_civ_clan(w, id) {
            come(w, id, civ, By::Land, quiet);
        }
    }
}

/// The `lean` step, after `people`. Every clan's leanings fade by `LEANING.fade` and any under
/// `LEANING.floor` go. Clans on a civ's land are that civ's (`land`). Then, in id order, an
/// unaligned clan whose strongest leaning (ties to the lower civ) is at least `LEANING.join` joins
/// that civ. Then a civ's clan whose home is off its civ's land, drawn to a rival for `leave_run`
/// turns, leaves for it (`leave`). Nothing before `Phase::Main`.
pub fn lean(w: &mut World) {
    if !w.phase.playing() {
        return;
    }
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for &id in &ids {
        let c = w.clans.get_mut(&id).unwrap();
        for v in c.leaning.values_mut().chain(c.gifts.values_mut()) {
            *v *= 1.0 - LEANING.fade;
        }
        c.leaning.retain(|_, v| *v >= LEANING.floor);
        let leaning = &c.leaning;
        c.gifts.retain(|k, _| leaning.contains_key(k));
    }
    land(w, false);
    for &id in &ids {
        let c = &w.clans[&id];
        if c.civ.is_some() {
            continue;
        }
        // Civs come in id order, so only a strictly stronger one replaces.
        let best = c.leaning.iter().fold(None, |b: Option<(u32, f64)>, (&k, &v)| match b {
            Some((_, bv)) if bv >= v => b,
            _ => Some((k, v)),
        });
        let Some((civ, _)) = best.filter(|&(_, v)| v >= LEANING.join) else { continue };
        come(w, id, civ, won_by(w, id, civ), false);
    }
    for &id in &ids {
        let Some(own) = w.clans[&id].civ else { continue };
        if civs::is_civ_clan(w, id) || claim::tile_civ(w, w.clans[&id].home) == Some(own) {
            continue;
        }
        let c = &w.clans[&id];
        let mine = c.leaning.get(&own).copied().unwrap_or(0.0);
        // The strongest leaning to a rival, the lowest civ on a tie.
        let rival = c.leaning.iter().filter(|(&k, _)| k != own).fold(None, |b: Option<(u32, f64)>, (&k, &v)| match b {
            Some((_, bv)) if bv >= v => b,
            _ => Some((k, v)),
        });
        let drawn = match rival {
            Some((_, v)) if v >= LEANING.leave_at && v > mine => c.drawn + 1,
            _ => 0,
        };
        let to = rival.map(|(k, _)| k);
        w.clans.get_mut(&id).unwrap().drawn = drawn;
        if let (true, Some(to)) = (drawn >= LEANING.leave_run, to) {
            leave(w, id, to);
        }
    }
}

/// Clan `id` leaves its civ for `to`, which it leans to, and so does each living daughter still of
/// its old civ whose home is off that civ's land (`come`).
fn leave(w: &mut World, id: u32, to: u32) {
    let from = w.clans[&id].civ.expect("a clan that leaves belongs to a civ");
    let by = won_by(w, id, to);
    let daughters: Vec<u32> = w
        .clans
        .values()
        .filter(|d| d.parent == Some(id) && d.civ == Some(from) && claim::tile_civ(w, d.home) != Some(from))
        .map(|d| d.id)
        .collect();
    for clan in std::iter::once(id).chain(daughters) {
        come(w, clan, to, by, false);
    }
}
