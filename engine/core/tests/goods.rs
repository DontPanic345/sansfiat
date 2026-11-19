//! Goods (`docs/design/goods.md`): the table, sources, the source job, salt in a store, obsidian at
//! the raising, and the market's worth.

mod common;

use common::*;
use engine::actors::{self, civ_worths, worths};
use engine::data::*;
use engine::market::{clear, take, total, Basket, Params};
use engine::units::{self, fight};
use engine::{sources, Config, World};

fn basket(food: f64, salt: f64, obsidian: f64) -> Basket {
    let mut b = [0.0; N];
    b[GRAIN] = food;
    b[SALT] = salt;
    b[OBSIDIAN] = obsidian;
    b
}

#[test]
fn every_good_has_a_use_and_only_food_is_eaten() {
    assert_eq!(GOODS, ["game", "grain", "fruit", "fish", "shellfish", "animals", "salt", "obsidian"]);
    for g in &GOOD {
        assert_eq!(g.eaten, matches!(g.uses, Use::Food | Use::Herd), "{}", g.name);
    }
    let mut b = basket(10.0, 3.0, 2.0);
    assert_eq!(total(&b), 10.0, "total is the food");
    let got = take(&mut b, 4.0);
    assert_eq!((total(&got), got[SALT], got[OBSIDIAN]), (4.0, 0.0, 0.0), "take takes food only");
    assert_eq!((b[SALT], b[OBSIDIAN]), (3.0, 2.0));
}

#[test]
fn nobody_eats_salt() {
    let mut a = actors::Actor::new(0, 4.0);
    a.store = basket(1.0, 5.0, 5.0);
    let got = a.eat_by_diet(4.0);
    assert!(got <= 1.0 + 1e-9, "fed {got} from 1 food");
    assert_eq!((a.store[SALT], a.store[OBSIDIAN]), (5.0, 5.0));
}

#[test]
fn salt_slows_a_stores_spoiling_and_is_used_up() {
    let mut bare = basket(20.0, 0.0, 1.0);
    let lost_bare = actors::spoil(&mut bare);
    assert!((lost_bare - 2.0).abs() < 1e-9, "a store with no salt loses a tenth: {lost_bare}");
    assert_eq!(bare[OBSIDIAN], 1.0, "obsidian keeps");
    // One salt keeps 10 of the 20 food at half its spoiling.
    let mut salted = basket(20.0, 1.0, 0.0);
    let lost = actors::spoil(&mut salted);
    assert!((lost - 1.5).abs() < 1e-9, "lost {lost}");
    assert!(salted[SALT].abs() < 1e-9, "the salt is used up: {}", salted[SALT]);
    // More salt than the food needs: only what kept food is used.
    let mut rich = basket(20.0, 5.0, 0.0);
    let lost = actors::spoil(&mut rich);
    assert!((lost - 1.0).abs() < 1e-9, "lost {lost}");
    assert!((rich[SALT] - 3.0).abs() < 1e-9, "2 of 5 salt used: {}", rich[SALT]);
}

#[test]
fn a_clan_wants_salt_by_its_spoiling_and_a_civ_wants_both() {
    // Four people with only their need: nothing will spoil, so salt is worth nothing to them.
    assert_eq!(worths(&basket(4.0, 0.0, 0.0), 4.0)[SALT], 0.0);
    let some = worths(&basket(6.0, 0.0, 0.0), 4.0)[SALT];
    let more = worths(&basket(12.0, 0.0, 0.0), 4.0)[SALT];
    assert!(some > 0.0 && more > some && more <= 1.0, "{some} {more}");
    // Salt it holds already covers the loss.
    assert_eq!(worths(&basket(12.0, 1.0, 0.0), 4.0)[SALT], 0.0);
    assert_eq!(worths(&basket(12.0, 0.0, 0.0), 4.0)[OBSIDIAN], 0.0, "a clan has no use for obsidian");
    let civ = civ_worths(&[0.0; N], CIV.store_cap);
    assert_eq!(
        civ_worths(&basket(0.0, 0.0, ARMS.hold), CIV.store_cap)[OBSIDIAN],
        0.0,
        "a store with its fill wants none"
    );
    assert!(civ[SALT] > 0.0 && civ[OBSIDIAN] > 0.0);
}

