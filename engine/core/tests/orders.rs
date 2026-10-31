//! The orders: each is refused for a unit of another civ, a unit that is not there, a unit of the
//! wrong kind, and outside `Main`; the player's order is logged, the AI's are not, and `World::order`
//! works for any civ the game has.

mod common;

use common::{led_camp, led_order, led_scout};
use engine::units;
use engine::{Action, Config, Order, Phase, World};

/// A real map at its pick: three civs, each with a camp and a scout.
fn at_pick() -> World {
    let mut w = World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true });
    while w.phase != Phase::Choose {
        w.act(Action::EndTurn).unwrap();
    }
    w
}

/// The same map with the first civ led.
fn led() -> World {
    let mut w = at_pick();
    let me = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(me) }).unwrap();
    w
}

/// One order for each kind that names a camp, with `unit` as its unit.
fn camp_orders(unit: u32) -> Vec<Order> {
    vec![
        Order::Disband { unit, n: Some(1) },
        Order::Move { unit, to: 0 },
        Order::Zone { unit, tiles: Vec::new() },
        Order::Switches { unit, forage: true, supply: true },
        Order::Resupply { unit },
        Order::Forage { unit },
        Order::Pillage { unit },
    ]
}

/// One order for each kind that names a scout, with `unit` as its unit.
fn scout_orders(unit: u32) -> Vec<Order> {
    vec![Order::Route { unit, from: 0, to: 1 }, Order::ClearRoute { unit }]
}

#[test]
fn every_order_has_its_wire_name() {
    let orders = [
        Order::Raise { n: 1 },
        Order::Disband { unit: 1, n: None },
        Order::Move { unit: 1, to: 2 },
        Order::Zone { unit: 1, tiles: vec![2] },
        Order::Switches { unit: 1, forage: true, supply: false },
        Order::Resupply { unit: 1 },
        Order::Forage { unit: 1 },
        Order::Pillage { unit: 1 },
        Order::Route { unit: 1, from: 2, to: 3 },
        Order::ClearRoute { unit: 1 },
    ];
    let names: Vec<String> =
        orders.iter().map(|o| serde_json::to_value(o).unwrap()["type"].as_str().unwrap().to_string()).collect();
    assert_eq!(
        names,
        ["raise", "disband", "move", "zone", "switches", "resupply", "forage", "pillage", "route", "clearRoute"]
    );
    for o in &orders {
        let json = serde_json::to_string(o).unwrap();
        assert_eq!(&serde_json::from_str::<Order>(&json).unwrap(), o, "{json} reads back");
    }
}

#[test]
fn every_order_is_refused_for_a_unit_of_another_civ() {
    let mut w = led();
    let me = w.player.unwrap();
    let rival = *w.civs.keys().find(|&&c| c != me).unwrap();
    let camp = units::camp(&w, rival).unwrap().id;
    let scout = units::scout(&w, rival).unwrap().id;
    let before = w.clone();
    for order in camp_orders(camp).into_iter().chain(scout_orders(scout)) {
        assert_eq!(w.check_order(me, &order), Err("that unit is not yours".to_string()), "{order:?}");
        assert_eq!(
            w.validate(&Action::Order { civ: me, order: order.clone() }),
            Err("that unit is not yours".to_string()),
            "{order:?}"
        );
        assert!(w.order(me, order.clone()).is_err(), "{order:?}");
    }
    assert_eq!(w, before, "a refused order changes nothing");
}

#[test]
fn every_order_is_refused_for_a_unit_that_is_not_there() {
    let w = led();
    let me = w.player.unwrap();
    let missing = 999_999;
    for order in camp_orders(missing).into_iter().chain(scout_orders(missing)) {
        assert_eq!(w.check_order(me, &order), Err(format!("{missing} is not a unit")), "{order:?}");
    }
}

