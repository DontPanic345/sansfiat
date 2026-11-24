//! Signposts and settlements, staged on bare plains with two civs: the order, the draw, founding, the
//! market and the store, the claim's roots, taking, and the planner's site.

use engine::claim;
use engine::clans::new_clan;
use engine::data::{CIV, CLAN, SETTLE};
use engine::events::EventKind;
use engine::hex::Hex;
use engine::homes;
use engine::land::steps;
use engine::map::index;
use engine::market::total;
use engine::stage::Stage;
use engine::{civs, dividing, markets, settle, towns, Action, Order, World};

const RADIUS: i32 = 9;

fn tile(q: i32, r: i32) -> usize {
    index(RADIUS, Hex { q, r }).unwrap()
}

/// Bare plains with Red at (−6, 0) and Blue at (6, 0), and no clans but those the test places.
fn staged() -> (World, u32, u32) {
    let (stage, red, blue) = stage();
    (stage.into_world(), red, blue)
}

fn stage() -> (Stage, u32, u32) {
    let mut stage = Stage::flat(RADIUS);
    let ids = stage.civs(&[tile(-6, 0), tile(6, 0)], 5.0);
    let w = stage.world_mut();
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    w.clans.clear();
    for t in &mut w.tiles {
        t.claim = None;
    }
    (stage, ids[0], ids[1])
}

/// Makes tile `t` rich: every food at 6, so it feeds a daughter in full.
fn rich(w: &mut World, t: usize) {
    let tile = &mut w.tiles[t];
    (tile.cap, tile.unworn, tile.stock) =
        ([6.0, 6.0, 6.0, 6.0, 6.0, 0.0, 0.0], [6.0, 6.0, 6.0, 6.0, 6.0, 0.0, 0.0], [6.0, 6.0, 6.0, 6.0, 6.0, 0.0, 0.0]);
}

/// A clan of `civ` with `people` living on `home`, which it holds.
fn clan(w: &mut World, civ: u32, home: usize, people: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let mut c = new_clan(id, home, people);
    c.civ = Some(civ);
    homes::claim_land(w, id, &[home]);
    w.clans.insert(id, c);
    id
}

/// Red's settlement on `t`, founded by a clan of 4 placed there.
fn settled(w: &mut World, red: u32, t: usize) -> (u32, u32) {
    w.order(red, Order::Signpost { tile: t }).unwrap();
    let k = clan(w, red, t, 4.0);
    settle::step(w);
    (settle::at(w, t).expect("a settlement").id, k)
}

#[test]
fn a_signpost_is_an_order_on_a_tile_a_clan_can_live_on_and_a_civ_keeps_a_few() {
    let (mut w, red, _) = staged();
    let near = tile(-6 + SETTLE.spacing - 1, 0);
    assert!(w.check_order(red, &Order::Signpost { tile: near }).unwrap_err().contains("steps from every capital"));
    let lived = tile(-1, 0);
    clan(&mut w, red, lived, 4.0);
    assert!(w.check_order(red, &Order::Signpost { tile: lived }).unwrap_err().contains("already lives"));
    let posts = [tile(-2, 0), tile(-2, 3), tile(-2, -3)];
    for &t in &posts {
        w.order(red, Order::Signpost { tile: t }).unwrap();
    }
    assert_eq!(w.civs[&red].signposts.len(), SETTLE.signposts);
    assert!(w.check_order(red, &Order::Signpost { tile: tile(0, 4) }).unwrap_err().contains("remove one first"));
    assert!(w.check_order(red, &Order::Unpost { tile: tile(0, 4) }).is_err());
    w.order(red, Order::Unpost { tile: posts[0] }).unwrap();
    assert_eq!(w.civs[&red].signposts.len(), SETTLE.signposts - 1);
    w.order(red, Order::Signpost { tile: tile(0, 4) }).unwrap();
    w.config.signposts = false;
    assert!(w.check_order(red, &Order::Signpost { tile: posts[0] }).unwrap_err().contains("control"));
}

#[test]
fn the_nearer_and_the_richer_the_likelier() {
    assert!(settle::chance(2.0, 1.0) > settle::chance(6.0, 1.0));
    assert!(settle::chance(2.0, 1.0) > settle::chance(2.0, 0.95));
    assert!(settle::chance(0.0, 5.0) <= SETTLE.draw);
}

