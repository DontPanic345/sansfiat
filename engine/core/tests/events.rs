mod common;

use common::*;
use engine::clans::eat;
use engine::data::*;
use engine::land::hold;
use engine::markets::fade;
use engine::pregame::check_end;
use engine::stats::digest;
use engine::{Config, Event, EventKind, World};

/// A small world run for `turns` turns.
fn run(seed: u32, turns: u32) -> World {
    let mut w = World::new(Config { seed, radius: 16, army: true });
    w.run(turns);
    w
}

fn kinds(w: &World) -> Vec<EventKind> {
    w.events.iter().map(|e| e.kind).collect()
}

#[test]
fn every_kind_fires_in_real_runs() {
    use EventKind::*;
    let all = [
        ClanDivided,
        ClanStarving,
        ClanMoved,
        MarketFormed,
        MarketFaded,
        HearthTamed,
        CentreFormed,
        PregameEnded,
        CivFounded,
        // The others (hungry, disbanded, starved) are held in units.rs.
        CampRaised,
    ];
    let mut seen = vec![0usize; all.len()];
    // Seed 3 ends on the turn limit with a top-up; the others end on their own.
    for seed in [1, 3, 5] {
        let w = run(seed, 500);
        for k in kinds(&w) {
            // Hungry and starved warriors also show up in these runs; units.rs holds those.
            if let Some(p) = all.iter().position(|&a| a == k) {
                seen[p] += 1;
            }
        }
        assert_eq!(kinds(&w).iter().filter(|&&k| k == PregameEnded).count(), 1, "seed {seed}: one end");
    }
    for (k, n) in all.iter().zip(&seen) {
        assert!(*n > 0, "{k:?} never fired over three small runs");
    }
}

#[test]
fn events_are_in_turn_order_with_valid_tiles_and_wording() {
    let w = run(2, 300);
    assert!(!w.events.is_empty());
    assert!(w.events.windows(2).all(|p| p[0].turn <= p[1].turn));
    assert!(w.events.iter().all(|e| e.turn < w.turn.max(1)));
    for e in &w.events {
        assert!(e.tile < w.tiles.len(), "{e:?}");
        assert!(!e.text(&w).is_empty());
        match e.kind {
            EventKind::ClanDivided => assert!(e.clan.is_some() && e.other.is_some(), "{e:?}"),
            EventKind::ClanStarving | EventKind::ClanMoved => assert!(e.clan.is_some(), "{e:?}"),
            EventKind::MarketFormed | EventKind::MarketFaded | EventKind::CentreFormed => {
                assert!(e.market.is_some(), "{e:?}")
            }
            _ => {}
        }
    }
    // A division names the daughter, who exists unless she has since died or divided away.
    let e = w.events.iter().find(|e| e.kind == EventKind::ClanDivided).expect("a division");
    let h = w.tiles[e.tile].hex();
    assert_eq!(
        e.text(&w),
        format!("Clan {} divides; clan {} settles at ({}, {})", e.clan.unwrap(), e.other.unwrap(), h.q, h.r)
    );
}

#[test]
fn a_replay_gives_the_same_events() {
    let w = run(1, 150);
    assert!(!w.events.is_empty());
    let again = World::replay(&w.save()).unwrap();
    assert_eq!(again.events, w.events);
    assert_eq!(again.centres_seen, w.centres_seen);
    assert_eq!(again, w);
}

#[test]
fn events_are_not_in_the_digest_or_the_state() {
    let mut w = run(1, 60);
    assert!(!w.events.is_empty());
    let (d, state) = (digest(&w), serde_json::to_string(&w).unwrap());
    assert!(!state.contains("ClanStarving") && !state.contains("clanStarving") && !state.contains("events"));
    w.events.clear();
    w.centres_seen.clear();
    assert_eq!(digest(&w), d);
    assert_eq!(serde_json::to_string(&w).unwrap(), state);
}

#[test]
fn a_tile_reaching_full_tame_is_a_hearth_event() {
    let mut w = flat();
    put(&mut w, &[at(0, 0), at(1, 0)], 3.0);
    for t in [at(0, 0), at(1, 0)] {
        w.tiles[t].wild_grain = 4.0;
        w.tiles[t].held = TENDING.after;
    }
    w.tiles[at(0, 0)].tame = TENDING.tame_turns - 1;
    w.tiles[at(1, 0)].tame = 3;
    w.turn = 7;
    hold(&mut w);
    assert_eq!(w.events.len(), 1);
    let e = &w.events[0];
    assert_eq!((e.kind, e.tile, e.turn, e.clan), (EventKind::HearthTamed, at(0, 0), 7, None));
    assert_eq!(e.text(&w), "A hearth is tamed at (0, 0)");
    // Already a hearth: no second event.
    hold(&mut w);
    assert_eq!(w.events.len(), 1);
}

