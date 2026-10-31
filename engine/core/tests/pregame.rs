mod common;

use common::*;
use engine::data::*;
use engine::map::Country;
use engine::pregame::{centres, check_end, homes, is_centre, land_cover};
use engine::{Config, World};

/// A centre clan of `people` at `tile`: a hearth on its home, settled `centre_age` turns, going
/// to a festival on its home that had `centre_clans` traders last turn. Its id and the festival's.
fn centre(w: &mut World, tile: usize, people: f64) -> (u32, u32) {
    hearth(w, tile, 4.0);
    let c = put(w, &[tile], people);
    let m = open(w, tile);
    w.markets.get_mut(&m).unwrap().traders = PREGAME.centre_clans;
    let clan = w.clans.get_mut(&c).unwrap();
    clan.market = Some(m);
    clan.since = PREGAME.centre_age;
    (c, m)
}

/// Three centre clans of `people` on the corners of the land, each 10 steps from the others;
/// the turn is 40. The clans' ids and their festivals'.
fn three(people: [f64; 3]) -> (World, [u32; 3], [u32; 3]) {
    let mut w = flat();
    w.turn = 40;
    let tiles = [at(5, 0), at(-5, 5), at(0, -5)];
    let (mut cs, mut ms) = ([0; 3], [0; 3]);
    for k in 0..3 {
        (cs[k], ms[k]) = centre(&mut w, tiles[k], people[k]);
    }
    for a in tiles {
        for b in tiles {
            assert!(a == b || steps(&w, a, b) >= PREGAME.centre_spacing);
        }
    }
    (w, cs, ms)
}

#[test]
fn three_centres_far_apart_are_all_taken_biggest_first() {
    let (w, cs, _) = three([4.0, 6.0, 5.0]);
    assert_eq!(centres(&w), vec![cs[1], cs[2], cs[0]]);
}

#[test]
fn a_centre_near_a_bigger_one_is_not_taken() {
    let mut w = flat();
    w.turn = 40;
    let (a, b) = (at(5, 0), at(-1, 0));
    assert_eq!(steps(&w, a, b), 6);
    let (ca, _) = centre(&mut w, a, 4.0);
    let (cb, _) = centre(&mut w, b, 5.0);
    assert!(is_centre(&w, &w.clans[&ca]) && is_centre(&w, &w.clans[&cb]));
    assert_eq!(centres(&w), vec![cb]);
}

#[test]
fn a_centre_is_settled_at_a_busy_festival_with_a_hearth_on_its_land() {
    let (mut w, cs, ms) = three([4.0, 4.0, 4.0]);
    let c = cs[0];
    assert!(is_centre(&w, &w.clans[&c]));
    // A hearth on its territory, not just near it.
    w.tiles[at(5, 0)].tame = 0;
    assert!(!is_centre(&w, &w.clans[&c]), "no hearth on its land");
    hearth(&mut w, at(4, 0), 4.0);
    assert!(!is_centre(&w, &w.clans[&c]), "a hearth next door is not its own");
    w.clans.get_mut(&c).unwrap().territory = vec![at(4, 0), at(5, 0)];
    assert!(is_centre(&w, &w.clans[&c]));

    let (mut w, cs, ms2) = three([4.0, 4.0, 4.0]);
    assert_eq!(ms, ms2);
    w.markets.get_mut(&ms[1]).unwrap().traders = PREGAME.centre_clans - 1;
    assert!(!is_centre(&w, &w.clans[&cs[1]]), "a quiet festival");
    w.clans.get_mut(&cs[2]).unwrap().market = None;
    assert!(!is_centre(&w, &w.clans[&cs[2]]), "no festival");
    w.clans.get_mut(&cs[0]).unwrap().since = PREGAME.centre_age - 1;
    assert!(!is_centre(&w, &w.clans[&cs[0]]), "not settled long enough");
    assert!(centres(&w).is_empty());
}

