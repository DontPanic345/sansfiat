//! Alerts: the led civ's trouble, flagged once as it starts, and `PeopleLeft` for the people a civ's
//! clans send to clans not of it.

mod common;

use common::*;
use engine::civs;
use engine::clans::surplus;
use engine::data::*;
use engine::markets;
use engine::units::{self, Unit};
use engine::{Action, End, Event, EventKind, World};

/// Markets for three civs on the big world, 10 or more steps apart.
fn markets3(w: &mut World) -> [u32; 3] {
    [open(w, at_big(-5, 0)), open(w, at_big(5, 0)), open(w, at_big(0, -10))]
}

/// Ends the pre-game on `centres` by hand and founds the civs.
fn found(w: &mut World, centres: &[u32]) {
    w.end = Some(End {
        turn: w.turn,
        centres: centres.to_vec(),
        clans: Vec::new(),
        cover: 1.0,
        by_limit: false,
        topped_up: Vec::new(),
    });
    civs::found(w);
}

/// A big world of three civs, each with a clan of 8 on its market's tile and a camp. Returns the
/// world and the civs (in id order).
fn three() -> (World, [u32; 3]) {
    let mut w = big();
    let ms = markets3(&mut w);
    put(&mut w, &[at_big(-5, 0)], 8.0);
    put(&mut w, &[at_big(5, 0)], 8.0);
    put(&mut w, &[at_big(0, -10)], 8.0);
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    (w, [ids[0], ids[1], ids[2]])
}

fn camp_of(w: &World, civ: u32) -> &Unit {
    units::camp(w, civ).expect("a camp")
}

/// Moves `civ`'s camp onto `tile`, empty of food and with no foraging or supply, so it starves.
fn starve_camp_on(w: &mut World, civ: u32, tile: usize) {
    let id = camp_of(w, civ).id;
    let u = w.units.get_mut(&id).unwrap();
    u.actor.home = tile;
    u.actor.territory = vec![tile];
    u.camp_mut().unwrap().held = vec![tile];
    u.actor.store = [0.0; N];
    u.camp_mut().unwrap().auto_forage = false;
    u.camp_mut().unwrap().auto_supply = false;
}

/// The events of `kind` about `civ`'s camp or scout, in order.
fn about(w: &World, kind: EventKind, civ: u32) -> Vec<&Event> {
    w.events.iter().filter(|e| e.kind == kind && e.civ == Some(civ)).collect()
}

#[test]
fn a_led_camp_going_hungry_alerts_once_and_its_starving_once_per_loss() {
    let (mut w, ids) = three();
    let red = ids[0];
    w.act(Action::ChooseCiv { civ: Some(red) }).unwrap();
    starve_camp_on(&mut w, red, at_big(-5, 8));
    for _ in 0..UNIT.hungry_turns + 3 {
        w.act(Action::EndTurn).unwrap();
    }
    let hungry = about(&w, EventKind::CampHungry, red);
    assert_eq!(hungry.len(), 1, "announced once");
    assert!(hungry[0].alert, "its hunger starts: an alert");
    // Each turn the starving camp loses a warrior: the first is an alert, the next turns' are not.
    let starved = about(&w, EventKind::WarriorsStarved, red);
    assert!(starved.len() >= 2, "warriors starve on more than one turn: {}", starved.len());
    assert!(starved[0].alert, "the first loss is an alert");
    assert!(starved[1..].iter().all(|e| !e.alert), "the losses that follow are not");
}

#[test]
fn nothing_is_an_alert_when_watching() {
    let (mut w, ids) = three();
    let red = ids[0];
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    starve_camp_on(&mut w, red, at_big(-5, 8));
    for _ in 0..UNIT.hungry_turns + 3 {
        w.act(Action::EndTurn).unwrap();
    }
    assert!(!about(&w, EventKind::CampHungry, red).is_empty(), "the camp went hungry");
    assert!(w.events.iter().all(|e| !e.alert), "watching, nothing is an alert");
}

#[test]
fn a_rival_clan_joining_alerts_and_the_players_does_not() {
    let (mut w, ids) = three();
    let (red, blue) = (ids[0], ids[1]);
    w.act(Action::ChooseCiv { civ: Some(red) }).unwrap();
    let mine = put(&mut w, &[at_big(-2, 9)], 8.0);
    let theirs = put(&mut w, &[at_big(2, 9)], 8.0);
    w.clans.get_mut(&mine).unwrap().leaning.insert(red, LEANING.join * 2.0);
    w.clans.get_mut(&theirs).unwrap().leaning.insert(blue, LEANING.join * 2.0);
    w.events.clear();
    w.act(Action::EndTurn).unwrap();
    let joined: Vec<&Event> = w.events.iter().filter(|e| e.kind == EventKind::ClanJoined).collect();
    let by = |civ: u32| joined.iter().find(|e| e.civ == Some(civ)).expect("a clan joined it");
    assert!(!by(red).alert, "the player's own clan joining is not an alert");
    assert!(by(blue).alert, "a rival's clan joining is an alert");
}