#[test]
fn a_starving_clan_is_announced_once_and_a_big_one_divides() {
    let mut w = flat();
    let id = put(&mut w, &[at(0, 0)], 10.0);
    w.clans.get_mut(&id).unwrap().basket = [4.0, 0.0, 0.0, 0.0, 0.0];
    w.turn = 3;
    eat(&mut w);
    assert_eq!(w.events.len(), 1);
    let e = &w.events[0];
    assert_eq!((e.kind, e.clan, e.tile, e.turn), (EventKind::ClanStarving, Some(id), at(0, 0), 3));
    assert_eq!(e.text(&w), format!("Clan {id} is starving"));
    // Still starving next turn: not announced again.
    w.clans.get_mut(&id).unwrap().basket = [4.0, 0.0, 0.0, 0.0, 0.0];
    eat(&mut w);
    assert_eq!(w.events.len(), 1);

    let mut w = flat();
    let id = put(&mut w, &[at(0, 0), at(1, 0)], CLAN.split_at);
    w.clans.get_mut(&id).unwrap().basket = [4.0 * CLAN.split_at; N];
    eat(&mut w);
    assert_eq!(w.events.len(), 1);
    let e = &w.events[0];
    assert_eq!((e.kind, e.clan), (EventKind::ClanDivided, Some(id)));
    let daughter = e.other.expect("a daughter");
    assert!(daughter != id && w.clans.contains_key(&daughter));
    assert_eq!(e.tile, w.clans[&daughter].home);
}

#[test]
fn a_market_that_fades_is_an_event() {
    let mut w = flat();
    let m = open(&mut w, at(2, 1));
    w.markets.get_mut(&m).unwrap().activity = 0.0;
    w.turn = 9;
    fade(&mut w);
    assert!(w.markets.is_empty());
    assert_eq!(
        w.events,
        vec![Event {
            turn: 9,
            kind: EventKind::MarketFaded,
            tile: at(2, 1),
            clan: None,
            market: Some(m),
            other: None,
            civ: None,
            unit: None
        }]
    );
    assert_eq!(w.events[0].text(&w), format!("Market {m} fades"), "worded after the market is gone");
}

#[test]
fn a_top_up_is_a_centre_each_then_the_end() {
    let mut w = big();
    let a = open(&mut w, at_big(8, 0));
    let b = open(&mut w, at_big(-8, 8));
    let c = open(&mut w, at_big(0, -8));
    w.turn = PREGAME.max_turns - 1;
    check_end(&mut w);
    let end = w.end.clone().unwrap();
    assert_eq!(end.centres, vec![a, b, c]);
    let got: Vec<(EventKind, Option<u32>)> = w.events.iter().map(|e| (e.kind, e.market)).collect();
    assert_eq!(
        got,
        vec![
            (EventKind::CentreFormed, Some(a)),
            (EventKind::CentreFormed, Some(b)),
            (EventKind::CentreFormed, Some(c)),
            (EventKind::PregameEnded, None),
            // Founding follows the end, a civ on each centre.
            (EventKind::CivFounded, Some(a)),
            (EventKind::CivFounded, Some(b)),
            (EventKind::CivFounded, Some(c)),
            // No clans, so no camps are raised.
        ]
    );
    let last = &w.events[3];
    assert_eq!((last.tile, last.turn), (at_big(8, 0), PREGAME.max_turns - 1));
    assert_eq!(last.text(&w), "The pre-game ends: 3 centres, 3 topped up");
    assert_eq!(w.events[0].text(&w), format!("Market {a} becomes a centre"));
    // Nothing more once it has ended.
    check_end(&mut w);
    assert_eq!(w.events.len(), 7);
}

#[test]
fn a_market_that_becomes_a_centre_is_announced_once() {
    let (mut w, ids) = {
        let mut w = flat();
        w.turn = 40;
        let tiles = [at(5, 0), at(-5, 5), at(0, -5)];
        let mut ids = [0; 3];
        for k in 0..3 {
            hearth(&mut w, tiles[k], 4.0);
            ids[k] = open(&mut w, tiles[k]);
            w.markets.get_mut(&ids[k]).unwrap().traders = 4;
        }
        (w, ids)
    };
    check_end(&mut w);
    check_end(&mut w);
    let formed: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::CentreFormed).collect();
    assert_eq!(formed.len(), 3, "once each, not each turn");
    assert!(ids.iter().all(|id| formed.iter().any(|e| e.market == Some(*id))));
    // Cover short: it carries on, with the end not yet reached.
    assert!(w.end.is_none());
}

#[test]
fn a_centre_that_dropped_out_and_is_topped_up_is_announced_once() {
    let mut w = big();
    let a = open(&mut w, at_big(8, 0));
    let b = open(&mut w, at_big(-8, 8));
    let c = open(&mut w, at_big(0, -8));
    hearth(&mut w, at_big(8, 0), 4.0);
    w.turn = 40;
    w.markets.get_mut(&a).unwrap().traders = 4;
    check_end(&mut w);
    assert!(w.end.is_none(), "one centre, not the end");
    // It loses its traders and is no longer a centre; the limit tops it up.
    w.markets.get_mut(&a).unwrap().traders = 0;
    w.turn = PREGAME.max_turns - 1;
    check_end(&mut w);
    assert_eq!(w.end.as_ref().unwrap().centres.len(), 3);
    let formed: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::CentreFormed).collect();
    for id in [a, b, c] {
        assert_eq!(formed.iter().filter(|e| e.market == Some(id)).count(), 1, "market {id} announced once");
    }
}
