//! The field job. After founding, a clan splits its days between gathering and fields: fields are
//! worked on the map's floodplains and river tiles (`ground`), and a field day yields grain by the
//! tile's fertility and its tame past wild (`yield_a_day`). Field grain comes from no stock, and wears
//! nothing. Each turn a clan's share of days on fields (`Clan.fields`) moves a notch toward whichever
//! share feeds it better (`notch`); a farmer's home follows its fields (`home_toward`).
//!
//! Nothing here acts before `open`: before the pick, and with `Config.fields` off, no clan farms.

use crate::actors::{self, Route};
use crate::clans::{self, Clan};
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::domestic;
use crate::world::World;

/// Whether the field job acts: the config has fields and civs are founded (past the pick).
pub fn open(w: &World) -> bool {
    w.config.fields && !w.civs.is_empty()
}

/// The field days clan `id` may sow on tile `t` this turn: the tile's ground less the days the other
/// clans sowed on it last turn, at least 0. Only the clans that still have `t` in their territory count:
/// a clan that has since let it go, or died, leaves the ground to whoever holds it now. A tile only one
/// clan sows has room equal to its ground.
pub fn room(w: &World, id: u32, t: usize) -> f64 {
    let others: f64 = w
        .sowers
        .iter()
        .filter(|&&k| k != id)
        .filter_map(|k| w.clans.get(k))
        .filter(|k| k.territory.binary_search(&t).is_ok())
        .filter_map(|k| k.sown.binary_search_by_key(&t, |&(s, _)| s).ok().map(|s| k.sown[s].1))
        .sum();
    (ground(w, t) - others).max(0.0)
}

/// Days of fields tile `i` takes a turn: `FIELD.flood` on a floodplain, and `FIELD.river` in
/// proportion to its river up to `MAP.big_river` on a river tile; both add. Land only, from no rng.
pub fn ground(w: &World, i: usize) -> f64 {
    let t = &w.tiles[i];
    if !TERRAIN[t.terrain].land {
        return 0.0;
    }
    let mut g = 0.0;
    if t.flood {
        g += FIELD.flood;
    }
    if t.river > 0 {
        g += FIELD.river * (t.river as f64 / MAP.big_river as f64).min(1.0);
    }
    g
}

/// Grain a field day yields on tile `i`: `FIELD.grain` × its fertility × its tame past wild, so none
/// until the grain has changed under the clan's hands.
pub fn yield_a_day(w: &World, i: usize) -> f64 {
    let t = &w.tiles[i];
    (FIELD.grain * t.fertility * (domestic(t.tame) - 1.0)).max(0.0)
}

/// The field days clan `id` works on each of `tiles` from the labour `work` its even share of the field
/// job gives it: a tile takes at most its room (`room`). What a tile can't take goes to the job's other
/// tiles below their room, evenly, round by round; what none can take stays unworked.
pub fn sow(w: &World, id: u32, tiles: &[usize], work: &[f64]) -> Vec<f64> {
    let cap: Vec<f64> = tiles.iter().map(|&t| room(w, id, t)).collect();
    let mut days: Vec<f64> = work.iter().zip(&cap).map(|(&k, &g)| k.min(g)).collect();
    let mut spare: f64 = work.iter().zip(&days).map(|(&k, &d)| (k - d).max(0.0)).sum();
    while spare > 1e-12 {
        let open: Vec<usize> = (0..days.len()).filter(|&k| days[k] < cap[k]).collect();
        if open.is_empty() {
            break;
        }
        let each = spare / open.len() as f64;
        let mut taken = 0.0;
        for k in open {
            let add = each.min(cap[k] - days[k]);
            days[k] = (days[k] + add).min(cap[k]);
            taken += add;
        }
        spare -= taken;
    }
    days
}

