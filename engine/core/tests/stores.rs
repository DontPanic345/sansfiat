//! Supply from any store, staged on bare plains: Red's capital at (−6, 0), its settlement at (−1, 0),
//! Blue's capital at (6, 0). A line ends at the store that feeds the unit for the least labour and
//! draws on it alone, the ring is every store's, a cut line looks for another store, a unit is raised
//! at a store from its market's clans, and the planner's front is its store nearest a rival.

use engine::clans::new_clan;
use engine::hex::Hex;
use engine::homes;
use engine::map::index;
use engine::market::total;
use engine::stage::Stage;
use engine::units::{self, Decision, UnitKind};
use engine::{settle, Action, Order, World};

const RADIUS: i32 = 9;

fn tile(q: i32, r: i32) -> usize {
    index(RADIUS, Hex { q, r }).unwrap()
}

const FOOD: [f64; 8] = [40.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

/// Red with a settlement on (−1, 0) founded by a clan of 4 that goes to its market, both stores
/// holding 40 food; and Blue.
fn stage() -> (Stage, u32, u32, usize) {
    let mut stage = Stage::flat(RADIUS);
    let ids = stage.civs(&[tile(-6, 0), tile(6, 0)], 5.0);
    let (red, blue) = (ids[0], ids[1]);
    let w = stage.world_mut();
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    w.clans.clear();
    for t in &mut w.tiles {
        t.claim = None;
    }
    let t = tile(-1, 0);
    w.order(red, Order::Signpost { tile: t }).unwrap();
    let id = w.next_id;
    w.next_id += 1;
    let mut c = new_clan(id, t, 4.0);
    c.civ = Some(red);
    homes::claim_land(w, id, &[t]);
    w.clans.insert(id, c);
    settle::step(w);
    let s = settle::at(w, t).expect("a settlement");
    let (sid, market) = (s.id, s.market);
    w.clans.get_mut(&id).unwrap().market = Some(market);
    w.settlements.get_mut(&sid).unwrap().store = FOOD;
    w.civs.get_mut(&red).unwrap().store = FOOD;
    (stage, red, blue, t)
}

fn settlement_store(w: &World, t: usize) -> f64 {
    total(&settle::at(w, t).unwrap().store)
}

#[test]
fn a_line_ends_at_the_nearest_store_that_holds_a_turn_of_food() {
    let (mut stage, red, _, t) = stage();
    let army = stage.army(red, tile(1, 0), 3.0);
    let mut w = stage.into_world();
    let capital = w.civs[&red].capital;
    let (path, cost, _) = units::line(&w, army).unwrap();
    assert_eq!(path.last(), Some(&t), "the settlement is two steps off, the capital seven");
    assert_eq!(units::store_of(&w, army), Some(t));
    // An empty settlement feeds nobody: the line runs on to the capital, and costs more.
    let sid = settle::at(&w, t).unwrap().id;
    w.settlements.get_mut(&sid).unwrap().store = [0.0; 8];
    let (far, far_cost, _) = units::line(&w, army).unwrap();
    assert_eq!(far.last(), Some(&capital));
    assert!(far_cost > cost);
    // With no store holding a turn of its food, the nearest is its line.
    w.civs.get_mut(&red).unwrap().store = [0.0; 8];
    assert_eq!(units::store_of(&w, army), Some(t));
}

#[test]
fn a_unit_draws_on_its_lines_store_alone() {
    // Twins: an army beside the settlement with a full pack, and with an empty one.
    let run = |empty: bool| {
        let (mut stage, red, _, t) = stage();
        let army = stage.army(red, tile(0, 0), 3.0);
        if empty {
            stage.world_mut().units.get_mut(&army).unwrap().actor.store = [0.0; 8];
        }
        stage.turn(1).unwrap();
        let w = stage.into_world();
        (total(&w.civs[&red].store), settlement_store(&w, t), total(&w.units[&army].store))
    };
    let (capital_full, settlement_full, _) = run(false);
    let (capital_empty, settlement_empty, pack) = run(true);
    assert!(pack > 0.0, "it filled its pack along its line");
    assert!(settlement_empty + 1.0 < settlement_full, "from the settlement's store");
    assert!((capital_empty - capital_full).abs() < 1e-9, "and none of it from the capital's");
}

#[test]
fn the_ring_is_every_stores_ring() {
    let (stage, red, _, t) = stage();
    let w = stage.into_world();
    let (ring, capital) = (units::ring(&w, red), units::ring_capital(&w, red));
    assert_eq!(ring[&t], 0.0, "a store is the middle of its ring");
    assert!(capital[&t] > 0.0);
    assert!(ring.iter().all(|(i, share)| *share <= capital[i] + 1e-9), "no tile is further than from the capital");
    let gained = ring.iter().filter(|(i, &share)| share < 1.0 && capital[*i] >= 1.0).count();
    assert!(gained > 0, "the settlement feeds tiles the capital can't");
}

#[test]
fn a_cut_line_looks_for_another_store() {
    let (mut stage, red, blue, t) = stage();
    let army = stage.army(red, tile(1, -1), 3.0);
    assert_eq!(units::store_of(stage.world(), army), Some(t));
    // Blue's land closes round the settlement: a line can't pass a rival's land.
    let round = [(0, 0), (-2, 0), (-1, 1), (-1, -1), (0, -1), (-2, 1)];
    let mut tiles: Vec<usize> = (0..6).map(|q| tile(q, 0)).collect();
    tiles.extend(round.iter().map(|&(q, r)| tile(q, r)));
    stage.order(blue, Order::Claim { tiles }).unwrap();
    let w = stage.into_world();
    let capital = w.civs[&red].capital;
    assert_eq!(units::store_of(&w, army), Some(capital), "its line runs round to the capital");
}

#[test]
fn a_unit_is_raised_at_a_store_from_its_markets_clans_with_a_pack_from_it() {
    let (stage, red, _, t) = stage();
    let mut w = stage.into_world();
    let raise = |kind, at| Order::Raise { n: 3, kind, fighter: Default::default(), into: None, at };
    let off = w.check_order(red, &raise(UnitKind::Camp, Some(tile(3, 3)))).unwrap_err();
    assert!(off.contains("raise at a store"), "{off}");
    let (capital_store, before) = (total(&w.civs[&red].store), settlement_store(&w, t));
    let clan = w.clans.values().find(|k| k.home == t).unwrap().id;
    w.order(red, raise(UnitKind::Camp, Some(t))).unwrap();
    let army = units::camp(&w, red).expect("an army");
    assert_eq!((army.home, army.people), (t, 1.0), "one clan goes to that market, so one warrior");
    assert_eq!(w.clans[&clan].people, 3.0);
    assert!((settlement_store(&w, t) - (before - units::pack_cap(1.0))).abs() < 1e-9, "its pack is the settlement's");
    assert_eq!(total(&w.civs[&red].store), capital_store);
    // A scout too; and more warriors into the army come from the store its line ends at.
    w.order(red, raise(UnitKind::Scout, Some(t))).unwrap();
    assert!(w.units.values().any(|u| u.civ == red && u.scout().is_some() && u.home == t));
    let id = units::camp(&w, red).unwrap().id;
    let into = Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Default::default(), into: Some(id), at: None };
    w.order(red, into.clone()).unwrap();
    assert_eq!((w.clans[&clan].people, units::camp(&w, red).unwrap().people), (1.0, 2.0));
    assert_eq!(total(&w.civs[&red].store), capital_store);
    // No clan goes to the capital's market here, and the settlement's clan has nobody left to give.
    assert!(w.check_order(red, &raise(UnitKind::Camp, None)).unwrap_err().contains("no clan"));
    assert!(w.check_order(red, &into).unwrap_err().contains("settlement's market"));
}

