//! The fight (`docs/design/war.md`): wherever people of two civs stand on one tile, they fight, every
//! turn, by one rule. Each civ's people on a tile are its side there, a list of parts (an army's
//! share, a guard's share, and from build-2 militia). Each side loses `FIGHT.rate` people per unit of
//! the others' strength (Lanchester's linear law), capped at its people there; a side far weaker
//! after the losses retreats (armies only); the stronger holds the tile. Holding between civs is
//! decided here and nowhere else: `hold` spreads each army and decides only between one civ's own.

use std::collections::{BTreeMap, BTreeSet};

use super::{name_of, path_between, zone_of, UnitKind, EPS};
use crate::actors::connected;
use crate::civs;
use crate::claim;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::world::World;

/// Who a part of a side is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Who {
    Army(u32),
    Guard(u32),
    /// A clan defending its home (build-2).
    Militia(u32),
}

/// One body of a civ's people on a tile: how many, and their strength per person attacking and
/// defending (entrenchment and hunger in).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Part {
    pub civ: u32,
    pub who: Who,
    pub people: f64,
    pub attack: f64,
    pub defence: f64,
}

/// What entrenchment multiplies a defender's strength by after `still` turns without a step.
pub fn entrenched(still: u32) -> f64 {
    1.0 + (f64::from(still) * FIGHT.entrench).min(FIGHT.entrench_cap)
}

/// The people on each tile, by part: each army's split over the tiles it holds by its guard shares
/// (all on its own tile when undeployed or marching), each guard's over its civ's held claim by the
/// guard's own shares (`claim::guard_shares`). Scouts are never a part.
pub fn presence(w: &World) -> BTreeMap<usize, Vec<Part>> {
    let mut out: BTreeMap<usize, Vec<Part>> = BTreeMap::new();
    for u in w.units.values() {
        if u.people <= EPS {
            continue;
        }
        match u.kind() {
            UnitKind::Camp => {
                let c = u.camp().expect("a camp");
                let sum: f64 = c.stack.iter().sum();
                let per = |f: fn(&FighterRules) -> f64| {
                    if sum <= EPS {
                        f(&FIGHTERS[0])
                    } else {
                        Fighter::ALL.iter().map(|k| c.stack[k.index()] * f(k.rules())).sum::<f64>() / sum
                    }
                };
                let hungry = if u.hungry > 0 { FIGHT.hungry } else { 1.0 };
                let attack = per(|r| r.attack) * hungry;
                let defence = per(|r| r.defence) * entrenched(c.still) * hungry;
                let shares: f64 = c.guard.iter().take(c.held.len()).sum();
                let spread: Vec<(usize, f64)> = if c.held.contains(&u.home) && shares > EPS {
                    c.held.iter().zip(&c.guard).map(|(&t, &g)| (t, u.people * g / shares)).collect()
                } else {
                    vec![(u.home, u.people)]
                };
                for (t, people) in spread.into_iter().filter(|&(_, p)| p > EPS) {
                    out.entry(t).or_default().push(Part { civ: u.civ, who: Who::Army(u.id), people, attack, defence });
                }
            }
            UnitKind::Guard => {
                let Some(civ) = w.civs.get(&u.civ) else { continue };
                let shares = claim::guard_shares(w, u.civ);
                let held = claim::held(w, u.civ);
                let on: Vec<(usize, f64)> = civ
                    .claim
                    .iter()
                    .zip(&shares)
                    .filter(|(t, _)| held.binary_search(t).is_ok())
                    .map(|(&t, &g)| (t, g))
                    .collect();
                let sum: f64 = on.iter().map(|&(_, g)| g).sum();
                let spread = if sum > EPS {
                    on.into_iter().map(|(t, g)| (t, u.people * g / sum)).collect()
                } else {
                    vec![(civ.capital, u.people)]
                };
                let (attack, defence) = (FIGHTERS[0].attack, FIGHTERS[0].defence * entrenched(u32::MAX));
                for (t, people) in spread.into_iter().filter(|&(_, p)| p > EPS) {
                    out.entry(t).or_default().push(Part { civ: u.civ, who: Who::Guard(u.id), people, attack, defence });
                }
            }
            UnitKind::Scout => {}
        }
    }
    out
}

/// Each civ's people among `parts`, in civ order.
fn by_civ(parts: &[Part]) -> BTreeMap<u32, f64> {
    let mut out = BTreeMap::new();
    for p in parts {
        *out.entry(p.civ).or_insert(0.0) += p.people;
    }
    out
}

/// The tiles where two or more civs each have over `FIGHT.floor` people, in tile order.
pub fn contested(presence: &BTreeMap<usize, Vec<Part>>) -> Vec<usize> {
    presence
        .iter()
        .filter(|(_, parts)| by_civ(parts).values().filter(|&&p| p > FIGHT.floor).count() >= 2)
        .map(|(&t, _)| t)
        .collect()
}

/// The civ whose army holds each tile, read before the marches: the turn's start, for `holder_of`.
/// Two civs' armies on one tile go to the lower unit id.
pub fn army_holders(w: &World) -> BTreeMap<usize, u32> {
    let mut out = BTreeMap::new();
    for u in w.units.values() {
        for &t in u.held() {
            out.entry(t).or_insert(u.civ);
        }
    }
    out
}

