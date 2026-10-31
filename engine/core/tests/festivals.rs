mod common;

use common::*;
use engine::clans::{surplus, Clan};
use engine::data::*;
use engine::markets::{fade, form, people, trade};
use engine::EventKind;
use engine::{Config, World};

fn sum_people(w: &World) -> f64 {
    w.clans.values().map(|c| c.people).sum()
}

fn come(w: &mut World, id: u32, mid: u32) {
    w.clans.get_mut(&id).unwrap().market = Some(mid);
}

fn starve(c: &mut Clan, ratio: f64) {
    c.starving = true;
    c.fed_ratio = ratio;
}

/// Gives the clan a harvest that leaves it a surplus.
fn grow(c: &mut Clan, each: f64) {
    c.harvest = [each; N];
    assert!(surplus(&c.harvest, c.people) > 0.0);
}

#[test]
fn a_settled_lone_clan_raises_a_festival_on_its_home_and_goes_to_it() {
    let mut w = flat();
    let o = at(0, 0);
    let id = put(&mut w, &[o], 5.0);
    w.clans.get_mut(&id).unwrap().since = MARKET.form_after - 1;
    form(&mut w);
    assert!(w.markets.is_empty(), "not settled long enough");
    w.clans.get_mut(&id).unwrap().since = MARKET.form_after;
    form(&mut w);
    assert_eq!(w.markets.len(), 1);
    let m = w.markets.values().next().unwrap().clone();
    assert_eq!(m.tile, o);
    assert_eq!(m.activity, 2.0 * MARKET.min_attendance);
    assert!(w.events.iter().any(|e| e.kind == EventKind::MarketFormed));
    engine::markets::go(&mut w);
    assert_eq!(w.clans[&id].market, Some(m.id));
    assert_eq!(w.clans[&id].trip(), 0.0);
}

#[test]
fn a_clan_near_a_festival_raises_none_and_the_biggest_goes_first() {
    let mut w = big();
    let (a, b, c) = (at_big(0, 0), at_big(3, 0), at_big(MARKET.spacing + 3, 0));
    let (ia, ib, ic) = (put(&mut w, &[a], 3.0), put(&mut w, &[b], 6.0), put(&mut w, &[c], 2.0));
    for id in [ia, ib, ic] {
        w.clans.get_mut(&id).unwrap().since = MARKET.form_after;
    }
    form(&mut w);
    let tiles: Vec<usize> = w.markets.values().map(|m| m.tile).collect();
    // b (the biggest) raises first; a is within spacing of it; c is not.
    assert_eq!(tiles.len(), 2, "{tiles:?}");
    assert!(tiles.contains(&b) && !tiles.contains(&a));
    // A clan within spacing of an old festival raises none.
    let mut w = big();
    open(&mut w, at_big(2, 0));
    let id = put(&mut w, &[at_big(0, 0)], 5.0);
    w.clans.get_mut(&id).unwrap().since = MARKET.form_after;
    form(&mut w);
    assert_eq!(w.markets.len(), 1);
}

#[test]
fn a_festival_one_clan_keeps_coming_to_stays_and_an_empty_one_fades() {
    let mut w = flat();
    let kept = open(&mut w, at(0, 0));
    let empty = open(&mut w, out(0, 5));
    let centre = open(&mut w, out(3, 5));
    w.end = Some(engine::pregame::End {
        turn: 0,
        centres: vec![centre],
        clans: vec![],
        topped_up: vec![],
        by_limit: false,
        cover: 1.0,
    });
    let id = put(&mut w, &[at(0, 1)], 4.0);
    come(&mut w, id, kept);
    for _ in 0..80 {
        trade(&mut w);
        fade(&mut w);
    }
    assert!(w.markets.contains_key(&kept), "one clan keeps it");
    assert!(!w.markets.contains_key(&empty), "nobody comes");
    assert!(w.markets.contains_key(&centre), "centres never fade");
}

#[test]
fn a_clan_brings_at_most_carry_times_its_people_and_trade_conserves() {
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    let baskets = [[0.0, 8.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 6.0, 2.0], [3.0, 1.0, 4.0, 0.0, 0.0]];
    let mut ids = Vec::new();
    for (k, b) in baskets.iter().enumerate() {
        let id = put(&mut w, &[out(k, 1)], 4.0);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = *b;
        come(&mut w, id, mid);
        ids.push(id);
    }
    let total = |w: &World| -> [f64; N] { std::array::from_fn(|f| ids.iter().map(|id| w.clans[id].basket[f]).sum()) };
    let before = total(&w);
    trade(&mut w);
    let after = total(&w);
    for f in 0..N {
        assert!((after[f] - before[f]).abs() < 1e-9);
    }
    // Most of each basket stayed: a clan of 4 brought at most 0.8, so it holds at least the rest.
    let m = &w.markets[&mid];
    assert!(m.volume <= MARKET.carry * 12.0 + 1e-9, "volume {}", m.volume);
    assert_eq!(m.traders, 3);
    assert!((m.activity - (MARKET.fade * 2.0 * MARKET.min_attendance + (1.0 - MARKET.fade) * 3.0)).abs() < 1e-9);
}

