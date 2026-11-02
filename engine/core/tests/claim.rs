//! The land: claims, the guard that holds them, held and paper tiles, rival units taking paper, the
//! AI's stand-in, the guard's food, and the levy and Standing on the held land. Built maps from
//! `common` (bare plains): Red's capital is at `at_big(-5, 0)`, Blue's at `at_big(5, 0)`.

mod common;

use common::*;
use engine::civs::levy;
use engine::claim;
use engine::data::*;
use engine::map::index;
use engine::stats::{digest, standing};
use engine::units::{self, UnitKind};
use engine::{Action, Config, EventKind, Order, Phase, World};

/// The tiles within `k` hex steps of `tile`, on a `big()` map, sorted.
fn within(w: &World, tile: usize, k: i32) -> Vec<usize> {
    let mut out: Vec<usize> = w.tiles[tile].hex().within(k).into_iter().filter_map(|h| index(BIG, h)).collect();
    out.sort_unstable();
    out
}

/// Red's guard's people (none before it is raised).
fn guard_people(w: &World, civ: u32) -> f64 {
    units::guard_of(w, civ).map_or(0.0, |u| u.people)
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
fn claims_none_refuses_a_claim_and_a_guard_and_only_k_lets_one_civ_claim() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    let tiles = within(&w, cap, 1);
    w.config.claims = Claims::None;
    assert!(w.order(red, Order::Claim { tiles: tiles.clone() }).is_err(), "claims none refuses a claim");
    assert!(w.order(red, Order::Raise { n: 1, kind: UnitKind::Guard }).is_err(), "and a guard");
    assert_eq!(
        claim::check(&w, red, &tiles),
        Err("this civ may not claim land: the claims control is off".to_string())
    );
    // Camps are not the claims' business: a camp still raises.
    fill_stores(&mut w);
    assert!(w.order(red, Order::Raise { n: 1, kind: UnitKind::Camp }).is_ok());
    // Only the k-th civ in id order (here Blue) may claim.
    w.config.claims = Claims::Only(1);
    assert!(!claim::may_claim(&w, red));
    assert!(claim::may_claim(&w, ids[1]));
    assert!(w.order(ids[1], Order::Claim { tiles: within(&w, w.civs[&ids[1]].capital, 1) }).is_ok());
}

#[test]
fn one_guard_holds_a_small_claim_and_leaves_the_far_tiles_of_a_wide_one_paper() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    // The guard of founding (three people): its whole labour over seven tiles holds every one.
    w.order(red, Order::Claim { tiles: within(&w, cap, 1) }).unwrap();
    assert_eq!(claim::held(&w, red), within(&w, cap, 1), "a small claim is all land");
    // Wide: everything within three steps. The guard's share falls with the walk out, and three steps
    // out is paper.
    w.order(red, Order::Claim { tiles: within(&w, cap, 3) }).unwrap();
    let held = claim::held(&w, red);
    let near = within(&w, cap, 1);
    assert!(near.iter().all(|t| held.contains(t)), "the near ring is held");
    let far: Vec<usize> = within(&w, cap, 3).into_iter().filter(|&t| steps(&w, cap, t) == 3).collect();
    assert!(far.iter().all(|t| !held.contains(t)), "the far tiles are paper");
    assert!(far.iter().all(|&t| tile_is_claimed(&w, red, t)), "but still claimed");
    // Paper is no land: the civ's land is what its guard holds.
    assert_eq!(engine::civs::land(&w, red), held);
}

fn tile_is_claimed(w: &World, civ: u32, t: usize) -> bool {
    w.civs[&civ].claim.binary_search(&t).is_ok()
}