#[test]
fn every_order_is_refused_for_a_unit_of_the_wrong_kind() {
    let w = led();
    let me = w.player.unwrap();
    let (camp, scout) = (led_camp(&w), led_scout(&w));
    for order in camp_orders(scout) {
        assert_eq!(w.check_order(me, &order), Err("that unit is not a camp".to_string()), "{order:?}");
    }
    for order in scout_orders(camp) {
        assert_eq!(w.check_order(me, &order), Err("that unit is not a scout".to_string()), "{order:?}");
    }
}

#[test]
fn every_order_is_refused_before_the_game_has_begun() {
    let w = at_pick();
    assert_eq!(w.phase, Phase::Choose);
    let civ = *w.civs.keys().next().unwrap();
    let camp = units::camp(&w, civ).unwrap().id;
    let scout = units::scout(&w, civ).unwrap().id;
    let raise = Order::Raise { n: 1 };
    assert_eq!(w.check_order(civ, &raise), Err("the game has not begun".to_string()));
    for order in camp_orders(camp).into_iter().chain(scout_orders(scout)) {
        assert_eq!(w.check_order(civ, &order), Err("the game has not begun".to_string()), "{order:?}");
        assert_eq!(
            w.validate(&Action::Order { civ, order: order.clone() }),
            Err("the game has not begun".to_string()),
            "{order:?}"
        );
    }
}

#[test]
fn the_players_order_is_logged_and_act_refuses_a_civ_it_does_not_lead() {
    let mut w = led();
    let me = w.player.unwrap();
    let rival = *w.civs.keys().find(|&&c| c != me).unwrap();
    let camp = led_camp(&w);
    let rival_camp = units::camp(&w, rival).unwrap().id;

    let logged = w.log.len();
    let order = led_order(&w, Order::Resupply { unit: camp });
    w.act(order.clone()).unwrap();
    assert_eq!(w.log.len(), logged + 1, "the player's order is logged");
    assert_eq!(w.log.last(), Some(&order));

    let before = w.clone();
    let stranger = Action::Order { civ: rival, order: Order::Resupply { unit: rival_camp } };
    assert_eq!(w.act(stranger), Err("that civ is not yours".to_string()));
    assert_eq!(w, before, "a refused order changes nothing and is not logged");
}

#[test]
fn a_watcher_gives_no_orders() {
    let mut w = at_pick();
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    let civ = *w.civs.keys().next().unwrap();
    let camp = units::camp(&w, civ).unwrap().id;
    let order = Action::Order { civ, order: Order::Resupply { unit: camp } };
    assert_eq!(w.act(order), Err("you're watching: you lead no civ".to_string()));
}

#[test]
fn the_ais_orders_are_not_in_the_log() {
    let mut w = at_pick();
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    for _ in 0..30 {
        w.act(Action::EndTurn).unwrap();
    }
    assert!(
        w.log.iter().all(|a| matches!(a, Action::ChooseCiv { .. } | Action::EndTurn)),
        "only the watch and the turns"
    );
    let posted =
        w.units.keys().any(|id| w.notes.get(id).is_some_and(|ns| ns.iter().any(|n| n.text.starts_with("posted at"))));
    assert!(posted, "the AI's camps still give their orders: one was posted");
}

#[test]
fn world_order_works_for_an_ai_civ_while_the_player_leads_another() {
    let mut w = led();
    let me = w.player.unwrap();
    let ai = *w.civs.keys().find(|&&c| c != me).unwrap();
    let camp = units::camp(&w, ai).unwrap().id;
    let people = w.units[&camp].people;
    let logged = w.log.len();
    w.order(ai, Order::Disband { unit: camp, n: Some(1) }).unwrap();
    assert_eq!(w.units[&camp].people, people - 1.0, "the AI's warrior went home");
    assert_eq!(w.log.len(), logged, "an order given directly is not logged");
    assert!(w.order(me, Order::Disband { unit: camp, n: Some(1) }).is_err(), "the AI's camp is not the player's");
}
