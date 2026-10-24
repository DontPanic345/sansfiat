//! The end of the pre-game: once enough centres have formed on their own, far apart, and people
//! have spread over the land they can reach. A turn limit is the backstop.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::data::*;
use crate::land::{is_hearth, neighbours, passable};
use crate::markets::{reach, Market};
use crate::round;
use crate::world::World;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct End {
    pub turn: u32,
    /// The centres' market ids.
    pub centres: Vec<u32>,
    pub cover: f64,
    /// Ended by `max_turns`, not by the centres and the cover.
    pub by_limit: bool,
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

/// The `end` step: the pre-game ends with `players` centres and the land covered, or at
/// `max_turns`. Only the first end is kept; turns may run on after it.
pub fn check_end(w: &mut World) {
    if w.end.is_some() {
        return;
    }
    let centres = centres(w);
    let cover = land_cover(w, &centres);
    let by_centres = centres.len() == PREGAME.players && cover >= PREGAME.cover;
    if by_centres || w.turn + 1 >= PREGAME.max_turns {
        w.end = Some(End { turn: w.turn, centres, cover, by_limit: !by_centres });
    }
}
