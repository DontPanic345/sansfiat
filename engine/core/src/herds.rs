//! The herd. After the pick, a clan may keep animals: its days split three ways, gathering over its
//! territory, fields on its river ground and herding over its grass (`jobs.rs`). The herd is the
//! animals in the clan's store. It breeds by the logistic rule toward what the clan's herding days and
//! its grass carry (`carries`), and the clan eats the increase (`offtake`). Each turn a clan's share of
//! days on herding (`Clan.herds`) moves a notch toward whichever share feeds it better (`notch`),
//! weighed on the herd's steady yield; a herder holds wider land (`radius`). A clan works fields or
//! herds, never both.
//!
//! Nothing here acts before `open`: before the pick, and with `Config.herds` off, no clan herds.

use crate::clans::{self, Clan};
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::market::Basket;
use crate::world::World;

/// Whether the herd job acts: the config has herds and civs are founded (past the pick).
pub fn open(w: &World) -> bool {
    w.config.herds && !w.civs.is_empty()
}

/// Animals tile `i` carries before any clan's days limit them: `HERD.graze` by terrain × its
/// fertility. Land only; grazing takes no stock and wears nothing.
pub fn graze(w: &World, i: usize) -> f64 {
    let t = &w.tiles[i];
    if !TERRAIN[t.terrain].land {
        return 0.0;
    }
    HERD.graze[t.terrain] * t.fertility
}

/// What a herd on `tiles` carries, its `K`: on each tile the grass (`graze`) or the animals the
/// `days` there keep (`HERD.tend` a day), whichever is less.
pub fn carries(w: &World, tiles: &[usize], days: &[f64]) -> f64 {
    tiles.iter().zip(days).map(|(&t, &d)| graze(w, t).min(d * HERD.tend)).sum()
}

/// The change in a herd `h` in a turn on land that carries `k`: `HERD.breed × h × (1 − h ÷ k)`. Above
/// `k` it is negative, and the herd never falls past `k`; with `k` 0 the herd falls to 0 by the same rule.
pub fn breed(h: f64, k: f64) -> f64 {
    if k <= 0.0 {
        return -h;
    }
    let inc = HERD.breed * h * (1.0 - h / k);
    if h > k {
        inc.max(k - h)
    } else {
        inc
    }
}

/// What a clan takes of its herd's increase `inc`: all of it. One rule, so it can change.
pub fn offtake(_h: f64, inc: f64, _k: f64) -> f64 {
    inc.max(0.0)
}

/// What a herd on land that carries `k` yields a turn at half of `k`, where it grows fastest.
pub fn steady(k: f64) -> f64 {
    HERD.breed * k / 4.0
}

/// Whether the clan is a herder: at least `HERD.herder` of its days on herding.
pub fn herder(c: &Clan) -> bool {
    c.herds >= HERD.herder - 1e-9
}

/// The radius of the territory a clan holds: a herder's is `HERD.radius`, a gatherer's `CLAN.territory_radius`.
pub fn radius(c: &Clan) -> i32 {
    if herder(c) {
        HERD.radius
    } else {
        CLAN.territory_radius
    }
}

/// The animals a clan of `people` with `store` and herding `share` keeps: its store's, or the herd it
/// would start (`HERD.start` a person) when it has a share and none. For reckoning.
pub fn held(people: f64, store: &Basket, share: f64) -> f64 {
    if store[ANIMALS] > 0.0 || share <= 0.0 {
        store[ANIMALS]
    } else {
        HERD.start * people
    }
}

/// `held` for a clan.
pub fn head(c: &Clan) -> f64 {
    held(c.people, &c.store, c.herds)
}