#[test]
fn the_planner_posts_its_army_at_the_store_nearest_a_rival_and_the_army_goes_home_to_it() {
    let (mut stage, red, _, t) = stage();
    assert_eq!(units::post(stage.world(), red), t, "the settlement is nearer Blue than the capital");
    // A hungry army off its land marches to the store its line ends at.
    let army = stage.army(red, tile(1, 0), 3.0);
    let mut w = stage.into_world();
    {
        let u = w.units.get_mut(&army).unwrap();
        u.actor.store = [0.0; 8];
        u.hungry = 1;
        u.camp_mut().unwrap().dest = None;
    }
    w.tiles[tile(1, 0)].stock = [0.0; 8];
    let mut seen = Vec::new();
    units::plan(&mut w, red, &mut |w, civ, d| {
        seen.push(d.clone());
        match d {
            Decision::Order(order) => w.order(civ, order),
            Decision::Gift(food) => w.set_gift(civ, food),
        }
    });
    assert!(
        seen.iter().any(|d| matches!(d, Decision::Order(Order::Move { unit, to }) if *unit == army && *to == t)),
        "{seen:?}"
    );
}

#[test]
fn an_army_marching_past_a_settlement_arrives_fed() {
    // Twins march from the capital to (2, 0), eight steps: one past a stocked settlement, one past an empty one.
    let run = |stocked: bool| {
        let (mut stage, red, _, t) = stage();
        let capital = stage.world().civs[&red].capital;
        let army = stage.army(red, capital, 3.0);
        // Red is led, so the order stands.
        stage.world_mut().player = Some(red);
        if !stocked {
            let sid = settle::at(stage.world(), t).unwrap().id;
            stage.world_mut().settlements.get_mut(&sid).unwrap().store = [0.0; 8];
        }
        stage.order(red, Order::Move { unit: army, to: tile(2, 0) }).unwrap();
        let mut hungry = 0;
        for _ in 0..6 {
            stage.turn(1).unwrap();
            hungry += u32::from(stage.world().units.get(&army).is_none_or(|u| u.hungry > 0));
        }
        let at = stage.world().units.get(&army).map(|u| (u.home, total(&u.store)));
        (at, hungry)
    };
    let (fed, hungry) = run(true);
    let (pack_at, pack) = fed.expect("it lives");
    assert_eq!((pack_at, hungry), (tile(2, 0), 0), "it got there and never went hungry");
    assert!(pack > 0.0);
    let (bare, _) = run(false);
    assert!(bare.is_none_or(|(_, b)| pack > b + 1.0), "its pack is fuller for the settlement: {pack} against {bare:?}");
}