#[test]
fn land_cover_is_the_share_near_some_territory() {
    // A row of 10 tiles, and an island of 2 off it.
    let row: Vec<usize> = (-5..5).map(|q| at(q, 0)).collect();
    let isle = [at(0, 3), at(1, 3)];
    let mut w = island(&[row.clone(), isle.to_vec()].concat());
    // Its territory is q = −5; within 3 steps of that is q = −5..=−2.
    put(&mut w, &[row[0]], 3.0);
    assert!((land_cover(&w, &[]) - 4.0 / 12.0).abs() < 1e-12);
    assert!((land_cover(&w, &[row[9]]) - 0.4).abs() < 1e-12);
    assert_eq!(land_cover(&w, &[isle[0]]), 0.0);
}

#[test]
fn the_pregame_ends_with_three_centres_the_land_covered_and_people_levelled_off() {
    let (mut w, cs, ms) = three([4.0, 4.0, 4.0]);
    // The centres alone leave the land short of covered.
    let levelled = |w: &mut World, share: f64| {
        let people: f64 = w.clans.values().map(|c| c.people).sum();
        w.people_seen = vec![people * share; PREGAME.settle_turns];
    };
    levelled(&mut w, 1.0);
    check_end(&mut w);
    assert_eq!(w.end, None, "the land is not covered");
    assert!(land_cover(&w, &homes(&w, &cs)) < PREGAME.cover);
    // One in the middle and six more, 4 out, cover it all.
    put(&mut w, &[at(0, 0)], 3.0);
    for d in 0..6 {
        put(&mut w, &[out(d, 4)], 3.0);
    }
    // Still growing: people were a tenth fewer `settle_turns` ago.
    levelled(&mut w, 0.9);
    check_end(&mut w);
    assert_eq!(w.end, None, "people have not levelled off");
    levelled(&mut w, 1.0);
    check_end(&mut w);
    let end = w.end.clone().expect("ended");
    // Each festival is on its centre's home, so it is the site: no market is raised.
    assert_eq!((end.turn, &end.centres, end.cover, end.by_limit), (40, &ms.to_vec(), 1.0, false));
    assert_eq!(end.clans, cs.map(Some).to_vec());
    assert_eq!(w.markets.len(), 3);
    w.markets.clear();
    w.turn += 1;
    check_end(&mut w);
    assert_eq!(w.end.as_ref().unwrap().turn, 40, "the first end stays");
}

#[test]
fn a_site_gets_a_market_and_the_festival_stays_where_it_is() {
    let (mut w, cs, ms) = three([4.0, 4.0, 4.0]);
    // The first centre walks to a festival two steps from home.
    let away = open(&mut w, at(3, 0));
    w.markets.get_mut(&away).unwrap().traders = PREGAME.centre_clans;
    w.clans.get_mut(&cs[0]).unwrap().market = Some(away);
    w.markets.remove(&ms[0]);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.clans, cs.map(Some).to_vec());
    let site = end.centres[0];
    assert!(site != away && !ms.contains(&site), "a market raised on the site");
    assert_eq!(w.markets[&site].tile, at(5, 0));
    assert_eq!(w.markets[&away].tile, at(3, 0), "the festival does not move");
    assert_eq!(end.centres[1..], ms[1..]);
    // The civ is founded on the site, with the centre clan its own.
    let civ = w.civs.values().find(|c| c.market == site).unwrap();
    assert_eq!((civ.capital, civ.clan), (at(5, 0), cs[0]));
    assert_eq!(w.clans[&cs[0]].civ, Some(civ.id));
}

#[test]
fn the_pregame_ends_at_the_turn_limit() {
    let (mut w, _, ms) = three([4.0, 4.0, 4.0]);
    w.markets.get_mut(&ms[2]).unwrap().traders = PREGAME.centre_clans - 1;
    put(&mut w, &[at(0, 0)], 3.0);
    w.turn = PREGAME.max_turns - 2;
    w.run(1);
    assert_eq!(w.end, None);
    w.run(1);
    let end = w.end.clone().expect("ended");
    assert_eq!((end.turn, end.by_limit), (PREGAME.max_turns - 1, true));
    assert_eq!(w.turn, PREGAME.max_turns);
}

// The top-up.

