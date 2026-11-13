//! The land: a civ's claim is its land with nobody on it, a rival army ending its turn on it takes it
//! (and the tiles cut off behind it), the first claim, and the levy and Standing on the land. Built
//! maps from `common` (bare plains): Red's capital is at `at_big(-5, 0)`, Blue's at `at_big(5, 0)`.

mod common;

use common::*;
use engine::civs::levy;
use engine::claim;
use engine::data::*;
use engine::map::index;
use engine::stats::{digest, standing};
use engine::units::{self, fight, UnitKind};
use engine::{Action, Config, EventKind, Order, Phase, World};

/// The tiles within `k` hex steps of `tile`, on a `big()` map, sorted.
fn within(w: &World, tile: usize, k: i32) -> Vec<usize> {
    let mut out: Vec<usize> = w.tiles[tile].hex().within(k).into_iter().filter_map(|h| index(BIG, h)).collect();
    out.sort_unstable();
    out
}

#[test]
fn a_claim_must_be_connected_to_the_capital_and_on_land() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    // Two tiles apart with nothing between: not joined.
    assert!(claim::check(&w, red, &[at_big(-3, 0)]).is_err(), "a gap is refused");
    let ok = claim::check(&w, red, &[at_big(-4, 0), at_big(-3, 0)]).expect("a joined claim");
    assert_eq!(ok, {
        let mut v = vec![cap, at_big(-4, 0), at_big(-3, 0)];
        v.sort_unstable();
        v
    });
    // The capital may be left out: it is added.
    assert_eq!(claim::check(&w, red, &[at_big(-4, 0)]).unwrap().len(), 2);
    // Water is refused.
    w.tiles[at_big(-4, 0)].terrain = SEA;
    assert_eq!(claim::check(&w, red, &[at_big(-4, 0)]), Err("a claim can only cover land".to_string()));
    // Nor another civ's capital.
    assert!(claim::check(&w, red, &[at_big(5, 0)]).is_err(), "Blue's capital is not Red's to claim");
}

#[test]
fn claims_none_refuses_a_claim_and_only_k_lets_one_civ_claim() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    let tiles = within(&w, cap, 1);
    w.config.claims = Claims::None;
    assert!(w.order(red, Order::Claim { tiles: tiles.clone() }).is_err(), "claims none refuses a claim");
    assert_eq!(
        claim::check(&w, red, &tiles),
        Err("this civ may not claim land: the claims control is off".to_string())
    );
    // Camps are not the claims' business: a camp still raises.
    fill_stores(&mut w);
    assert!(w.order(red, Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Default::default(), into: None }).is_ok());
    // Only the k-th civ in id order (here Blue) may claim.
    w.config.claims = Claims::Only(1);
    assert!(!claim::may_claim(&w, red));
    assert!(claim::may_claim(&w, ids[1]));
    assert!(w.order(ids[1], Order::Claim { tiles: within(&w, w.civs[&ids[1]].capital, 1) }).is_ok());
}

#[test]
fn the_first_claim_is_the_capital_ring_and_a_civ_that_may_not_claim_keeps_its_capital() {
    let (w, ids, _) = three();
    for &civ in &ids {
        let cap = w.civs[&civ].capital;
        // The clans are on the capitals, so the first claim is the ring one step out, the capital in it.
        assert_eq!(w.civs[&civ].claim, within(&w, cap, 1), "{civ}: the claim at founding");
        assert_eq!(claim::first(&w, civ), w.civs[&civ].claim, "{civ}: as `first` reads it");
    }
    // Under `none`, founding gives each civ its capital alone.
    let mut w = big();
    let ms = markets3(&mut w);
    put(&mut w, &[at_big(-5, 0)], 14.0);
    w.config.claims = Claims::None;
    found(&mut w, &ms);
    let red = *w.civs.keys().next().unwrap();
    assert_eq!(w.civs[&red].claim, vec![w.civs[&red].capital]);
}

