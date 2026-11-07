//! The scout's orders: `Move` takes a scout as it takes a camp, `Route` and `Move` replace each
//! other, the search's view (`search_view`) says what it leaves and why, and a line's reach is read
//! from any tile. Each test is on a stage: the retro map's land painted plain where the test needs
//! it, the capital and the rays out of it found on the map.

mod common;

use common::{led_order, led_scout};
use engine::data::*;
use engine::hex::{Hex, DIRS};
use engine::land::{neighbours, steps};
use engine::map::index;
use engine::stage::Stage;
use engine::stats::digest;
use engine::units::{self, search_view};
use engine::{EventKind, Order, World};

/// The real map of seed 2 (the small retro map), led by its first civ.
fn lead() -> Stage {
    let mut s = Stage::retro("small", 2).unwrap();
    let first = s.world().civs.keys().next().copied().expect("a civ");
    s.lead(first).unwrap();
    s
}

/// Ends a turn with the led civ's store full and the scout's pack set to one of each good, so the
/// scout's feeding is not what these tests are about.
fn end_fed(w: &mut World) {
    let me = w.player.unwrap();
    w.civs.get_mut(&me).unwrap().store = [100.0; N];
    let id = led_scout(w);
    w.units.get_mut(&id).unwrap().store = [1.0; N];
    w.act(engine::Action::EndTurn).unwrap();
}

/// The `n` tiles from `from` straight out in the first direction that stays on land and the civ's
/// own, in order; `from` is not among them.
fn open_ray(w: &World, from: usize, n: i32) -> Vec<usize> {
    let h = w.tiles[from].hex();
    for d in DIRS {
        let tiles: Vec<usize> =
            (1..=n).filter_map(|k| index(w.config.radius, Hex { q: h.q + d.0 * k, r: h.r + d.1 * k })).collect();
        let mine = |i: usize| engine::civs::tile_civ(w, i).is_none_or(|c| Some(c) == w.player);
        if tiles.len() == n as usize && tiles.iter().all(|&i| w.tiles[i].terrain != SEA && mine(i)) {
            return tiles;
        }
    }
    panic!("no open ray from {from}");
}

/// The capital's tile.
fn capital(w: &World) -> usize {
    w.civs[&w.player.unwrap()].capital
}

/// The scout's tile, read from its unit.
fn home(w: &World, id: u32) -> usize {
    w.units[&id].home
}

#[test]
fn a_scout_moved_four_steps_out_walks_there_and_stays() {
    let mut s = lead();
    s.plain();
    let cap = capital(s.world());
    let target = open_ray(s.world(), cap, 4)[3];
    let mut w = s.into_world();
    let id = led_scout(&w);
    w.act(led_order(&w, Order::Move { unit: id, to: target })).unwrap();
    assert_eq!(w.units[&id].dest(), Some(target), "the move sets the scout's destination");
    let mut turns = 0;
    while home(&w, id) != target {
        turns += 1;
        assert!(turns <= 20, "the scout has not walked four steps in 20 turns");
        end_fed(&mut w);
    }
    for _ in 0..5 {
        end_fed(&mut w);
        assert_eq!(home(&w, id), target, "the scout stays where it arrived");
    }
    let u = &w.units[&id];
    assert!(u.scout().unwrap().route.is_none(), "a moved scout has no route");
    assert_eq!(u.dest(), Some(target), "the destination stays its errand once it arrives");
    assert!(w.notes[&id].iter().any(|n| n.text.starts_with("at ") && n.text.ends_with("foraging")));
}

