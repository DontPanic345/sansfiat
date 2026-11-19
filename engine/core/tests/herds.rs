//! The herd (`herds.rs`, `jobs.rs`): the good, how a herd breeds and what its land carries, the notch,
//! fields or herds, a herd's start, wide land, the breeding stock kept out of the basket, a short clan's
//! lean, the herd that goes with a daughter, herds off, and a staged game on plains that herds.

use engine::clans::{adjust, eat, harvest, new_clan, reap, walks};
use engine::data::*;
use engine::dividing::divide_land;
use engine::fields;
use engine::herds::{self, breed, carries, offtake, steady};
use engine::hex::Hex;
use engine::jobs::jobs;
use engine::map::index;
use engine::stage::Stage;
use engine::{EventKind, World};

const EPS: f64 = 1e-9;
const RADIUS: i32 = 8;

fn at(q: i32, r: i32) -> usize {
    index(RADIUS, Hex { q, r }).unwrap()
}

/// The civ's capital, far from where the tests put their clans.
const CAP: (i32, i32) = (5, -5);

/// A flat map of plains with fertility 1, a civ founded at `CAP`, and no food on any tile.
fn founded() -> Stage {
    let mut s = Stage::flat(RADIUS);
    s.civs(&[at(CAP.0, CAP.1)], 6.0);
    for t in &mut s.world_mut().tiles {
        t.fertility = 1.0;
    }
    s
}

/// Puts a clan of `people` on `tiles` (its home first), holding them, at herding share `herds`.
fn hold(w: &mut World, tiles: &[usize], people: f64, herds: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let mut c = new_clan(id, tiles[0], people);
    let mut territory = tiles.to_vec();
    territory.sort_unstable();
    c.territory = territory;
    c.herds = herds;
    for &t in tiles {
        w.tiles[t].claim = Some(id);
    }
    w.clans.insert(id, c);
    id
}

/// Four plains tiles around (-2, 0).
fn grass() -> Vec<usize> {
    vec![at(-2, 0), at(-1, 0), at(-2, 1), at(-1, -1)]
}

/// Plays one turn's harvest and meal, keeping the clan's people as they are.
fn turn(w: &mut World, id: u32) {
    let people = w.clans[&id].people;
    harvest(w);
    eat(w);
    w.clans.get_mut(&id).unwrap().people = people;
}

#[test]
fn animals_are_a_food_that_keeps_and_breeds_nowhere_in_the_wild() {
    let a = &GOOD[ANIMALS];
    assert_eq!((a.name, a.eaten, a.spoil, a.weight, a.uses), ("animals", true, 0.0, 0.5, Use::Herd));
    assert_eq!(FOODS.len(), 6);
    assert_eq!((SALT, OBSIDIAN, N), (6, 7, 8));
    for t in &TERRAIN {
        assert_eq!(t.food[ANIMALS], 0.0, "{}: no tile has a wild herd", t.name);
    }
    let mut store = [0.0; N];
    store[ANIMALS] = 10.0;
    let lost = engine::actors::spoil(&mut store);
    assert_eq!((store[ANIMALS], lost), (10.0, 0.0), "a herd spoils nothing");
}

#[test]
fn a_herd_breeds_toward_what_its_land_carries_and_no_further() {
    for k in [5.0, 40.0] {
        let mut h = 1.0;
        for _ in 0..300 {
            let inc = breed(h, k);
            assert!(inc >= 0.0 && h + inc <= k + EPS, "h {h}, k {k}, inc {inc}");
            h += inc;
        }
        assert!((h - k).abs() < 1e-6, "{h} reaches {k}");
    }
    assert_eq!(breed(0.0, 10.0), 0.0, "no herd, no increase");
    assert!((breed(5.0, 10.0) - HERD.breed * 5.0 * 0.5).abs() < EPS);
    assert!((steady(10.0) - HERD.breed * 10.0 / 4.0).abs() < EPS, "a herd at half its land yields most");
    assert!((breed(5.0, 10.0) - steady(10.0)).abs() < EPS);
}