/// What a clan short of `CLAN.stable_at` of its need eats from its herd `herd` (the breeding stock it
/// held before the meal): at most `HERD.lean` of it, enough to reach `stable_at`. `got` is what the meal
/// gave. Animals left in its basket count at full value up to `CLAN.diet_share × need`, and at
/// `CLAN.excess_value` beyond. Returns the nutrition it gave and the herd that is left.
pub fn lean(c: &Clan, need: f64, got: f64, herd: f64) -> (f64, f64) {
    let short = CLAN.stable_at * need - got;
    if herd <= 0.0 || short <= 0.0 {
        return (0.0, herd);
    }
    let eaten = c.basket[ANIMALS] - c.store[ANIMALS];
    let room = (CLAN.diet_share * need - eaten).max(0.0);
    let animals = if short <= room { short } else { room + (short - room) / CLAN.excess_value };
    let animals = animals.min(HERD.lean * herd);
    (animals.min(room) + CLAN.excess_value * (animals - room).max(0.0), herd - animals)
}

/// Starts clan `id`'s herd if it has none: `HERD.start` a person. The game on the clan's own tiles pays
/// what it can, in proportion to their stock; the rest comes from beyond its land.
pub fn start(w: &mut World, id: u32) {
    let c = &w.clans[&id];
    if c.store[ANIMALS] > 0.0 {
        return;
    }
    let want = HERD.start * c.people;
    let mine: Vec<usize> = c.territory.iter().copied().filter(|&t| w.tiles[t].claim == Some(id)).collect();
    let game: f64 = mine.iter().map(|&t| w.tiles[t].stock[GAME]).sum();
    let paid = want.min(game);
    if paid > 0.0 {
        for &t in &mine {
            let s = w.tiles[t].stock[GAME];
            w.tiles[t].stock[GAME] = s - paid * s / game;
        }
    }
    w.clans.get_mut(&id).unwrap().store[ANIMALS] = want;
}

/// The value of clan `c` (a copy) at herding share `share`.
fn value(w: &World, c: &Clan, share: f64) -> f64 {
    let mut k = c.clone();
    k.herds = share;
    clans::value(w, &k, &k.territory)
}

/// The `move` step's herding share for clan `id`, once its territory and home are settled: it moves one
/// notch toward whichever of its share ± `HERD.notch` feeds it better by `value` (which reckons the
/// herd's steady yield), if that beats the current share by more than `HERD.margin`. A clan with fields
/// does not raise herds, and one with herds does not raise fields (`fields::notch`). A starving clan
/// with herds drops a notch instead. A share that rises from 0 starts a herd and emits `ClanHerding`;
/// one that falls to 0 leaves a note.
pub fn notch(w: &mut World, id: u32) {
    if !open(w) {
        return;
    }
    let c = w.clans[&id].clone();
    if c.fields > 0.0 && c.herds <= 0.0 {
        return;
    }
    let n = HERD.notch;
    let next = if c.starving && c.herds > 0.0 {
        (c.herds - n).max(0.0)
    } else {
        let bar = value(w, &c, c.herds) + HERD.margin;
        let up = (c.herds < 1.0 - 1e-9).then(|| (c.herds + n).min(1.0));
        let down = (c.herds > 1e-9).then(|| (c.herds - n).max(0.0));
        let up = up.map(|s| (s, value(w, &c, s))).filter(|&(_, v)| v > bar);
        let down = down.map(|s| (s, value(w, &c, s))).filter(|&(_, v)| v > bar);
        match (up, down) {
            (Some(u), Some(d)) => {
                if u.1 >= d.1 {
                    u.0
                } else {
                    d.0
                }
            }
            (Some(u), None) => u.0,
            (None, Some(d)) => d.0,
            (None, None) => c.herds,
        }
    };
    if next == c.herds {
        return;
    }
    w.clans.get_mut(&id).unwrap().herds = next;
    if c.herds <= 0.0 && next > 0.0 {
        start(w, id);
        w.emit(Event::new(EventKind::ClanHerding, c.home).clan(id));
    }
    if next <= 0.0 && c.herds > 0.0 {
        w.note(id, "gave up its herd: grass fed it less than gathering".to_string());
    }
}
