//! The fight (`units/fight.rs`): stacks of kinds, many armies, and the one rule by which people of two
//! civs on a tile fight. Built maps from `common` (bare plains): Red's capital is at `at_big(-5, 0)`,
//! Blue's at `at_big(5, 0)`, Gold's at `at_big(0, -10)`.

mod common;

use std::collections::BTreeMap;

use common::*;
use engine::claim;
use engine::data::*;
use engine::units::fight::{self, Part, Who};
use engine::units::{self, UnitKind};
use engine::{civs, clans, EventKind, Order, World};

/// Puts `civ`'s first army on `tile` with a stack of `stack` (warriors, archers), undeployed, its pack
/// full. Returns its id.
fn army(w: &mut World, civ: u32, tile: usize, stack: [f64; 2]) -> u32 {
    let id = put_camp(w, civ, tile);
    let u = camp_mut(w, id);
    u.actor.people = stack[0] + stack[1];
    u.actor.store = [0.0, 50.0, 0.0, 0.0, 0.0];
    let c = u.camp_mut().unwrap();
    c.stack = stack;
    c.guard = vec![1.0];
    c.dest = None;
    id
}

fn sums(w: &World) {
    for u in w.units.values() {
        if let Some(c) = u.camp() {
            assert!(
                (c.stack.iter().sum::<f64>() - u.people).abs() < 1e-9,
                "army {}: {:?} vs {}",
                u.id,
                c.stack,
                u.people
            );
        }
    }
}

/// Every person of the world: clans and units.
fn people(w: &World) -> f64 {
    w.clans.values().map(|c| c.people).sum::<f64>() + w.units.values().map(|u| u.people).sum::<f64>()
}

fn fallen(w: &World) -> f64 {
    w.civs.values().map(|c| c.army.fallen).sum()
}

/// Runs the fight step alone, with `holders` as the armies holding tiles at the turn's start.
fn fight_with(w: &mut World, holders: &[(usize, u32)]) {
    let h: BTreeMap<usize, u32> = holders.iter().copied().collect();
    fight::step(w, &h);
}

/// Red's 10 (or `red`) against Blue's `blue` on an unclaimed tile Blue held, Blue still `still` turns.
fn duel(red: f64, blue: f64, still: u32, blue_archers: bool) -> (World, u32, u32, usize) {
    let (mut w, ids, _) = led();
    let t = at_big(0, 5);
    assert_eq!(claim::tile_civ(&w, t), None);
    let r = army(&mut w, ids[0], t, [red, 0.0]);
    let b = army(&mut w, ids[1], t, if blue_archers { [0.0, blue] } else { [blue, 0.0] });
    camp_mut(&mut w, b).camp_mut().unwrap().still = still;
    fight_with(&mut w, &[(t, ids[1])]);
    (w, r, b, t)
}

#[test]
fn the_stack_sums_to_people_through_raise_join_disband_hunger_and_a_fight() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    fill_stores(&mut w);
    let first = led_camp(&w);
    w.act(led_order(&w, Order::Raise { n: 2, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: Some(first) }))
        .unwrap();
    assert_eq!(w.units[&first].camp().unwrap().stack, [w.units[&first].people, 0.0]);
    assert!(w.units[&first].people > 3.0, "joined");
    sums(&w);
    // A mixed stack, then disbanding by kind and in proportion.
    {
        let u = camp_mut(&mut w, first);
        u.actor.people = 5.0;
        u.camp_mut().unwrap().stack = [3.0, 2.0];
    }
    w.act(led_order(&w, Order::Disband { unit: first, n: Some(1), fighter: Some(Fighter::Archers) })).unwrap();
    assert_eq!(w.units[&first].camp().unwrap().stack, [3.0, 1.0]);
    w.act(led_order(&w, Order::Disband { unit: first, n: Some(2), fighter: None })).unwrap();
    let s = w.units[&first].camp().unwrap().stack;
    assert!((s[0] - 1.5).abs() < 1e-9 && (s[1] - 0.5).abs() < 1e-9, "in proportion: {s:?}");
    sums(&w);
    // Hunger's take comes off the stack.
    {
        let u = camp_mut(&mut w, first);
        u.actor.store = [0.0; N];
        u.hungry = UNIT.hungry_turns;
        let c = u.camp_mut().unwrap();
        c.auto_forage = false;
        c.auto_supply = false;
    }
    let before = w.units[&first].people;
    end_turn(&mut w);
    assert!(w.units[&first].people < before, "a warrior starved");
    sums(&w);
    // A fight.
    let (w, _, _, _) = duel(10.0, 4.0, 0, false);
    sums(&w);
    let _ = red;
}

