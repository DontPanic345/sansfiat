//! The query (`inspect.rs`): a target names one thing, a reading agrees with the world, a replay stopped
//! at a turn is the world at that turn, and the diff names each field that changed.

use engine::inspect::{self, Target};
use engine::{Config, World};
use serde_json::json;

/// A watched game on the small map, run to the pick and `after` turns on.
fn watched(seed: u32, after: u32) -> World {
    let mut w = World::new(Config { seed, ..Config::default() });
    while w.end.is_none() {
        w.run(1);
    }
    w.run(after);
    w
}

#[test]
fn a_target_is_an_id_a_hex_or_a_tile_index() {
    let w = World::new(Config::default());
    let i = 40;
    let (q, r) = (w.tiles[i].q, w.tiles[i].r);
    assert_eq!(Target::parse(&w, &format!("{q},{r}")), Ok(Target::Tile(i)));
    assert_eq!(Target::parse(&w, &format!(" {q}, {r} ")), Ok(Target::Tile(i)));
    assert_eq!(Target::parse(&w, "40"), Ok(Target::Tile(40)));
    assert_eq!(Target::parse(&w, "#7"), Ok(Target::Id(7)));
    assert!(Target::parse(&w, "99,99").is_err(), "off the map");
    assert!(Target::parse(&w, &w.tiles.len().to_string()).is_err(), "past the last tile");
    assert!(Target::parse(&w, "#x").is_err());
    assert!(Target::parse(&w, "camp").is_err());
}

#[test]
fn a_reading_of_each_kind_agrees_with_the_world() {
    let w = watched(2, 30);
    let unit = w.units.values().next().expect("a watched game has units 30 turns past the pick");
    let v = inspect::inspect(&w, Target::Id(unit.id)).unwrap();
    assert_eq!(v["kind"], unit.kind().name());
    assert_eq!(v["at"]["tile"], unit.actor.home);
    assert_eq!(v["state"]["people"], unit.actor.people);
    assert_eq!(v["turn"], w.turn);
    // Reasons newest first, as the panel shows them.
    let turns: Vec<u64> = v["notes"].as_array().unwrap().iter().map(|n| n["turn"].as_u64().unwrap()).collect();
    assert!(turns.windows(2).all(|p| p[0] >= p[1]), "newest first: {turns:?}");
    for e in v["events"].as_array().unwrap() {
        assert_eq!(e["unit"], unit.id, "each event is about the unit: {e}");
        assert!(e["text"].is_string());
    }
    // The tile it stands on lists it.
    let t = inspect::inspect(&w, Target::Tile(unit.actor.home)).unwrap();
    assert!(t["here"]["units"].as_array().unwrap().iter().any(|u| u["id"] == unit.id));

    for (kind, id) in [
        ("clan", *w.clans.keys().next().unwrap()),
        ("civ", *w.civs.keys().next().unwrap()),
        ("market", *w.markets.keys().next().unwrap()),
    ] {
        assert_eq!(inspect::inspect(&w, Target::Id(id)).unwrap()["kind"], kind);
    }
    assert!(inspect::inspect(&w, Target::Id(w.next_id)).is_err(), "no id is given out past next_id");
    for kind in ["units", "clans", "civs", "markets", "settlements"] {
        let rows = inspect::list(&w, kind).unwrap();
        for row in rows.as_array().unwrap() {
            let id = row["id"].as_u64().unwrap() as u32;
            assert_eq!(inspect::inspect(&w, Target::Id(id)).unwrap()["at"], row["at"], "{kind} {id}");
        }
    }
    assert!(inspect::list(&w, "armies").is_err());
}

#[test]
fn a_replay_stopped_at_a_turn_is_the_world_at_that_turn() {
    let w = watched(3, 6);
    let save = w.save();
    let turn = w.turn - 4;
    let early = World::replay_to(&save, Some(turn)).unwrap();
    assert_eq!(early.turn, turn);
    let mut again = World::new(save.config);
    while again.turn < turn {
        again.run(1);
    }
    assert_eq!(engine::stats::digest(&early), engine::stats::digest(&again));
    assert_eq!(World::replay_to(&save, Some(w.turn + 50)).unwrap().turn, w.turn, "past the end: all of it");
}

#[test]
fn the_diff_names_each_changed_field_by_its_path() {
    let a = json!({ "at": { "tile": 4 }, "state": { "people": 3.0, "routes": [1, 2], "same": true } });
    let b = json!({ "at": { "tile": 5 }, "state": { "people": 3.0, "routes": [1], "new": 1, "same": true } });
    assert_eq!(
        inspect::diff(&a, &b),
        json!([
            { "path": "at.tile", "from": 4, "to": 5 },
            { "path": "state.routes.1", "from": 2, "to": null },
            { "path": "state.new", "from": null, "to": 1 },
        ])
    );
    assert_eq!(inspect::diff(&a, &a), json!([]));
}
