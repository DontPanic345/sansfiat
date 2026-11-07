//! The behaviour every actor shares: spoiling, eating from a store and walking a path.

mod common;

use common::*;
use engine::actors::{stride, Actor};
use engine::data::*;
use engine::land::step_cost;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn spoil_removes_the_spoilage_share_of_every_food() {
    let mut a = Actor::new(0, 0.0);
    a.store = [10.0, 0.0, 2.5, 4.0, 1.0];
    a.spoil();
    for (got, held) in a.store.iter().zip([10.0, 0.0, 2.5, 4.0, 1.0]) {
        assert!(close(*got, held * (1.0 - CLAN.spoilage)), "{got} of {held}");
    }
}

#[test]
fn eat_by_share_takes_in_proportion_and_never_more_than_is_held() {
    let mut a = Actor::new(0, 0.0);
    a.store = [3.0, 1.0, 0.0, 0.0, 0.0];
    // 2 of 4 held: half of each good.
    let eaten = a.eat_by_share(2.0);
    assert!(close(eaten[GAME], 1.5) && close(eaten[GRAIN], 0.5), "{eaten:?}");
    assert!(close(a.store[GAME], 1.5) && close(a.store[GRAIN], 0.5), "{:?}", a.store);

    // Asked for more than it holds: it eats all of it, and nothing is left.
    let eaten = a.eat_by_share(10.0);
    assert!(close(eaten[GAME], 1.5) && close(eaten[GRAIN], 0.5), "{eaten:?}");
    assert_eq!(a.store, [0.0; N]);

    // An empty store eats nothing.
    assert_eq!(a.eat_by_share(1.0), [0.0; N]);
}

#[test]
fn eat_by_diet_equals_the_clans_old_rule_on_a_hand_worked_case() {
    // Need 6.5. Each food gives up to 0.4 × 6.5 = 2.6 of itself: game 2.6, grain 1, fruit 0, fish
    // 2, shellfish 0, so 5.6 and 0.9 short. The game left over (6 − 2.6 = 3.4) is the excess, worth
    // 0.3 a unit: 0.9 ÷ 0.3 = 3 units of it are eaten, which leaves 0.4 of game in the store. The
    // people get 5.6 + 3 × 0.3 = 6.5, all of their need. Then the spoilage is applied by the clan.
    let mut a = Actor::new(0, 0.0);
    a.store = [6.0, 1.0, 0.0, 2.0, 0.0];
    let got = a.eat_by_diet(6.5);
    assert!(close(got, 6.5), "{got}");
    assert!(close(a.store[GAME], 0.4), "{:?}", a.store);
    assert!(a.store[1..].iter().all(|&v| v == 0.0), "{:?}", a.store);
}

#[test]
fn eat_by_diet_eats_the_excess_only_as_far_as_the_need_goes() {
    // Need 1. Grain gives 0.4 of itself, leaving 2.6 of excess; the 0.6 still short is 2 units of
    // excess at 0.3 each, so 2 of the 2.6 are eaten and 0.6 stays in the store. Total got: 0.4 + 0.6.
    let mut a = Actor::new(0, 0.0);
    a.store = [0.0, 3.0, 0.0, 0.0, 0.0];
    let got = a.eat_by_diet(1.0);
    assert!(close(got, 1.0), "{got}");
    assert!(close(a.store[GRAIN], 0.6), "{:?}", a.store);
}

/// Two steps from `at(0, 0)` on the flat world, and the cost of the first (every plain costs alike).
fn two_steps() -> (engine::World, usize, [usize; 2], f64) {
    let w = flat();
    let from = at(0, 0);
    let path = [at(1, 0), at(2, 0)];
    let c = step_cost(&w, from, path[0]).expect("a step");
    assert!(close(c, step_cost(&w, path[0], path[1]).unwrap()));
    (w, from, path, c)
}

#[test]
fn stride_stops_at_the_first_step_it_cannot_pay() {
    let (w, from, path, c) = two_steps();
    // Enough for one step and a half: the second costs a whole step, so it is not taken.
    let (taken, left) = stride(&w, from, &path, 1.5 * c, 0.0);
    assert_eq!(taken, 1);
    assert!(close(left, 0.5 * c), "{left}");
    // Exactly enough for both: both are taken, and nothing is left.
    let (taken, left) = stride(&w, from, &path, 2.0 * c, 0.0);
    assert_eq!(taken, 2);
    assert!(close(left, 0.0), "{left}");
    // Not enough for the first: no step, all the labour left.
    let (taken, left) = stride(&w, from, &path, 0.5 * c, 0.0);
    assert_eq!((taken, left), (0, 0.5 * c));
}