#[test]
fn a_festival_moves_salt_to_who_wants_it_and_obsidian_only_to_a_civ() {
    let holder = basket(1.0, 2.0, 2.0);
    let other = basket(3.0, 0.0, 0.0);
    let w = |b: &Basket, people: f64| worths(b, people);
    // Two clans: the other's store would spoil, so it buys salt; nobody wants obsidian, so it stays.
    let brought = [holder, other];
    let c = clear(&brought, &[w(&holder, 4.0), w(&other, 1.0)], &[1.0; N], &Params::default());
    assert!(c.cleared);
    assert!(c.after[1][SALT] > 0.1, "salt moved: {}", c.after[1][SALT]);
    assert!(c.after[0][GRAIN] > 1.0, "for food");
    assert_eq!(c.after[0][OBSIDIAN], 2.0, "obsidian nobody wants is not traded");
    assert!(!c.goods.contains(&OBSIDIAN));
    // With a civ's store at the market the obsidian goes to it.
    let store = basket(5.0, 0.0, 0.0);
    let brought = [holder, other, store];
    let c = clear(
        &brought,
        &[w(&holder, 4.0), w(&other, 1.0), civ_worths(&store, CIV.store_cap)],
        &[1.0; N],
        &Params::default(),
    );
    assert!(c.after[2][OBSIDIAN] > 1.9, "the civ took the obsidian: {}", c.after[2][OBSIDIAN]);
    assert!(c.after[1][OBSIDIAN].abs() < 1e-9);
    for g in 0..N {
        let (before, after): (f64, f64) = (brought.iter().map(|b| b[g]).sum(), c.after.iter().map(|b| b[g]).sum());
        assert!((before - after).abs() < 1e-9, "{} conserved", GOODS[g]);
    }
}

#[test]
fn sources_lie_on_their_ground_and_the_control_has_none() {
    let w = World::new(Config { seed: 2, ..Config::default() });
    let (salt, obsidian) = (sources::of(&w, SALT), sources::of(&w, OBSIDIAN));
    assert_eq!((salt.len(), obsidian.len()), (SOURCE.salt, SOURCE.obsidian), "a small map's count");
    assert!(salt.iter().all(|&t| [COAST, MARSH].contains(&w.tiles[t].terrain)));
    assert!(obsidian.iter().all(|&t| w.tiles[t].terrain == HILLS));
    let off = World::new(Config { seed: 2, goods: false, ..Config::default() });
    assert!(off.tiles.iter().all(|t| t.source.is_none()));
    // Sources move nothing else on the map.
    assert!(w
        .tiles
        .iter()
        .zip(&off.tiles)
        .all(|(a, b)| (a.terrain, a.fertility, a.river) == (b.terrain, b.fertility, b.river)));
}

#[test]
fn a_clan_that_holds_a_source_gathers_it_as_a_job() {
    let mut w = flat();
    let home = at(0, 0);
    let id = put(&mut w, &[home, at(1, 0)], 6.0);
    end_turn(&mut w);
    assert_eq!(w.clans[&id].store[SALT], 0.0, "no source, no salt");
    for t in w.clans[&id].territory.clone() {
        w.tiles[t].source = Some(SALT);
    }
    let c = w.clans.get_mut(&id).unwrap();
    (c.store, c.starving) = ([0.0; N], false);
    end_turn(&mut w);
    let c = &w.clans[&id];
    // The basket is what it had at the harvest.
    assert!(c.basket[SALT] > 0.1, "it gathered salt: {}", c.basket[SALT]);
    assert!(c.basket[SALT] <= SOURCE.hold * c.people + 1e-9);
}

#[test]
fn obsidian_in_the_store_arms_the_warriors_raised_from_it() {
    let (mut w, ids, _) = three();
    let (red, blue) = (ids[0], ids[1]);
    let plain = fight::might(units::camp(&w, blue).unwrap());
    assert_eq!(units::camp(&w, red).unwrap().camp().unwrap().armed, 0.0);
    // Two more warriors, with obsidian for one and a half of them.
    let before = units::camp(&w, red).unwrap().people;
    let store = &mut w.civs.get_mut(&red).unwrap().store;
    store[OBSIDIAN] = 1.5 * ARMS.each;
    store[GRAIN] += 2.0 * units::pack_cap(1.0);
    for _ in 0..2 {
        let into = units::camp(&w, red).map(|u| u.id);
        units::raise(&mut w, red, 1, Fighter::Warriors, into, None).expect("a warrior");
    }
    assert!(w.civs[&red].store[OBSIDIAN].abs() < 1e-9, "the obsidian is used up at the raising");
    let u = units::camp(&w, red).unwrap();
    let armed = u.camp().unwrap().armed;
    assert!((armed - 1.5 / (before + 2.0)).abs() < 1e-9, "armed share {armed}");
    let strong = fight::might(u);
    assert!((strong.0 / plain.0 - (1.0 + (ARMS.edge - 1.0) * armed)).abs() < 1e-9, "{strong:?} against {plain:?}");
}