#[test]
fn above_what_it_carries_a_herd_falls_to_it_and_no_further() {
    assert!(breed(11.0, 10.0) < 0.0 && 11.0 + breed(11.0, 10.0) > 10.0, "a little over falls toward k");
    assert_eq!(100.0 + breed(100.0, 10.0), 10.0, "far over falls to k, not below");
    let mut h = 20.0;
    for _ in 0..80 {
        let inc = breed(h, 10.0);
        assert!(inc < 0.0 && h + inc >= 10.0, "h {h}");
        h += inc;
    }
    assert!((h - 10.0).abs() < 1e-6, "it settles at k: {h}");
    assert_eq!(5.0 + breed(5.0, 0.0), 0.0, "with nothing carried the herd falls to 0");
    assert_eq!(offtake(20.0, breed(20.0, 10.0), 10.0), 0.0, "a clan takes nothing of a fall");
    assert_eq!(offtake(5.0, 1.5, 10.0), 1.5, "and all of an increase");
}

#[test]
fn what_a_herd_carries_is_capped_by_the_herding_days_and_by_the_grass() {
    let mut s = founded();
    let w = s.world_mut();
    let hills = at(-1, 0);
    w.tiles[hills].terrain = HILLS;
    let plains = at(-2, 0);
    let sea = at(RADIUS, 0);
    assert_eq!(herds::graze(w, plains), HERD.graze[PLAINS]);
    assert_eq!(herds::graze(w, hills), HERD.graze[HILLS]);
    assert_eq!(herds::graze(w, sea), 0.0, "the sea carries none");
    w.tiles[plains].fertility = 0.5;
    assert_eq!(herds::graze(w, plains), HERD.graze[PLAINS] * 0.5, "grass scales by fertility");
    w.tiles[plains].fertility = 1.0;
    let few = 1.0;
    let many = 100.0;
    let k = carries(w, &[plains, hills], &[few, many]);
    assert!((k - (HERD.tend * few).min(HERD.graze[PLAINS]) - HERD.graze[HILLS]).abs() < EPS, "{k}");
    assert!((carries(w, &[plains], &[many]) - HERD.graze[PLAINS]).abs() < EPS, "the grass binds");
    assert!((carries(w, &[plains], &[0.5]) - 0.5 * HERD.tend).abs() < EPS, "the days bind");

    // Through a clan's reap: a small share keeps few, a whole one keeps all the grass.
    let id = hold(w, &grass(), 10.0, 0.0);
    let c = w.clans[&id].clone();
    let (_, yearly) = walks(w, c.home, &c.territory);
    let gathers = vec![true; c.territory.len()];
    let mut last = 0.0;
    for share in [0.1, 0.3, 0.6] {
        let r = reap(w, id, 10.0, &[0.0; N], &[], &c.territory, &yearly, 0.0, share, &gathers);
        let j = jobs(w, 10.0, &[], &c.territory, &yearly, 0.0, share, &gathers, false);
        let tiles: Vec<usize> = j.hk.iter().map(|&k| c.territory[k]).collect();
        assert_eq!(r.carry, carries(w, &tiles, &j.herd), "share {share}");
        assert!(r.carry > last - EPS && r.carry <= 4.0 * HERD.graze[PLAINS] + EPS, "share {share}: {}", r.carry);
        last = r.carry;
    }
    let days: f64 = jobs(w, 10.0, &[], &c.territory, &yearly, 0.0, 0.1, &gathers, false).herd.iter().sum();
    let small = reap(w, id, 10.0, &[0.0; N], &[], &c.territory, &yearly, 0.0, 0.1, &gathers).carry;
    assert!(
        (small - days * HERD.tend).abs() < 1e-6 && small < 4.0 * HERD.graze[PLAINS],
        "days bind at a tenth: {small}"
    );
    // A big clan on one tile, all its days on herding: the grass binds, whatever the days.
    let big = hold(w, &[at(-3, 3)], 50.0, 1.0);
    let (_, yearly) = walks(w, at(-3, 3), &[at(-3, 3)]);
    let r = reap(w, big, 50.0, &[0.0; N], &[], &[at(-3, 3)], &yearly, 0.0, 1.0, &[true]);
    assert!((r.carry - HERD.graze[PLAINS]).abs() < 1e-6, "the grass binds: {}", r.carry);
}

