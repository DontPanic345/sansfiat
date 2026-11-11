//! The planner (`units/ai.rs`): one civ's turn of decisions, read on the built map of `common`
//! (bare plains, Red's capital at `at_big(-5, 0)`, Blue's at `at_big(5, 0)`, Gold's at
//! `at_big(0, -10)`, each with an army of three and a guard of three).

mod common;

use std::collections::BTreeMap;

use common::*;
use engine::data::*;
use engine::judge::rank_changes;
use engine::script::Auto;
use engine::units::{self, Decision};
use engine::{claim, Action, Order, Route, World};

/// Three civs, watched: every civ is the planner's.
fn watched() -> (World, [u32; 3], [u32; 3]) {
    let (mut w, ids, clans) = three();
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    (w, ids, clans)
}

/// One turn of planning for `civ`, applied as the AI applies it; returns what it decided.
fn planned(w: &mut World, civ: u32) -> Vec<Decision> {
    let mut seen = Vec::new();
    units::plan(w, civ, &mut |w, civ, d| {
        seen.push(d.clone());
        match d {
            Decision::Order(order) => w.order(civ, order),
            Decision::Gift(food) => w.set_gift(civ, food),
        }
    });
    seen
}

fn orders(seen: &[Decision]) -> Vec<&Order> {
    seen.iter().filter_map(|d| if let Decision::Order(o) = d { Some(o) } else { None }).collect()
}

/// A store and a levy that feed anything the planner could want this turn.
fn rich(w: &mut World, civ: u32) {
    let c = w.civs.get_mut(&civ).unwrap();
    c.store = [0.0, CIV.store_cap, 0.0, 0.0, 0.0];
    c.levied = 100.0;
}

/// A store and a levy that feed nothing more.
fn poor(w: &mut World, civ: u32) {
    let c = w.civs.get_mut(&civ).unwrap();
    c.store = [0.0; N];
    c.levied = 0.0;
}

fn army_of(w: &World, civ: u32) -> u32 {
    units::camp(w, civ).expect("an army").id
}

/// Stands `civ`'s army on `tile` with `people` and a full pack, nothing ordered.
fn stand(w: &mut World, civ: u32, tile: usize, people: f64) -> u32 {
    let id = put_camp(w, civ, tile);
    let u = camp_mut(w, id);
    u.actor.people = people;
    u.actor.store = [0.0, units::pack_cap(people), 0.0, 0.0, 0.0];
    let c = u.camp_mut().unwrap();
    c.stack = [people, 0.0];
    c.dest = None;
    id
}

#[test]
fn the_gift_follows_who_comes_to_the_festival() {
    let (mut w, ids, _) = watched();
    let (red, blue) = (ids[0], ids[1]);
    // Nobody but its own clan comes: it gives nothing.
    planned(&mut w, red);
    assert_eq!(w.civs[&red].gift, 0.0);
    // An unaligned clan comes: the default.
    let k = put(&mut w, &[at_big(-3, 0)], 6.0);
    w.clans.get_mut(&k).unwrap().market = Some(w.civs[&red].market);
    let seen = planned(&mut w, red);
    assert!(seen.contains(&Decision::Gift(LEANING.gift_default)), "{seen:?}");
    // It leans to a rival, who gives there too or holds its home: the contested gift.
    w.clans.get_mut(&k).unwrap().leaning.insert(blue, 0.3);
    planned(&mut w, red);
    assert_eq!(w.civs[&red].gift, PLAN.gift_contested);
    assert!(w.orders[&red]["setGift"] >= 3, "the gift is counted as the player's is");
}

#[test]
fn the_claim_widens_toward_a_clan_while_the_guard_holds_and_gives_up_paper_it_cannot_guard() {
    let (mut w, ids, _) = watched();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    let before = w.civs[&red].claim.clone();
    assert!(claim::held(&w, red).len() == before.len(), "the guard holds the first claim");
    // A clan three steps out, past the claim: with the levy to feed one more guard, the claim
    // takes the tile next to it nearest that home.
    let home = at_big(-2, 0);
    put(&mut w, &[home], 6.0);
    rich(&mut w, red);
    let want = units::claim_plan(&w, red);
    assert_eq!(want.len(), before.len() + PLAN.widen, "one tile a turn");
    let new: Vec<usize> = want.iter().copied().filter(|t| !before.contains(t)).collect();
    assert_eq!(new, vec![at_big(-3, 0)], "the tile toward the clan's home");
    let seen = planned(&mut w, red);
    assert!(orders(&seen).iter().any(|o| matches!(o, Order::Claim { tiles } if *tiles == want)), "{seen:?}");
    // A levy that feeds no more: it widens no further.
    poor(&mut w, red);
    assert_eq!(units::claim_plan(&w, red), w.civs[&red].claim);
    // Its guard gone and nothing to raise one with: the claim is paper, and it gives up the
    // farthest tile each turn.
    no_guard(&mut w, red);
    let had = w.civs[&red].claim.clone();
    let now = units::claim_plan(&w, red);
    assert_eq!(now.len(), had.len() - 1);
    let gone: Vec<usize> = had.iter().copied().filter(|t| !now.contains(t)).collect();
    assert_eq!(gone, vec![at_big(-3, 0)], "the farthest paper tile, {} steps out", steps(&w, cap, gone[0]));
}