#[test]
fn a_camp_holding_paper_of_its_own_claim_makes_it_held() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    w.order(red, Order::Claim { tiles: within(&w, cap, 3) }).unwrap();
    let far = within(&w, cap, 3).into_iter().find(|&t| steps(&w, cap, t) == 3).unwrap();
    assert!(!claim::held(&w, red).contains(&far));
    // A camp of Red's holds it, with a share of its own: the guard share of the camp is added (`divide`).
    let id = camp_id(&w, red);
    {
        let c = camp_mut(&mut w, id).camp_mut().unwrap();
        c.held = vec![far];
        c.guard = vec![0.5];
    }
    claim::divide(&mut w, red);
    assert!(claim::held(&w, red).contains(&far), "held by its camp");
    assert_eq!(claim::tile_civ(&w, far), Some(red));
}

#[test]
fn a_rival_unit_on_paper_takes_it_and_the_tiles_cut_off_behind_it() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&red].capital;
    // No guard: every claimed tile but the capital is paper.
    no_guard(&mut w, red);
    w.order(red, Order::Claim { tiles: vec![at_big(-4, 0), at_big(-3, 0), at_big(-2, 0), at_big(-1, 0)] }).unwrap();
    assert_eq!(claim::held(&w, red), vec![cap]);
    // Blue's camp ends its turn on the paper at (-3, 0): the tiles beyond it lose their join to the capital.
    put_camp(&mut w, blue, at_big(-3, 0));
    w.events.clear();
    claim::step(&mut w);
    assert_eq!(w.civs[&red].claim, vec![cap, at_big(-4, 0)], "the taken tile and the ones cut off behind it");
    let taken: Vec<_> = w.events.iter().filter(|e| e.kind == EventKind::ClaimTaken).collect();
    assert_eq!(taken.len(), 1);
    assert_eq!((taken[0].civ, taken[0].other, taken[0].tile), (Some(red), Some(blue), at_big(-3, 0)));
    assert_eq!(taken[0].amount, Some(3.0), "three tiles lost");
    assert!(taken[0].text(&w).contains("is taken by"), "{}", taken[0].text(&w));
}

#[test]
fn a_rival_unit_takes_nothing_on_held_land_nor_the_capital() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&red].capital;
    // Held: Red's guard holds the seven tiles round the capital.
    w.order(red, Order::Claim { tiles: within(&w, cap, 1) }).unwrap();
    let held = claim::held(&w, red);
    put_camp(&mut w, blue, at_big(-4, 0));
    claim::step(&mut w);
    assert_eq!(w.civs[&red].claim, held, "a held tile is not taken");
    // The capital: a rival standing on it takes nothing, even with no guard.
    no_guard(&mut w, red);
    put_camp(&mut w, blue, cap);
    claim::step(&mut w);
    assert!(w.civs[&red].claim.contains(&cap), "never the capital");
    assert_eq!(claim::tile_civ(&w, cap), Some(red));
}

#[test]
fn two_held_claims_on_one_tile_go_to_the_thicker_guard_and_a_tie_to_the_lower_id() {
    let (mut w, ids, _) = led();
    let (a, b) = (ids[0], ids[1]);
    let (ca, cb) = (w.civs[&a].capital, w.civs[&b].capital);
    let t = at_big(0, 0);
    let claim_of = |cap: usize| {
        let mut v = vec![cap, t];
        v.sort_unstable();
        v
    };
    let set = |w: &mut World, civ: u32, cap: usize, g: f64| {
        let claim = claim_of(cap);
        let guard = claim.iter().map(|&k| if k == t { g } else { 0.0 }).collect();
        let c = w.civs.get_mut(&civ).unwrap();
        c.claim = claim;
        c.guard = guard;
    };
    set(&mut w, a, ca, 0.5);
    set(&mut w, b, cb, 0.9);
    assert_eq!(claim::tile_civ(&w, t), Some(b), "the thicker guard");
    set(&mut w, a, ca, 0.9);
    set(&mut w, b, cb, 0.5);
    assert_eq!(claim::tile_civ(&w, t), Some(a));
    set(&mut w, a, ca, 0.5);
    set(&mut w, b, cb, 0.5);
    assert_eq!(claim::tile_civ(&w, t), Some(a.min(b)), "a tie goes to the lower id");
}