/// The civ holding tile `t` at the turn's start: an army holding it, else its land's civ.
pub fn holder_of(w: &World, holders: &BTreeMap<usize, u32>, t: usize) -> Option<u32> {
    holders.get(&t).copied().or_else(|| claim::tile_civ(w, t))
}

/// The strength of `parts` for `civ`, defending when it holds the tile.
fn strength(parts: &[Part], civ: u32, holder: Option<u32>) -> f64 {
    parts
        .iter()
        .filter(|p| p.civ == civ)
        .map(|p| p.people * if holder == Some(civ) { p.defence } else { p.attack })
        .sum()
}

/// The fight step, after `hold`: every contested tile in tile order. `holders` is `army_holders`
/// read before the marches.
pub fn step(w: &mut World, holders: &BTreeMap<usize, u32>) {
    let pres = presence(w);
    let tiles = contested(&pres);
    let fighting = w.config.fight;
    let set: BTreeSet<usize> = tiles.iter().copied().collect();
    for &t in &tiles {
        let all = &pres[&t];
        let sides = by_civ(all);
        let parts: Vec<Part> = all.iter().copied().filter(|p| sides[&p.civ] > FIGHT.floor).collect();
        let civs: Vec<u32> = sides.iter().filter(|(_, &p)| p > FIGHT.floor).map(|(&c, _)| c).collect();
        let holder = holder_of(w, holders, t);
        let before: BTreeMap<u32, f64> = civs.iter().map(|&c| (c, strength(&parts, c, holder))).collect();
        let total: f64 = before.values().sum();

        // The losses, each side's from the others' strength before any falls.
        let mut lost: BTreeMap<u32, f64> = BTreeMap::new();
        let mut after: Vec<Part> = parts.clone();
        for &c in &civs {
            let people = sides[&c];
            let loss = if fighting { (FIGHT.rate * (total - before[&c])).min(people) } else { 0.0 };
            lost.insert(c, loss);
            for (p, q) in parts.iter().zip(after.iter_mut()).filter(|(p, _)| p.civ == c) {
                let k = loss * p.people / people;
                q.people -= k;
                fall(w, p, k);
            }
        }
        let now: BTreeMap<u32, f64> = civs.iter().map(|&c| (c, strength(&after, c, holder))).collect();
        let total_now: f64 = now.values().sum();
        let wiped = |c: u32| lost[&c] + EPS >= sides[&c];
        let has_army = |c: u32| parts.iter().any(|p| p.civ == c && matches!(p.who, Who::Army(_)));
        let retreats: Vec<u32> = civs
            .iter()
            .copied()
            .filter(|&c| fighting && !wiped(c) && has_army(c) && now[&c] < FIGHT.retreat * (total_now - now[&c]))
            .collect();
        // The stronger side left holds; a tie stays with the holder, else the lower civ.
        let winner = civs.iter().copied().filter(|&c| !wiped(c) && !retreats.contains(&c)).max_by(|&a, &b| {
            now[&a].total_cmp(&now[&b]).then((holder == Some(a)).cmp(&(holder == Some(b)))).then(b.cmp(&a))
        });

        // Skirmish, the first turn the tile is contested.
        if !w.contested.contains(&t) {
            let first = holder.filter(|h| civs.contains(h)).unwrap_or(civs[0]);
            let other = civs.iter().copied().find(|&c| c != first).unwrap_or(first);
            let unit = parts.iter().find_map(|p| if let Who::Army(id) = p.who { Some(id) } else { None });
            let mut e = Event::new(EventKind::Skirmish, t).civ(first).other(other);
            if let Some(id) = unit {
                e = e.unit(id);
            }
            w.emit(e);
        }

        // The outcome for each side's armies and guard.
        let mut outcome: BTreeMap<u32, String> = BTreeMap::new();
        for &c in &civs {
            let text = if Some(c) == winner {
                "holds it".to_string()
            } else if wiped(c) {
                "none of its people there are left".to_string()
            } else if retreats.contains(&c) {
                "retreats".to_string()
            } else {
                "fights on".to_string()
            };
            outcome.insert(c, text);
        }
        for p in &parts {
            let Who::Army(id) = p.who else { continue };
            if Some(p.civ) == winner || !w.units.contains_key(&id) {
                continue;
            }
            unhold(w, id, t);
            if retreats.contains(&p.civ) && w.units[&id].home == t {
                if let Some(to) = retreat(w, id, &set, t) {
                    let at = w.at(to);
                    outcome.insert(p.civ, format!("retreats to {at}"));
                    let mut e = Event::new(EventKind::Retreated, t).civ(p.civ).unit(id);
                    if let Some(by) = winner {
                        e = e.other(by);
                    }
                    w.emit(e);
                } else {
                    outcome.insert(p.civ, "has nowhere to retreat: fights on".to_string());
                }
            }
        }

        // A guard all of whose people there fell loses the tile to an army that beat it.
        if let Some(by) = winner.filter(|&c| has_army(c)) {
            let army = parts.iter().find_map(|p| match p.who {
                Who::Army(id) if p.civ == by => Some(id),
                _ => None,
            });
            for &c in civs.iter().filter(|&&c| c != by && wiped(c)) {
                let guarded = parts.iter().any(|p| p.civ == c && matches!(p.who, Who::Guard(_)));
                let civ = &w.civs[&c];
                if guarded && civ.capital != t && civ.claim.binary_search(&t).is_ok() {
                    claim::take(w, c, army.unwrap_or(0), by, t, true);
                }
            }
            if holder.is_some_and(|h| h != by && (wiped(h) || retreats.contains(&h))) {
                w.civs.get_mut(&by).unwrap().army.taken += 1;
            }
        }

        // The notes: one on each army and on each civ whose guard fought.
        let at = w.at(t);
        for p in &parts {
            let c = p.civ;
            let others: Vec<String> = civs
                .iter()
                .filter(|&&o| o != c)
                .map(|&o| format!("{}'s {}", name_of(w, o), side_name(&parts, o)))
                .collect();
            let ent = if holder == Some(c) {
                match p.who {
                    Who::Army(id) => {
                        w.units.get(&id).and_then(|u| u.camp()).map(|k| entrenched(k.still)).unwrap_or(1.0)
                    }
                    _ => entrenched(u32::MAX),
                }
            } else {
                1.0
            };
            let ent = if ent > 1.0 + EPS { format!(", entrenched ×{ent:.1}") } else { String::new() };
            let killed: f64 = civs.iter().filter(|&&o| o != c).map(|o| lost[o]).sum();
            let text = format!(
                "fought {} at {at}: strength {:.1} against {:.1}{ent}; lost {:.2}, killed {:.2}; {}",
                others.join(" and "),
                before[&c],
                total - before[&c],
                lost[&c] * p.people / sides[&c],
                killed,
                outcome[&c]
            );
            let subject = match p.who {
                Who::Army(id) => id,
                Who::Guard(_) | Who::Militia(_) => c,
            };
            w.note(subject, text);
        }
    }
    w.contested = tiles;
}