#[test]
fn a_first_claim_takes_in_the_ring_of_a_clan_near_the_capital() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    // A clan of Red's two steps out, within the reach: its home's ring joins the first claim.
    let far = at_big(-3, 0);
    let k = put(&mut w, &[far], 6.0);
    w.clans.get_mut(&k).unwrap().civ = Some(red);
    let want = claim::first(&w, red);
    assert_ne!(want, within(&w, cap, 1), "the clan's home widens the claim");
    assert!(within(&w, far, 1).iter().all(|t| want.contains(t)), "by its whole ring");
}

#[test]
fn an_unaligned_clan_on_the_civs_land_pays_the_levy_at_its_market_and_one_off_it_does_not() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let mid = w.civs[&red].market;
    hold_land(&mut w, red, &[at_big(-4, 0)]);
    let on = put(&mut w, &[at_big(-4, 0)], 4.0);
    let off = put(&mut w, &[at_big(-9, -3)], 4.0);
    for k in [on, off] {
        let c = w.clans.get_mut(&k).unwrap();
        c.market = Some(mid);
        c.basket = [1.0; N];
    }
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    levy(&mut w, mid);
    assert!((total(&w.civs[&red].store) - CIV.levy * N as f64).abs() < 1e-9, "the land's clan paid");
    assert_eq!(w.clans[&off].basket, [1.0; N], "the stranger off the land paid nothing");
}

#[test]
fn raising_an_army_leaves_standing_as_it_was_and_an_unaligned_clan_on_the_land_counts_toward_it() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let before = standing(&w)[&red];
    w.order(red, Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Default::default(), into: None }).unwrap();
    assert!((standing(&w)[&red] - before).abs() < 1e-12, "raising moves nobody between civs");
    // An unaligned clan on Red's land: its people count for Red.
    hold_land(&mut w, red, &[at_big(-4, 0)]);
    let k = put(&mut w, &[at_big(-4, 0)], 4.0);
    let on_land = standing(&w)[&red];
    let (mut mine, mut all) = (0.0, 0.0);
    for c in w.clans.values() {
        all += c.people;
        if c.civ == Some(red) || (c.civ.is_none() && claim::tile_civ(&w, c.home) == Some(red)) {
            mine += c.people;
        }
    }
    for u in w.units.values() {
        all += u.people;
        if u.civ == red {
            mine += u.people;
        }
    }
    assert!((on_land - mine / all).abs() < 1e-12, "the unaligned clan counts: {on_land} vs {}", mine / all);
    assert!(w.clans[&k].civ.is_none());
}

#[test]
fn replaying_a_save_with_claims_rebuilds_the_same_world() {
    let mut w =
        World::new(Config { seed: 2, radius: engine::data::size_radius("small").unwrap(), ..Config::default() });
    while w.phase != Phase::Choose {
        w.run(1);
    }
    let red = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(red) }).unwrap();
    let cap = w.civs[&red].capital;
    let land: Vec<usize> = within(&w, cap, 2).into_iter().filter(|&t| TERRAIN[w.tiles[t].terrain].land).collect();
    let tiles = claim::connected_part(&w, cap, &land);
    w.act(led_order(&w, Order::Claim { tiles })).unwrap();
    for _ in 0..30 {
        w.act(Action::EndTurn).unwrap();
    }
    let r = World::replay(&w.save()).expect("the save replays");
    assert_eq!(digest(&r), digest(&w), "the same digest");
    assert_eq!(r.civs, w.civs, "the same claims");
}

#[test]
fn a_claim_is_refused_before_the_game_begins() {
    let w = World::new(Config { seed: 2, radius: 6, ..Config::default() });
    assert_eq!(w.check_order(1, &Order::Claim { tiles: vec![0] }), Err("the game has not begun".to_string()));
}

/// Moves `civ`'s scout onto `tile`.
fn put_scout(w: &mut World, civ: u32, tile: usize) {
    let id = units::scout(w, civ).expect("a scout").id;
    let u = w.units.get_mut(&id).unwrap();
    u.actor.home = tile;
    u.actor.territory = vec![tile];
}

