//! The outlook: the reach line names what the led civ can order now that brings a clan or a rival's
//! land, and says so when nothing does; the capital line names its guard and the nearest rival army.
//! Built maps from `common` (bare plains): Red's capital is at `at_big(-5, 0)`, Blue's at `at_big(5, 0)`.

mod common;

use common::*;
use engine::data::*;
use engine::outlook;
use engine::Order;

#[test]
fn a_claim_in_reach_is_the_fewest_new_tiles_to_a_clan_home_and_pressing_it_brings_the_clan() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    // An unaligned clan three steps south of Red's capital: its first claim is the ring one step out.
    let home = at_big(-5, 3);
    let id = 9_000;
    let mut k = w.clans.values().next().unwrap().clone();
    k.id = id;
    k.civ = None;
    k.actor.home = home;
    k.actor.territory = vec![home];
    k.actor.market = None;
    k.actor.people = 7.0;
    k.leaning.clear();
    w.clans.insert(id, k);
    let to = outlook::claim_to(&w, red).expect("a claim reaches the clan");
    assert_eq!((to.clan, to.home), (id, home));
    assert_eq!(to.tiles.len(), 2, "two new tiles: {:?}", to.tiles);
    assert_eq!(to.tiles.last(), Some(&home), "the path ends on the home");
    let line = outlook::reach_line(&w, red);
    assert!(line.starts_with("in reach: claim 2 tiles to (-5, 3) ((-5, 2), (-5, 3)): brings clan 9000"), "{line}");
    // Pressing it: the claim the line names brings the clan.
    let mut tiles = w.civs[&red].claim.clone();
    tiles.extend(&to.tiles);
    w.order(red, Order::Claim { tiles }).expect("the named claim is accepted");
    assert_eq!(w.clans[&id].civ, Some(red), "the clan joins at once");
}

#[test]
fn the_gift_is_in_reach_when_a_clan_not_its_own_trades_at_its_market() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    assert_eq!(outlook::reach(&w, red).gift, 0);
    // A clan of Blue's trades at Red's capital market.
    let market = w.civs[&red].market;
    let k = w.clans.values_mut().find(|k| k.civ == Some(blue)).unwrap();
    k.actor.market = Some(market);
    let r = outlook::reach(&w, red);
    assert_eq!(r.gift, 1);
    assert!(outlook::reach_line(&w, red).contains("the gift reaches 1 clan at your market"));
}

#[test]
fn a_rival_tile_inside_the_ring_is_the_army_reach_and_a_capital_comes_first() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    assert_eq!(outlook::army_to(&w, red), None, "Blue's land ten steps off is past Red's ring");
    // Blue's land runs west to two steps from Red's capital.
    let claim = &mut w.civs.get_mut(&blue).unwrap().claim;
    claim.extend((-3..5).map(|q| at_big(q, 0)));
    claim.sort_unstable();
    claim.dedup();
    let a = outlook::army_to(&w, red).expect("Blue's land two steps off is in reach");
    assert_eq!((a.rival, a.capital), (blue, false));
    assert_eq!(a.tile, at_big(-3, 0), "the cheapest line: the nearest tile");
    // A rival capital in reach comes before a nearer tile.
    w.civs.get_mut(&blue).unwrap().capital = at_big(-1, 0);
    let a = outlook::army_to(&w, red).expect("in reach");
    assert_eq!((a.tile, a.capital), (at_big(-1, 0), true));
    assert!(outlook::reach_line(&w, red).contains("can take Blue's capital at (-1, 0)"), "{}", outlook::reach_line(&w, red));
    assert_eq!(a.army, Some(camp_id(&w, red)), "the army named is Red's");
    if a.capital {
        assert_eq!(w.civs[&a.rival].capital, a.tile);
    }
    let line = outlook::reach_line(&w, red);
    assert!(line.contains(&format!("army {} can take", camp_id(&w, red))), "{line}");
    // With no army the order is to raise one.
    let mut w2 = w.clone();
    w2.units.retain(|_, u| u.civ != red || u.kind() != engine::units::UnitKind::Camp);
    let line = outlook::reach_line(&w2, red);
    assert!(line.contains("raise one and march it there"), "{line}");
}

#[test]
fn nothing_in_reach_says_so_and_names_the_nearest_of_each() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    // Red's land is an island: the sea runs round its first claim.
    let keep = w.civs[&red].claim.clone();
    for i in 0..w.tiles.len() {
        if !keep.contains(&i) && engine::land::steps(&w, w.civs[&red].capital, i) <= 4 {
            w.tiles[i].terrain = SEA;
        }
    }
    let r = outlook::reach(&w, red);
    assert!(!r.any(), "{r:?}");
    assert!(r.near_clan.is_some() && r.near_rival.is_some());
    let line = outlook::reach_line(&w, red);
    assert!(line.starts_with("nothing you can order now brings a clan or a rival's land"), "{line}");
    assert!(line.contains("the nearest clan not yours is clan"), "{line}");
    assert!(line.contains("past every store's reach"), "{line}");
}

#[test]
fn the_capital_line_names_its_guard_and_the_nearest_rival_army() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let line = outlook::capital_line(&w, red);
    assert!(line.starts_with("held by your army"), "{line}");
    assert!(line.contains("'s army of"), "{line}");
    let camp = camp_id(&w, red);
    w.units.remove(&camp);
    let line = outlook::capital_line(&w, red);
    assert!(line.starts_with("no army of yours on it: a rival army that ends its turn there takes it, and the game"), "{line}");
}