#[test]
fn the_top_up_takes_the_biggest_spaced_clans_before_markets() {
    let mut w = big();
    let m = market(&mut w, at_big(8, 0), 5);
    let small = put(&mut w, &[at_big(-8, 8)], 3.0);
    let big = put(&mut w, &[at_big(0, -8)], 5.0);
    let near = put(&mut w, &[at_big(-7, 8)], 4.0);
    assert!(centres(&w).is_empty());
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.clans, vec![Some(big), Some(near), None], "the smaller clan is too near the bigger");
    assert_eq!(end.centres[2], m);
    let tiles: Vec<usize> = end.centres.iter().map(|id| w.markets[id].tile).collect();
    assert_eq!(tiles, vec![at_big(0, -8), at_big(-7, 8), at_big(8, 0)], "markets raised on the clans' homes");
    assert_eq!(end.topped_up, end.centres);
    assert!(!end.clans.contains(&Some(small)));
    assert_spaced(&w, &end.centres);
}

/// Sets the turn to the last before the limit and runs the `end` step.
fn at_the_limit(w: &mut World) {
    w.turn = PREGAME.max_turns - 1;
    check_end(w);
}

/// A market with `traders` last turn, founded turn 0, on `tile`; it is no centre (no hearth).
fn market(w: &mut World, tile: usize, traders: usize) -> u32 {
    let id = open(w, tile);
    w.markets.get_mut(&id).unwrap().traders = traders;
    id
}

/// Gives `tile` wild grain, `tame` turns tamed and a fertility.
fn grain(w: &mut World, tile: usize, tame: u32, fertility: f64) {
    let t = &mut w.tiles[tile];
    t.wild_grain = 4.0;
    t.unworn[GRAIN] = 4.0;
    t.cap[GRAIN] = 4.0;
    t.tame = tame;
    t.fertility = fertility;
}

fn assert_spaced(w: &World, ids: &[u32]) {
    for a in ids {
        for b in ids {
            if a != b {
                assert!(steps(w, w.markets[a].tile, w.markets[b].tile) >= PREGAME.centre_spacing, "{a} and {b}");
            }
        }
    }
}

// Four sites at least 10 apart in `big()`: the corners (8,0), (-8,8), (0,-8) are 16 apart.

#[test]
fn with_no_centres_the_best_spaced_markets_make_three() {
    let mut w = big();
    // Two close together: only the busier is taken. The fifth is least busy and too near others.
    let a = market(&mut w, at_big(8, 0), 5);
    let near_a = market(&mut w, at_big(7, 1), 4);
    let b = market(&mut w, at_big(-8, 8), 3);
    let c = market(&mut w, at_big(0, -8), 2);
    let e = market(&mut w, at_big(0, 8), 1);
    assert!(centres(&w).is_empty());
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert!(end.by_limit);
    assert_eq!(end.centres, vec![a, b, c]);
    assert_eq!(end.topped_up, vec![a, b, c]);
    assert!(!end.centres.contains(&near_a) && !end.centres.contains(&e));
    assert_spaced(&w, &end.centres);
    assert_eq!(w.markets.len(), 5, "no market was raised");
}

#[test]
fn two_centres_and_no_more_markets_raise_one_on_the_most_tended_tile() {
    let mut w = big();
    let ((ca, a), (cb, b)) = (centre(&mut w, at_big(8, 0), 4.0), centre(&mut w, at_big(-8, 8), 4.0));
    w.turn = 40;
    assert_eq!(centres(&w), vec![ca, cb]);
    // Two tended tiles that fit; the one with more tame wins though it is less fertile. A third,
    // more tended still, is too near the first centre.
    grain(&mut w, at_big(0, -8), 3, 2.0);
    grain(&mut w, at_big(0, -9), 8, 0.5);
    grain(&mut w, at_big(6, 0), 9, 2.0);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.centres.len(), 3);
    assert_eq!(end.centres[..2], [a, b]);
    assert_eq!(end.clans, vec![Some(ca), Some(cb), None]);
    assert_eq!(end.topped_up.len(), 1);
    let m = &w.markets[&end.topped_up[0]];
    assert_eq!(m.tile, at_big(0, -9));
    assert_eq!((m.founded, m.traders, m.volume), (PREGAME.max_turns - 1, 0, 0.0));
    assert_eq!((m.activity, &m.goods[..], m.prices), (2.0 * MARKET.min_attendance, &[][..], [1.0; N]));
    assert!(end.by_limit);
    assert_spaced(&w, &end.centres);
}