#[test]
fn two_armies_lose_by_the_others_strength_and_the_weaker_retreats() {
    let (w, r, b, t) = duel(10.0, 4.0, 0, false);
    let (red, blue) = (w.units[&r].civ, w.units[&b].civ);
    // Red attacks at 10, Blue defends at 4.
    assert!((w.units[&r].people - (10.0 - FIGHT.rate * 4.0)).abs() < 1e-9);
    assert!((w.units[&b].people - (4.0 - FIGHT.rate * 10.0)).abs() < 1e-9);
    assert!((w.civs[&red].army.fallen - FIGHT.rate * 4.0).abs() < 1e-9);
    assert!((w.civs[&blue].army.fallen - FIGHT.rate * 10.0).abs() < 1e-9);
    // Blue is under `FIGHT.retreat` of Red after the losses: it retreats a step, and Red holds.
    assert_ne!(w.units[&b].home, t, "Blue's army retreated");
    assert!(engine::land::neighbours(&w, t).any(|n| n == w.units[&b].home), "a step back");
    assert_eq!(claim::tile_civ(&w, w.units[&b].home).or(engine::civs::holder(&w, w.units[&b].home)), Some(blue));
    assert!(w.units[&r].held().contains(&t));
    assert!(!w.units[&b].held().contains(&t));
    assert_eq!(kinds(&w, EventKind::Retreated), 1);
    assert_eq!(kinds(&w, EventKind::Skirmish), 1);
    assert_eq!(w.civs[&red].army.taken, 1, "the tile changed hands");
}

#[test]
fn an_entrenched_defender_costs_the_attacker_more_and_holds() {
    let (open, r0, b0, t) = duel(8.0, 4.0, 0, false);
    let (dug, r1, b1, _) = duel(8.0, 4.0, 5, false);
    let lost = |w: &World, id: u32, n: f64| n - w.units[&id].people;
    assert!(
        lost(&dug, r1, 8.0) > lost(&open, r0, 8.0) + 1e-9,
        "the attacker loses more against an entrenched defender"
    );
    assert_ne!(open.units[&b0].home, t, "in the open Blue retreats");
    assert_eq!(dug.units[&b1].home, t, "entrenched it stays");
}

#[test]
fn archers_defend_better_and_attack_worse() {
    let (w, r, _, _) = duel(6.0, 4.0, 0, false);
    let (wa, ra, _, _) = duel(6.0, 4.0, 0, true);
    assert!(wa.units[&ra].people < w.units[&r].people, "Red loses more against archers holding");
    // Archers attacking: Blue's warriors holding lose less than to warriors.
    let attack = |archers: bool| {
        let (mut w, ids, _) = led();
        let t = at_big(0, 5);
        army(&mut w, ids[0], t, if archers { [0.0, 6.0] } else { [6.0, 0.0] });
        let b = army(&mut w, ids[1], t, [4.0, 0.0]);
        fight_with(&mut w, &[(t, ids[1])]);
        4.0 - w.units[&b].people
    };
    assert!(attack(true) < attack(false));
}

#[test]
fn a_stack_of_ten_takes_a_guarded_tile_and_loses_almost_nothing() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&blue].capital;
    let ring: Vec<usize> =
        w.tiles[cap].hex().within(2).into_iter().filter_map(|h| engine::map::index(BIG, h)).collect();
    hold_land(&mut w, blue, &ring);
    let t = at_big(3, 0);
    assert_eq!(claim::tile_civ(&w, t), Some(blue));
    let r = army(&mut w, red, t, [10.0, 0.0]);
    let guard = units::guard_of(&w, blue).unwrap().people;
    let on = fight::presence(&w)[&t].iter().filter(|p| p.civ == blue).map(|p| p.people).sum::<f64>();
    fight_with(&mut w, &[]);
    assert!((units::guard_of(&w, blue).unwrap().people - (guard - on)).abs() < 1e-9, "its presence there is wiped");
    assert!(!w.civs[&blue].claim.contains(&t), "the tile left the claim");
    assert_eq!(kinds(&w, EventKind::TileTaken), 1);
    assert_eq!(kinds(&w, EventKind::ClaimTaken), 0);
    assert!(10.0 - w.units[&r].people < 0.05, "lost {}", 10.0 - w.units[&r].people);
    assert_eq!(w.civs[&red].army.taken, 1);
}