/// Two civs' clans and a third, at market `mid`: a starving clan of `red` (4 people, half fed), a
/// rich clan of `blue`, and a rich clan of `red` (for the people who stay in the civ). Returns the
/// starving clan's id and the rich clans' ids.
fn festival(w: &mut World, civs: [u32; 2], mid: u32) -> (u32, u32, u32) {
    let [red, blue] = civs;
    let starving = put(w, &[at_big(-5, 3)], 4.0);
    let rich_blue = put(w, &[at_big(-5, 5)], 4.0);
    let rich_red = put(w, &[at_big(-4, 5)], 4.0);
    for (id, civ) in [(starving, red), (rich_blue, blue), (rich_red, red)] {
        let c = w.clans.get_mut(&id).unwrap();
        c.civ = Some(civ);
        c.market = Some(mid);
        c.harvest = [0.0, 10.0, 0.0, 0.0, 0.0];
    }
    let c = w.clans.get_mut(&starving).unwrap();
    c.starving = true;
    c.fed_ratio = 0.5;
    (starving, rich_blue, rich_red)
}

#[test]
fn people_leaving_a_civs_clans_for_another_civ_are_one_event_and_sum_what_moved() {
    let (mut w, ids) = three();
    let (red, blue) = (ids[0], ids[1]);
    let mid = w.civs[&red].market;
    let (starving, rich_blue, rich_red) = festival(&mut w, [red, blue], mid);
    // What the starving clan offers (the rule's own), and the rooms the two rich clans have.
    let offer = MARKET.shed * 4.0 * (1.0 - 0.5);
    let rooms = [surplus(&w.clans[&rich_blue].harvest, 4.0), surplus(&w.clans[&rich_red].harvest, 4.0)];
    let room = rooms[0] + rooms[1];
    let moved = offer.min(room);
    let away = moved * rooms[0] / room;
    w.events.clear();
    markets::people(&mut w);
    let left: Vec<&Event> = w.events.iter().filter(|e| e.kind == EventKind::PeopleLeft).collect();
    assert_eq!(left.len(), 1, "one event for the one civ that loses people: {left:?}");
    assert_eq!(left[0].civ, Some(red));
    assert_eq!(left[0].tile, w.civs[&red].capital, "it is at the civ's capital");
    let amount = left[0].amount.expect("an amount");
    assert!((amount - away).abs() < 1e-9, "the share that went to blue's clan: {amount} vs {away}");
    let text = left[0].text(&w);
    assert!(text.ends_with("for others") && text.contains("people leave"), "{text}");
    // The people moved are the people the rule moved, to the clans that took them.
    let gained = (w.clans[&rich_blue].people - 4.0) + (w.clans[&rich_red].people - 4.0);
    assert!((gained - moved).abs() < 1e-9, "the rich clans took the offer, {gained} of {moved}");
    assert!((w.clans[&starving].people - (4.0 - moved)).abs() < 1e-9, "and the starving clan gave it");
}

#[test]
fn a_clan_merged_into_another_civs_clan_is_people_leaving_its_civ() {
    let (mut w, ids) = three();
    let (red, blue) = (ids[0], ids[1]);
    let mid = w.civs[&red].market;
    let small = put(&mut w, &[at_big(-5, 2)], 1.0);
    let other = put(&mut w, &[at_big(-5, 3)], 4.0);
    for (id, civ) in [(small, red), (other, blue)] {
        let c = w.clans.get_mut(&id).unwrap();
        c.civ = Some(civ);
        c.market = Some(mid);
    }
    w.events.clear();
    markets::people(&mut w);
    assert!(!w.clans.contains_key(&small), "the small clan merged");
    let left: Vec<&Event> = w.events.iter().filter(|e| e.kind == EventKind::PeopleLeft).collect();
    assert_eq!(left.len(), 1, "{left:?}");
    assert_eq!(left[0].civ, Some(red));
    assert!((left[0].amount.unwrap() - 1.0).abs() < 1e-9, "its one person left for blue's clan");
}