#[test]
fn a_herd_with_nothing_to_spare_breeds_to_its_land_and_stops() {
    let mut s = founded();
    let w = s.world_mut();
    let id = hold(w, &grass(), 6.0, 1.0);
    let (_, yearly) = walks(w, w.clans[&id].home, &w.clans[&id].territory.clone());
    let territory = w.clans[&id].territory.clone();
    let k = reap(w, id, 6.0, &[0.0; N], &[], &territory, &yearly, 0.0, 1.0, &[true; 4]).carry;
    assert!(k > 5.0 && k <= 4.0 * HERD.graze[PLAINS] + EPS, "{k}");
    // A full store leaves the budget nothing to take: the whole increase stays in the herd.
    let c = w.clans.get_mut(&id).unwrap();
    c.store[GRAIN] = 100.0;
    c.store[ANIMALS] = 2.0;
    let mut last = 2.0;
    for t in 0..60 {
        let c = w.clans.get_mut(&id).unwrap();
        c.store[GRAIN] = 100.0;
        turn(w, id);
        let h = w.clans[&id].store[ANIMALS];
        assert!(h >= last - EPS && h <= k + 1e-6, "turn {t}: {last} to {h} of {k}");
        last = h;
    }
    assert!((last - k).abs() < 0.01 * k, "it reaches {k}: {last}");

    // Above its land it falls to it.
    w.clans.get_mut(&id).unwrap().store[ANIMALS] = 3.0 * k;
    w.clans.get_mut(&id).unwrap().store[GRAIN] = 100.0;
    turn(w, id);
    assert!(w.clans[&id].store[ANIMALS] < 3.0 * k && w.clans[&id].store[ANIMALS] > k, "it falls toward k");
    for _ in 0..40 {
        w.clans.get_mut(&id).unwrap().store[GRAIN] = 100.0;
        turn(w, id);
    }
    let h = w.clans[&id].store[ANIMALS];
    assert!((h - k).abs() < 1e-4 && h >= k - EPS, "it falls to {k} and stops: {h}");
}

#[test]
fn a_clan_with_fields_never_raises_herds_and_one_with_herds_never_raises_fields() {
    // Hunted-out grass: herding is the only food a clan can add.
    let mut s = founded();
    let w = s.world_mut();
    let free = hold(w, &grass(), 10.0, 0.0);
    herds::notch(w, free);
    assert!(w.clans[&free].herds > 0.0, "a clan with nothing else to eat takes up herding on bare grass");

    let mut s = founded();
    let w = s.world_mut();
    let farmer = hold(w, &grass(), 10.0, 0.0);
    w.clans.get_mut(&farmer).unwrap().fields = 0.3;
    for _ in 0..5 {
        herds::notch(w, farmer);
    }
    assert_eq!(w.clans[&farmer].herds, 0.0, "a clan with fields never raises herds");

    // And a herder's field notch does nothing, even on ground that would feed it.
    let mut s = founded();
    let w = s.world_mut();
    let tiles = grass();
    let t = tiles[0];
    w.tiles[t].flood = true;
    w.tiles[t].wild_grain = 10.0;
    w.tiles[t].tame = TENDING.tame_turns;
    w.tiles[t].unworn[GRAIN] = 10.0 * engine::land::domestic(TENDING.tame_turns);
    w.tiles[t].cap[GRAIN] = w.tiles[t].unworn[GRAIN];
    let control = hold(w, &tiles, 10.0, 0.0);
    fields::notch(w, control);
    assert!(w.clans[&control].fields > 0.0, "the same land raises fields for a clan without herds");
    let herder = hold(w, &[at(-3, 0), at(-3, 1)], 10.0, 0.5);
    w.tiles[at(-3, 0)].flood = true;
    for _ in 0..5 {
        fields::notch(w, herder);
    }
    assert_eq!(w.clans[&herder].fields, 0.0, "a clan with herds never raises fields");
}