#[test]
fn two_guards_on_an_overlap_both_lose_and_neither_gives_way() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let line = |q0: i32, q1: i32| (q0.min(q1)..=q0.max(q1)).map(|q| at_big(q, 0)).collect::<Vec<_>>();
    hold_land(&mut w, red, &line(-4, 0));
    hold_land(&mut w, blue, &line(0, 4));
    units::guard::raise(&mut w, red, 3);
    let t = at_big(0, 0);
    let share = |w: &World, c: u32| fight::presence(w)[&t].iter().filter(|p| p.civ == c).map(|p| p.people).sum::<f64>();
    let (r, b) = (share(&w, red), share(&w, blue));
    assert!((r / b - 2.0).abs() < 0.2, "Red's guard there is twice Blue's: {r} and {b}");
    let (gr, gb) = (units::guard_of(&w, red).unwrap().people, units::guard_of(&w, blue).unwrap().people);
    let before = (w.civs[&red].claim.clone(), w.civs[&blue].claim.clone());
    fight_with(&mut w, &[]);
    assert!(units::guard_of(&w, red).unwrap().people < gr && units::guard_of(&w, blue).unwrap().people < gb);
    assert_eq!((w.civs[&red].claim.clone(), w.civs[&blue].claim.clone()), before, "neither tile leaves its claim");
    assert_eq!(kinds(&w, EventKind::Retreated), 0);
    assert_eq!(kinds(&w, EventKind::Skirmish), 1);
    // The fight goes on the next turn without a second skirmish.
    fight_with(&mut w, &[]);
    assert_eq!(kinds(&w, EventKind::Skirmish), 1);
}

#[test]
fn a_march_stops_on_a_rivals_held_tile_and_not_on_paper() {
    let march = |paper: bool| {
        let (mut w, ids, _) = led();
        let (red, blue) = (ids[0], ids[1]);
        let line: Vec<usize> = (1..=4).map(|q| at_big(q, 0)).collect();
        hold_land(&mut w, blue, &line);
        if paper {
            no_guard(&mut w, blue);
        }
        let r = army(&mut w, red, at_big(-1, 0), [3.0, 0.0]);
        camp_mut(&mut w, r).camp_mut().unwrap().dest = Some(at_big(3, 0));
        units::step(&mut w);
        w.units[&r].home
    };
    assert_eq!(march(false), at_big(1, 0), "it stops on the first of Blue's held tiles");
    assert_ne!(march(true), at_big(1, 0), "paper doesn't stop it");
}

#[test]
fn the_fight_conserves_people_and_with_it_off_nobody_falls() {
    for on in [true, false] {
        let (mut w, ids, _) = led();
        w.config.fight = on;
        let cap = w.civs[&ids[1]].capital;
        let ring: Vec<usize> =
            w.tiles[cap].hex().within(2).into_iter().filter_map(|h| engine::map::index(BIG, h)).collect();
        hold_land(&mut w, ids[1], &ring);
        army(&mut w, ids[0], at_big(3, 0), [10.0, 0.0]);
        army(&mut w, ids[1], at_big(3, 0), [2.0, 0.0]);
        let before = people(&w);
        fight_with(&mut w, &[]);
        assert!((before - people(&w) - fallen(&w)).abs() < 1e-9, "conserved");
        if on {
            assert!(fallen(&w) > 0.0);
        } else {
            assert_eq!(fallen(&w), 0.0, "fight off: nobody falls");
            assert_eq!(kinds(&w, EventKind::Retreated), 0);
            assert!(w.civs[&ids[1]].claim.contains(&at_big(3, 0)));
        }
    }
}

#[test]
fn a_scout_on_a_rivals_held_tile_neither_fights_nor_takes_it() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let line: Vec<usize> = (1..=4).map(|q| at_big(q, 0)).collect();
    hold_land(&mut w, blue, &line);
    let s = led_scout(&w);
    camp_mut(&mut w, s).actor.home = at_big(2, 0);
    let claim = w.civs[&blue].claim.clone();
    units::step(&mut w);
    claim::step(&mut w);
    assert_eq!(fallen(&w), 0.0);
    assert_eq!(w.civs[&blue].claim, claim);
    // Nor paper.
    no_guard(&mut w, blue);
    claim::step(&mut w);
    assert_eq!(w.civs[&blue].claim, claim, "a scout takes no paper");
    let _ = red;
}