#[test]
fn a_daughter_goes_to_the_signpost_by_chance_and_a_clan_that_cannot_get_there_does_not_try() {
    let (mut w, red, _) = staged();
    let post = tile(-2, 0);
    rich(&mut w, post);
    w.order(red, Order::Signpost { tile: post }).unwrap();
    // A clan ringed by ground nobody can walk has no way there.
    let far = clan(&mut w, red, tile(5, -5), CLAN.split_at);
    let wall = engine::TERRAIN.iter().position(|t| t.move_cost.is_none()).unwrap();
    let ring: Vec<usize> = engine::land::neighbours(&w, tile(5, -5)).collect();
    for n in ring {
        w.tiles[n].terrain = wall;
    }
    let rng = w.rng;
    assert!(!dividing::split(&mut w, far, true), "a signpost past its reach is not weighed");
    assert_eq!(w.rng, rng, "and no chance is drawn");
    let mother = clan(&mut w, red, tile(-3, 0), CLAN.split_at);
    let before = w.clans.len();
    let mut went = 0;
    for seed in 1..=40 {
        let mut v = w.clone();
        v.rng = seed;
        if dividing::split(&mut v, mother, true) {
            went += 1;
            let d = v.clans.values().last().unwrap();
            assert_eq!((d.home, d.civ, v.clans.len()), (post, Some(red), before + 1));
            settle::step(&mut v);
            assert!(settle::at(&v, post).is_some(), "the daughter founds the settlement");
        }
    }
    assert!(went > 10 && went < 40, "she goes by chance: {went} of 40");
    // A rival's clan does not weigh Red's signpost.
    w.clans.get_mut(&mother).unwrap().civ = None;
    assert!(!dividing::split(&mut w, mother, true));
}

#[test]
fn the_first_clan_to_settle_at_a_signpost_founds_a_settlement_and_the_signpost_goes() {
    let (mut w, red, blue) = staged();
    let t = tile(-1, 0);
    w.order(red, Order::Signpost { tile: t }).unwrap();
    let rival = clan(&mut w, blue, t, 4.0);
    settle::step(&mut w);
    assert!(settle::at(&w, t).is_none(), "a rival's clan founds nothing");
    w.clans.remove(&rival);
    let k = clan(&mut w, red, t, 4.0);
    settle::step(&mut w);
    let s = settle::at(&w, t).expect("a settlement").clone();
    assert_eq!((s.civ, w.clans[&k].market), (red, Some(s.market)));
    assert!(w.civs[&red].signposts.is_empty());
    assert!(w.civs[&red].claim.contains(&t), "its tile is the civ's land");
    assert_eq!(w.events.iter().filter(|e| e.kind == EventKind::SettlementFounded).count(), 1);
    // Its market never fades, and counts for the town rule as any festival does.
    w.markets.get_mut(&s.market).unwrap().activity = 0.0;
    markets::fade(&mut w);
    assert!(w.markets.contains_key(&s.market));
    clan(&mut w, red, t, 4.0);
    clan(&mut w, red, t, 4.0);
    assert!(towns::towns(&w).iter().any(|town| town.market == s.market && town.clans.len() == 3));
}

