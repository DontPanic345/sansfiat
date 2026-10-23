mod common;

use common::*;
use engine::data::*;
use engine::pregame::{centres, check_end, is_centre, land_cover};
use engine::World;

/// Three markets on the corners of the land, each 10 steps from the others, with a hearth on
/// its tile, `traders` clans last turn, founded on turn 0; the turn is 40.
fn three(traders: [usize; 3]) -> (World, [u32; 3]) {
    let mut w = flat();
    w.turn = 40;
    let tiles = [at(5, 0), at(-5, 5), at(0, -5)];
    let mut ids = [0; 3];
    for k in 0..3 {
        hearth(&mut w, tiles[k], 4.0);
        ids[k] = open(&mut w, tiles[k]);
        w.markets.get_mut(&ids[k]).unwrap().traders = traders[k];
    }
    for a in tiles {
        for b in tiles {
            assert!(a == b || steps(&w, a, b) >= PREGAME.centre_spacing);
        }
    }
    (w, ids)
}

#[test]
fn three_centres_far_apart_are_all_taken_busiest_first() {
    let (w, ids) = three([4, 6, 5]);
    assert_eq!(centres(&w), vec![ids[1], ids[2], ids[0]]);
}

#[test]
fn a_centre_near_a_busier_one_is_not_taken() {
    let mut w = flat();
    w.turn = 40;
    let (a, b) = (at(5, 0), at(-1, 0));
    assert_eq!(steps(&w, a, b), 6);
    hearth(&mut w, a, 4.0);
    hearth(&mut w, b, 4.0);
    let (ma, mb) = (open(&mut w, a), open(&mut w, b));
    w.markets.get_mut(&ma).unwrap().traders = 4;
    w.markets.get_mut(&mb).unwrap().traders = 5;
    assert!(is_centre(&w, &w.markets[&ma]) && is_centre(&w, &w.markets[&mb]));
    assert_eq!(centres(&w), vec![mb]);
}

#[test]
fn a_centre_needs_age_traders_and_a_hearth_in_reach() {
    let (mut w, ids) = three([4, 4, 4]);
    let m = w.markets[&ids[0]].clone();
    assert!(is_centre(&w, &m));
    // A hearth anywhere in reach will do.
    w.tiles[m.tile].tame = 0;
    assert!(!is_centre(&w, &m), "no hearth in reach");
    hearth(&mut w, at(3, 0), 4.0);
    assert!(is_centre(&w, &m));
    w.tiles[at(3, 0)].tame = 0;
    hearth(&mut w, at(1, 0), 4.0);
    assert!(!is_centre(&w, &m), "a hearth 4 steps away, out of reach");

    let (mut w, ids) = three([4, 3, 4]);
    assert!(!is_centre(&w, &w.markets[&ids[1]]), "three traders");
    w.turn = PREGAME.centre_age - 1;
    assert!(!is_centre(&w, &w.markets[&ids[0]]), "too young");
    assert!(centres(&w).is_empty());
}

#[test]
fn land_cover_is_the_share_near_some_territory() {
    // A row of 10 tiles, and an island of 2 off it.
    let row: Vec<usize> = (-5..5).map(|q| at(q, 0)).collect();
    let isle = [at(0, 3), at(1, 3)];
    let mut w = island(&[row.clone(), isle.to_vec()].concat());
    // Its territory is q = −5 and −4; within 3 steps of that is q = −5..=−1.
    put(&mut w, &[row[0]], 3.0);
    assert!((land_cover(&w, &[]) - 5.0 / 12.0).abs() < 1e-12);
    let m = open(&mut w, row[9]);
    assert!((land_cover(&w, &[m]) - 0.5).abs() < 1e-12);
    let m2 = open(&mut w, isle[0]);
    assert_eq!(land_cover(&w, &[m2]), 0.0);
}

#[test]
fn the_pregame_ends_with_three_centres_and_the_land_covered() {
    let (mut w, ids) = three([4, 4, 4]);
    check_end(&mut w);
    assert_eq!(w.end, None, "nobody lives on the land");
    // A clan in the middle covers 4 steps out, short of the rim.
    put(&mut w, &[at(0, 0)], 3.0);
    check_end(&mut w);
    assert_eq!(w.end, None);
    assert!((land_cover(&w, &ids) - 61.0 / 91.0).abs() < 1e-12);
    // Six more, 4 out, cover it all.
    for d in 0..6 {
        put(&mut w, &[out(d, 4)], 3.0);
    }
    check_end(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!((end.turn, end.centres, end.cover, end.by_limit), (40, ids.to_vec(), 1.0, false));
    w.markets.clear();
    w.turn += 1;
    check_end(&mut w);
    assert_eq!(w.end.as_ref().unwrap().turn, 40, "the first end stays");
}

#[test]
fn the_pregame_ends_at_the_turn_limit() {
    let (mut w, _) = three([4, 4, 3]);
    put(&mut w, &[at(0, 0)], 3.0);
    w.turn = PREGAME.max_turns - 2;
    w.run(1);
    assert_eq!(w.end, None);
    w.run(1);
    let end = w.end.clone().expect("ended");
    assert_eq!((end.turn, end.by_limit), (PREGAME.max_turns - 1, true));
    assert_eq!(w.turn, PREGAME.max_turns);
}