#[test]
fn the_ai_stand_in_is_the_capital_ring_and_a_civ_that_may_not_claim_keeps_its_capital() {
    let (w, ids, _) = three();
    for &civ in &ids {
        let cap = w.civs[&civ].capital;
        // The clans are on the capitals, so the stand-in is the ring one step out, the capital in it.
        assert_eq!(w.civs[&civ].claim, within(&w, cap, 1), "{civ}: the stand-in at founding");
        assert_eq!(claim::ai_claim(&w, civ), w.civs[&civ].claim, "{civ}: and it stays the stand-in");
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
fn an_ai_civ_claims_and_guards_its_land_on_the_stand_in_when_the_clans_are_moved() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    // A clan of Red's two steps out, within the reach: its home's ring joins the stand-in.
    let far = at_big(-3, 0);
    let k = put(&mut w, &[far], 6.0);
    w.clans.get_mut(&k).unwrap().civ = Some(red);
    w.player = Some(ids[2]);
    engine::units::ai(&mut w);
    let want = claim::ai_claim(&w, red);
    assert_ne!(want, within(&w, cap, 1), "the clan's home widens the stand-in");
    assert_eq!(w.civs[&red].claim, want, "the AI sent its stand-in claim");
}

#[test]
fn the_guard_eats_from_the_store_and_loses_a_person_to_the_own_clan_after_hungry_turns() {
    let (mut w, ids, clans) = led();
    let red = ids[0];
    let guard = units::guard_of(&w, red).expect("a guard at founding").id;
    let before = guard_people(&w, red);
    let own = w.clans[&clans[0]].people;
    for _ in 0..UNIT.hungry_turns {
        w.civs.get_mut(&red).unwrap().store = [0.0; N];
        units::step(&mut w);
    }
    assert_eq!(guard_people(&w, red), before - 1.0, "one person out of the guard");
    assert!(w.clans[&clans[0]].people >= own + 1.0, "they went home to the civ's own clan");
    let starved: Vec<_> =
        w.events.iter().filter(|e| e.kind == EventKind::WarriorsStarved && e.unit == Some(guard)).collect();
    assert_eq!(starved.len(), 1, "the loss is an event, and only the guard's");
    // Fed, the guard's hunger starts over.
    w.civs.get_mut(&red).unwrap().store = CIV.endowment;
    units::step(&mut w);
    assert_eq!(units::guard_of(&w, red).unwrap().hungry, 0);
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
fn raising_a_guard_leaves_standing_as_it_was_and_an_unaligned_clan_on_the_land_counts_toward_it() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let before = standing(&w)[&red];
    w.order(red, Order::Raise { n: 1, kind: UnitKind::Guard }).unwrap();
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
fn the_orders_of_the_guard_are_refused_with_the_reason_and_the_guard_takes_disband() {
    let (w, ids, _) = led();
    let red = ids[0];
    let guard = units::guard_of(&w, red).unwrap().id;
    let me = w.player.unwrap();
    assert_eq!(red, me);
    let refused = w.check_order(me, &Order::Move { unit: guard, to: 0 });
    assert_eq!(refused, Err("a guard only holds the claim: it takes no order but disband".to_string()));
    assert!(w.check_order(me, &Order::Disband { unit: guard, n: Some(1) }).is_ok());
}

#[test]
fn replaying_a_save_with_claims_and_guards_rebuilds_the_same_world() {
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
    w.act(led_order(&w, Order::Raise { n: 1, kind: UnitKind::Guard })).unwrap();
    for _ in 0..30 {
        w.act(Action::EndTurn).unwrap();
    }
    let r = World::replay(&w.save()).expect("the save replays");
    assert_eq!(digest(&r), digest(&w), "the same digest");
    assert_eq!(r.civs, w.civs, "the same claims and guards");
}

#[test]
fn a_claim_is_refused_before_the_game_begins() {
    let w = World::new(Config { seed: 2, radius: 6, ..Config::default() });
    assert_eq!(w.check_order(1, &Order::Claim { tiles: vec![0] }), Err("the game has not begun".to_string()));
}