#[test]
fn a_starting_herd_comes_out_of_the_game_on_the_clans_tiles() {
    let mut s = founded();
    let w = s.world_mut();
    let (a, b, other) = (at(-2, 0), at(-1, 0), at(2, 2));
    let id = hold(w, &[a, b], 10.0, 0.0);
    let rival = hold(w, &[other], 4.0, 0.0);
    w.tiles[a].stock[GAME] = 15.0;
    w.tiles[b].stock[GAME] = 5.0;
    w.tiles[other].stock[GAME] = 9.0;
    herds::start(w, id);
    let want = HERD.start * 10.0;
    assert_eq!(w.clans[&id].store[ANIMALS], want);
    assert!((w.tiles[a].stock[GAME] - (15.0 - want * 0.75)).abs() < 1e-9, "paid in proportion to the stock");
    assert!((w.tiles[b].stock[GAME] - (5.0 - want * 0.25)).abs() < 1e-9);
    assert_eq!(w.tiles[other].stock[GAME], 9.0, "another clan's game is not touched");
    assert_eq!(w.clans[&rival].store[ANIMALS], 0.0);
    herds::start(w, id);
    assert_eq!(w.clans[&id].store[ANIMALS], want, "a clan that has a herd does not start another");

    // Game hunted out: the herd still starts, from beyond its land, and the tiles lose what they had.
    let poor = hold(w, &[at(-3, 2), at(-3, 3)], 10.0, 0.0);
    w.tiles[at(-3, 2)].stock[GAME] = 1.0;
    herds::start(w, poor);
    assert_eq!(w.clans[&poor].store[ANIMALS], want);
    assert_eq!(w.tiles[at(-3, 2)].stock[GAME], 0.0);
}

#[test]
fn the_first_turn_a_share_rises_from_nothing_starts_the_herd_and_tells_the_feed() {
    let mut s = founded();
    let w = s.world_mut();
    let id = hold(w, &grass(), 10.0, 0.0);
    herds::notch(w, id);
    let c = &w.clans[&id];
    assert_eq!((c.herds, c.store[ANIMALS]), (HERD.notch, HERD.start * 10.0));
    let n = w.events.iter().filter(|e| e.kind == EventKind::ClanHerding).count();
    assert_eq!(n, 1);
    let e = w.events.iter().find(|e| e.kind == EventKind::ClanHerding).unwrap();
    assert!(e.text(w).starts_with(&format!("Clan {id} takes up herding at ")), "{}", e.text(w));
    herds::notch(w, id);
    assert_eq!(w.events.iter().filter(|e| e.kind == EventKind::ClanHerding).count(), 1, "once, as the share rises");
}

#[test]
fn a_herders_territory_reaches_radius_three_and_a_gatherers_stays_at_two() {
    assert_eq!((HERD.radius, CLAN.territory_radius), (3, 2));
    for (share, radius) in [(0.0, 2), (HERD.herder - 0.05, 2), (HERD.herder, 3), (1.0, 3)] {
        let mut c = new_clan(1, 0, 8.0);
        c.herds = share;
        assert_eq!(herds::radius(&c), radius, "share {share}");
    }
    let reach = |herding: f64| {
        let mut s = founded();
        let w = s.world_mut();
        let home = at(-2, 0);
        let id = hold(w, &[home], 40.0, herding);
        // Grass and game all around: land worth adding.
        for t in &mut w.tiles {
            if TERRAIN[t.terrain].land {
                t.cap[GAME] = 6.0;
                t.unworn[GAME] = 6.0;
                t.stock[GAME] = 6.0;
            }
        }
        w.clans.get_mut(&id).unwrap().store[ANIMALS] = if herding > 0.0 { 4.0 } else { 0.0 };
        // The farthest tile it holds from its home in any turn.
        let mut far = 0;
        for _ in 0..40 {
            w.clans.get_mut(&id).unwrap().herds = herding;
            adjust(w);
            let c = &w.clans[&id];
            far = far.max(c.territory.iter().map(|&t| w.tiles[t].hex().distance(w.tiles[c.home].hex())).max().unwrap());
        }
        far
    };
    assert!(reach(0.0) <= 2, "a gatherer holds to 2");
    assert_eq!(reach(0.5), 3, "a herder reaches 3");
}