#[test]
fn clans_that_walk_to_a_settlement_pay_its_levy_into_its_store_which_spoils_and_is_capped() {
    let (mut w, red, _) = staged();
    let (sid, k) = settled(&mut w, red, tile(-1, 0));
    let mid = w.settlements[&sid].market;
    let capital_store = total(&w.civs[&red].store);
    w.clans.get_mut(&k).unwrap().basket = [10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    civs::levy(&mut w, mid);
    assert!((total(&w.settlements[&sid].store) - 10.0 * CIV.levy).abs() < 1e-9);
    assert_eq!(total(&w.civs[&red].store), capital_store, "the capital's store takes none of it");
    w.settlements.get_mut(&sid).unwrap().store = [SETTLE.store_cap * 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    settle::upkeep(&mut w);
    let s = &w.settlements[&sid];
    assert!((total(&s.store) - SETTLE.store_cap).abs() < 1e-9 && s.lost > 0.0);
    w.clans.get_mut(&k).unwrap().basket = [10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    civs::levy(&mut w, mid);
    assert!((total(&w.settlements[&sid].store) - SETTLE.store_cap).abs() < 1e-9, "a full store takes no levy");
    settle::upkeep(&mut w);
    assert!(total(&w.settlements[&sid].store) < SETTLE.store_cap, "it spoils");
}

#[test]
fn a_claim_holds_where_it_joins_the_capital_or_a_settlement() {
    let (mut w, red, _) = staged();
    let island = vec![tile(-1, 0), tile(0, 0)];
    assert!(w.check_order(red, &Order::Claim { tiles: island.clone() }).unwrap_err().contains("joined"));
    settled(&mut w, red, tile(-1, 0));
    w.order(red, Order::Claim { tiles: island.clone() }).unwrap();
    let c = &w.civs[&red];
    assert!(island.iter().all(|t| c.claim.contains(t)) && c.claim.contains(&c.capital));
    assert!(w.check_order(red, &Order::Claim { tiles: vec![tile(0, 5)] }).is_err());
}

#[test]
fn a_rival_army_on_an_unguarded_settlement_takes_it_with_its_store() {
    let (mut stage, red, blue) = stage();
    let t = tile(-1, 0);
    let (sid, _) = settled(stage.world_mut(), red, t);
    stage.order(red, Order::Claim { tiles: vec![t, tile(0, 0)] }).unwrap();
    stage.world_mut().settlements.get_mut(&sid).unwrap().store = [12.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    stage.army(blue, t, 3.0);
    let mut w = stage.into_world();
    claim::step(&mut w);
    let s = &w.settlements[&sid];
    assert_eq!((s.civ, total(&s.store)), (blue, 12.0));
    assert!(!w.civs[&red].claim.contains(&t) && !w.civs[&red].claim.contains(&tile(0, 0)));
    assert!(w.civs[&blue].claim.contains(&t), "the tile joins the taker's land");
    assert_eq!(w.events.iter().filter(|e| e.kind == EventKind::SettlementTaken).count(), 1);
    assert_eq!(settle::roots(&w, blue).len(), 2);
}

#[test]
fn the_planner_posts_a_signpost_a_clan_can_reach_clear_of_every_seat() {
    let (mut w, red, _) = staged();
    assert_eq!(engine::units::signpost_plan(&w, red), None, "bare plains feed no daughter");
    for t in 0..w.tiles.len() {
        rich(&mut w, t);
    }
    let t = engine::units::signpost_plan(&w, red).expect("a site on rich plains");
    let capital = w.civs[&red].capital;
    assert!(steps(&w, capital, t) >= SETTLE.spacing && steps(&w, capital, t) <= SETTLE.plan_range);
    w.order(red, Order::Signpost { tile: t }).unwrap();
    let next = engine::units::signpost_plan(&w, red).expect("another site");
    assert!(steps(&w, t, next) >= SETTLE.spacing);
}

#[test]
fn a_signpost_that_draws_nobody_says_why() {
    let (mut w, red, blue) = staged();
    let post = tile(-2, 0);
    rich(&mut w, post);
    w.order(red, Order::Signpost { tile: post }).unwrap();
    assert!(settle::worth(&w, post) > SETTLE.fed);
    assert_eq!(settle::idle(&w, post), None, "a free tile that feeds a daughter in full draws her");
    let mother = clan(&mut w, red, tile(-3, 0), CLAN.split_at);
    let drawn = |w: &mut World| {
        (1..=40)
            .filter(|&seed| {
                w.rng = seed;
                settle::draw(w, Some(red), mother, tile(-3, 0), CLAN.split_at, &[0.0; 7], 0.0).is_some()
            })
            .count()
    };
    assert!(drawn(&mut w) > 0, "the signpost draws the clan before a clan gathers it");
    // A clan of Blue's claims the tile: it gathers the tile, so a newcomer gets nothing from it.
    let gatherer = clan(&mut w, blue, tile(5, 0), CLAN.split_at);
    w.tiles[post].claim = Some(gatherer);
    let why = settle::idle(&w, post).expect("a gathered tile draws nobody");
    assert!(why.starts_with(&format!("clan {gatherer} gathers this tile")), "{why}");
    assert!(why.ends_with("remove it and post on a free site"), "{why}");
    assert_eq!(drawn(&mut w), 0, "draw passes over the tile for a clan of the civ");
}