/// Red claims the line from its capital out to `(-1, 0)`; Blue's army is its camp.
fn red_line(w: &mut World, red: u32) -> Vec<usize> {
    let line: Vec<usize> = (-4..=-1).map(|q| at_big(q, 0)).collect();
    w.order(red, Order::Claim { tiles: line.clone() }).unwrap();
    line
}

#[test]
fn a_claimed_tile_is_the_civs_land_with_no_unit_anywhere_near_it() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let far = at_big(-1, 0);
    red_line(&mut w, red);
    // Every unit of Red's is on its capital, five steps back.
    assert!(w.units.values().filter(|u| u.civ == red).all(|u| steps(&w, u.home, far) >= 4));
    assert_eq!(claim::tile_civ(&w, far), Some(red));
    assert!(claim::held(&w, red).contains(&far));
    assert_eq!(claim::held(&w, red), w.civs[&red].claim, "the land is the claim");
    assert_eq!(claim::tile_civ(&w, at_big(0, 0)), None, "an unclaimed tile is nobody's");
}

#[test]
fn a_rival_army_ending_its_turn_on_land_takes_it_and_nobody_loses_people() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&red].capital;
    w.order(red, Order::Claim { tiles: vec![at_big(-4, 0), at_big(-3, 1)] }).unwrap();
    let people: Vec<f64> = w.units.values().map(|u| u.people).collect();
    put_camp(&mut w, blue, at_big(-3, 1));
    w.events.clear();
    claim::step(&mut w);
    assert_eq!(w.civs[&red].claim, vec![cap.min(at_big(-4, 0)), cap.max(at_big(-4, 0))], "the tile left the claim");
    assert_eq!(claim::tile_civ(&w, at_big(-3, 1)), None, "it does not touch Blue's land: nobody's");
    let taken: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClaimTaken).collect();
    assert_eq!(taken.len(), 1);
    assert_eq!((taken[0].civ, taken[0].other, taken[0].tile), (Some(red), Some(blue), at_big(-3, 1)));
    assert_eq!(taken[0].amount, Some(1.0));
    assert_eq!(w.units.values().map(|u| u.people).collect::<Vec<_>>(), people, "no losses to anyone");
    // The next turn it takes nothing more.
    w.events.clear();
    claim::step(&mut w);
    assert_eq!(kinds(&w, EventKind::ClaimTaken), 0);
}

#[test]
fn taking_a_tile_that_cuts_the_claim_takes_the_cut_off_part_and_the_tile_joins_only_where_it_touches() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&red].capital;
    red_line(&mut w, red);
    // Blue's army ends its turn on (-3, 0): the tile and the two behind it go, three tiles lost.
    put_camp(&mut w, blue, at_big(-3, 0));
    w.events.clear();
    claim::step(&mut w);
    assert_eq!(w.civs[&red].claim, vec![cap, at_big(-4, 0)].into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect::<Vec<_>>());
    let taken: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClaimTaken).collect();
    assert_eq!(taken.len(), 1);
    assert_eq!(taken[0].amount, Some(3.0), "three tiles lost");
    for q in [-3, -2, -1] {
        assert_eq!(claim::tile_civ(&w, at_big(q, 0)), None, "({q}, 0) is nobody's: it does not touch Blue's land");
    }
    // Where it touches Blue's claim the tile joins it.
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    hold_land(&mut w, blue, &(-2..=4).map(|q| at_big(q, 0)).collect::<Vec<_>>());
    w.order(red, Order::Claim { tiles: vec![at_big(-4, 0), at_big(-3, 0)] }).unwrap();
    put_camp(&mut w, blue, at_big(-3, 0));
    w.events.clear();
    claim::step(&mut w);
    assert_eq!(claim::tile_civ(&w, at_big(-3, 0)), Some(blue), "it joins the taker's claim");
    assert_eq!(claim::tile_civ(&w, at_big(-4, 0)), Some(red));
    let taken: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClaimTaken).collect();
    assert_eq!(taken[0].amount, Some(1.0));
}