#[test]
fn the_planner_marches_on_a_rival_once_a_store_is_in_reach_of_it() {
    // Blue holds land before its capital, which an army too big to beat stands on. Red's army of six
    // is no party: it takes a rival's land only inside its ring.
    let decide = |settled: bool| {
        let (mut stage, red, blue, t) = stage();
        let land: Vec<usize> = (1..6).map(|q| tile(q, 0)).collect();
        stage.order(blue, Order::Claim { tiles: land.clone() }).unwrap();
        let capital = stage.world().civs[&red].capital;
        let army = stage.army(red, if settled { t } else { capital }, 6.0);
        let far = stage.world().civs[&blue].capital;
        stage.army(blue, far, 20.0);
        let mut w = stage.into_world();
        if !settled {
            w.settlements.clear();
        }
        w.units.get_mut(&army).unwrap().camp_mut().unwrap().dest = None;
        let mut marches = false;
        units::plan(&mut w, red, &mut |w, civ, d| {
            if let Decision::Order(Order::Move { unit, to }) = &d {
                marches |= *unit == army && land.contains(to);
            }
            match d {
                Decision::Order(order) => w.order(civ, order),
                Decision::Gift(food) => w.set_gift(civ, food),
            }
        });
        marches
    };
    assert!(decide(true), "Blue's land is inside the settlement's ring");
    assert!(!decide(false), "and outside the capital's");
}