#[test]
fn stride_honours_slack() {
    let (w, from, path, c) = two_steps();
    // A quarter of a step short, with half a step of slack: the step is taken and the labour goes
    // negative by the shortfall, as the scout's march always let it.
    let (taken, left) = stride(&w, from, &path, c - 0.25 * c, 0.5 * c);
    assert_eq!(taken, 1);
    assert!(close(left, -0.25 * c), "{left}");
    // The same labour with no slack takes nothing.
    assert_eq!(stride(&w, from, &path, c - 0.25 * c, 0.0).0, 0);
}

#[test]
fn a_path_of_no_steps_takes_none() {
    let (w, from, _, _) = two_steps();
    assert_eq!(stride(&w, from, &[], 5.0, 0.0), (0, 5.0));
    assert_eq!(stride(&w, from, &[], 0.0, 0.0), (0, 0.0));
}

#[test]
fn one_job_of_share_one_is_divide_bit_for_bit() {
    use engine::actors::{divide, split, Route};
    let routes = [Route { to: 3, path: vec![1, 2, 3], cost: 0.37 }, Route { to: 5, path: vec![4, 5], cost: 0.11 }];
    let cases: [(f64, &[Route], Vec<f64>); 4] = [
        (2.0, &[], vec![0.1, 0.2, 0.3]),
        (1.7, &routes, vec![0.05, 0.4, 0.0, 0.31]),
        (0.0, &[], vec![0.5]),
        (3.3, &routes[..1], vec![]),
    ];
    for (labour, routes, walks) in cases {
        let one = split(labour, routes, &[1.0], std::slice::from_ref(&walks));
        let old = divide(labour, routes, &walks);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].len(), old.len());
        for (a, b) in one[0].iter().zip(&old) {
            assert_eq!(a.to_bits(), b.to_bits(), "{a} vs {b}");
        }
    }
}

#[test]
fn two_jobs_each_get_their_share_less_their_walks() {
    use engine::actors::split;
    // Labour 1.0, no routes: job A (share 0.25) has two tiles, B (0.75) has one.
    let out = split(1.0, &[], &[0.25, 0.75], &[vec![0.1, 0.0], vec![0.2]]);
    assert!(close(out[0][0], 0.125 - 0.1) && close(out[0][1], 0.125), "{:?}", out[0]);
    assert!(close(out[1][0], 0.75 - 0.2), "{:?}", out[1]);
}

#[test]
fn routes_are_paid_once_not_per_job() {
    use engine::actors::{split, Route};
    let routes = [Route { to: 2, path: vec![1, 2], cost: 0.4 }];
    let out = split(1.0, &routes, &[0.5, 0.5], &[vec![0.0], vec![0.0]]);
    // What is left after the route is 0.6, shared: 0.3 each. Paying the route twice would leave 0.2.
    assert!(close(out[0][0], 0.3) && close(out[1][0], 0.3), "{out:?}");
}

#[test]
fn a_job_with_no_tiles_gets_nothing_and_its_share_is_not_handed_on() {
    use engine::actors::split;
    let out = split(1.0, &[], &[0.5, 0.5], &[vec![], vec![0.0]]);
    assert!(out[0].is_empty());
    // B keeps its own half, not the whole labour.
    assert!(close(out[1][0], 0.5), "{out:?}");
}

#[test]
fn meal_eats_the_store_then_kin_biggest_first_ties_to_the_lower_id() {
    use engine::actors::meal;
    let mut store = [1.0, 0.0, 0.0, 0.0, 0.0];
    let mut small = [0.5, 0.0, 0.0, 0.0, 0.0];
    let mut big = [0.0, 2.0, 0.0, 0.0, 0.0];
    let mut tie = [0.0, 0.0, 2.0, 0.0, 0.0];
    // Need 3: the store gives 1, then the biggest kin (`big`, 2) gives 2; `tie` and `small` stay.
    let got = meal(3.0, &mut store, [(7, &mut small), (4, &mut tie), (2, &mut big)]);
    assert!(close(got, 3.0), "{got}");
    assert_eq!(store, [0.0; N]);
    assert_eq!(big, [0.0; N]);
    assert_eq!(small, [0.5, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(tie, [0.0, 0.0, 2.0, 0.0, 0.0]);

    // Equal kin (2 each): the lower id gives first, and need 2.5 takes 2.5 of it.
    let mut none = [0.0; N];
    let mut a = [0.0, 2.0, 0.0, 0.0, 0.0];
    let mut b = [0.0, 0.0, 2.0, 0.0, 0.0];
    let got = meal(2.5, &mut none, [(9, &mut b), (3, &mut a)]);
    assert!(close(got, 2.5), "{got}");
    assert_eq!(a, [0.0; N]);
    assert_eq!(b, [0.0, 0.0, 1.5, 0.0, 0.0]);
}
