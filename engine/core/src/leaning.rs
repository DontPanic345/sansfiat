//! Leaning: how unaligned clans come to belong to a civ. A civ gives food to the clans at its
//! market and trades there; each clan leans towards the civ for it (`give`, and `markets::trade`
//! for trade); the `lean` step fades every leaning and joins a clan to the civ it leans to most
//! once that reaches `LEANING.join`.

use crate::data::*;
use crate::events::{Event, EventKind};
use crate::market::{take, Basket};
use crate::world::{Phase, World};

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
    if pool <= 0.0 || sum <= 0.0 {
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
    }
    let name = w.civs[&civ].name.clone();
    for &id in ids {
        w.note(id, format!("received a gift from {name}"));
    }
    given
}

/// The `lean` step, after `people`: each unaligned clan's leanings fade by `LEANING.fade` and any
/// under `LEANING.floor` go; then, in id order, a clan whose strongest leaning (ties to the lower
/// civ) is at least `LEANING.join` joins that civ, its leanings cleared (`ClanJoined`). Nothing
/// before `Phase::Main`.
pub fn lean(w: &mut World) {
    if w.phase != Phase::Main {
        return;
    }
    let ids: Vec<u32> = w.clans.values().filter(|c| c.civ.is_none() && !c.leaning.is_empty()).map(|c| c.id).collect();
    for id in ids {
        let c = w.clans.get_mut(&id).unwrap();
        for v in c.leaning.values_mut() {
            *v *= 1.0 - LEANING.fade;
        }
        c.leaning.retain(|_, v| *v >= LEANING.floor);
        // Civs come in id order, so only a strictly stronger one replaces.
        let best = c.leaning.iter().fold(None, |b: Option<(u32, f64)>, (&k, &v)| match b {
            Some((_, bv)) if bv >= v => b,
            _ => Some((k, v)),
        });
        let Some((civ, lean)) = best.filter(|&(_, v)| v >= LEANING.join) else { continue };
        c.civ = Some(civ);
        c.leaning.clear();
        let home = c.home;
        w.civs.get_mut(&civ).unwrap().joined += 1;
        w.emit(Event::new(EventKind::ClanJoined, home).civ(civ).clan(id));
        let name = w.civs[&civ].name.clone();
        w.note(id, format!("joined {name}: its leaning reached {lean:.2}"));
        w.note(civ, format!("clan {id} joined it"));
    }
}
