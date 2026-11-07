//! The guard: a civ's people who stand at its capital and hold its claim (`claim.rs`). A guard takes
//! no order but raising and disbanding. It eats from the civ's store, `UNIT.food` a person a turn,
//! with no pack and no line. What the store lacks, its civ's clans give from their own stores; a turn it
//! goes unfed it is hungry, and hungry `UNIT.hungry_turns` turns in a row it loses a person a turn to the civ's own clan, as a camp's
//! warriors do.

use super::{disband, take_people, Unit, UnitState, EPS};
use crate::actors::{self, Actor};
use crate::claim;
use crate::data::*;
use crate::world::World;

/// Raises up to `n` of the guard's people for `civ`: one person from each of the `n` biggest clans
/// able to give, as warriors come (`take_people`), into the guard at the capital, making it if there
/// is none. Its claim is divided again. Returns how many people it took.
pub fn raise(w: &mut World, civ: u32, n: u32) -> u32 {
    let k = take_people(w, civ, n);
    if k == 0.0 {
        return 0;
    }
    let id = match super::guard_of(w, civ).map(|u| u.id) {
        Some(id) => {
            w.units.get_mut(&id).unwrap().actor.people += k;
            id
        }
        None => {
            let id = w.next_id;
            w.next_id += 1;
            let capital = w.civs[&civ].capital;
            let unit = Unit { id, civ, actor: Actor::new(capital, k), banked: 0.0, hungry: 0, state: UnitState::Guard };
            w.units.insert(id, unit);
            id
        }
    };
    let people = w.units[&id].people;
    w.note(id, format!("raised {k:.0} guard(s) from its clans: {people:.0} on its claim"));
    claim::divide(w, civ);
    k as u32
}

/// The guard's turn, in the `units` step: it eats its `UNIT.food` a person from the civ's store, then
/// from its civ's clans' stores. A
/// turn it isn't fed it is hungry; hungry `UNIT.hungry_turns` in a row, it disbands a person.
pub(super) fn turn(w: &mut World, id: u32) {
    let (civ, need) = {
        let u = &w.units[&id];
        (u.civ, u.people * UNIT.food)
    };
    // What the store lacks, the civ's clans give from their own stores, biggest first: the guard
    // are their people.
    let kin = w.clans.values_mut().filter(|c| c.civ == Some(civ)).map(|c| (c.id, &mut c.actor.store));
    let got = actors::meal(need, &mut w.civs.get_mut(&civ).unwrap().store, kin);
    let fed = got + EPS >= need;
    let u = w.units.get_mut(&id).unwrap();
    u.hungry = if fed { 0 } else { u.hungry + 1 };
    let hungry = u.hungry;
    if hungry == 1 {
        w.note(id, "went hungry: its civ's store could not feed its guard".to_string());
    }
    if hungry >= UNIT.hungry_turns {
        disband(w, id, Some(1), true);
    }
}
