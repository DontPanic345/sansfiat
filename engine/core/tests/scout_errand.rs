//! The scout's first errand on the seven retro maps: the first civ is led, and its scout is routed at
//! the pick to the nearest rival capital, and separately to `Lead`'s target (a forest or hills tile
//! seven steps out). Over 50 turns each scout's hungry turns, people starved and survival are counted.
//! `cargo test --release -p engine --test scout_errand -- --ignored --nocapture` prints the table (`relay/specs/104/scout.md`).

use engine::data::{FOREST, HILLS};
use engine::Config;
use engine::{land, units, Action, Order, Phase, World};

/// The retro maps: `(name, radius, seed)`.
const MAPS: [(&str, i32, u32); 7] = [
    ("small", 16, 2),
    ("small", 16, 3),
    ("small", 16, 6),
    ("small", 16, 9),
    ("medium", 23, 8),
    ("medium", 23, 9),
    ("large", 33, 8),
];
const TURNS: u32 = 50;

struct Errand {
    hungry: u32,
    starved: f64,
    alive: bool,
}

fn errand(radius: i32, seed: u32, rival: bool, fed: bool) -> Errand {
    let mut w = World::new(Config { seed, radius, ..Config::default() });
    while w.phase != Phase::Choose {
        w.act(Action::EndTurn).unwrap();
    }
    let me = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(me) }).unwrap();
    let cap = w.civs[&me].capital;
    let to = if rival {
        w.civs
            .values()
            .filter(|c| c.id != me)
            .min_by_key(|c| (land::steps(&w, cap, c.capital), c.id))
            .map(|c| c.capital)
            .unwrap()
    } else {
        (0..w.tiles.len())
            .find(|&i| matches!(w.tiles[i].terrain, FOREST | HILLS) && land::steps(&w, i, cap) == 7)
            .unwrap()
    };
    let id = units::scout(&w, me).unwrap().id;
    let start = w.units[&id].people;
    w.act(Action::Order { civ: me, order: Order::Route { unit: id, from: cap, to } }).unwrap();
    let mut hungry = 0;
    for _ in 0..TURNS {
        if fed {
            // The control: the civ's store is full each turn, so the camp at home leaves the scout its share.
            w.civs.get_mut(&me).unwrap().store = [100.0; engine::data::N];
        }
        w.act(Action::EndTurn).unwrap();
        if w.units.get(&id).is_none_or(|u| u.hungry > 0) {
            hungry += 1;
        }
    }
    let left = w.units.get(&id).map_or(0.0, |u| u.people);
    Errand { hungry, starved: start - left, alive: left > 0.0 }
}

#[test]
#[ignore = "a report: run it with --ignored --nocapture"]
fn the_scouts_first_errand_on_the_retro_maps() {
    let mut lost = Vec::new();
    println!("| map | to | hungry turns | people starved | alive |");
    println!("|---|---|---|---|---|");
    for (name, radius, seed) in MAPS {
        for rival in [true, false] {
            let e = errand(radius, seed, rival, std::env::var("FED").is_ok());
            let to = if rival { "nearest rival capital" } else { "Lead's target" };
            println!(
                "| {name} {seed} | {to} | {} | {} | {} |",
                e.hungry,
                e.starved,
                if e.alive { "yes" } else { "no" }
            );
            if !e.alive {
                lost.push(format!("{name} {seed} to {to}"));
            }
        }
    }
    println!("scouts lost: {} of {}: {lost:?}", lost.len(), MAPS.len() * 2);
}