/// What a side is called in a note: "army", "guard" or both.
fn side_name(parts: &[Part], civ: u32) -> &'static str {
    let army = parts.iter().any(|p| p.civ == civ && matches!(p.who, Who::Army(_)));
    let guard = parts.iter().any(|p| p.civ == civ && !matches!(p.who, Who::Army(_)));
    match (army, guard) {
        (true, true) => "army and guard",
        (true, false) => "army",
        _ => "guard",
    }
}

/// `k` of part `p`'s people fall: off its unit (an army's off its stack in proportion), counted in
/// its civ's `fallen`. A unit left with nobody is gone.
fn fall(w: &mut World, p: &Part, k: f64) {
    if k <= 0.0 {
        return;
    }
    let id = match p.who {
        Who::Army(id) | Who::Guard(id) => id,
        Who::Militia(_) => return,
    };
    let Some(u) = w.units.get_mut(&id) else { return };
    u.take_people(k);
    let gone = u.people <= EPS;
    if let Some(c) = w.civs.get_mut(&p.civ) {
        c.army.fallen += k;
    }
    if gone {
        w.units.remove(&id);
    }
}

/// Army `id` stops holding tile `t`.
fn unhold(w: &mut World, id: u32, t: usize) {
    let Some(c) = w.units.get_mut(&id).and_then(|u| u.camp_mut()) else { return };
    if let Some(k) = c.held.iter().position(|&h| h == t) {
        c.held.remove(k);
        if k < c.guard.len() {
            c.guard.remove(k);
        }
    }
}

/// Moves army `id` off tile `from` at once, to the nearest tile by path cost its civ holds
/// (`civs::holder`) that isn't contested, a tie to the lower tile. It ends its march and keeps its
/// zone only if the zone is still connected. Returns where it went; none when there is nowhere.
fn retreat(w: &mut World, id: u32, contested: &BTreeSet<usize>, from: usize) -> Option<usize> {
    let civ = w.units[&id].civ;
    let mut cand: Vec<usize> = claim::held(w, civ);
    cand.extend(w.units.values().filter(|u| u.civ == civ).flat_map(|u| u.held().to_vec()));
    cand.sort_unstable();
    cand.dedup();
    let to = cand
        .into_iter()
        .filter(|&i| i != from && !contested.contains(&i) && civs::holder(w, i) == Some(civ))
        .filter_map(|i| path_between(w, from, i).map(|(_, cost)| (i, cost)))
        .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))?
        .0;
    let zone = zone_of(to, &w.units[&id].actor.territory);
    let keep = zone.len() > 1 && connected(w, to, &zone);
    let u = w.units.get_mut(&id).unwrap();
    u.actor.home = to;
    u.actor.territory = if keep { zone } else { vec![to] };
    u.banked = 0.0;
    let c = u.camp_mut().expect("an army");
    c.dest = None;
    c.held = vec![to];
    c.guard = vec![0.0];
    c.full = false;
    c.still = 0;
    Some(to)
}