#[test]
fn raise_into_a_cut_army_is_refused_and_raise_with_none_makes_another() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    fill_stores(&mut w);
    let first = led_camp(&w);
    put_camp(&mut w, red, at_big(-5, 11));
    assert!(units::cut_off(&w, first));
    assert_eq!(
        w.act(led_order(
            &w,
            Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: Some(first) }
        )),
        Err("its line is cut: raise at the capital".to_string())
    );
    w.act(led_order(&w, Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Warriors, into: None })).unwrap();
    let armies = w.units.values().filter(|u| u.civ == red && u.kind() == UnitKind::Camp).count();
    assert_eq!(armies, 2, "a second army");
    assert!(w
        .act(led_order(&w, Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Archers, into: None }))
        .is_err());
}

#[test]
fn a_tile_taken_from_an_ai_civ_is_not_fought_for_again() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let t = at_big(4, 0);
    assert_eq!(claim::tile_civ(&w, t), Some(blue), "the AI's first claim holds it");
    let r = army(&mut w, red, t, [10.0, 0.0]);
    camp_mut(&mut w, r).camp_mut().unwrap().auto_supply = false;
    // The AI's claim gives the tile up under the stack (its step runs before the fight's), and its
    // army doesn't march onto a stack it can't beat, so the tile is taken at most once and never
    // fought for again.
    for _ in 0..3 {
        end_turn(&mut w);
    }
    let at_t = |kind: EventKind| w.events.iter().filter(|e| e.kind == kind && e.tile == t).count();
    assert!(at_t(EventKind::TileTaken) <= 1, "taken once at most");
    assert_eq!(at_t(EventKind::Skirmish), at_t(EventKind::TileTaken), "and fought for no more than that");
    assert!(!w.civs[&blue].claim.contains(&t), "the AI doesn't paint it back under the stack");
}

/// A new army of `stack` on `tile`: a copy of `civ`'s first army, set down there undeployed with a
/// full pack (so no clan is drawn on).
fn new_army(w: &mut World, civ: u32, tile: usize, stack: [f64; 2]) -> u32 {
    let mut u = units::camp(w, civ).expect("a camp to copy").clone();
    let id = w.next_id;
    w.next_id += 1;
    u.id = id;
    u.actor.people = stack[0] + stack[1];
    u.actor.home = tile;
    u.actor.territory = vec![tile];
    u.actor.store = [0.0, 50.0, 0.0, 0.0, 0.0];
    u.hungry = 0;
    u.banked = 0.0;
    let c = u.camp_mut().unwrap();
    c.stack = stack;
    c.held = vec![tile];
    c.guard = vec![0.0];
    c.dest = None;
    c.still = 0;
    w.units.insert(id, u);
    id
}

/// The people of civ's parts on a tile of a presence map.
fn parts_of(w: &World, t: usize, civ: u32) -> Vec<Part> {
    fight::presence(w).get(&t).map(|ps| ps.iter().copied().filter(|p| p.civ == civ).collect()).unwrap_or_default()
}

#[test]
fn an_army_on_a_rival_clans_home_meets_its_militia_and_both_lose() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let t = at_big(0, 5);
    let k = put(&mut w, &[t], 10.0);
    w.clans.get_mut(&k).unwrap().civ = Some(blue);
    army(&mut w, red, t, [10.0, 0.0]);
    let parts = parts_of(&w, t, blue);
    assert!(
        parts.iter().any(|p| p.who == Who::Militia(k) && (p.people - 10.0).abs() < 1e-9),
        "the clan's people defend its home: {parts:?}"
    );
    let p = parts.iter().find(|p| p.who == Who::Militia(k)).unwrap();
    assert!((p.defence - FIGHT.militia * FIGHTERS[0].defence).abs() < 1e-9, "a fraction of a warrior's defence");
    fight_with(&mut w, &[(t, red)]);
    assert!(w.clans[&k].people < 10.0, "the clan loses people");
    assert!(w.units[&camp_id(&w, red)].people < 10.0, "the army loses people");
    assert!(fallen(&w) > 0.0);
    sums(&w);
}

#[test]
fn an_unaligned_clan_on_a_rivals_army_gives_no_militia() {
    let (mut w, ids, _) = led();
    let (red, _) = (ids[0], ids[1]);
    let t = at_big(0, 5);
    let k = put(&mut w, &[t], 10.0);
    assert_eq!(w.clans[&k].civ, None);
    army(&mut w, red, t, [10.0, 0.0]);
    assert!(parts_of(&w, t, ids[1]).is_empty(), "nobody's clan");
    fight_with(&mut w, &[(t, red)]);
    assert_eq!(w.clans[&k].people, 10.0, "the unaligned clan is not drawn in");
}