#[test]
fn with_no_tame_land_the_most_fertile_grain_makes_the_rest() {
    let mut w = big();
    let (ca, _) = centre(&mut w, at_big(8, 0), 4.0);
    w.turn = 40;
    assert_eq!(centres(&w), vec![ca]);
    // Wild grain: the better of two close tiles wins, one near the centre is out of reach.
    grain(&mut w, at_big(-8, 8), 0, 1.5);
    grain(&mut w, at_big(-7, 8), 0, 1.9);
    grain(&mut w, at_big(0, -8), 0, 1.0);
    grain(&mut w, at_big(0, 8), 0, 1.2);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.centres.len(), 3);
    assert_eq!(end.topped_up.len(), 2);
    let tiles: Vec<usize> = end.topped_up.iter().map(|id| w.markets[id].tile).collect();
    assert_eq!(tiles, vec![at_big(-7, 8), at_big(0, -8)]);
    assert_spaced(&w, &end.centres);
}

#[test]
fn the_tiers_fill_in_order_market_then_tended_then_fertile() {
    let mut w = big();
    // One market (tier a), one tended tile (tier b), one fertile tile (tier c).
    let m = market(&mut w, at_big(8, 0), 1);
    grain(&mut w, at_big(-8, 8), 5, 0.5);
    grain(&mut w, at_big(0, -8), 0, 1.8);
    // Fertile wild grain beside the market is passed over by spacing.
    grain(&mut w, at_big(6, 0), 0, 2.0);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.centres.len(), 3);
    assert_eq!(end.topped_up, end.centres);
    assert_eq!(end.centres[0], m);
    let tiles: Vec<usize> = end.centres.iter().map(|id| w.markets[id].tile).collect();
    assert_eq!(tiles, vec![at_big(8, 0), at_big(-8, 8), at_big(0, -8)]);
    assert_spaced(&w, &end.centres);
}

#[test]
fn a_sealed_or_too_close_candidate_is_skipped() {
    let mut w = big();
    let a = market(&mut w, at_big(8, 0), 5);
    // Busier, but in a sealed valley; and one too close to the first.
    let sealed = market(&mut w, at_big(-8, 8), 4);
    w.tiles[at_big(-8, 8)].country = Country::Sealed;
    let close = market(&mut w, at_big(2, 0), 3);
    let valley = market(&mut w, at_big(0, -8), 2);
    w.tiles[at_big(0, -8)].country = Country::Valley;
    let neck = market(&mut w, at_big(-9, 9), 1);
    w.tiles[at_big(-9, 9)].country = Country::Neck;
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.centres, vec![a, valley, neck], "valleys and necks are allowed");
    assert!(!end.centres.contains(&sealed) && !end.centres.contains(&close));
    // Raised markets are skipped there too.
    let mut w = big();
    grain(&mut w, at_big(8, 0), 0, 1.0);
    grain(&mut w, at_big(-8, 8), 0, 2.0);
    w.tiles[at_big(-8, 8)].country = Country::Sealed;
    grain(&mut w, at_big(0, -8), 0, 0.5);
    grain(&mut w, at_big(-9, 9), 0, 0.4);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    let tiles: Vec<usize> = end.centres.iter().map(|id| w.markets[id].tile).collect();
    assert_eq!(tiles, vec![at_big(8, 0), at_big(0, -8), at_big(-9, 9)]);
}

#[test]
fn topped_up_lists_exactly_the_new_ids() {
    let mut w = big();
    let (ca, a) = centre(&mut w, at_big(8, 0), 4.0);
    w.turn = 40;
    assert_eq!(centres(&w), vec![ca]);
    let before: Vec<u32> = w.markets.keys().copied().collect();
    grain(&mut w, at_big(-8, 8), 4, 1.0);
    grain(&mut w, at_big(0, -8), 3, 1.0);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.centres.len(), 3);
    assert_eq!(end.centres[0], a);
    assert_eq!(end.topped_up, end.centres[1..].to_vec());
    assert!(end.topped_up.iter().all(|id| !before.contains(id)), "both were raised");
}

