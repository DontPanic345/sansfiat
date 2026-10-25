//! The end of the pre-game: once enough centres have formed on their own, far apart, and people
//! have spread over the land they can reach. A turn limit is the backstop.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::civs;
use crate::data::*;
use crate::land::{is_hearth, neighbours, passable};
use crate::map::{Tile, Zone};
use crate::markets::{raise, reach, Market};
use crate::round;
use crate::world::{EventKind, World};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct End {
    pub turn: u32,
    /// The centres' market ids.
    pub centres: Vec<u32>,
    pub cover: f64,
    /// Ended by `max_turns`, not by the centres and the cover.
    pub by_limit: bool,
    /// The market ids added to reach `players` centres at the limit, in order; a subset of `centres`.
    pub topped_up: Vec<u32>,
}

fn steps(w: &World, a: usize, b: usize) -> i32 {
    w.tiles[a].hex().distance(w.tiles[b].hex())
}

/// A market that has lasted `centre_age` turns, had `centre_clans` traders last turn, and has a
/// hearth in its reach.
pub fn is_centre(w: &World, m: &Market) -> bool {
    w.turn.saturating_sub(m.founded) >= PREGAME.centre_age
        && m.traders >= PREGAME.centre_clans
        && reach(w, m.tile).keys().any(|&t| is_hearth(&w.tiles[t]))
}

/// The centres, busiest first (traders, then older, then lower id), each `centre_spacing` steps
/// from those taken before it, up to `players`. Greedy, not the best set.
pub fn centres(w: &World) -> Vec<u32> {
    let mut ms: Vec<&Market> = w.markets.values().filter(|m| is_centre(w, m)).collect();
    ms.sort_by_key(|m| (std::cmp::Reverse(m.traders), m.founded, m.id));
    let mut taken: Vec<&Market> = Vec::new();
    for m in ms {
        if taken.len() == PREGAME.players {
            break;
        }
        if taken.iter().all(|t| steps(w, t.tile, m.tile) >= PREGAME.centre_spacing) {
            taken.push(m);
        }
    }
    taken.iter().map(|m| m.id).collect()
}

/// The share of the land connected over passable land to the centres' markets (all land with no
/// centres) that lies within `cover_steps` steps of some clan's territory.
pub fn land_cover(w: &World, centres: &[u32]) -> f64 {
    let n = w.tiles.len();
    let passable = |i: usize| TERRAIN[w.tiles[i].terrain].land && passable(w, i);
    let land: Vec<usize> = if centres.is_empty() {
        (0..n).filter(|&i| TERRAIN[w.tiles[i].terrain].land).collect()
    } else {
        let mut seen = vec![false; n];
        let mut todo: Vec<usize> = centres.iter().map(|id| w.markets[id].tile).collect();
        for &t in &todo {
            seen[t] = true;
        }
        let mut out = todo.clone();
        while let Some(i) = todo.pop() {
            for j in neighbours(w, i) {
                if !seen[j] && passable(j) {
                    seen[j] = true;
                    out.push(j);
                    todo.push(j);
                }
            }
        }
        out
    };
    if land.is_empty() {
        return 0.0;
    }
    // Steps from the nearest territory tile, by a breadth-first search over the whole map.
    let mut dist: Vec<Option<i32>> = vec![None; n];
    let mut queue = VecDeque::new();
    for c in w.clans.values() {
        for t in round::territory(w, &c.round) {
            if dist[t].is_none() {
                dist[t] = Some(0);
                queue.push_back(t);
            }
        }
    }
    while let Some(i) = queue.pop_front() {
        let d = dist[i].unwrap();
        if d >= PREGAME.cover_steps {
            continue;
        }
        for j in neighbours(w, i) {
            if dist[j].is_none() {
                dist[j] = Some(d + 1);
                queue.push_back(j);
            }
        }
    }
    land.iter().filter(|&&i| dist[i].is_some()).count() as f64 / land.len() as f64
}

/// Labels each passable land tile with its region, the passable land it is connected to;
/// `usize::MAX` for the rest. Regions are numbered by their lowest tile.
fn regions(w: &World) -> Vec<usize> {
    let ok = |i: usize| TERRAIN[w.tiles[i].terrain].land && passable(w, i);
    let mut label = vec![usize::MAX; w.tiles.len()];
    let mut next = 0;
    for start in (0..w.tiles.len()).filter(|&i| ok(i)) {
        if label[start] != usize::MAX {
            continue;
        }
        label[start] = next;
        let mut todo = vec![start];
        while let Some(i) = todo.pop() {
            for j in neighbours(w, i) {
                if label[j] == usize::MAX && ok(j) {
                    label[j] = next;
                    todo.push(j);
                }
            }
        }
        next += 1;
    }
    label
}

/// One site the top-up takes: an existing market, or a tile to raise one on.
enum Pick {
    Market(u32),
    Tile(usize),
}