#[test]
fn the_breeding_stock_is_never_in_the_basket() {
    let mut s = founded();
    let w = s.world_mut();
    let id = hold(w, &grass(), 6.0, 0.5);
    w.clans.get_mut(&id).unwrap().store[ANIMALS] = 5.0;
    harvest(w);
    let c = &w.clans[&id];
    assert!(c.harvest[ANIMALS] > 0.0, "the increase is harvested");
    assert_eq!(c.basket[ANIMALS], c.harvest[ANIMALS], "the basket holds the increase, not the herd");
    assert!(c.store[ANIMALS] >= 5.0 - EPS, "the herd is the store's: {}", c.store[ANIMALS]);
    assert!(c.basket[ANIMALS] < c.store[ANIMALS]);
    // At the go step the clan weighs a festival by what it would bring.
    let festivals = engine::markets::worths(w, c);
    let _ = festivals;
}

#[test]
fn a_short_clan_eats_at_most_a_lean_share_of_its_herd() {
    for (herd, basket) in [(20.0, 0.0), (4.0, 0.0), (20.0, 7.0), (100.0, 0.0)] {
        let mut s = founded();
        let w = s.world_mut();
        let id = hold(w, &grass(), 6.0, 0.5);
        let c = w.clans.get_mut(&id).unwrap();
        c.store[ANIMALS] = herd;
        c.basket = [0.0; N];
        c.basket[GRAIN] = basket;
        eat(w);
        let c = &w.clans[&id];
        let eaten = herd - c.store[ANIMALS];
        assert!(eaten >= 0.0 && eaten <= HERD.lean * herd + EPS, "herd {herd}, basket {basket}: ate {eaten}");
        if basket == 0.0 {
            assert!(eaten > 0.0 && c.fed_ratio > 0.0, "the animals fed it: ate {eaten}, fed {}", c.fed_ratio);
        }
    }
    // A clan fed enough leaves its herd alone, and animals left in its basket go back to it.
    let mut s = founded();
    let w = s.world_mut();
    let id = hold(w, &grass(), 6.0, 0.5);
    let c = w.clans.get_mut(&id).unwrap();
    c.store[ANIMALS] = 20.0;
    c.basket = [0.0; N];
    c.basket[GRAIN] = 7.0;
    c.basket[ANIMALS] = 30.0;
    eat(w);
    let c = &w.clans[&id];
    assert!(c.fed, "fed");
    assert!(c.store[ANIMALS] > 20.0 + 15.0, "the animals it didn't eat go back to the herd: {}", c.store[ANIMALS]);
    assert!(c.store[ANIMALS] < 20.0 + 30.0, "and it ate some");
}

#[test]
fn a_daughter_takes_half_the_herd_and_her_mothers_share() {
    let mut s = founded();
    let w = s.world_mut();
    let tiles = vec![at(-3, 0), at(-2, 0), at(-1, 0), at(-2, 1), at(-1, -1), at(-3, 1)];
    let id = hold(w, &tiles, 16.0, 0.4);
    w.clans.get_mut(&id).unwrap().store[ANIMALS] = 12.0;
    divide_land(w, id);
    let d = w.clans.values().find(|c| c.parent == Some(id)).expect("a daughter");
    assert_eq!(d.store[ANIMALS], 6.0, "half the herd");
    assert_eq!(w.clans[&id].store[ANIMALS], 6.0);
    assert_eq!(d.herds, 0.4, "the mother's share");
}