#[test]
fn a_rival_army_on_the_capital_draws_the_whole_guard_there_and_the_claim_is_paper() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&blue].capital;
    hold_land(&mut w, blue, &[at_big(4, 0), at_big(3, 0)]);
    assert!(claim::held(&w, blue).len() > 1, "the claim holds past the capital");
    let people = units::guard_of(&w, blue).unwrap().people;
    army(&mut w, red, cap, [4.0, 0.0]);
    fight::posture(&mut w);
    assert!(w.civs[&blue].pressed, "pressed while a rival stands on the capital");
    assert_eq!(claim::held(&w, blue), vec![cap], "the rest of the claim is paper");
    let guard: Vec<Part> = fight::presence(&w)
        .into_iter()
        .flat_map(|(t, ps)| {
            ps.into_iter().filter(|p| p.civ == blue && matches!(p.who, Who::Guard(_))).map(move |p| (t, p))
        })
        .map(|(t, p)| {
            assert_eq!(t, cap, "the guard is on the capital");
            p
        })
        .collect();
    assert_eq!(guard.len(), 1);
    assert!((guard[0].people - people).abs() < 1e-9, "all its people, entrenched at the cap");
    assert!((guard[0].defence - FIGHTERS[0].defence * fight::entrenched(u32::MAX)).abs() < 1e-9);
    // The army leaves: the capital is no longer pressed and the guard spreads again.
    let r = camp_id(&w, red);
    camp_mut(&mut w, r).actor.home = at_big(0, 5);
    fight::posture(&mut w);
    assert!(!w.civs[&blue].pressed);
    assert!(claim::held(&w, blue).len() > 1);
}

#[test]
fn a_capital_ringed_by_rival_armies_is_besieged_and_its_clans_harvest_nothing() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let cap = w.civs[&blue].capital;
    let mid = w.civs[&blue].market;
    let bc = w.civs[&blue].clan;
    w.clans.get_mut(&bc).unwrap().market = Some(mid);
    w.tiles[cap].stock = [5.0; N];
    clans::harvest(&mut w);
    assert!(w.clans[&bc].harvest.iter().sum::<f64>() > 0.0, "the capital's clan harvests");
    let ring: Vec<usize> = engine::land::neighbours(&w, cap).collect();
    let rings: Vec<u32> = ring.iter().map(|&n| new_army(&mut w, red, n, [2.0, 0.0])).collect();
    fight::posture(&mut w);
    assert!(w.civs[&blue].besieged, "every neighbour is held or stood on by a rival army");
    w.tiles[cap].stock = [5.0; N];
    clans::harvest(&mut w);
    assert_eq!(w.clans[&bc].harvest.iter().sum::<f64>(), 0.0, "the siege stops its clans working");
    // The levy puts nothing in the store while besieged, and does once the siege is lifted.
    w.clans.get_mut(&bc).unwrap().basket = [1.0; N];
    let store = w.civs[&blue].store;
    civs::levy(&mut w, mid);
    assert_eq!(w.civs[&blue].store, store, "no levy");
    for id in rings {
        w.units.remove(&id);
    }
    fight::posture(&mut w);
    assert!(!w.civs[&blue].besieged);
    w.clans.get_mut(&bc).unwrap().basket = [1.0; N];
    civs::levy(&mut w, mid);
    assert!(w.civs[&blue].store.iter().sum::<f64>() > store.iter().sum::<f64>(), "the levy comes in again");
}

/// Blue's neighbours of `t` are held by its own armies, so Red's army at `t` is cut off from its
/// capital by rival ground. Returns Red's army.
fn ringed(w: &mut World, red: u32, blue: u32, t: usize) -> u32 {
    for n in engine::land::neighbours(w, t).collect::<Vec<_>>() {
        new_army(w, blue, n, [2.0, 0.0]);
    }
    new_army(w, red, t, [3.0, 0.0])
}

/// How many times army `id` lost a warrior to hunger, over the world's events.
fn starved(w: &World, id: u32) -> usize {
    w.events.iter().filter(|e| e.kind == EventKind::WarriorsStarved && e.unit == Some(id)).count()
}

/// Red's army at `t` with an empty pack and no food coming: it goes hungry each turn.
fn starving(w: &mut World, id: u32) {
    let u = camp_mut(w, id);
    u.actor.store = [0.0; N];
    let c = u.camp_mut().unwrap();
    c.auto_forage = false;
    c.auto_supply = false;
}