#[test]
fn it_raises_what_its_levy_feeds_and_sends_a_warrior_home_when_the_store_cannot() {
    let (mut w, ids, clans) = watched();
    let red = ids[0];
    w.clans.get_mut(&clans[0]).unwrap().people = 40.0;
    let army = army_of(&w, red);
    // A levy that covers its mouths and one more, and a store with the pack: one warrior more.
    rich(&mut w, red);
    let seen = planned(&mut w, red);
    assert!(
        orders(&seen)
            .iter()
            .any(|o| matches!(o, Order::Raise { n: 1, kind: units::UnitKind::Camp, into: Some(a), .. } if *a == army)),
        "{seen:?}"
    );
    assert_eq!(w.units[&army].people, 4.0);
    // A levy that doesn't: none.
    w.civs.get_mut(&red).unwrap().levied = 1.0;
    w.civs.get_mut(&red).unwrap().store = [0.0, 30.0, 0.0, 0.0, 0.0];
    let seen = planned(&mut w, red);
    assert!(!orders(&seen).iter().any(|o| matches!(o, Order::Raise { kind: units::UnitKind::Camp, .. })), "{seen:?}");
    // A store that can't feed its mouths a turn: a warrior goes home.
    poor(&mut w, red);
    let seen = planned(&mut w, red);
    assert!(
        orders(&seen).iter().any(|o| matches!(o, Order::Disband { unit, n: Some(1), .. } if *unit == army)),
        "{seen:?}"
    );
    assert_eq!(w.units[&army].people, 3.0);
}

#[test]
fn an_army_marches_onto_a_rivals_land_it_outweighs_and_pillages_it() {
    let (mut w, ids, _) = watched();
    let (red, blue) = (ids[0], ids[1]);
    let army = stand(&mut w, red, at_big(2, 0), 4.0);
    let seen = planned(&mut w, red);
    let to = orders(&seen).iter().find_map(|o| {
        if let Order::Move { unit, to } = o {
            (*unit == army).then_some(*to)
        } else {
            None
        }
    });
    let to = to.unwrap_or_else(|| panic!("a march: {seen:?}"));
    assert_eq!(claim::tile_civ(&w, to), Some(blue), "onto Blue's land");
    assert_ne!(to, w.civs[&blue].capital);
    // Standing on it, with room in its pack: it pillages.
    let army = stand(&mut w, red, to, 4.0);
    w.units.get_mut(&army).unwrap().actor.store = [0.0, 10.0, 0.0, 0.0, 0.0];
    w.tiles[to].stock = [0.0, 20.0, 0.0, 0.0, 0.0];
    let seen = planned(&mut w, red);
    assert!(orders(&seen).iter().any(|o| matches!(o, Order::Pillage { unit } if *unit == army)), "{seen:?}");
}

#[test]
fn it_marches_on_a_capital_it_outweighs_and_pulls_back_from_one_it_does_not() {
    let (mut w, ids, clans) = watched();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&blue].capital;
    let army = stand(&mut w, red, at_big(2, 0), 4.0);
    assert!(!units::outweighs(&w, army, blue), "four against a guard of three, an army and the militia");
    // Blue's army and guard gone and its clan small: four outweigh the militia.
    let theirs = army_of(&w, blue);
    w.units.remove(&theirs);
    no_guard(&mut w, blue);
    w.clans.get_mut(&clans[1]).unwrap().people = 3.0;
    assert!(units::outweighs(&w, army, blue));
    let seen = planned(&mut w, red);
    assert!(
        orders(&seen).iter().any(|o| matches!(o, Order::Move { unit, to } if *unit == army && *to == cap)),
        "{seen:?}"
    );
    // On the capital, a guard raised against it that it no longer outweighs: it pulls back home.
    let army = stand(&mut w, red, cap, 4.0);
    units::guard::raise(&mut w, blue, 1);
    let g = units::guard_of(&w, blue).unwrap().id;
    w.units.get_mut(&g).unwrap().actor.people = 12.0;
    assert!(!units::outweighs(&w, army, blue));
    let seen = planned(&mut w, red);
    let home = w.civs[&red].capital;
    assert!(
        orders(&seen).iter().any(|o| matches!(o, Order::Move { unit, to } if *unit == army && *to == home)),
        "{seen:?}"
    );
}

