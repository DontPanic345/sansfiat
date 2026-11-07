//! The ring in the judge: each civ's ring at the pick and +200, the armies and scouts inside it and
//! outside it turn by turn (and the hungry turns among each), and the territory of a clan. `Watch`
//! reads these after each turn and reports them as the `supply` group; the world is only read.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::claim;
use crate::units::{self, UnitKind};
use crate::world::World;

/// The counts of one civ's units and ring, or the sum over all civs (`SupplyScores.total`).
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SupplyCiv {
    /// The civ's ring tiles (share under one) at the pick, and those of them the civ holds.
    pub ring_pick: usize,
    pub held_ring_pick: usize,
    /// As `ring_pick`, at +200.
    pub ring_200: usize,
    pub held_ring_200: usize,
    /// Army-turns with the army inside the civ's ring (its tile's share under one), and outside it.
    pub army_inside: u32,
    pub army_outside: u32,
    /// The hungry army-turns among each.
    pub army_inside_hungry: u32,
    pub army_outside_hungry: u32,
    /// Scout-turns with the scout inside the ring, and outside it, and the hungry turns among each.
    pub scout_inside: u32,
    pub scout_outside: u32,
    pub scout_inside_hungry: u32,
    pub scout_outside_hungry: u32,
}

impl SupplyCiv {
    /// Adds `o`'s counts to these.
    pub fn add(&mut self, o: &SupplyCiv) {
        self.ring_pick += o.ring_pick;
        self.held_ring_pick += o.held_ring_pick;
        self.ring_200 += o.ring_200;
        self.held_ring_200 += o.held_ring_200;
        self.army_inside += o.army_inside;
        self.army_outside += o.army_outside;
        self.army_inside_hungry += o.army_inside_hungry;
        self.army_outside_hungry += o.army_outside_hungry;
        self.scout_inside += o.scout_inside;
        self.scout_outside += o.scout_outside;
        self.scout_inside_hungry += o.scout_inside_hungry;
        self.scout_outside_hungry += o.scout_outside_hungry;
    }
}

/// The supply group: per civ and in total, the ring and the units inside and outside it, and the
/// mean territory of a clan at the pick and +200.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SupplyScores {
    pub civs: BTreeMap<u32, SupplyCiv>,
    pub total: SupplyCiv,
    /// The mean territory tiles a clan has at the pick, and at +200 (`None` before +200).
    pub clan_territory_pick: f64,
    pub clan_territory_200: Option<f64>,
}

/// The ring tiles of a civ, as the judge counts them: the tiles whose share is under one, and of
/// them the ones held by the civ at this moment.
fn ring_counts(w: &World, civ: u32) -> (usize, usize) {
    let inside: BTreeSet<usize> =
        units::ring(w, civ).into_iter().filter(|&(_, share)| share < 1.0).map(|(i, _)| i).collect();
    let held = claim::held(w, civ).into_iter().filter(|i| inside.contains(i)).count();
    (inside.len(), held)
}

/// The judge's supply counts as the watch carries them: each civ's counts so far, and the clan
/// territory means.
#[derive(Clone, Debug, Default)]
pub(crate) struct Supply {
    civs: BTreeMap<u32, SupplyCiv>,
    clan_territory_pick: f64,
    clan_territory_200: Option<f64>,
}

/// The mean of a clan's territory tiles over the clans of `w`; 0 with none.
fn clan_territory(w: &World) -> f64 {
    if w.clans.is_empty() {
        return 0.0;
    }
    w.clans.values().map(|c| c.territory.len() as f64).sum::<f64>() / w.clans.len() as f64
}

impl Supply {
    /// The counts at the pick: each civ's ring and the clans' territory.
    pub(crate) fn pick(w: &World) -> Supply {
        let civs = w
            .civs
            .keys()
            .map(|&id| {
                let (ring, held) = ring_counts(w, id);
                (id, SupplyCiv { ring_pick: ring, held_ring_pick: held, ..Default::default() })
            })
            .collect();
        Supply { civs, clan_territory_pick: clan_territory(w), clan_territory_200: None }
    }

    /// The ring at +200, as `pick` reads it.
    pub(crate) fn at_200(&mut self, w: &World) {
        for (&id, c) in self.civs.iter_mut() {
            let (ring, held) = ring_counts(w, id);
            c.ring_200 = ring;
            c.held_ring_200 = held;
        }
        self.clan_territory_200 = Some(clan_territory(w));
    }

    /// Scores the turn that has just ended: each army and scout alive, inside its civ's ring or
    /// outside it, and whether it was hungry. A unit at its capital is inside (share 0); a civ's
    /// ring is computed once, and only if one of its units is away from the capital.
    pub(crate) fn turn(&mut self, w: &World) {
        let mut rings: BTreeMap<u32, BTreeMap<usize, f64>> = BTreeMap::new();
        for u in w.units.values() {
            let kind = u.kind();
            if kind != UnitKind::Camp && kind != UnitKind::Scout {
                continue;
            }
            let Some(capital) = w.civs.get(&u.civ).map(|c| c.capital) else { continue };
            let inside = if u.home == capital {
                true
            } else {
                let ring = rings.entry(u.civ).or_insert_with(|| units::ring(w, u.civ));
                ring.get(&u.home).is_some_and(|&share| share < 1.0)
            };
            let hungry = u.hungry > 0;
            let c = self.civs.entry(u.civ).or_default();
            match (kind, inside) {
                (UnitKind::Camp, true) => {
                    c.army_inside += 1;
                    c.army_inside_hungry += hungry as u32;
                }
                (UnitKind::Camp, false) => {
                    c.army_outside += 1;
                    c.army_outside_hungry += hungry as u32;
                }
                (_, true) => {
                    c.scout_inside += 1;
                    c.scout_inside_hungry += hungry as u32;
                }
                (_, false) => {
                    c.scout_outside += 1;
                    c.scout_outside_hungry += hungry as u32;
                }
            }
        }
    }

    /// The group's scores: per civ, and the total over civs.
    pub(crate) fn scores(&self) -> SupplyScores {
        let mut total = SupplyCiv::default();
        for c in self.civs.values() {
            total.add(c);
        }
        SupplyScores {
            civs: self.civs.clone(),
            total,
            clan_territory_pick: self.clan_territory_pick,
            clan_territory_200: self.clan_territory_200,
        }
    }
}