#[test]
fn a_route_clears_a_move_and_a_move_clears_a_route() {
    let mut s = lead();
    s.plain();
    let cap = capital(s.world());
    let ray = open_ray(s.world(), cap, 4);
    let mut w = s.into_world();
    let id = led_scout(&w);
    w.act(led_order(&w, Order::Move { unit: id, to: ray[3] })).unwrap();
    w.act(led_order(&w, Order::Route { unit: id, from: cap, to: ray[1] })).unwrap();
    let u = &w.units[&id];
    assert_eq!(u.dest(), None, "a route clears the destination");
    assert_eq!(u.scout().unwrap().route, Some((cap, ray[1])));
    w.act(led_order(&w, Order::Move { unit: id, to: ray[3] })).unwrap();
    let u = &w.units[&id];
    assert_eq!(u.scout().unwrap().route, None, "a move clears the route");
    assert_eq!(u.dest(), Some(ray[3]));
}

#[test]
fn a_scout_move_to_the_sea_is_refused_with_its_own_message() {
    let w = lead().into_world();
    let id = led_scout(&w);
    let sea = (0..w.tiles.len()).find(|&i| w.tiles[i].terrain == SEA).expect("the map has sea");
    let err = w.check_order(w.player.unwrap(), &Order::Move { unit: id, to: sea }).unwrap_err();
    assert_eq!(err, "a scout can only go over land");
}

#[test]
fn search_view_names_the_mountain_a_range_leaves_and_what_it_costs_through() {
    let mut s = lead();
    s.plain();
    let me = s.world().player.unwrap();
    let cap = capital(s.world());
    let line = open_ray(s.world(), cap, 4);
    let target = line[3];
    // The known route goes round the mountain on the way (`b`, span one) over two hills (`p`, `q`),
    // and a mountain beside the detour (`off`, span one) is left: its cost through is longer than the
    // known route's.
    let (a, b, c) = (line[0], line[1], line[2]);
    let w0 = s.world();
    let nb = |t: usize| neighbours(w0, t).collect::<Vec<usize>>();
    let p = nb(a).into_iter().find(|&p| p != cap && p != b && p != c && !line.contains(&p)).expect("a detour tile");
    let q = nb(c)
        .into_iter()
        .find(|&q| nb(p).contains(&q) && q != a && q != b && q != target && !line.contains(&q))
        .expect("a detour tile beside the far end");
    let off = nb(q)
        .into_iter()
        .find(|&y| y != p && y != a && y != b && y != c && y != target && y != cap && y != q)
        .expect("a tile beside the detour");
    s.paint(b, MOUNTAIN, Some(MAP.mountain_cost), Some(1));
    s.paint(off, MOUNTAIN, Some(MAP.mountain_cost), Some(1));
    for t in [p, q] {
        s.paint(t, HILLS, None, None);
    }
    let mut w = s.into_world();
    let id = led_scout(&w);
    w.act(led_order(&w, Order::Route { unit: id, from: cap, to: target })).unwrap();
    let view = search_view(&w, id).expect("a scout with a route has a search");
    let known = view.known.expect("the known route is finite");
    let through = |m: usize| {
        let a = engine::units::route_plan(&w, me, cap, m).possible?.1;
        let b = engine::units::route_plan(&w, me, m, target).possible?.1;
        Some(a + b)
    };
    let &(left_tile, left_cost) =
        view.left.iter().find(|&&(m, _)| m == off).expect("the mountain beside the detour is left");
    assert_eq!(left_tile, off);
    let expect = through(off).unwrap();
    assert!((left_cost - expect).abs() < 1e-9, "left costs {left_cost}, its cost through is {expect}");
    assert!(left_cost > known, "a mountain left costs more through than the known route ({known})");
    assert!(view.left.len() <= 10);
}

/// On each retro map to the rival, the errand the search order is judged on (the maps whose search
/// has open mountains; the pre-game's picks move with every rule, so none is named): the search's `next` holds the open
/// mountains (cost through below the known route's) in best-case order, and every mountain `left`
/// costs at least the known route through it. The numbers are what `relay/specs/153/search.md` shows.
#[test]
fn search_view_to_the_rival_orders_its_open_mountains_by_best_case() {
    let mut tried = 0;
    for (size, seeds) in engine::stage::RETRO {
        for &seed in seeds {
            tried += search_view_on(size, seed) as usize;
        }
    }
    assert!(tried > 0, "some retro map's search has open mountains to try");
}

