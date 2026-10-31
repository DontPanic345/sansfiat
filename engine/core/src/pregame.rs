//! The end of the pre-game: once enough centres have formed on their own, far apart, and people
//! have spread over the land they can reach. A turn limit is the backstop. A centre is a clan;
//! the pre-game ends with a market on each centre's site, where founding puts a civ.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::civs;
use crate::clans::Clan;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::{is_hearth, neighbours, passable, steps};
use crate::map::{Country, Tile};
use crate::markets::{raise, Market};
use crate::world::World;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct End {
    pub turn: u32,
    /// The markets on the centres' sites, where the civs are founded.
    pub centres: Vec<u32>,
    /// Each site's centre clan, in `centres` order; `None` for a site the top-up found without one.
    #[serde(default)]
    pub clans: Vec<Option<u32>>,
    pub cover: f64,
    /// Ended by `max_turns`, not by the centres and the cover.
    pub by_limit: bool,
    /// The market ids added to reach `players` centres at the limit, in order; a subset of `centres`.
    pub topped_up: Vec<u32>,
}

/// A clan settled at home `centre_age` turns, going to a festival that had `centre_clans` traders
/// last turn, with a hearth on its territory.
pub fn is_centre(w: &World, c: &Clan) -> bool {
    c.since >= PREGAME.centre_age
        && c.market.and_then(|m| w.markets.get(&m)).is_some_and(|m| m.traders >= PREGAME.centre_clans)
        && c.territory.iter().any(|&t| is_hearth(&w.tiles[t]))
}

/// The centre clans, biggest first (people, then lower id), each home `centre_spacing` steps from
/// those taken before it, up to `players`. Greedy, not the best set.
pub fn centres(w: &World) -> Vec<u32> {
    let mut cs: Vec<&Clan> = w.clans.values().filter(|c| is_centre(w, c)).collect();
    cs.sort_by(|a, b| b.people.total_cmp(&a.people).then(a.id.cmp(&b.id)));
    let mut taken: Vec<&Clan> = Vec::new();
    for c in cs {
        if taken.len() == PREGAME.players {
            break;
        }
        if taken.iter().all(|t| steps(w, t.home, c.home) >= PREGAME.centre_spacing) {
            taken.push(c);
        }
    }
    taken.iter().map(|c| c.id).collect()
}

/// The centres' sites: the homes of the clans `ids`, before the end.
pub fn homes(w: &World, ids: &[u32]) -> Vec<usize> {
    ids.iter().filter_map(|id| w.clans.get(id)).map(|c| c.home).collect()
}

/// The centres' sites: the tiles of the markets `ids`, after the end.
pub fn sites(w: &World, ids: &[u32]) -> Vec<usize> {
    ids.iter().filter_map(|id| w.markets.get(id)).map(|m| m.tile).collect()
}