/// The sites that bring `centres` up to `players` within `region`, in order: existing markets,
/// best first; then the most tended land; then any land with grain. Every site is
/// `centre_spacing` steps from those taken, on passable land that isn't a sealed valley.
fn plan(w: &World, centres: &[u32], label: &[usize], region: usize) -> Vec<Pick> {
    let mut taken: Vec<usize> = centres.iter().map(|id| w.markets[id].tile).collect();
    let mut picks = Vec::new();
    let fits = |taken: &mut Vec<usize>, i: usize| {
        let ok = label[i] != usize::MAX
            && label[i] == region
            && w.tiles[i].zone != Zone::Sealed
            && taken.iter().all(|&k| steps(w, k, i) >= PREGAME.centre_spacing);
        if ok {
            taken.push(i);
        }
        ok
    };

    let mut markets: Vec<&Market> = w.markets.values().filter(|m| !centres.contains(&m.id)).collect();
    markets.sort_by_key(|m| (std::cmp::Reverse(m.traders), m.founded, m.id));
    for m in markets {
        if centres.len() + picks.len() == PREGAME.players {
            return picks;
        }
        if fits(&mut taken, m.tile) {
            picks.push(Pick::Market(m.id));
        }
    }

    let grain = |t: &Tile| t.wild_grain > 0.0 || t.cap[GRAIN] > 0.0;
    for tended in [true, false] {
        let mut tiles: Vec<usize> = (0..w.tiles.len())
            .filter(|&i| grain(&w.tiles[i]) && (!tended || w.tiles[i].tame > 0) && label[i] != usize::MAX)
            .filter(|&i| !w.markets.values().any(|m| m.tile == i))
            .collect();
        tiles.sort_by(|&a, &b| {
            let (ta, tb) = (&w.tiles[a], &w.tiles[b]);
            let tame = if tended { tb.tame.cmp(&ta.tame) } else { std::cmp::Ordering::Equal };
            tame.then(tb.fertility.total_cmp(&ta.fertility)).then(a.cmp(&b))
        });
        for tile in tiles {
            if centres.len() + picks.len() == PREGAME.players {
                return picks;
            }
            if fits(&mut taken, tile) {
                picks.push(Pick::Tile(tile));
            }
        }
    }
    picks
}

/// Adds sites to `centres` until there are `players`, at the turn limit; returns the ids added.
/// The sites are in the first centre's region of passable land. With no centres, the region of
/// the most populated clan's home comes first, then the others by their people: the first that
/// can hold all the sites wins, else the one that holds most. It ends short only if no region
/// holds `players` sites, `centre_spacing` apart.
fn top_up(w: &mut World, centres: &mut Vec<u32>) -> Vec<u32> {
    let label = regions(w);
    let order: Vec<usize> = match centres.first() {
        Some(id) => vec![label[w.markets[id].tile]],
        None => {
            let count = label.iter().filter(|&&r| r != usize::MAX).max().map_or(0, |&r| r + 1);
            let (mut people, mut size) = (vec![0.0; count], vec![0usize; count]);
            for &r in label.iter().filter(|&&r| r != usize::MAX) {
                size[r] += 1;
            }
            for c in w.clans.values().filter(|c| label[c.home] != usize::MAX) {
                people[label[c.home]] += c.pop;
            }
            let lead = w
                .clans
                .values()
                .max_by(|a, b| a.pop.total_cmp(&b.pop).then(b.id.cmp(&a.id)))
                .map(|c| label[c.home])
                .filter(|&r| r != usize::MAX);
            let mut rest: Vec<usize> = (0..count).filter(|&r| Some(r) != lead).collect();
            rest.sort_by(|&a, &b| people[b].total_cmp(&people[a]).then(size[b].cmp(&size[a])).then(a.cmp(&b)));
            lead.into_iter().chain(rest).collect()
        }
    };
    let mut best: Vec<Pick> = Vec::new();
    for region in order {
        let picks = plan(w, centres, &label, region);
        let done = centres.len() + picks.len() == PREGAME.players;
        if done || picks.len() > best.len() {
            best = picks;
        }
        if done {
            break;
        }
    }
    let mut added = Vec::new();
    for pick in best {
        let id = match pick {
            Pick::Market(id) => id,
            Pick::Tile(tile) => raise(w, tile),
        };
        centres.push(id);
        added.push(id);
    }
    added
}

/// The `end` step: the pre-game ends with `players` centres and the land covered, or at
/// `max_turns`, topping up to `players` centres if it got there with fewer. Only the first end
/// is kept; turns may run on after it.
pub fn check_end(w: &mut World) {
    if w.end.is_some() {
        return;
    }
    let mut centres = centres(w);
    for &id in &centres {
        if !w.centres_seen.contains(&id) {
            let tile = w.markets[&id].tile;
            w.event(EventKind::CentreFormed, tile, None, Some(id), None);
        }
    }
    // A market that drops out of the greedy set and back in is not announced twice.
    for &id in &centres {
        if !w.centres_seen.contains(&id) {
            w.centres_seen.push(id);
        }
    }
    let by_centres = centres.len() == PREGAME.players && land_cover(w, &centres) >= PREGAME.cover;
    if !by_centres && w.turn + 1 < PREGAME.max_turns {
        return;
    }
    let topped_up = if !by_centres && centres.len() < PREGAME.players { top_up(w, &mut centres) } else { Vec::new() };
    for &id in &topped_up {
        // A market that was a centre before and dropped out of the greedy set was announced then.
        if !w.centres_seen.contains(&id) {
            let tile = w.markets[&id].tile;
            w.event(EventKind::CentreFormed, tile, None, Some(id), None);
            w.centres_seen.push(id);
        }
    }
    let cover = land_cover(w, &centres);
    let first = centres.first().map_or(0, |id| w.markets[id].tile);
    w.end = Some(End { turn: w.turn, centres, cover, by_limit: !by_centres, topped_up });
    w.event(EventKind::PregameEnded, first, None, None, None);
    civs::found(w);
}