fn search_view_on(size: &str, seed: u32) -> bool {
    let mut s = Stage::retro(size, seed).unwrap();
    let me = *s.world().civs.keys().next().unwrap();
    s.lead(me).unwrap();
    let w0 = s.world();
    let cap = w0.civs[&me].capital;
    let to = w0.civs.values().filter(|c| c.id != me).min_by_key(|c| (steps(w0, cap, c.capital), c.id)).unwrap().capital;
    let scout = units::scout(w0, me).unwrap().id;
    let mut w = s.into_world();
    w.act(led_order(&w, Order::Route { unit: scout, from: cap, to })).unwrap();
    let view = search_view(&w, scout).expect("a route has a search");
    let known = view.known.expect("the known route is finite");
    let through = |m: usize| {
        let a = engine::units::route_plan(&w, me, cap, m).possible?.1;
        let b = engine::units::route_plan(&w, me, m, to).possible?.1;
        Some(a + b)
    };
    let home_tile = w.units[&scout].home;
    let best = |m: usize| {
        let walk = if steps(&w, home_tile, m) <= 1 {
            0.0
        } else {
            neighbours(&w, m)
                .filter_map(|n| engine::units::route_plan(&w, me, home_tile, n).known.map(|k| k.1))
                .fold(f64::INFINITY, f64::min)
        };
        through(m).unwrap() + walk
    };
    let open: Vec<usize> = view.next.iter().copied().filter(|&m| w.tiles[m].terrain == MOUNTAIN).collect();
    if open.is_empty() {
        return false;
    }
    assert!(open.iter().all(|&m| through(m).unwrap() < known - 1e-9), "every mountain tried is open");
    let bests: Vec<f64> = open.iter().map(|&m| best(m)).collect();
    assert!(bests.windows(2).all(|p| p[0] <= p[1] + 1e-9), "mountains in best-case order: {bests:?}");
    assert!(view.left.iter().all(|&(m, x)| x >= known - 1e-9 && (x - through(m).unwrap()).abs() < 1e-9));
    true
}

#[test]
fn past_reach_is_false_at_the_capital_and_true_eight_steps_out_and_the_move_says_so() {
    let mut s = lead();
    s.plain();
    let cap = capital(s.world());
    let far = open_ray(s.world(), cap, 8)[7];
    let mut w = s.into_world();
    let id = led_scout(&w);
    assert!(!units::past_reach(&w, id, cap), "the capital's line carries without limit");
    assert!(units::past_reach(&w, id, far), "eight steps out, the line carries less than the scout eats");
    w.act(led_order(&w, Order::Move { unit: id, to: far })).unwrap();
    assert!(
        w.notes[&id].iter().any(|n| n.text.starts_with("past its line's reach: its line there would carry")),
        "the move notes the reach: {:?}",
        w.notes[&id].iter().map(|n| &n.text).collect::<Vec<_>>()
    );
    let (cost, carry) = units::line_from(&w, id, far).expect("a line reaches the capital from eight steps out");
    assert!(cost > 0.0 && carry < units::pack_cap(1.0) * 1000.0, "a finite cost and carry");
}

#[test]
fn a_scout_move_replays_to_the_same_world() {
    let mut w = lead().into_world();
    let cap = capital(&w);
    let target = open_ray(&w, cap, 4)[3];
    let id = led_scout(&w);
    w.act(led_order(&w, Order::Move { unit: id, to: target })).unwrap();
    w.run(10);
    if !w.units.contains_key(&id) {
        assert!(w.events.iter().any(|e| e.kind == EventKind::ScoutStarved), "the scout starved");
    }
    let again = World::replay(&w.save()).unwrap();
    assert_eq!(digest(&again), digest(&w));
}