#[test]
fn an_army_whose_line_a_rival_cuts_marches_home() {
    let (mut w, ids, _) = watched();
    let (red, blue) = (ids[0], ids[1]);
    let army = stand(&mut w, red, at_big(-1, 0), 3.0);
    w.units.get_mut(&army).unwrap().camp_mut().unwrap().cut_by = Some(blue);
    let seen = planned(&mut w, red);
    let home = w.civs[&red].capital;
    assert!(
        orders(&seen).iter().any(|o| matches!(o, Order::Move { unit, to } if *unit == army && *to == home)),
        "{seen:?}"
    );
}

#[test]
fn it_stands_across_the_line_of_a_rival_army_abroad_and_paints_its_zone_there() {
    let (mut w, ids, _) = watched();
    let (red, blue) = (ids[0], ids[1]);
    // Blue's army abroad, three steps from Red's capital, its line running east to its own.
    let foe = stand(&mut w, blue, at_big(-2, 0), 3.0);
    let (path, cost, _) = units::line(&w, foe).expect("Blue's line home");
    let to = w.civs[&blue].capital;
    w.units.get_mut(&foe).unwrap().actor.routes = vec![Route { to, path: path.clone(), cost }];
    // Red's army beside that line, where it needn't pass Blue's to reach it.
    let army = stand(&mut w, red, at_big(0, -1), 3.0);
    let seen = planned(&mut w, red);
    let first = path[0];
    assert!(
        orders(&seen).iter().any(|o| matches!(o, Order::Move { unit, to } if *unit == army && *to == first)),
        "{seen:?}"
    );
    // There, it paints its zone over the line's next tile.
    let army = stand(&mut w, red, first, 3.0);
    let seen = planned(&mut w, red);
    assert!(
        orders(&seen)
            .iter()
            .any(|o| matches!(o, Order::Zone { unit, tiles } if *unit == army && tiles.contains(&path[1]))),
        "{seen:?}"
    );
}

#[test]
fn the_ais_orders_are_not_logged_and_the_automatic_players_are() {
    // Watched: the planner plays every civ through `World::order`, and the log holds only turns.
    let (mut w, ids, _) = watched();
    w.run(15);
    assert!(w.log.iter().all(|a| !matches!(a, Action::Order { .. } | Action::SetGift { .. })));
    assert!(ids.iter().all(|c| w.orders.get(c).is_some_and(|o| !o.is_empty())), "each civ gave orders: {:?}", w.orders);
    // Led by the automatic player: the same planner through `act`, logged, so its game replays.
    let (mut w, ids, _) = led();
    for _ in 0..15 {
        Auto::orders(&mut w);
        w.act(Action::EndTurn).unwrap();
    }
    let mine = w.log.iter().filter(|a| matches!(a, Action::Order { civ, .. } if *civ == ids[0])).count();
    assert!(mine > 0, "the led civ's orders are in the log");
    assert!(w.log.iter().all(|a| !matches!(a, Action::Order { civ, .. } if *civ != ids[0])), "and only its own");
}

#[test]
fn a_place_changes_only_past_the_margin() {
    let s = |a: f64, b: f64, c: f64| BTreeMap::from([(1, a), (2, b), (3, c)]);
    // Civ 1 starts second; civ 3 edges past it by less than the margin, then by more; then civ 1
    // passes both.
    let game =
        vec![s(0.20, 0.30, 0.10), s(0.20, 0.30, 0.203), s(0.20, 0.30, 0.21), s(0.35, 0.30, 0.21), s(0.35, 0.30, 0.21)];
    assert_eq!(rank_changes(&game[..2], 1), 0, "inside the margin");
    assert_eq!(rank_changes(&game[..3], 1), 1, "passed by more than {}", JUDGE.lead_margin);
    assert_eq!(rank_changes(&game, 1), 2, "and one change for the turn it passes both");
    assert_eq!(rank_changes(&game, 2), 1);
}