#[test]
fn people_move_from_a_starving_clan_to_a_growing_one_up_to_its_room() {
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    let (s, g) = (put(&mut w, &[out(0, 1)], 10.0), put(&mut w, &[out(2, 1)], 10.0));
    starve(w.clans.get_mut(&s).unwrap(), 0.2);
    grow(w.clans.get_mut(&g).unwrap(), 4.0);
    let room = surplus(&w.clans[&g].harvest, 10.0);
    let offer = MARKET.shed * 10.0 * 0.8;
    come(&mut w, s, mid);
    come(&mut w, g, mid);
    let before = sum_people(&w);
    people(&mut w);
    let moved = offer.min(room);
    assert!((w.clans[&s].people - (10.0 - moved)).abs() < 1e-9);
    assert!((w.clans[&g].people - (10.0 + moved)).abs() < 1e-9);
    assert!((sum_people(&w) - before).abs() < 1e-9);
    assert!((w.markets[&mid].moved - moved).abs() < 1e-9 && (w.moved - moved).abs() < 1e-9);

    // A tiny room caps the move.
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    let (s, g) = (put(&mut w, &[out(0, 1)], 10.0), put(&mut w, &[out(2, 1)], 10.0));
    starve(w.clans.get_mut(&s).unwrap(), 0.0);
    w.clans.get_mut(&g).unwrap().harvest = [2.2, 2.2, 2.2, 2.2, 2.2];
    let room = surplus(&w.clans[&g].harvest, 10.0);
    assert!(room > 0.0 && room < 3.0, "{room}");
    come(&mut w, s, mid);
    come(&mut w, g, mid);
    people(&mut w);
    assert!((w.clans[&g].people - 10.0 - room).abs() < 1e-9);
    assert!((w.clans[&s].people - 10.0 + room).abs() < 1e-9);
}

#[test]
fn a_starving_clan_at_no_festival_sheds_nobody() {
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    let (s, g) = (put(&mut w, &[out(0, 1)], 10.0), put(&mut w, &[out(2, 1)], 10.0));
    starve(w.clans.get_mut(&s).unwrap(), 0.2);
    grow(w.clans.get_mut(&g).unwrap(), 4.0);
    come(&mut w, g, mid);
    people(&mut w);
    assert_eq!((w.clans[&s].people, w.clans[&g].people, w.moved), (10.0, 10.0, 0.0));
}

#[test]
fn two_growing_clans_share_the_offer_by_room() {
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    let s = put(&mut w, &[out(0, 1)], 20.0);
    let (g1, g2) = (put(&mut w, &[out(2, 1)], 10.0), put(&mut w, &[out(4, 1)], 10.0));
    starve(w.clans.get_mut(&s).unwrap(), 0.95);
    w.clans.get_mut(&g1).unwrap().harvest = [2.1; N];
    w.clans.get_mut(&g2).unwrap().harvest = [2.3; N];
    let (r1, r2) = (surplus(&w.clans[&g1].harvest, 10.0), surplus(&w.clans[&g2].harvest, 10.0));
    assert!(r1 > 0.0 && r2 > r1);
    let offer = MARKET.shed * 20.0 * 0.05;
    assert!(offer < r1 + r2);
    for id in [s, g1, g2] {
        come(&mut w, id, mid);
    }
    people(&mut w);
    let (d1, d2) = (w.clans[&g1].people - 10.0, w.clans[&g2].people - 10.0);
    assert!((d1 + d2 - offer).abs() < 1e-9);
    assert!((d1 / d2 - r1 / r2).abs() < 1e-9);
}

#[test]
fn a_small_clan_merges_into_the_nearest_at_its_festival() {
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    let small = put(&mut w, &[at(0, 0), at(-1, 0)], 1.5);
    w.clans.get_mut(&small).unwrap().store = [1.0; N];
    let far = put(&mut w, &[at(3, 0)], 5.0);
    let (near_hi, near_lo) = (put(&mut w, &[at(1, 0)], 5.0), put(&mut w, &[at(0, 1)], 5.0));
    // near_lo has the lower id? ids follow creation order: far < near_hi < near_lo, so make the tie
    // between far-id-excluded ones: near_hi and near_lo are the same steps from `small`.
    let tied = [near_hi, near_lo]
        .map(|id| engine::hex::Hex::distance(w.tiles[w.clans[&id].home].hex(), w.tiles[w.clans[&small].home].hex()));
    assert_eq!(tied[0], tied[1]);
    for id in [small, far, near_hi, near_lo] {
        come(&mut w, id, mid);
    }
    let before = sum_people(&w);
    people(&mut w);
    assert!(!w.clans.contains_key(&small));
    assert!((w.clans[&near_hi].people - 6.5).abs() < 1e-9, "ties go to the lower id");
    assert!((w.clans[&near_hi].store[0] - 1.0).abs() < 1e-9);
    assert!((sum_people(&w) - before).abs() < 1e-9);
    assert!(w.tiles[at(0, 0)].claim.is_none() && w.tiles[at(-1, 0)].claim.is_none());
    check_territories(&w);
    let e = w.events.iter().find(|e| e.kind == EventKind::ClanMerged).unwrap();
    assert_eq!((e.clan, e.other), (Some(small), Some(near_hi)));
    assert_eq!(e.text(&w), format!("Clan {small} merges into clan {near_hi}"));
}

#[test]
fn a_small_clan_alone_at_its_festival_does_not_merge() {
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    let small = put(&mut w, &[out(0, 1)], 1.5);
    let other = put(&mut w, &[out(3, 4)], 5.0);
    come(&mut w, small, mid);
    people(&mut w);
    assert!(w.clans.contains_key(&small) && w.clans.contains_key(&other));
}

#[test]
fn a_save_with_merges_and_moves_replays_to_the_same_world() {
    let mut merged = 0;
    let mut moved = 0.0;
    for seed in 1..=3 {
        let mut w = World::new(Config { seed, radius: 12, army: true, gifts: true, zone: true });
        w.run(300);
        merged += w.events.iter().filter(|e| e.kind == EventKind::ClanMerged).count();
        moved += w.moved;
        let mut r = World::replay(&w.save()).unwrap();
        assert_eq!(r.clans, w.clans);
        assert_eq!(r.markets, w.markets);
        assert_eq!(r.moved, w.moved);
        r.run(0);
    }
    eprintln!("merged {merged}, moved {moved}");
}