#[test]
fn nothing_to_top_up_with_ends_short_and_a_full_set_is_left_alone() {
    let mut w = big();
    let a = market(&mut w, at_big(8, 0), 4);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!((end.centres, end.topped_up), (vec![a], vec![a]));
    // Three centres with the cover short: by the limit, nothing added.
    let (mut w, _, ms) = three([4.0, 4.0, 4.0]);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert!(end.by_limit);
    assert_eq!((end.centres, end.topped_up), (ms.to_vec(), vec![]));
}

#[test]
fn topped_up_markets_survive_the_turns_after_the_end() {
    let mut w = big();
    let a = market(&mut w, at_big(8, 0), 1);
    grain(&mut w, at_big(-8, 8), 5, 1.0);
    grain(&mut w, at_big(0, -8), 5, 1.0);
    // A market that nobody trades at, for contrast: it fades at once.
    let doomed = market(&mut w, at_big(0, 8), 0);
    w.markets.get_mut(&doomed).unwrap().activity = 0.0;
    w.turn = PREGAME.max_turns - 1;
    w.run(1);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.centres.len(), 3);
    assert_eq!(end.centres[0], a);
    assert!(!w.markets.contains_key(&doomed));
    for id in &end.centres {
        w.markets.get_mut(id).unwrap().activity = 0.0;
    }
    w.run(5);
    for id in &end.centres {
        assert!(w.markets.contains_key(id), "centre {id} faded");
    }
}

fn spaced(w: &World) {
    let end = w.end.as_ref().unwrap();
    assert_eq!(end.centres.len(), PREGAME.players, "seed {} radius {}", w.config.seed, w.config.radius);
    for a in &end.centres {
        for b in &end.centres {
            if a != b {
                let d = steps(w, w.markets[a].tile, w.markets[b].tile);
                assert!(d >= PREGAME.centre_spacing, "seed {} radius {}: {d} steps", w.config.seed, w.config.radius);
            }
        }
    }
}

fn pregame_ends_with_three(radius: i32, seeds: std::ops::RangeInclusive<u32>) {
    for seed in seeds {
        let mut w = World::new(Config { seed, radius, army: true, gifts: true, zone: true });
        w.run(PREGAME.max_turns);
        assert!(w.end.is_some(), "seed {seed} radius {radius}: no end");
        spaced(&w);
    }
}

#[test]
fn small_pregames_always_end_with_three_spaced_centres() {
    pregame_ends_with_three(size_radius("small").unwrap(), 1..=5);
}

#[test]
fn medium_pregames_always_end_with_three_spaced_centres() {
    pregame_ends_with_three(size_radius("medium").unwrap(), 1..=2);
}

#[test]
fn large_pregames_always_end_with_three_spaced_centres() {
    pregame_ends_with_three(size_radius("large").unwrap(), 1..=1);
}

#[test]
fn with_no_centres_a_region_too_small_for_three_is_passed_over() {
    let mut w = big();
    // A column of sea splits the map; the east is three tiles, where the most populated clan
    // lives. The west has the room.
    let east = [at_big(1, 0), at_big(2, 0), at_big(3, 0)];
    for i in 0..w.tiles.len() {
        let q = w.tiles[i].q;
        if q == 0 || (q > 0 && !east.contains(&i)) {
            w.tiles[i].terrain = SEA;
        } else if q < 0 && w.tiles[i].terrain == PLAINS {
            grain(&mut w, i, 0, 1.0 + (i % 7) as f64 / 10.0);
        }
    }
    put(&mut w, &[east[1]], 20.0);
    grain(&mut w, east[0], 0, 2.0);
    at_the_limit(&mut w);
    let end = w.end.clone().expect("ended");
    assert_eq!(end.centres.len(), 3);
    // Read where each civ was founded.
    let founded: Vec<_> = w.events.iter().filter(|e| e.kind == engine::EventKind::CivFounded).collect();
    assert_eq!(founded.len(), 3);
    assert!(founded.iter().all(|e| w.tiles[e.tile].q < 0), "all in the west");
}
