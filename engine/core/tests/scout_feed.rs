//! The scout's feeding on the stage: it takes only what a tile regrows and never fallows it, leaves its
//! errand for rich ground when it runs low and comes back fed, waits on rich ground when idle, and says
//! why it went hungry.

mod common;

use common::{led_order, led_scout};
use engine::data::*;
use engine::hex::{Hex, DIRS};
use engine::land::{spare, steps, yield_of};
use engine::map::index;
use engine::stage::Stage;
use engine::units;
use engine::{Action, Order, World};

/// Small map 2, led by its first civ, plain, with no food and no clans anywhere.
fn bare() -> Stage {
    let mut s = Stage::retro("small", 2).unwrap();
    let first = s.world().civs.keys().next().copied().expect("a civ");
    s.lead(first).unwrap();
    s.plain();
    let w = s.world_mut();
    w.clans.clear();
    for t in &mut w.tiles {
        t.cap = [0.0; N];
        t.unworn = [0.0; N];
        t.stock = [0.0; N];
    }
    s
}

fn capital(w: &World) -> usize {
    w.civs[&w.player.unwrap()].capital
}

/// The `n` land tiles straight out from `from`, none of them a rival's.
fn ray(w: &World, from: usize, n: i32) -> Vec<usize> {
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

fn scout(w: &World) -> &units::Unit {
    units::scout(w, w.player.unwrap()).unwrap()
}

/// Ends a turn with the civ's store empty: the scout lives off what it forages.
fn end_hungry(w: &mut World) {
    let me = w.player.unwrap();
    w.civs.get_mut(&me).unwrap().store = [0.0; N];
    w.act(Action::EndTurn).unwrap();
}

fn noted(w: &World, id: u32, part: &str) -> bool {
    w.notes.get(&id).is_some_and(|l| l.iter().any(|n| n.text.contains(part)))
}

#[test]
fn an_idle_scout_waits_on_rich_ground() {
    let mut s = bare();
    let cap = capital(s.world());
    let food = ray(s.world(), cap, 2)[1];
    s.food(food, GRAIN, 40.0);
    let mut w = s.into_world();
    assert_eq!(steps(&w, cap, food), 2);
    for _ in 0..50 {
        end_hungry(&mut w);
    }
    let u = scout(&w);
    assert_eq!(u.people, SCOUT.people as f64, "no one starved");
    assert!(steps(&w, u.home, food) <= SCOUT.reach as i32, "it waits where the food is within its reach");
    assert_eq!(u.scout().unwrap().wander, Some(u.home), "on the ground it picked");
    assert!(noted(&w, u.id, "waits on rich ground"));
    assert!(w.tiles.iter().all(|t| t.fallow == 0), "no tile it took from is fallow");
}

#[test]
fn a_scout_takes_only_what_a_tile_spares_and_an_army_fallows_it() {
    let mut s = bare();
    let cap = capital(s.world());
    let food = ray(s.world(), cap, 2)[1];
    s.food(food, GRAIN, 40.0);
    let me = s.world().player.unwrap();
    let mut army = Stage::retro("small", 2).unwrap();
    army.lead(me).unwrap();
    army.plain();
    {
        let w = army.world_mut();
        w.clans.clear();
        for t in &mut w.tiles {
            t.cap = [0.0; N];
            t.unworn = [0.0; N];
            t.stock = [0.0; N];
        }
    }
    army.food(food, GRAIN, 40.0);
    let id = led_scout(s.world());
    let mut w = s.into_world();
    let u = w.units.get_mut(&id).unwrap();
    u.actor.home = food;
    u.actor.territory = vec![food];
    u.actor.store = [0.0; N];
    let before = spare(&w.tiles[food]);
    end_hungry(&mut w);
    let t = &w.tiles[food];
    let took: f64 = t.take.iter().sum();
    assert!(took > 0.0, "the scout forages the tile");
    assert!(took <= before.iter().sum::<f64>() + 1e-9, "no more than its spare: took {took}");
    assert!(t.take[GRAIN] <= yield_of(t, GRAIN) + 1e-9);
    assert_eq!(t.fallow, 0, "a scout never fallows a tile");

    army.army(me, food, 4.0);
    let mut w = army.into_world();
    w.units.values_mut().filter(|u| u.kind() == units::UnitKind::Camp).for_each(|u| u.actor.store = [0.0; N]);
    end_hungry(&mut w);
    assert!(w.tiles[food].fallow > 0, "an army stripping the same tile leaves it fallow");
}

/// A scout routed over ten tiles of forest and hills, with food two steps off its route's start.
fn routed() -> (World, u32, usize, usize) {
    let mut s = bare();
    let cap = capital(s.world());
    let line = ray(s.world(), cap, 10);
    for (k, &i) in line.iter().enumerate() {
        s.paint(i, if k % 2 == 0 { FOREST } else { HILLS }, Some(1.0 * DAY), None);
    }
    let food = line[1];
    s.food(food, GRAIN, 60.0);
    let w = s.into_world();
    let id = led_scout(&w);
    (w, id, cap, line[9])
}

#[test]
fn a_routed_scout_with_an_empty_pack_leaves_for_rich_ground_and_comes_back_fed() {
    let (mut w, id, cap, to) = routed();
    w.act(led_order(&w, Order::Route { unit: id, from: cap, to })).unwrap();
    w.units.get_mut(&id).unwrap().actor.store = [0.0; N];
    let need = SCOUT.people as f64 * UNIT.food;
    let (mut left, mut back, mut searched, mut wandering) = (false, false, false, false);
    for _ in 0..40 {
        end_hungry(&mut w);
        let u = &w.units[&id];
        let s = u.scout().unwrap();
        assert_eq!(s.route, Some((cap, to)), "its route is unchanged");
        if s.wander.is_some() {
            left = true;
            wandering = true;
        } else if wandering {
            wandering = false;
            back = true;
            assert!(noted(&w, id, "back to its route"));
            // It had `fed` turns of eating in the pack before it ate this turn's.
            assert!(u.store.iter().sum::<f64>() + need + 1e-9 >= SCOUT.fed * need);
        }
        if back && w.civs[&w.player.unwrap()].revealed.len() > 1 {
            searched = true;
        }
    }
    assert!(noted(&w, id, "leaves its route to forage at"), "it says it leaves");
    assert!(left && back, "it leaves for rich ground and comes back");
    assert!(searched || w.tiles.iter().any(|t| t.trail_work > 0.0 || t.trail), "it searches again");
}

#[test]
fn the_hunger_note_names_the_turns_forage_and_line() {
    let mut s = bare();
    let me = s.world().player.unwrap();
    let id = led_scout(s.world());
    let cap = capital(s.world());
    let far = ray(s.world(), cap, 6)[5];
    s.order(me, Order::Move { unit: id, to: far }).unwrap();
    let mut w = s.into_world();
    w.units.get_mut(&id).unwrap().actor.store = [0.0; N];
    end_hungry(&mut w);
    assert!(noted(&w, id, "goes hungry at"));
    assert!(noted(&w, id, "foraging brought 0.0 and its line 0.0 of the 2 it eats"));
    let h = scout(&w).scout().unwrap().hunger.expect("hungry this turn");
    assert_eq!((h.forage, h.line, h.eats), (0.0, 0.0, 2.0));
}
