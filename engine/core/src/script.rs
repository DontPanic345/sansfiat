//! The scripted game: the engine's second player, run by `sansfiat digest --lead` and by the wasm
//! build's `Sim::lead_orders` (`cross-build.mjs` checks the two agree).

use crate::data::{FOREST, HILLS, LEANING};
use crate::world::{Action, Phase};
use crate::{land, units, Order, World};

/// The scripted game, between turns and before each `EndTurn`. `pick` is the turn `ChooseCiv` was
/// acted, and `n` the turns since. Each order is the led civ's `Action::Order`; its camp or scout is
/// the unit it names.
///
/// 1. At `Choose`: lead the lowest civ id.
/// 2. In `Main`, when `n > 0 && n.is_multiple_of(20)`: `Order::Raise { n: 1 }` (a refusal is ignored).
/// 3. When `n == 30`: `Order::Switches` both off; when `n == 40`: both on. When `n > 0 &&
///    n.is_multiple_of(25)`: `Order::Resupply` (a refusal is ignored).
/// 4. If the player's camp has no `dest`: `Order::Move` to the first accepted of the other civs'
///    capitals in civ id order, then its own capital.
/// 5. When `n == 60`: `Order::Disband { n: Some(1) }`, if there is a camp.
/// 6. When `n == 10`: `SetGift` twice the default (`2 × LEANING.gift_default`); when `n == 50`:
///    `SetKeep { food: Some(5.0) }`; when `n == 70`: `SetKeep { food: None }`.
/// 7. When `n == 2`: `Order::Route` for the scout from the capital to the first forest or hills tile by
///    index seven steps out (ignored when there is none).
/// 8. When `n == 45`, if there is a camp: `Order::Move` to its own tile (it halts), then `Order::Zone`
///    over the first `people − 1` tiles of its line (the capital left out; a refusal is ignored). Rule 4
///    rests while `45 <= n < 55`, so the zone spreads and is held.
#[derive(Default)]
pub struct Lead {
    pick: Option<u32>,
}

impl Lead {
    /// Gives the scripted game's orders for the current turn, then leaves the turn to the caller.
    pub fn orders(&mut self, w: &mut World) {
        if w.phase == Phase::Choose {
            let first = w.civs.keys().next().copied();
            w.act(Action::ChooseCiv { civ: first }).expect("a civ can be chosen at the pick");
            self.pick = Some(w.turn);
        }
        let (Some(pick), Some(me)) = (self.pick, w.player) else { return };
        if w.phase != Phase::Main {
            return;
        }
        let n = w.turn - pick;
        let order = |order: Order| Action::Order { civ: me, order };
        let camp = |w: &World| units::camp(w, me).map(|u| u.id);
        let scout = |w: &World| units::scout(w, me).map(|u| u.id);
        if n > 0 && n.is_multiple_of(20) {
            let _ = w.act(order(Order::Raise { n: 1 }));
        }
        if let (30, Some(unit)) = (n, camp(w)) {
            let _ = w.act(order(Order::Switches { unit, forage: false, supply: false }));
        }
        if let (40, Some(unit)) = (n, camp(w)) {
            let _ = w.act(order(Order::Switches { unit, forage: true, supply: true }));
        }
        if let (true, Some(unit)) = (n > 0 && n.is_multiple_of(25), camp(w)) {
            let _ = w.act(order(Order::Resupply { unit }));
        }
        let targets: Vec<usize> = w
            .civs
            .values()
            .filter(|c| c.id != me)
            .map(|c| c.capital)
            .chain(std::iter::once(w.civs[&me].capital))
            .collect();
        if n == 2 {
            let cap = w.civs[&me].capital;
            let target = (0..w.tiles.len())
                .find(|&i| matches!(w.tiles[i].terrain, FOREST | HILLS) && land::steps(w, i, cap) == 7);
            if let (Some(to), Some(unit)) = (target, scout(w)) {
                let _ = w.act(order(Order::Route { unit, from: cap, to }));
            }
        }
        if let Some(unit) = camp(w)
            .filter(|&id| w.units[&id].camp().is_some_and(|c| c.dest.is_none()))
            .filter(|_| !(45..55).contains(&n))
        {
            for &to in &targets {
                if w.act(order(Order::Move { unit, to })).is_ok() {
                    break;
                }
            }
        }
        if n == 45 {
            if let Some((id, home, people)) = units::camp(w, me).map(|u| (u.id, u.home, u.people)) {
                let _ = w.act(order(Order::Move { unit: id, to: home }));
                if let Some((path, _, _)) = units::line(w, id) {
                    let capital = w.civs[&me].capital;
                    let k = (people as usize).saturating_sub(1);
                    let tiles: Vec<usize> = path.into_iter().filter(|&t| t != capital).take(k).collect();
                    let _ = w.act(order(Order::Zone { unit: id, tiles }));
                }
            }
        }
        if let (60, Some(unit)) = (n, camp(w)) {
            let _ = w.act(order(Order::Disband { unit, n: Some(1) }));
        }
        if n == 10 {
            let _ = w.act(Action::SetGift { food: 2.0 * LEANING.gift_default });
        }
        if n == 50 {
            let _ = w.act(Action::SetKeep { food: Some(5.0) });
        }
        if n == 70 {
            let _ = w.act(Action::SetKeep { food: None });
        }
    }
}