#[test]
fn an_army_of_the_owner_standing_on_or_holding_the_tile_keeps_it_and_the_two_fight() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let t = at_big(-4, 0);
    w.order(red, Order::Claim { tiles: vec![t] }).unwrap();
    // Red's army stands on it.
    put_camp(&mut w, red, t);
    put_camp(&mut w, blue, t);
    let holders = fight::army_holders(&w);
    w.events.clear();
    claim::step(&mut w);
    assert_eq!(kinds(&w, EventKind::ClaimTaken), 0, "it stands on the tile");
    assert!(w.civs[&red].claim.contains(&t));
    let (r, b) = (camp_id(&w, red), camp_id(&w, blue));
    let (pr, pb) = (w.units[&r].people, w.units[&b].people);
    fight::step(&mut w, &holders);
    assert!(w.units[&r].people < pr && w.units[&b].people < pb, "the armies fight instead");
    // Red's army holds it from the next tile instead of standing on it.
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    w.order(red, Order::Claim { tiles: vec![t] }).unwrap();
    let id = put_camp(&mut w, red, at_big(-4, 1));
    {
        let c = camp_mut(&mut w, id).camp_mut().unwrap();
        c.held = vec![at_big(-4, 1), t];
        c.guard = vec![0.5, 0.5];
    }
    put_camp(&mut w, blue, t);
    claim::step(&mut w);
    assert_eq!(kinds(&w, EventKind::ClaimTaken), 0, "an army of the owner holds it");
    assert_eq!(claim::tile_civ(&w, t), Some(red));
}

#[test]
fn a_scout_takes_nothing_and_fights_nobody() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let t = at_big(-4, 0);
    w.order(red, Order::Claim { tiles: vec![t] }).unwrap();
    let claim_before = w.civs[&red].claim.clone();
    put_scout(&mut w, blue, t);
    put_camp(&mut w, red, t);
    let people: Vec<f64> = w.units.values().map(|u| u.people).collect();
    let holders = fight::army_holders(&w);
    claim::step(&mut w);
    fight::step(&mut w, &holders);
    assert_eq!(w.civs[&red].claim, claim_before, "a scout takes no land");
    assert_eq!(w.units.values().map(|u| u.people).collect::<Vec<_>>(), people, "and fights nobody");
    assert_eq!(kinds(&w, EventKind::ClaimTaken), 0);
}

#[test]
fn the_capital_is_not_taken_by_the_claim_step() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&red].capital;
    w.order(red, Order::Claim { tiles: vec![at_big(-4, 0)] }).unwrap();
    let held = w.civs[&red].claim.clone();
    // No army of Red's on it, and a rival's army does: the claim step leaves it (the fall rule is the fight's).
    let id = camp_id(&w, red);
    w.units.remove(&id);
    put_camp(&mut w, blue, cap);
    claim::step(&mut w);
    assert_eq!(w.civs[&red].claim, held);
    assert_eq!(claim::tile_civ(&w, cap), Some(red));
    assert_eq!(kinds(&w, EventKind::ClaimTaken), 0);
}

#[test]
fn a_raise_of_kind_guard_does_not_parse() {
    let order = r#"{"type":"raise","n":1,"kind":"guard"}"#;
    assert!(serde_json::from_str::<Order>(order).is_err(), "there is no guard to raise");
    let camp = r#"{"type":"raise","n":1,"kind":"camp"}"#;
    assert!(serde_json::from_str::<Order>(camp).is_ok());
}

#[test]
fn a_claim_changes_in_words_naming_the_tiles_added_and_dropped() {
    let (w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    let (a, b) = (at_big(-4, 0), at_big(-3, 0));
    let was = claim::check(&w, red, &[a]).unwrap();
    let now = claim::check(&w, red, &[a, b]).unwrap();
    let said = claim::change(&w, &was, &now);
    assert!(said.contains("adds 1") && said.contains(&w.at(b)) && !said.contains("drops"), "{said}");
    assert!(said.contains("its land is 3 tile(s)"), "{said}");
    let said = claim::change(&w, &now, &[cap]);
    assert!(said.contains("drops 2") && said.contains(&w.at(a)) && said.contains(&w.at(b)), "{said}");
    assert!(claim::change(&w, &now, &now).contains("changes nothing"));
}
