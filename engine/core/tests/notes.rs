//! The reasons: where the rules write them, how many a subject keeps, that a gone subject has none,
//! and that a replay rebuilds them.

mod common;

use common::*;
use engine::data::{FOREST, HILLS, NOTES};
use engine::units::{self, UnitKind};
use engine::{Action, Config, EventKind, World};

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
fn a_watched_game_gives_reasons_to_clans_markets_civs_and_camps() {
    let w = watched(1, 50);
    assert!(w.clans.keys().any(|id| w.notes.contains_key(id)), "a clan has a reason");
    assert!(w.markets.keys().any(|id| w.notes.contains_key(id)), "a market has a reason");
    assert!(w.civs.keys().all(|id| w.notes.contains_key(id)), "every civ has a reason: it was founded");
    let camps: Vec<u32> = w.units.values().filter(|u| u.kind() == UnitKind::Camp).map(|u| u.id).collect();
    assert!(!camps.is_empty() && camps.iter().all(|id| w.notes.contains_key(id)), "every camp has a reason");
}

#[test]
fn a_led_scouts_reasons_follow_its_route() {
    let mut w = World::new(Config { seed: 2, ..Config::default() });
    while w.end.is_none() {
        w.run(1);
    }
    let red = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(red) }).unwrap();
    let cap = w.civs[&red].capital;
    let to = (0..w.tiles.len())
        .find(|&i| matches!(w.tiles[i].terrain, FOREST | HILLS) && w.tiles[i].hex().distance(w.tiles[cap].hex()) == 7)
        .expect("a forest or hills tile seven steps out");
    w.act(Action::SetRoute { from: cap, to }).unwrap();
    w.run(60);
    let scout = units::scout(&w, red).expect("a scout").id;
    let texts: Vec<&str> = w.notes[&scout].iter().map(|n| n.text.as_str()).collect();
    assert!(texts.iter().any(|t| t.starts_with("route set from")), "the route is set: {texts:?}");
    assert!(texts.iter().any(|t| t.starts_with("has its route") || t.starts_with("builds a trail")), "{texts:?}");
}

#[test]
fn a_division_names_its_daughter_and_the_daughter_its_mother() {
    let mut w = World::new(Config { seed: 1, ..Config::default() });
    let (mut seen, mut named) = (0, 0);
    let mut turn_events = 0;
    while w.end.is_none() {
        w.run(1);
        let fresh: Vec<_> = w.events[turn_events..].to_vec();
        turn_events = w.events.len();
        for e in fresh.iter().filter(|e| e.kind == EventKind::ClanDivided) {
            let (mother, daughter) = (e.clan.unwrap(), e.other.unwrap());
            let t = e.turn;
            seen += 1;
            // A clan merged away in the same turn has no reasons left; the rest name each other.
            let says = |id: u32, marker: String| {
                w.notes.get(&id).is_some_and(|l| l.iter().any(|n| n.turn == t && n.text.contains(&marker)))
            };
            if w.clans.contains_key(&mother) {
                assert!(says(mother, format!("clan {daughter} ")), "clan {mother} names clan {daughter} on turn {t}");
                named += 1;
            }
            if w.clans.contains_key(&daughter) {
                assert!(says(daughter, format!("clan {mother} ")), "clan {daughter} names clan {mother} on turn {t}");
                named += 1;
            }
        }
    }
    assert!(seen > 0 && named > 0, "some clan divided before the pick, and was named");
}

#[test]
fn no_subject_keeps_more_than_the_kept_reasons() {
    let w = watched(2, 80);
    assert!(w.notes.values().any(|l| l.len() == NOTES.kept), "a subject fills its reasons");
    assert!(w.notes.values().all(|l| l.len() <= NOTES.kept), "none keeps more than {}", NOTES.kept);
}

#[test]
fn a_merged_clan_has_no_reasons_once_it_is_gone() {
    let mut w = World::new(Config { seed: 2, ..Config::default() });
    let mut turn_events = 0;
    let mut merged = 0;
    while w.end.is_none() {
        w.run(1);
        let fresh: Vec<_> = w.events[turn_events..].to_vec();
        turn_events = w.events.len();
        for e in fresh.iter().filter(|e| e.kind == EventKind::ClanMerged) {
            let id = e.clan.unwrap();
            assert!(!w.clans.contains_key(&id), "clan {id} is gone");
            assert!(!w.notes.contains_key(&id), "clan {id}, gone, has no reasons");
            merged += 1;
        }
    }
    w.run(40);
    assert!(merged > 0, "a clan merged before the pick");
    assert!(
        w.notes.keys().all(|id| w.clans.contains_key(id)
            || w.markets.contains_key(id)
            || w.civs.contains_key(id)
            || w.units.contains_key(id)),
        "every reason's subject is alive"
    );
}

#[test]
fn a_replay_rebuilds_the_reasons() {
    let w = watched(3, 50);
    let replayed = World::replay(&w.save()).unwrap();
    assert_eq!(replayed.notes, w.notes);
    assert_eq!(replayed, w);
}

#[test]
fn a_repeated_reason_moves_on_and_the_oldest_go() {
    let mut w = flat();
    let id = 99;
    w.turn = 5;
    w.note(id, "a".to_string());
    w.turn = 6;
    w.note(id, "a".to_string());
    w.turn = 7;
    w.note(id, "b".to_string());
    w.turn = 8;
    w.note(id, "b".to_string());
    let list: Vec<(u32, &str)> = w.notes[&id].iter().map(|n| (n.turn, n.text.as_str())).collect();
    assert_eq!(list, [(6, "a"), (8, "b")], "a repeat of last turn's reason moves to now, to the newest place");
    // Ten more reasons: the two oldest go, and so the first of the eight kept is `n2`.
    for k in 0..10 {
        w.turn = 10 + k;
        w.note(id, format!("n{k}"));
    }
    let list = &w.notes[&id];
    assert_eq!(list.len(), NOTES.kept, "the kept reasons, no more");
    assert_eq!(list[0].text, "n2", "the oldest go first");
    assert_eq!(list.last().unwrap().text, "n9");
}
