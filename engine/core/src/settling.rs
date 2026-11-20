//! Settling: a way of life a clan takes up and leaves. A clan settles once its field share has stood at
//! `SETTLING.farms` or more for `SETTLING.settle_after` turns in a row, and a settled clan goes back to
//! gathering once its share has stood under `SETTLING.gathers` for `SETTLING.leave_after` turns in a row;
//! between the two it stays as it was. A clan that moves its home stays as it was. The clan a civ is
//! founded on is settled at founding (`found`). Growth reads `Clan.settled` (`clans::growth`,
//! `clans::child_days`), and the feed tells each change (`ClanSettled`, `ClanGathers`).

use crate::data::SETTLING;
use crate::events::{Event, EventKind};
use crate::world::World;

/// Serde's skip for `Clan.nursing` while it is 0.
pub fn is_none(x: &f64) -> bool {
    *x == 0.0
}

/// Serde's skip for `Clan.settle_run` while it is 0.
pub fn is_zero(n: &u32) -> bool {
    *n == 0
}

/// Whether a clan with field share `fields` and settled state `settled` stands past the line that
/// would change it this turn.
pub fn past(settled: bool, fields: f64) -> bool {
    if settled {
        fields < SETTLING.gathers - 1e-9
    } else {
        fields >= SETTLING.farms - 1e-9
    }
}

/// The `settling` step: each clan's run moves on or resets, and a run that reaches its turns flips the
/// clan's state, with a feed line.
pub fn step(w: &mut World) {
    let ids: Vec<u32> = w.clans.keys().copied().collect();
    for id in ids {
        let c = w.clans.get_mut(&id).unwrap();
        if !past(c.settled, c.fields) {
            c.settle_run = 0;
            continue;
        }
        c.settle_run += 1;
        let after = if c.settled { SETTLING.leave_after } else { SETTLING.settle_after };
        if c.settle_run < after {
            continue;
        }
        c.settle_run = 0;
        c.settled = !c.settled;
        let (settled, home) = (c.settled, c.home);
        let kind = if settled { EventKind::ClanSettled } else { EventKind::ClanGathers };
        w.emit(Event::new(kind, home).clan(id).amount(after as f64));
    }
}

/// Founding's settling: the clan civ `civ` is founded on settles, whatever its ground.
pub fn found(w: &mut World, civ: u32) {
    let id = w.civs[&civ].clan;
    let Some(c) = w.clans.get_mut(&id).filter(|c| c.civ == Some(civ)) else { return };
    if c.settled {
        return;
    }
    c.settled = true;
    c.settle_run = 0;
    let name = w.civs[&civ].name.clone();
    w.note(id, format!("settled at founding: {name} is founded on it"));
}

/// How a civ's people live: the people of its clans that gather and of its settled clans, which farm,
/// the people of all its units (camps and scouts), and its settled clans out of its clans.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Living {
    pub gathering: f64,
    pub farming: f64,
    pub army: f64,
    pub settled: usize,
    pub clans: usize,
}

/// How civ `civ`'s people live (`Living`).
pub fn living(w: &World, civ: u32) -> Living {
    let mut l = Living::default();
    for k in w.clans.values().filter(|k| k.civ == Some(civ)) {
        l.clans += 1;
        if k.settled {
            l.settled += 1;
            l.farming += k.people;
        } else {
            l.gathering += k.people;
        }
    }
    l.army = w.units.values().filter(|u| u.civ == civ).map(|u| u.people).sum();
    l
}