/// The two jobs of a clan's labour over `tiles` (its territory, with `yearly` walks): gathering at
/// `1 − fields` over the tiles `gathers` marks, and the field job at `fields` over the tiles with
/// ground, each tile paying its walk. A field tile is a tile with ground that yields: days are not sown
/// on grain still wild. Returns each tile's gathering work (0 where `gathers` is false), the field job's
/// work on each field tile, and the field tiles' indices into `tiles`. With no field tile, `fields`
/// gives nothing and all labour gathers.
pub fn jobs(
    w: &World,
    people: f64,
    routes: &[Route],
    tiles: &[usize],
    yearly: &[f64],
    fields: f64,
    gathers: &[bool],
) -> (Vec<f64>, Vec<f64>, Vec<usize>) {
    let fk: Vec<usize> =
        (0..tiles.len()).filter(|&k| ground(w, tiles[k]) > 0.0 && yield_a_day(w, tiles[k]) > 0.0).collect();
    let share = if fk.is_empty() { 0.0 } else { fields };
    let gk: Vec<usize> = (0..tiles.len()).filter(|&k| gathers[k]).collect();
    let gy: Vec<f64> = gk.iter().map(|&k| yearly[k]).collect();
    let fy: Vec<f64> = fk.iter().map(|&k| yearly[k]).collect();
    let mut parts = actors::split(people, people * CLAN.work, routes, &[1.0 - share, share], &[gy, fy]);
    let field = parts.pop().unwrap();
    let gathered = parts.pop().unwrap();
    let mut gather = vec![0.0; tiles.len()];
    for (&k, g) in gk.iter().zip(gathered) {
        gather[k] = g;
    }
    (gather, field, fk)
}

/// Whether the clan with `fields` is a farmer: at least `FIELD.farmer` of its days on fields.
pub fn farmer(c: &Clan) -> bool {
    c.fields >= FIELD.farmer - 1e-9
}

/// The field tile a farmer's home steps toward: its field tile with the most field days on this turn's
/// split over `territory` (sorted) from its home, ties to the lower tile. `None` for a clan that is not
/// a farmer, or whose fields work no days.
pub fn home_toward(w: &World, c: &Clan, territory: &[usize]) -> Option<usize> {
    if !farmer(c) {
        return None;
    }
    let (_, yearly) = clans::walks(w, c.home, territory);
    // The field job's labour does not depend on the gathering tiles: they are not counted here.
    let (_, field, fk) = jobs(w, c.people, &c.routes, territory, &yearly, c.fields, &vec![true; territory.len()]);
    let fields: Vec<usize> = fk.iter().map(|&k| territory[k]).collect();
    let days = sow(w, c.id, &fields, &field);
    let mut best: Option<(f64, usize)> = None;
    for (&t, &d) in fields.iter().zip(&days) {
        if d <= 0.0 {
            continue;
        }
        if best.is_none_or(|(bd, bt)| d > bd || (d == bd && t < bt)) {
            best = Some((d, t));
        }
    }
    best.map(|(_, t)| t)
}

/// A clan's fields this turn, as (tile, its own field days, its grain) for each tile it sowed: read from
/// `Clan.sown`, which the harvest step set this turn. A shared tile gives each clan its own days.
pub fn plots(w: &World, c: &Clan) -> Vec<(usize, f64, f64)> {
    c.sown.iter().map(|&(t, d)| (t, d, d * yield_a_day(w, t))).collect()
}

/// The `move` step's split for clan `id`, once its territory and home are settled: its share moves one
/// notch toward whichever of its share ± `FIELD.notch` feeds it better by `value` on this turn's stock,
/// if that beats the current share by more than `FIELD.margin`. A starving clan with fields drops a
/// notch instead. A share that rises from 0 emits `ClanFarming`; one that falls to 0 leaves a note.
pub fn notch(w: &mut World, id: u32) {
    if !open(w) {
        return;
    }
    let c = w.clans[&id].clone();
    let n = FIELD.notch;
    let next = if c.starving && c.fields > 0.0 {
        (c.fields - n).max(0.0)
    } else {
        let now = clans::value_at(w, &c, &c.territory, c.fields);
        let bar = now + FIELD.margin;
        let up = (c.fields < 1.0 - 1e-9).then(|| (c.fields + n).min(1.0));
        let down = (c.fields > 1e-9).then(|| (c.fields - n).max(0.0));
        let up = up.map(|s| (s, clans::value_at(w, &c, &c.territory, s))).filter(|&(_, v)| v > bar);
        let down = down.map(|s| (s, clans::value_at(w, &c, &c.territory, s))).filter(|&(_, v)| v > bar);
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
            (None, None) => c.fields,
        }
    };
    if next == c.fields {
        return;
    }
    w.clans.get_mut(&id).unwrap().fields = next;
    if c.fields <= 0.0 && next > 0.0 {
        w.emit(Event::new(EventKind::ClanFarming, c.home).clan(id));
    }
    if next <= 0.0 && c.fields > 0.0 {
        w.note(id, "gave up its fields: they fed it less than the hills".to_string());
    }
}