#[test]
fn a_clan_that_moves_on_keeps_its_herd_and_its_share() {
    let mut s = founded();
    let w = s.world_mut();
    let home = at(-2, 0);
    let id = hold(w, &[home], 6.0, 0.5);
    // Its land is bare and a rich site lies within range: after `patience` starving turns it moves on.
    let site = at(-2, 2);
    for t in [site, at(-2, 1), at(-1, 1)] {
        w.tiles[t].cap[GAME] = 30.0;
        w.tiles[t].unworn[GAME] = 30.0;
        w.tiles[t].stock[GAME] = 30.0;
    }
    let c = w.clans.get_mut(&id).unwrap();
    c.store[ANIMALS] = 3.0;
    c.starving = true;
    c.hungry = CLAN.patience;
    c.fields = 0.0;
    adjust(w);
    let c = &w.clans[&id];
    assert_ne!(c.home, home, "it moved on");
    assert_eq!((c.herds, c.store[ANIMALS], c.since), (0.5, 3.0, 0), "its herd walks with it");
}

#[test]
fn with_herds_off_nothing_herds() {
    for off in [true, false] {
        let mut s = founded();
        let w = s.world_mut();
        w.config.herds = !off;
        let id = hold(w, &grass(), 10.0, 0.0);
        for _ in 0..3 {
            herds::notch(w, id);
        }
        assert_eq!(w.clans[&id].herds > 0.0, !off, "herds off is the control");
        if off {
            assert_eq!(w.clans[&id].store[ANIMALS], 0.0);
            let c = w.clans[&id].clone();
            let (_, yearly) = walks(w, c.home, &c.territory);
            let r = reap(w, id, 10.0, &[0.0; N], &[], &c.territory, &yearly, 0.0, 0.5, &[true; 4]);
            assert_eq!((r.herd, r.change, r.carry), (0.0, 0.0, 0.0), "no herding job");
        }
    }
    // Before the pick no clan herds either.
    let mut s = Stage::flat(RADIUS);
    let id = hold(s.world_mut(), &grass(), 10.0, 0.0);
    let w = s.world_mut();
    herds::notch(w, id);
    assert_eq!(w.clans[&id].herds, 0.0);
}

/// The turn, counted from the stage's start, by which the staged clan herds: it holds animals and has
/// eaten some. `www/play/herds.mjs` caps its play at it.
const HERDING_BY: u32 = 10;

#[test]
fn on_plains_with_no_river_a_clan_takes_up_herding_and_eats_animals() {
    let mut s = Stage::flat(RADIUS);
    s.civs(&[at(0, 0)], 6.0);
    let w = s.world_mut();
    // The map's own food on its plains: game and grain, from the terrain table.
    for t in &mut w.tiles {
        if TERRAIN[t.terrain].land {
            t.fertility = 1.0;
            t.cap = TERRAIN[t.terrain].food;
            t.unworn = t.cap;
            t.stock = t.cap;
        }
    }
    let id = *w.clans.keys().next().unwrap();
    let mut ate = 0.0;
    for _ in 0..HERDING_BY {
        w.run(1);
        ate += w.clans[&id].harvest[ANIMALS];
    }
    let c = &w.clans[&id];
    assert!(c.herds > 0.0, "its herding share is above 0 by turn {HERDING_BY}: {}", c.herds);
    assert!(c.store[ANIMALS] > 0.0, "it holds animals");
    assert!(ate > 0.0, "it ate animals: {ate}");
    assert_eq!(c.fields, 0.0, "no river, no fields");
    assert_eq!(w.events.iter().filter(|e| e.kind == EventKind::ClanHerding && e.clan == Some(id)).count(), 1);
}