/// The share of the land connected over passable land to the centres' sites `tiles` (all land
/// with none) that lies within `cover_steps` steps of some clan's territory.
pub fn land_cover(w: &World, tiles: &[usize]) -> f64 {
    let n = w.tiles.len();
    let passable = |i: usize| TERRAIN[w.tiles[i].terrain].land && passable(w, i);
    let land: Vec<usize> = if tiles.is_empty() {
        (0..n).filter(|&i| TERRAIN[w.tiles[i].terrain].land).collect()
    } else {
        let mut seen = vec![false; n];
        let mut todo: Vec<usize> = tiles.to_vec();
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
        for &t in &c.territory {
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

/// One site the top-up takes: a clan's home, an existing market, or a tile to raise one on.
enum Pick {
    Clan(u32),
    Market(u32),
    Tile(usize),
}

/// The sites that bring the centre clans `centres` up to `players` within `region`, in order:
/// other clans' homes, biggest first; then existing markets, best first; then the most tended
/// land; then any land with grain. Every site is `centre_spacing` steps from those taken, on
/// passable land that isn't a sealed valley.
fn plan(w: &World, centres: &[u32], label: &[usize], region: usize) -> Vec<Pick> {
    let mut taken: Vec<usize> = homes(w, centres);
    let mut picks = Vec::new();
    let fits = |taken: &mut Vec<usize>, i: usize| {
        let ok = label[i] != usize::MAX
            && label[i] == region
            && w.tiles[i].country != Country::Sealed
            && taken.iter().all(|&k| steps(w, k, i) >= PREGAME.centre_spacing);
        if ok {
            taken.push(i);
        }
        ok
    };

    let mut clans: Vec<&Clan> = w.clans.values().filter(|c| !centres.contains(&c.id)).collect();
    clans.sort_by(|a, b| b.people.total_cmp(&a.people).then(a.id.cmp(&b.id)));
    for c in clans {
        if centres.len() + picks.len() == PREGAME.players {
            return picks;
        }
        if fits(&mut taken, c.home) {
            picks.push(Pick::Clan(c.id));
        }
    }

    let mut markets: Vec<&Market> = w.markets.values().collect();
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

/// The sites that bring the centre clans `centres` up to `players`, at the turn limit, from
/// `plan`. The sites are in the first centre's region of passable land. With no centres, the region of
/// the most populated clan's home comes first, then the others by their people: the first that
/// can hold all the sites wins, else the one that holds most. It ends short only if no region
/// holds `players` sites, `centre_spacing` apart.
fn top_up(w: &World, centres: &[u32]) -> Vec<Pick> {
    let label = regions(w);
    let order: Vec<usize> = match centres.first() {
        Some(id) => vec![label[w.clans[id].home]],
        None => {
            let count = label.iter().filter(|&&r| r != usize::MAX).max().map_or(0, |&r| r + 1);
            let (mut people, mut size) = (vec![0.0; count], vec![0usize; count]);
            for &r in label.iter().filter(|&&r| r != usize::MAX) {
                size[r] += 1;
            }
            for c in w.clans.values().filter(|c| label[c.home] != usize::MAX) {
                people[label[c.home]] += c.people;
            }
            let lead = w
                .clans
                .values()
                .max_by(|a, b| a.people.total_cmp(&b.people).then(b.id.cmp(&a.id)))
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
    best
}

/// The market on `tile`, raising one if there is none.
fn site(w: &mut World, tile: usize) -> u32 {
    match w.markets.values().find(|m| m.tile == tile) {
        Some(m) => m.id,
        None => raise(w, tile),
    }
}

/// Whether people have levelled off: counted every turn, they changed by under `settled` over
/// the last `settle_turns` turns. Records this turn's count.
fn settled(w: &mut World) -> bool {
    let people: f64 = w.clans.values().map(|c| c.people).sum();
    w.people_seen.push(people);
    if w.people_seen.len() > PREGAME.settle_turns + 1 {
        w.people_seen.remove(0);
    }
    let first = w.people_seen[0];
    w.people_seen.len() > PREGAME.settle_turns && first > 0.0 && ((people - first) / first).abs() < PREGAME.settled
}

/// The `end` step: the pre-game ends with `players` centres, the land covered and people
/// levelled off (`settled`), or at
/// `max_turns`, topping up to `players` centres if it got there with fewer. Only the first end
/// is kept; turns may run on after it.
pub fn check_end(w: &mut World) {
    if w.end.is_some() {
        return;
    }
    let centres = centres(w);
    for &id in &centres {
        if !w.centres_seen.contains(&id) {
            let home = w.clans[&id].home;
            w.emit(Event::new(EventKind::CentreFormed, home).clan(id));
            w.note(id, "became a centre of the pre-game".to_string());
        }
    }
    // A clan that drops out of the greedy set and back in is not announced twice.
    for &id in &centres {
        if !w.centres_seen.contains(&id) {
            w.centres_seen.push(id);
        }
    }
    let settled = settled(w);
    let by_centres = settled && centres.len() == PREGAME.players && land_cover(w, &homes(w, &centres)) >= PREGAME.cover;
    if !by_centres && w.turn + 1 < PREGAME.max_turns {
        return;
    }
    let picks = if !by_centres && centres.len() < PREGAME.players { top_up(w, &centres) } else { Vec::new() };
    // Each site gets a market where it has none; a centre clan's festival stays where it is.
    let mut sites: Vec<(u32, Option<u32>)> = Vec::new();
    for id in centres {
        let home = w.clans[&id].home;
        sites.push((site(w, home), Some(id)));
    }
    let mut topped_up = Vec::new();
    for pick in picks {
        let (mid, clan) = match pick {
            Pick::Clan(id) => (site(w, w.clans[&id].home), Some(id)),
            Pick::Market(mid) => (mid, None),
            Pick::Tile(tile) => (raise(w, tile), None),
        };
        // A clan that was a centre before and dropped out of the greedy set was announced then.
        if clan.is_none_or(|id| !w.centres_seen.contains(&id)) {
            let tile = w.markets[&mid].tile;
            w.emit(Event::new(EventKind::CentreFormed, tile).clan(clan).market(mid));
            w.note(mid, "became a centre of the pre-game, topped up".to_string());
            if let Some(id) = clan {
                w.note(id, "became a centre of the pre-game".to_string());
            }
            w.centres_seen.extend(clan);
        }
        sites.push((mid, clan));
        topped_up.push(mid);
    }
    let (centres, clans): (Vec<u32>, Vec<Option<u32>>) = sites.into_iter().unzip();
    let cover = land_cover(w, &self::sites(w, &centres));
    let first = centres.first().map_or(0, |id| w.markets[id].tile);
    w.end = Some(End { turn: w.turn, centres, clans, cover, by_limit: !by_centres, topped_up });
    w.emit(Event::new(EventKind::PregameEnded, first));
    civs::found(w);
}
