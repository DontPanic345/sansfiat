//! The scripted game: two equal worlds led by two `Lead`s end equal, and the log holds the orders
//! the script gives.

use engine::script::Lead;
use engine::{Action, Config, Order, Phase, World};

/// The small map at its pick: the pre-game ended, no civ chosen yet.
fn small() -> World {
    let mut w = World::new(Config { seed: 2, radius: 16, ..Config::default() });
    while w.phase != Phase::Choose {
        w.act(Action::EndTurn).unwrap();
    }
    w
}

#[test]
fn two_leads_on_equal_worlds_end_equal() {
    let (mut a, mut b) = (small(), small());
    let (mut la, mut lb) = (Lead::default(), Lead::default());
    for _ in 0..80 {
        la.orders(&mut a);
        a.act(Action::EndTurn).expect("ending a turn is allowed once the orders are given");
        lb.orders(&mut b);
        b.act(Action::EndTurn).expect("ending a turn is allowed once the orders are given");
    }
    assert_eq!(a.log, b.log);
    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
}

#[test]
fn the_script_logs_the_orders_it_gives() {
    let mut w = small();
    let mut lead = Lead::default();
    for _ in 0..80 {
        lead.orders(&mut w);
        w.act(Action::EndTurn).expect("ending a turn is allowed once the orders are given");
    }
    let orders: Vec<&Order> = w
        .log
        .iter()
        .filter_map(|a| match a {
            Action::Order { order, .. } => Some(order),
            _ => None,
        })
        .collect();
    assert!(orders.iter().any(|o| matches!(o, Order::Raise { .. })), "a Raise");
    assert!(orders.iter().any(|o| matches!(o, Order::Move { .. })), "a Move");
    assert!(orders.iter().any(|o| matches!(o, Order::Route { .. })), "a Route");
    assert!(orders.iter().any(|o| matches!(o, Order::Zone { .. })), "a Zone");
    assert!(orders.iter().any(|o| matches!(o, Order::Disband { .. })), "a Disband");
    assert!(w.log.iter().any(|a| matches!(a, Action::SetGift { .. })), "a SetGift");
    assert!(w.log.iter().any(|a| matches!(a, Action::SetKeep { .. })), "a SetKeep");
}
