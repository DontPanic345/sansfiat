mod common;

use std::collections::BTreeMap;

use common::*;
use engine::clans::{adjust, value, Clan};
use engine::data::*;
use engine::round::{cost, insertion, middle, territory};
use engine::{Config, World, TURN};

const EPS: f64 = 1e-9;

#[test]
fn a_tile_goes_in_where_it_lengthens_the_loop_least() {
    let mut w = flat();
    let o = at(0, 0);
    let ring = [out(0, 1), out(1, 1), out(2, 1), out(3, 1)];
    // Next to ring[1] and ring[2], and dearer to enter than plains.
    let t = at(1, -2);
    assert!(steps(&w, t, ring[1]) == 1 && steps(&w, t, ring[2]) == 1);
    w.tiles[t].terrain = FOREST;
    let round = [o, ring[0], ring[1], ring[2], ring[3]];
    assert_eq!(insertion(&w, &round, t), Some((3, 2.0)));

    // A one-tile round goes there and back: 2 in, 1 home.
    let n = out(0, 1);
    w.tiles[n].terrain = FOREST;
    assert_eq!(insertion(&w, &[o], n), Some((1, 3.0)));
}

/// A hungry clan of 5 walking the ring around home, with grain at home.
fn ring_clan() -> (World, u32, usize) {
    let mut w = flat();
    let o = at(0, 0);
    w.tiles[o].stock[GRAIN] = 20.0;
    let round: Vec<usize> = std::iter::once(o).chain((0..6).map(|d| out(d, 1))).collect();
    let id = put(&mut w, &round, 5.0);
    w.clans.get_mut(&id).unwrap().fed = false;
    // Next to the ring's 2nd and 3rd tiles.
    (w, id, at(1, -2))
}

#[test]
fn a_tile_is_added_only_if_its_food_is_worth_the_walk() {
    let (mut w, id, t) = ring_clan();
    let before = w.clans[&id].round.clone();
    adjust(&mut w);
    assert_eq!(w.clans[&id].round, before, "added a tile with no food");

    let (mut w, id, t2) = ring_clan();
    assert_eq!(t, t2);
    w.tiles[t].stock[FRUIT] = 20.0;
    let c = w.clans[&id].clone();
    let now = value(&w, &c, &c.round);
    adjust(&mut w);
    let c = &w.clans[&id];
    assert!(c.round.contains(&t), "{:?}", c.round);
    assert_eq!(c.round.len(), before.len() + 1);
    assert!(value(&w, c, &c.round) > now + CLAN.land_margin);
    check_rounds(&w);
}

#[test]
fn a_fed_clan_drops_land_it_cannot_use() {
    let mut w = flat();
    let (o, n) = (at(0, 0), out(0, 1));
    w.tiles[o].stock[GRAIN] = 20.0;
    w.tiles[o].stock[GAME] = 20.0;
    w.tiles[n].terrain = FOREST;
    let id = put(&mut w, &[o, n], 5.0);
    let before = cost(&w, &w.clans[&id].round);
    adjust(&mut w);
    let c = &w.clans[&id];
    assert_eq!(c.round, vec![o]);
    assert!(cost(&w, &c.round) < before);
    assert_eq!(w.tiles[n].claim, None);
}

#[test]
fn a_starving_clan_swaps_its_worst_tile() {
    // An island: home with grain, a fruit tile beside it, and a bare tile 2 out.
    let (o, n0, x, n1) = (at(0, 0), out(0, 1), out(0, 2), out(1, 1));
    let mut w = island(&[o, n0, x, n1]);
    w.tiles[o].stock[GRAIN] = 20.0;
    w.tiles[n1].stock[FRUIT] = 20.0;
    let id = put(&mut w, &[o, n0, x], 2.0);
    let c = w.clans.get_mut(&id).unwrap();
    c.fed = false;
    c.starving = true;
    let c = c.clone();
    // Adding the fruit tile alone costs more walking than it gives.
    let added = [o, n1, n0, x];
    assert!(value(&w, &c, &added) <= value(&w, &c, &c.round) + CLAN.land_margin);
    adjust(&mut w);
    let mut got = w.clans[&id].round.clone();
    got.sort_unstable();
    let mut want = vec![o, n0, n1];
    want.sort_unstable();
    assert_eq!(got, want, "one tile in, one out");
    assert_eq!(w.tiles[x].claim, None);
    check_rounds(&w);
}

#[test]
fn home_follows_the_middle_a_tile_a_turn() {
    let mut w = flat();
    let o = at(0, 0);
    // A round off to one side of home, with no food: nothing to add, swap or drop.
    let round = [o, out(0, 1), out(0, 2), at(2, -1), at(1, -1)];
    let id = put(&mut w, &round, 5.0);
    w.clans.get_mut(&id).unwrap().fed = false;
    let mut moved = 0;
    for _ in 0..4 {
        let before = w.clans[&id].clone();
        let m = middle(&w, &territory(&w, &before.round), before.home);
        adjust(&mut w);
        let c = &w.clans[&id];
        assert_eq!(c.round[0], c.home);
        assert!(steps(&w, before.home, c.home) <= 1);
        if c.home != before.home {
            moved += 1;
            assert!(steps(&w, c.home, m) < steps(&w, before.home, m));
            let (mut a, mut b) = (before.round.clone(), c.round.clone());
            a.sort_unstable();
            b.sort_unstable();
            assert_eq!(a, b);
        }
        check_rounds(&w);
    }
    assert!(moved >= 1);
    let c = &w.clans[&id];
    assert_eq!(middle(&w, &territory(&w, &c.round), c.home), c.home);
}

/// The round tiles gained and lost between two rounds.
fn changes(a: &[usize], b: &[usize]) -> (usize, usize) {
    (b.iter().filter(|t| !a.contains(t)).count(), a.iter().filter(|t| !b.contains(t)).count())
}

#[test]
fn rounds_keep_their_invariants() {
    for seed in 1..=3 {
        let mut w = World::new(Config { seed, radius: 16 });
        for turn in 0..300 {
            for step in TURN {
                let before: BTreeMap<u32, Clan> = w.clans.clone();
                (step.run)(&mut w);
                check_rounds(&w);
                for (id, c) in &w.clans {
                    let Some(b) = before.get(id) else { continue };
                    let (gained, lost) = changes(&b.round, &c.round);
                    let at = format!("seed {seed} turn {turn} {}: clan {id} {:?} -> {:?}", step.name, b.round, c.round);
                    match step.name {
                        "move" => {
                            let moved_on = b.hungry >= CLAN.patience && c.round == [c.home] && c.since == 0;
                            if !moved_on {
                                assert!(gained <= 1, "{at}");
                                assert!(steps(&w, b.home, c.home) <= 1, "{at}");
                                // Only a home that moved makes the limit drop tiles.
                                if c.home == b.home {
                                    assert!(lost <= 1, "{at}");
                                }
                            }
                        }
                        // Dividing only gives tiles away.
                        "eat" => assert_eq!(gained, 0, "{at}"),
                        _ => assert_eq!(b.round, c.round, "{at}"),
                    }
                }
            }
        }
        assert!(w.clans.values().all(|c| c.pop > 0.0 && c.store.iter().all(|&s| s >= -EPS)));
    }
}
