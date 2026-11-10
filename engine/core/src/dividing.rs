//! Dividing: a big clan divides its land into two clans, and a clan with no land to divide, or a
//! starving one, sends a daughter to a site it can settle.

use crate::actors;
use crate::clans::{self, new_clan, pick_site, site_value, sites};
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::homes;
use crate::land::steps;
use crate::world::World;

/// Splits `tiles` in two halves grown from its two farthest tiles: each half in turn takes the
/// unassigned tile next to it that's nearest its seed; leftovers go to the half with the nearer
/// seed. Ties go to the lower tile.
fn halve(w: &World, tiles: &[usize]) -> [Vec<usize>; 2] {
    let mut seeds = [tiles[0], tiles[0]];
    for &a in tiles {
        for &b in tiles {
            if steps(w, a, b) > steps(w, seeds[0], seeds[1]) {
                seeds = [a, b];
            }
        }
    }
    let mut parts = [vec![seeds[0]], vec![seeds[1]]];
    let mut left: Vec<usize> = tiles.iter().copied().filter(|t| !seeds.contains(t)).collect();
    left.sort_unstable();
    loop {
        let mut grew = false;
        for (p, part) in parts.iter_mut().enumerate() {
            let next = left
                .iter()
                .copied()
                .filter(|&k| part.iter().any(|&q| steps(w, q, k) == 1))
                .min_by_key(|&k| (steps(w, k, seeds[p]), k));
            if let Some(k) = next {
                part.push(k);
                left.retain(|&x| x != k);
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }
    for k in left {
        let p = usize::from(steps(w, k, seeds[1]) < steps(w, k, seeds[0]));
        parts[p].push(k);
    }
    for part in &mut parts {
        part.sort_unstable();
    }
    parts
}

/// A big clan divides like a cell: the daughter takes the half of the land it holds away from home,
/// with half the people and half the store, and settles its middle. Each half keeps only the
/// tiles connected to its home; the rest are freed.
pub fn divide_land(w: &mut World, id: u32) {
    let c = w.clans[&id].clone();
    // A home another clan holds is not land to divide: the halves are of the land it holds, and the
    // mother keeps the half nearer home, with the home.
    let held = homes::gathers(w, id, c.home);
    let land: Vec<usize> = c.territory.iter().copied().filter(|&t| held || t != c.home).collect();
    let [a, b] = halve(w, &land);
    let near = |part: &[usize]| part.iter().map(|&t| (steps(w, c.home, t), t)).min();
    let (mut keep, give) = if a.contains(&c.home) || (!held && near(&a) <= near(&b)) { (a, b) } else { (b, a) };
    if !held {
        keep.push(c.home);
        keep.sort_unstable();
    }
    // No preference among ties but the lowest tile.
    let home = actors::middle(w, &give, usize::MAX);
    let mine = actors::trim(w, c.home, &keep, CLAN.territory_radius);
    let hers = actors::trim(w, home, &give, CLAN.territory_radius);
    clans::free(w, id, &c.territory);

    let did = w.next_id;
    w.next_id += 1;
    homes::claim_land(w, id, &mine);
    homes::claim_land(w, did, &hers);
    let mut d = new_clan(did, home, c.people / 2.0);
    d.store = c.store.map(|v| v / 2.0);
    d.since = c.since;
    d.fed_ratio = c.fed_ratio;
    d.civ = c.civ;
    d.leaning = c.leaning.clone();
    d.parent = Some(id);
    d.fields = c.fields;
    let took = hers.len();
    d.territory = hers;
    let m = w.clans.get_mut(&id).unwrap();
    m.territory = mine;
    m.people -= d.people;
    m.store = std::array::from_fn(|f| m.store[f] - d.store[f]);
    w.clans.insert(did, d);
    w.emit(Event::new(EventKind::ClanDivided, home).clan(id).other(did));
    w.note(
        id,
        format!(
            "divided at {:.0} people (it splits at {:.0}); clan {did} took the far {took} tiles",
            c.people, CLAN.split_at
        ),
    );
    w.note(did, format!("born when clan {id} divided its land"));
}

/// A clan with no land to divide, or a starving one, sends a daughter of `min(people, split_at) /
/// 2` people to a site within `scout_range` that would feed her at least `settle_fed`.
pub fn split(w: &mut World, id: u32) {
    let c = w.clans[&id].clone();
    let people = c.people.min(CLAN.split_at) / 2.0;
    let share = people / c.people;
    let mut d = new_clan(w.next_id, c.home, people);
    d.civ = c.civ;
    d.leaning = c.leaning.clone();
    d.parent = Some(id);
    d.store = c.store.map(|v| v * share);
    d.fields = c.fields;
    let scored: Vec<(usize, f64)> =
        sites(w, &c, CLAN.scout_range).into_iter().map(|s| (s, site_value(w, d.id, s, people, &d.store))).collect();
    let Some(site) = pick_site(w, &scored, CLAN.settle_fed) else { return };
    w.next_id += 1;
    d.home = site;
    d.territory = vec![site];
    homes::claim_land(w, d.id, &[site]);
    let m = w.clans.get_mut(&id).unwrap();
    m.people -= people;
    m.store = std::array::from_fn(|f| m.store[f] - d.store[f]);
    m.hungry = 0;
    w.emit(Event::new(EventKind::ClanDivided, site).clan(id).other(d.id));
    let why = if c.starving { "it is starving" } else { "it has no land to divide" };
    let at = w.at(site);
    w.note(id, format!("sent clan {} to {at} with {people:.0} people: {why}", d.id));
    w.note(d.id, format!("set out from clan {id} with {people:.0} people"));
    w.clans.insert(d.id, d);
}