#[test]
fn an_army_whose_line_a_rivals_land_cuts_surrenders_to_it_before_hunger_takes_a_warrior() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    let t = at_big(-3, 1);
    let r = ringed(&mut w, red, blue, t);
    starving(&mut w, r);
    let bc = w.civs[&blue].clan;
    assert!(units::cut_off(&w, r) && units::line(&w, r).is_none(), "the ring cuts every way home");
    assert_eq!(units::cut_by_rival(&w, r), Some(blue));
    let before = people(&w);
    for _ in 0..(FIGHT.surrender - 1) {
        units::step(&mut w);
    }
    assert!(w.units.contains_key(&r), "two hungry turns: still an army");
    units::step(&mut w);
    assert!(!w.units.contains_key(&r), "the third hungry turn: it surrenders");
    assert_eq!(starved(&w, r), 0, "hunger took no warrior");
    assert_eq!(kinds(&w, EventKind::ArmySurrendered), 1);
    let e = w.events.iter().find(|e| e.kind == EventKind::ArmySurrendered).unwrap().clone();
    assert_eq!((e.civ, e.other, e.unit), (Some(red), Some(blue), Some(r)));
    assert!((e.amount.unwrap() - 3.0).abs() < 1e-9, "all three");
    assert_eq!(e.clan, Some(bc), "the rival's nearest clan");
    assert_eq!(w.civs[&red].army.surrendered, 1);
    assert_eq!(w.civs[&red].army.fallen, 0.0, "the people are not fallen");
    assert!((people(&w) - before).abs() < 1e-9, "people are conserved");
    assert!(w.notes[&red].iter().any(|n| n.text.contains("surrendered")), "Red hears");
    assert!(w.notes[&blue].iter().any(|n| n.text.contains("surrendered")), "Blue hears");
}

#[test]
fn an_army_cut_off_by_length_alone_starves_as_it_does_on_main() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let t = at_big(-5, 11);
    put_camp(&mut w, red, t);
    let r = camp_id(&w, red);
    starving(&mut w, r);
    camp_mut(&mut w, r).actor.people = 3.0;
    assert!(units::cut_off(&w, r));
    assert_eq!(units::cut_by_rival(&w, r), None, "no rival's ground is on its line");
    for _ in 0..UNIT.hungry_turns {
        units::step(&mut w);
    }
    assert_eq!(kinds(&w, EventKind::ArmySurrendered), 0);
    assert!(starved(&w, r) >= 1, "hunger takes a warrior");
}

#[test]
fn with_fighting_off_an_army_cut_off_by_rival_land_does_not_surrender() {
    let (mut w, ids, _) = led();
    let (red, blue) = (ids[0], ids[1]);
    w.config.fight = false;
    let t = at_big(-3, 1);
    let r = ringed(&mut w, red, blue, t);
    starving(&mut w, r);
    for _ in 0..UNIT.hungry_turns {
        units::step(&mut w);
    }
    assert_eq!(kinds(&w, EventKind::ArmySurrendered), 0);
    assert!(starved(&w, r) >= 1, "hunger works as on main");
    assert_eq!(kinds(&w, EventKind::Skirmish), 0);
}

#[test]
fn archers_need_held_forest_and_their_army_makes_bows_a_turn() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let cap = w.civs[&red].capital;
    let refused =
        w.check_order(red, &Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Archers, into: None });
    assert_eq!(refused, Err("you hold no forest to make bows on".to_string()));
    w.tiles[cap].terrain = FOREST;
    fill_stores(&mut w);
    let first = led_camp(&w);
    let home = w.units[&first].home;
    camp_mut(&mut w, first).camp_mut().unwrap().dest = Some(at_big(-3, 0));
    w.act(led_order(&w, Order::Raise { n: 1, kind: UnitKind::Camp, fighter: Fighter::Archers, into: Some(first) }))
        .unwrap();
    let c = w.units[&first].camp().unwrap();
    assert!(c.bows, "the bows take the turn");
    assert_eq!(c.stack[Fighter::Archers.index()], 1.0);
    units::step(&mut w);
    assert_eq!(w.units[&first].home, home, "no march that turn");
    assert!(!w.units[&first].camp().unwrap().bows, "cleared at the turn's end");
    assert_eq!(kinds(&w, EventKind::ArchersRaised), 1);
    sums(&w);
    units::step(&mut w);
    assert_ne!(w.units[&first].home, home, "it marches the turn after");
}
