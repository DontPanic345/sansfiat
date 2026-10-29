mod common;

use common::*;
use engine::civs::{self, levy};
use engine::clans::{adjust, eat, harvest};
use engine::data::*;
use engine::land::regrow;
use engine::units::{self, Unit};
use engine::{Action, Config, End, EventKind, Phase, World};

/// Markets for three civs on the big world, 10 or more steps apart.
fn markets3(w: &mut World) -> [u32; 3] {
    [open(w, at_big(-5, 0)), open(w, at_big(5, 0)), open(w, at_big(0, -10))]
}

/// Ends the pre-game on `centres` by hand and founds the civs.
fn found(w: &mut World, centres: &[u32]) {
    w.end = Some(End { turn: w.turn, centres: centres.to_vec(), cover: 1.0, by_limit: false, topped_up: Vec::new() });
    civs::found(w);
}

/// A big world of three civs, each with a clan of 8 on its market's tile and a camp of
/// `UNIT.start` on it. Returns the world, the civs and their clans.
fn three() -> (World, [u32; 3], [u32; 3]) {
    let mut w = big();
    let ms = markets3(&mut w);
    let clans =
        [put(&mut w, &[at_big(-5, 0)], 8.0), put(&mut w, &[at_big(5, 0)], 8.0), put(&mut w, &[at_big(0, -10)], 8.0)];
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    (w, [ids[0], ids[1], ids[2]], clans)
}

/// As `three`, led by the first civ.
fn led() -> (World, [u32; 3], [u32; 3]) {
    let (mut w, ids, clans) = three();
    w.act(Action::ChooseCiv { civ: Some(ids[0]) }).unwrap();
    w.events.clear();
    (w, ids, clans)
}

fn camp_id(w: &World, civ: u32) -> u32 {
    units::camp(w, civ).expect("a camp").id
}

/// Moves `civ`'s camp onto `tile`.
fn put_camp(w: &mut World, civ: u32, tile: usize) -> u32 {
    let id = camp_id(w, civ);
    let u = w.units.get_mut(&id).unwrap();
    u.actor.home = tile;
    u.actor.territory = vec![tile];
    u.held = vec![tile];
    id
}

fn camp_mut(w: &mut World, id: u32) -> &mut Unit {
    w.units.get_mut(&id).unwrap()
}

fn total(b: &[f64; N]) -> f64 {
    b.iter().sum()
}

fn kinds(w: &World, kind: EventKind) -> usize {
    w.events.iter().filter(|e| e.kind == kind).count()
}

fn end_turn(w: &mut World) {
    w.act(Action::EndTurn).unwrap();
}

#[test]
fn founding_makes_the_biggest_near_clan_the_civ_with_the_market_on_its_home() {
    let mut w = big();
    let ms = markets3(&mut w);
    let small = put(&mut w, &[at_big(-4, 1)], 5.0);
    // The biggest clan joins Red by a tile of its land, but its home is out of the festival's reach.
    let far = put(&mut w, &[at_big(-1, 0), at_big(-2, 0), at_big(-3, 0)], 20.0);
    found(&mut w, &ms);
    let red = w.civs.values().next().unwrap();
    assert_eq!(w.clans[&far].civ, Some(red.id), "the far clan joined");
    assert_eq!(red.clan, small, "but the near one is the civ");
    assert_eq!(red.capital, at_big(-4, 1));
    assert_eq!(w.markets[&red.market].tile, at_big(-4, 1), "the festival moved to its home");
    assert_eq!(red.army.raised, 1);
    let camp = units::camp(&w, red.id).unwrap();
    assert_eq!((camp.tile(), camp.people), (at_big(-4, 1), f64::from(UNIT.start)));
    // One person from each clan in turn, biggest first: the far clan twice.
    assert_eq!((w.clans[&small].people, w.clans[&far].people), (4.0, 18.0));
    // A pack each from the store.
    assert!((total(&camp.store) - pack_total(camp.people)).abs() < 1e-9);
    assert!((total(&red.store) - (total(&CIV.endowment) - pack_total(camp.people))).abs() < 1e-9);
    // The civs with no clan in reach are left without a camp, but Blue and Gold had no clans at all.
    assert_eq!(w.units.len(), 1);
}

fn pack_total(people: f64) -> f64 {
    people * UNIT.pack * UNIT.food
}

#[test]
fn the_civs_clan_stays_on_the_capital_through_adjusting_and_dividing() {
    let (mut w, ids, clans) = three();
    let (red, a) = (ids[0], clans[0]);
    let cap = w.civs[&red].capital;
    assert_eq!(w.civs[&red].clan, a);
    // Its land stretches one way, so a clan would recentre; the civ's stays put.
    {
        let c = w.clans.get_mut(&a).unwrap();
        c.territory = vec![cap, at_big(-4, 0), at_big(-3, 0)];
        c.territory.sort_unstable();
    }
    for t in w.clans[&a].territory.clone() {
        w.tiles[t].claim = Some(a);
    }
    adjust(&mut w);
    assert_eq!(w.clans[&a].home, cap, "pinned to the capital");
    // Dividing: the half holding the capital stays the civ's clan.
    {
        let c = w.clans.get_mut(&a).unwrap();
        c.people = CLAN.split_at + 1.0;
        c.basket = [30.0; N];
    }
    let before = w.clans.len();
    eat(&mut w);
    assert_eq!(w.clans.len(), before + 1, "the clan divided");
    assert_eq!(w.civs[&red].clan, a);
    assert_eq!(w.clans[&a].home, cap);
    let daughter = w.clans.values().find(|c| c.id != a && c.civ == Some(red)).unwrap();
    let (daughter, daughter_home) = (daughter.id, daughter.home);
    assert_ne!(daughter_home, cap);
    // If the civ's clan is gone, the civ's biggest takes its place; the capital stays.
    w.clans.remove(&a);
    civs::upkeep(&mut w);
    assert_eq!(w.civs[&red].clan, daughter);
    assert_eq!(w.civs[&red].capital, cap);
}

/// A harvest a bit over a clan of 4's need, so it grows by less than the most.
const HARVEST: [f64; N] = [1.03, 1.03, 1.03, 1.03, 0.0];

/// Gives clan `id` a harvest and a basket of it, its people unchanged.
fn harvested(w: &mut World, id: u32, market: Option<u32>, civ: Option<u32>) {
    let c = w.clans.get_mut(&id).unwrap();
    c.harvest = HARVEST;
    c.basket = c.harvest;
    c.market = market;
    c.civ = civ;
}

#[test]
fn the_levy_comes_only_from_clans_walking_to_the_capital_out_of_their_basket_before_growth() {
    let (mut w, ids, clans) = three();
    let (red, a) = (ids[0], clans[0]);
    let mid = w.civs[&red].market;
    let walker = put(&mut w, &[at_big(-4, 1)], 4.0);
    let stayer = put(&mut w, &[at_big(-6, 1)], 4.0);
    let stranger = put(&mut w, &[at_big(-4, -1)], 4.0);
    let twin = put(&mut w, &[at_big(-7, -1)], 4.0);
    harvested(&mut w, walker, Some(mid), Some(red));
    harvested(&mut w, stayer, None, Some(red));
    harvested(&mut w, stranger, Some(mid), None);
    harvested(&mut w, twin, None, None);
    w.clans.get_mut(&a).unwrap().harvest = [0.0; N];
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    let s = total(&w.clans[&walker].basket);
    assert!(CIV.levy * s < CIV.store_cap);
    levy(&mut w, mid);
    let paid = total(&w.civs[&red].store);
    assert!((paid - CIV.levy * s).abs() < 1e-9, "{paid} vs {}", CIV.levy * s);
    assert!((total(&w.clans[&walker].harvest) - (total(&HARVEST) - paid)).abs() < 1e-9);
    for id in [stayer, stranger, twin] {
        assert_eq!(w.clans[&id].harvest, HARVEST, "clan {id} paid nothing");
    }
    // Out of the harvest, so the levied clan grows less than the same clan unlevied.
    let (before, after) = (w.clans[&twin].people, w.clans[&walker].people);
    assert_eq!(before, after);
    eat(&mut w);
    assert!(w.clans[&twin].people > before, "the unlevied clan grows");
    assert!(w.clans[&walker].people < w.clans[&twin].people, "the levied clan grows less");
}

#[test]
fn the_civs_own_clan_always_walks_to_the_capitals_market() {
    let (mut w, ids, clans) = three();
    engine::markets::go(&mut w);
    let c = &w.clans[&clans[0]];
    assert_eq!(c.market, Some(w.civs[&ids[0]].market));
    assert_eq!(c.routes.len(), 1);
    assert_eq!(c.routes[0].cost, 0.0);
}

#[test]
fn raising_drops_n_clans_by_one_each() {
    let (mut w, ids, clans) = led();
    let red = ids[0];
    let b = put(&mut w, &[at_big(-6, 1)], 6.0);
    let c = put(&mut w, &[at_big(-4, 1)], 4.0);
    let d = put(&mut w, &[at_big(-7, -1)], 1.5);
    for id in [b, c, d] {
        w.clans.get_mut(&id).unwrap().civ = Some(red);
    }
    let people = |w: &World, id: u32| w.clans[&id].people;
    let before = [people(&w, clans[0]), people(&w, b), people(&w, c), people(&w, d)];
    w.civs.get_mut(&red).unwrap().store = CIV.endowment;
    let (camp_before, store_before) = (units::camp(&w, red).unwrap().people, total(&w.civs[&red].store));
    w.act(Action::Raise { n: 2 }).unwrap();
    // The two biggest gave one each; the one under UNIT.min_clan and the third gave nothing.
    assert_eq!([people(&w, clans[0]), people(&w, b)], [before[0] - 1.0, before[1] - 1.0]);
    assert_eq!([people(&w, c), people(&w, d)], [before[2], before[3]]);
    let camp = units::camp(&w, red).unwrap();
    assert_eq!(camp.people, camp_before + 2.0);
    assert!(
        (total(&w.civs[&red].store) - (store_before - pack_total(2.0))).abs() < 1e-9,
        "their packs came from the store"
    );
    assert_eq!(kinds(&w, EventKind::WarriorsRaised), 1);
    assert_eq!(w.units.len(), 3, "still one camp a civ");
    // The store can't fill the packs.
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    assert!(w.act(Action::Raise { n: 1 }).is_err());
    assert!(w.act(Action::Raise { n: 0 }).is_err());
}

#[test]
fn raising_with_no_camp_makes_one_at_the_capital() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    w.act(Action::Disband { n: None }).unwrap();
    assert!(units::camp(&w, red).is_none());
    w.act(Action::Raise { n: 3 }).unwrap();
    let camp = units::camp(&w, red).unwrap();
    assert_eq!(camp.tile(), w.civs[&red].capital);
    assert_eq!(camp.people, 1.0, "only one clan to give");
    assert_eq!(kinds(&w, EventKind::CampRaised), 1);
    assert_eq!(w.civs[&red].army.raised, 2);
}

#[test]
fn disbanding_returns_the_warriors_to_the_civs_clan() {
    let (mut w, ids, clans) = led();
    let red = ids[0];
    // A bigger clan of the civ is nearer the camp, but they go to the civ's own.
    let near = put(&mut w, &[at_big(-2, 0)], 9.0);
    w.clans.get_mut(&near).unwrap().civ = Some(red);
    put_camp(&mut w, red, at_big(-2, 1));
    let own = w.clans[&clans[0]].people;
    let held = total(&units::camp(&w, red).unwrap().store);
    w.act(Action::Disband { n: Some(2) }).unwrap();
    assert_eq!(w.clans[&clans[0]].people, own + 2.0);
    assert_eq!(w.clans[&near].people, 9.0);
    let camp = units::camp(&w, red).unwrap();
    assert_eq!(camp.people, 1.0);
    assert!((total(&w.clans[&clans[0]].store) - held * 2.0 / 3.0).abs() < 1e-9, "their share of the pack went too");
    assert!((total(&camp.store) - held / 3.0).abs() < 1e-9);
    w.act(Action::Disband { n: None }).unwrap();
    assert!(units::camp(&w, red).is_none());
    assert_eq!(w.clans[&clans[0]].people, own + 3.0);
    assert_eq!(w.civs[&red].army.lost, 0, "disbanding is not starving");
    assert_eq!(kinds(&w, EventKind::WarriorsDisbanded), 2);
    assert!(w.act(Action::Disband { n: None }).is_err(), "no camp");
    w.act(Action::Raise { n: 1 }).unwrap();
}

#[test]
fn a_camp_at_the_capital_fills_with_no_labour() {
    let (mut w, ids, _) = three();
    let red = ids[0];
    let id = camp_id(&w, red);
    assert_eq!(units::line(&w, id), Some((Vec::new(), 0.0, f64::INFINITY)));
    assert!(!units::cut_off(&w, id));
    let store = total(&w.civs[&red].store);
    camp_mut(&mut w, id).store = [0.0; N];
    units::step(&mut w);
    // Filled to the pack, at nothing, then fed.
    let want = pack_total(3.0).min(store);
    let camp = units::camp(&w, red).unwrap();
    assert!((total(&camp.store) - (want - 3.0)).abs() < 1e-9, "{}", total(&camp.store));
    assert!((total(&w.civs[&red].store) - (store - want)).abs() < 1e-9);
    assert_eq!(camp.hungry, 0);
    assert!((w.civs[&red].army.resupplied - want).abs() < 1e-9);
}

/// A flat world with a Red civ (capital at (-5, 0)) and a camp of `people` at `tile`, dest `dest`.
fn marcher(people: f64, tile: usize, dest: usize) -> (World, u32) {
    let (mut w, ids, _) = three();
    let id = put_camp(&mut w, ids[0], tile);
    let c = camp_mut(&mut w, id);
    c.people = people;
    c.dest = Some(dest);
    (w, ids[0])
}

#[test]
fn a_camp_of_six_marches_as_far_in_a_turn_as_a_camp_of_two() {
    let (mut w2, red) = marcher(2.0, at_big(-4, 3), at_big(4, 3));
    let (mut w6, _) = marcher(6.0, at_big(-4, 3), at_big(4, 3));
    units::step(&mut w2);
    units::step(&mut w6);
    let (a, b) = (units::camp(&w2, red).unwrap().tile(), units::camp(&w6, red).unwrap().tile());
    assert_eq!(a, b);
    assert_eq!(steps(&w2, a, at_big(-4, 3)), 3, "three plains steps on 3.5 labour");
    // A march that isn't done leaves nothing: neither forages nor supplies.
    assert_eq!(w2.civs[&red].army.resupplied, 0.0);
    assert_eq!(units::camp(&w6, red).unwrap().banked, 0.5);
}

#[test]
fn a_far_camps_line_costs_more_and_carries_less() {
    let (mut w, ids, _) = three();
    let id = camp_id(&w, ids[0]);
    let cap = w.civs[&ids[0]].capital;
    let mut seen = Vec::new();
    for k in [1, 2, 5, 6] {
        let tile = at_big(-5 + k, 0);
        put_camp(&mut w, ids[0], tile);
        let (path, cost, carry) = units::line(&w, id).unwrap();
        assert_eq!(path.len() as i32, k);
        assert_eq!(*path.last().unwrap(), cap);
        assert_eq!(cost, 2.0 * f64::from(k));
        assert!((carry - 3.0 * UNIT.labour * UNIT.carry / cost).abs() < 1e-9);
        seen.push((cost, carry, units::cut_off(&w, id)));
    }
    assert!(seen.windows(2).all(|p| p[0].0 < p[1].0 && p[0].1 > p[1].1));
    // Cut off once the line costs more than a warrior's labour carries of their food, whatever the
    // camp's size: at the tuned numbers five plains steps out is fed and six is not.
    for s in &seen {
        assert_eq!(s.2, s.0 > UNIT.labour * UNIT.carry / UNIT.food, "{s:?}");
    }
    assert_eq!(seen.iter().map(|s| s.2).collect::<Vec<_>>(), [false, false, false, true]);
}

/// A corridor of land along r = 0 from Red's capital (-5, 0) to (9, 0), Blue's (5, 0) in the middle.
fn corridor() -> (World, u32, u32) {
    let mut w = big();
    for i in 0..w.tiles.len() {
        let t = &w.tiles[i];
        if !(t.r == 0 && (-5..=9).contains(&t.q)) {
            w.tiles[i].terrain = SEA;
        }
    }
    let ms = [open(&mut w, at_big(-5, 0)), open(&mut w, at_big(5, 0))];
    put(&mut w, &[at_big(-5, 0)], 8.0);
    put(&mut w, &[at_big(5, 0)], 8.0);
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    (w, ids[0], ids[1])
}

#[test]
fn a_line_through_rival_land_is_refused() {
    let (mut w, red, blue) = corridor();
    let id = camp_id(&w, red);
    assert_eq!(civs::tile_civ(&w, at_big(7, 0)), Some(blue));
    // Beyond Blue's land, the only way home is through it.
    put_camp(&mut w, red, at_big(9, 0));
    assert_eq!(units::line(&w, id), None);
    assert!(units::cut_off(&w, id));
    // The camp's own tile never blocks: just over the border, it still draws.
    put_camp(&mut w, red, at_big(3, 0));
    assert_eq!(civs::tile_civ(&w, at_big(3, 0)), Some(blue));
    let (path, _, _) = units::line(&w, id).expect("a line from a tile of rival land");
    assert_eq!(path.len(), 8);
    // And on the near side it's a line.
    put_camp(&mut w, red, at_big(0, 0));
    assert!(units::line(&w, id).is_some());
    // The unit's turn records it.
    put_camp(&mut w, red, at_big(9, 0));
    units::step(&mut w);
    assert_eq!(w.civs[&red].army.cut_off, 1);
    assert_eq!(kinds(&w, EventKind::CampCutOff), 1);
    units::step(&mut w);
    assert_eq!(w.civs[&red].army.cut_off, 2);
    assert_eq!(kinds(&w, EventKind::CampCutOff), 1, "announced when it starts");
}

#[test]
fn foraging_strips_a_tile_so_a_clan_harvests_nothing_there_for_fallow_turns() {
    let (mut w, ids, _) = three();
    let red = ids[0];
    let t = at_big(-1, 4);
    w.tiles[t].cap = [0.0, 10.0, 0.0, 0.0, 0.0];
    w.tiles[t].stock = [0.0, 10.0, 0.0, 0.0, 0.0];
    let id = put_camp(&mut w, red, t);
    {
        let c = camp_mut(&mut w, id);
        c.store = [0.0; N];
        c.auto_supply = false;
    }
    let clan = put(&mut w, &[t], 6.0);
    units::step(&mut w);
    // It took what eating needed this turn and the tile is stripped.
    assert!((w.civs[&red].army.foraged - 3.0).abs() < 1e-9, "{}", w.civs[&red].army.foraged);
    assert_eq!(w.civs[&red].army.foraged_tiles, 1);
    assert_eq!((w.tiles[t].stock, w.tiles[t].fallow), ([0.0; N], UNIT.fallow));
    assert_eq!(units::camp(&w, red).unwrap().hungry, 0);
    assert_eq!(kinds(&w, EventKind::CampForaged), 1);
    for turn in 1..=UNIT.fallow {
        regrow(&mut w);
        harvest(&mut w);
        assert_eq!(total(&w.clans[&clan].harvest), 0.0, "turn {turn}: nothing to harvest");
        units::step(&mut w);
    }
    assert_eq!(w.tiles[t].fallow, 0);
    regrow(&mut w);
    harvest(&mut w);
    assert!(total(&w.clans[&clan].harvest) > 0.0, "and then it yields again");
}

#[test]
fn pillage_yields_more_than_forage() {
    let take = |pillage: bool| {
        let (mut w, ids, _) = led();
        let red = ids[0];
        let t = if pillage { at_big(4, 0) } else { at_big(0, 3) };
        assert_eq!(civs::tile_civ(&w, t), if pillage { Some(ids[1]) } else { None });
        w.tiles[t].cap = [0.0, 100.0, 0.0, 0.0, 0.0];
        w.tiles[t].unworn = w.tiles[t].cap;
        w.tiles[t].stock = [0.0, 100.0, 0.0, 0.0, 0.0];
        let id = put_camp(&mut w, red, t);
        camp_mut(&mut w, id).store = [0.0; N];
        w.act(Action::SetSwitches { forage: false, supply: false }).unwrap();
        assert!(w.act(if pillage { Action::Forage } else { Action::Pillage }).is_err(), "the wrong one is refused");
        w.act(if pillage { Action::Pillage } else { Action::Forage }).unwrap();
        end_turn(&mut w);
        let a = &w.civs[&red].army;
        (a.foraged, a.pillaged, w.tiles[t].fallow)
    };
    let (forage, none, fallow) = take(false);
    assert!((forage - 3.5 * 3.0 * UNIT.forage).abs() < 1e-9 && none == 0.0 && fallow > 0);
    let (none, pillage, fallow) = take(true);
    assert!((pillage - 3.5 * 3.0 * UNIT.pillage).abs() < 1e-9 && none == 0.0 && fallow > 0);
    assert!(pillage > forage);
}

#[test]
fn with_the_switches_off_it_neither_forages_nor_supplies() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let t = at_big(-3, 1);
    w.tiles[t].stock = [0.0, 10.0, 0.0, 0.0, 0.0];
    let id = put_camp(&mut w, red, t);
    camp_mut(&mut w, id).store = [0.0; N];
    w.act(Action::SetSwitches { forage: false, supply: false }).unwrap();
    let store = w.civs[&red].store;
    units::step(&mut w);
    assert_eq!(w.tiles[t].stock, [0.0, 10.0, 0.0, 0.0, 0.0]);
    assert_eq!(w.civs[&red].store, store);
    let camp = units::camp(&w, red).unwrap();
    assert_eq!((total(&camp.store), camp.hungry), (0.0, 1));
    // The one-shot resupply works with the switch off.
    w.act(Action::Resupply).unwrap();
    units::step(&mut w);
    assert!(w.civs[&red].army.resupplied > 0.0, "it brought food along the line");
    assert!(total(&w.civs[&red].store) < total(&store));
    // With the switches on, the empty pack is covered by foraging.
    camp_mut(&mut w, id).store = [0.0; N];
    w.act(Action::SetSwitches { forage: true, supply: true }).unwrap();
    units::step(&mut w);
    assert!(w.civs[&red].army.foraged > 0.0);
}

#[test]
fn hunger_loses_warriors() {
    let (mut w, ids, clans) = led();
    let red = ids[0];
    let id = put_camp(&mut w, red, at_big(-5, 8));
    {
        let c = camp_mut(&mut w, id);
        c.store = [0.0; N];
        c.auto_forage = false;
        c.auto_supply = false;
    }
    let own = w.clans[&clans[0]].people;
    for _ in 1..UNIT.hungry_turns {
        units::step(&mut w);
    }
    assert_eq!(units::camp(&w, red).unwrap().people, 3.0, "hungry, not yet starving");
    assert_eq!(kinds(&w, EventKind::CampHungry), 1, "announced once");
    units::step(&mut w);
    assert_eq!(units::camp(&w, red).unwrap().people, 2.0, "one warrior a turn");
    assert_eq!(w.clans[&clans[0]].people, own + 1.0);
    units::step(&mut w);
    units::step(&mut w);
    assert!(units::camp(&w, red).is_none());
    let a = &w.civs[&red].army;
    assert_eq!((a.starved, a.lost, a.hungry), (3, 1, 5));
    assert_eq!(kinds(&w, EventKind::WarriorsStarved), 3);
    assert_eq!(w.clans[&clans[0]].people, own + 3.0, "they went home");
    // Fed again, the count starts over.
    w.act(Action::Raise { n: 1 }).unwrap();
}

#[test]
fn a_fed_camp_starts_its_hungry_count_again() {
    let (mut w, ids, _) = three();
    let id = camp_id(&w, ids[0]);
    {
        let c = camp_mut(&mut w, id);
        c.hungry = 2;
        c.store = [0.0; N];
    }
    units::step(&mut w);
    assert_eq!(camp_mut(&mut w, id).hungry, 0, "at the capital the store fills the pack");
}

#[test]
fn with_the_army_off_no_camp_is_founded_and_nothing_is_levied() {
    let mut w = big();
    w.config.army = false;
    let ms = markets3(&mut w);
    let a = put(&mut w, &[at_big(-5, 0)], 8.0);
    found(&mut w, &ms);
    let red = *w.civs.keys().next().unwrap();
    assert!(w.units.is_empty());
    assert_eq!(w.clans[&a].people, 8.0);
    assert_eq!(w.civs[&red].store, CIV.endowment);
    let mid = w.civs[&red].market;
    harvested(&mut w, a, Some(mid), Some(red));
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    levy(&mut w, mid);
    assert_eq!(w.civs[&red].store, [0.0; N]);
    assert_eq!(w.clans[&a].harvest, HARVEST);
    w.act(Action::ChooseCiv { civ: Some(red) }).unwrap();
    assert!(w.act(Action::Raise { n: 1 }).is_err(), "armies are off");
    w.act(Action::ChooseCiv { civ: None }).unwrap_err();
    // The AI raises none either.
    let mut v = big();
    v.config.army = false;
    let ms = markets3(&mut v);
    put(&mut v, &[at_big(-5, 0)], 8.0);
    found(&mut v, &ms);
    v.phase = Phase::Main;
    units::ai(&mut v);
    assert!(v.units.is_empty());
}

#[test]
fn the_ai_raises_a_lost_camp_and_posts_it_off_the_capital() {
    let (mut w, ids, _) = three();
    w.phase = Phase::Main;
    // The AI leads Blue and Gold; Red is the player's: its camp is left alone.
    w.player = Some(ids[0]);
    w.act(Action::Disband { n: None }).unwrap();
    w.units.remove(&camp_id(&w, ids[1]));
    units::ai(&mut w);
    assert!(units::camp(&w, ids[0]).is_none(), "not the player's civ");
    let blue = units::camp(&w, ids[1]).expect("the AI raised its camp");
    assert_eq!(w.civs[&ids[1]].army.raised, 2);
    let id = blue.id;
    let dest = units::camp(&w, ids[1]).unwrap().dest.expect("posted");
    assert_ne!(dest, w.civs[&ids[1]].capital);
    assert!(steps(&w, dest, w.civs[&ids[1]].capital) <= UNIT.post as i32);
    assert_eq!(civs::tile_civ(&w, dest), Some(ids[1]), "on its own land");
    for _ in 0..4 {
        units::step(&mut w);
    }
    assert_eq!(w.units[&id].tile(), dest);
}

#[test]
fn a_real_map_replays_with_the_camp_under_orders() {
    let mut w = World::new(Config { seed: 2, radius: 16, army: true, gifts: true, zone: true });
    while w.phase != Phase::Choose {
        w.act(Action::EndTurn).unwrap();
    }
    assert_eq!(w.units.len(), 3, "a camp each at the pick");
    for u in w.units.values() {
        assert_eq!(u.tile(), w.civs[&u.civ].capital);
        assert_eq!(u.people, f64::from(UNIT.start));
        assert_eq!(w.civs[&u.civ].army.raised, 1);
    }
    let me = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(me) }).unwrap();
    let pick = w.turn;
    for _ in 0..120 {
        let n = w.turn - pick;
        if n > 0 && n.is_multiple_of(20) {
            let _ = w.act(Action::Raise { n: 1 });
        }
        if n == 3 {
            let to = w.civs.values().find(|c| c.id != me).unwrap().capital;
            let _ = w.act(Action::MoveCamp { to });
        }
        if n == 1 {
            let _ = w.act(Action::SetSwitches { forage: false, supply: false });
        }
        if n == 2 {
            let _ = w.act(Action::SetSwitches { forage: true, supply: true });
        }
        if n == 4 {
            let _ = w.act(Action::Resupply);
        }
        if n == 5 {
            let _ = w.act(Action::Disband { n: Some(1) });
        }
        end_turn(&mut w);
        for u in w.units.values() {
            assert!(TERRAIN[w.tiles[u.tile()].terrain].land, "camp {} is off land", u.id);
        }
        for c in w.civs.values() {
            assert!(total(&c.store) <= CIV.store_cap + 1e-9);
        }
    }
    assert!(w.log.iter().any(|a| matches!(a, Action::MoveCamp { .. })));
    assert!(w.log.iter().any(|a| matches!(a, Action::Disband { .. })));
    assert!(w.events.iter().any(|e| e.kind == EventKind::CampRaised));
    let json = serde_json::to_string(&w.save()).unwrap();
    let save: engine::Save = serde_json::from_str(&json).unwrap();
    let r = World::replay(&save).unwrap();
    assert_eq!(r, w);
    assert_eq!(r.events, w.events);
    // The orders' JSON is the page's.
    assert_eq!(serde_json::to_string(&Action::Raise { n: 2 }).unwrap(), r#"{"type":"raise","n":2}"#);
    assert_eq!(serde_json::to_string(&Action::Disband { n: None }).unwrap(), r#"{"type":"disband","n":null}"#);
    assert_eq!(serde_json::to_string(&Action::MoveCamp { to: 812 }).unwrap(), r#"{"type":"moveCamp","to":812}"#);
    assert_eq!(
        serde_json::to_string(&Action::SetSwitches { forage: true, supply: false }).unwrap(),
        r#"{"type":"setSwitches","forage":true,"supply":false}"#
    );
    assert_eq!(serde_json::to_string(&Action::Pillage).unwrap(), r#"{"type":"pillage"}"#);
    // An old save, with no army field, loads with the army on.
    let old = r#"{"seed":1,"radius":16}"#;
    assert!(serde_json::from_str::<Config>(old).unwrap().army);
}

#[test]
fn the_control_run_has_no_army_and_never_levies() {
    let mut w = World::new(Config { seed: 2, radius: 16, army: false, gifts: true, zone: true });
    w.run(400);
    assert!(w.phase == Phase::Main && w.turn > w.end.as_ref().unwrap().turn + 50);
    assert!(w.units.is_empty());
    for c in w.civs.values() {
        assert_eq!(c.army.raised, 0);
        assert!(total(&c.store) <= total(&CIV.endowment), "only spoiling: nothing came in");
    }
}

#[test]
fn on_a_real_map_every_ai_civ_keeps_the_camp_it_raised_at_the_pick_on_a_line_from_its_capital() {
    let mut w =
        World::new(Config { seed: 2, radius: size_radius("small").unwrap(), army: true, gifts: true, zone: true });
    while w.phase == Phase::Pregame {
        w.run(1);
    }
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    w.run(100);
    for c in w.civs.values() {
        let camp = units::camp(&w, c.id).unwrap_or_else(|| panic!("civ {} has its camp", c.id));
        assert_eq!(c.army.lost, 0, "civ {} lost no camp", c.id);
        assert!(camp.people >= f64::from(UNIT.start), "civ {} has {} warriors", c.id, camp.people);
        let (path, cost, _) = units::line(&w, camp.id).unwrap();
        assert!(!path.is_empty() && cost > 0.0, "civ {}'s camp is posted off its capital", c.id);
        assert!(!units::cut_off(&w, camp.id), "civ {}'s camp is fed by its line", c.id);
    }
}

fn held(w: &World, civ: u32) -> Vec<usize> {
    units::camp(w, civ).unwrap().held.clone()
}

#[test]
fn a_zone_must_be_connected_within_its_cap_and_clear_of_capitals() {
    let (mut w, ids, _) = led();
    put_camp(&mut w, ids[0], at_big(-4, 0));
    let zone = |tiles: &[(i32, i32)]| Action::Zone { tiles: tiles.iter().map(|&(q, r)| at_big(q, r)).collect() };
    let err = |w: &World, a: Action| w.validate(&a).unwrap_err();
    assert!(err(&w, zone(&[(-5, 0)])).contains("capital"));
    assert!(err(&w, zone(&[(-2, 0)])).contains("connected"));
    assert!(err(&w, zone(&[(-3, 0), (-2, 0), (-1, 0)])).contains("at most 3"), "three warriors, three tiles");
    w.tiles[at_big(-3, 1)].terrain = SEA;
    assert!(err(&w, zone(&[(-3, 1)])).contains("land"));
    // The camp's own tile may be listed or left out.
    w.act(zone(&[(-3, 0), (-2, 0)])).unwrap();
    w.act(zone(&[(-4, 0), (-3, 0), (-2, 0)])).unwrap();
    assert_eq!(units::camp(&w, ids[0]).unwrap().territory, vec![at_big(-4, 0), at_big(-3, 0), at_big(-2, 0)]);
    assert_eq!(kinds(&w, EventKind::Deployed), 2);
    w.act(Action::Zone { tiles: Vec::new() }).unwrap();
    assert_eq!(units::camp(&w, ids[0]).unwrap().territory, vec![at_big(-4, 0)], "none undeploys");
}

#[test]
fn the_camps_tile_is_held_and_the_spread_fills_one_ring_a_turn_until_a_move_clears_it() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let home = at_big(-3, 0);
    put_camp(&mut w, red, home);
    w.act(Action::Zone { tiles: vec![at_big(-2, 0), at_big(-1, 0)] }).unwrap();
    assert_eq!(held(&w, red), vec![home], "only its own tile at first");
    units::step(&mut w);
    assert_eq!(held(&w, red), vec![home, at_big(-2, 0)], "one ring");
    assert_eq!(kinds(&w, EventKind::Held), 0);
    units::step(&mut w);
    assert_eq!(held(&w, red), vec![home, at_big(-2, 0), at_big(-1, 0)], "two rings");
    assert_eq!(kinds(&w, EventKind::Held), 1);
    let u = units::camp(&w, red).unwrap();
    assert_eq!(u.guard.len(), 3);
    assert!(u.guard.iter().all(|&g| g >= UNIT.guard), "{:?}", u.guard);
    assert_eq!(w.civs[&red].army.held, 1 + 2);
    units::step(&mut w);
    assert_eq!(kinds(&w, EventKind::Held), 1, "announced once");
    // Unpainting drops a tile at once.
    w.act(Action::Zone { tiles: vec![at_big(-2, 0)] }).unwrap();
    assert_eq!(held(&w, red), vec![home, at_big(-2, 0)]);
    // Moving undeploys.
    w.act(Action::MoveCamp { to: at_big(-3, 3) }).unwrap();
    let u = units::camp(&w, red).unwrap();
    assert_eq!((u.territory.clone(), u.held.clone()), (vec![home], vec![home]));
}

#[test]
fn guarding_costs_labour_only_when_deployed() {
    let supplied = |deploy: bool| {
        let (mut w, ids, _) = led();
        put_camp(&mut w, ids[0], at_big(-2, 0));
        let id = camp_id(&w, ids[0]);
        camp_mut(&mut w, id).store = [0.0; N];
        w.civs.get_mut(&ids[0]).unwrap().store = [0.0, 50.0, 0.0, 0.0, 0.0];
        if deploy {
            w.act(Action::Zone { tiles: vec![at_big(-1, 0)] }).unwrap();
        }
        units::step(&mut w);
        w.civs[&ids[0]].army.resupplied
    };
    let (off, on) = (supplied(false), supplied(true));
    let (cost, need) = (2.0 * 3.0, 3.0 * UNIT.food);
    assert!((off - (3.0 * UNIT.labour * UNIT.carry / cost).min(pack_total(3.0))).abs() < 1e-9, "{off}");
    assert!((on - (3.0 * (UNIT.labour - UNIT.guarding) * UNIT.carry / cost)).abs() < 1e-9, "{on}");
    assert!(on < off && on + 1e-9 >= need, "guarding leaves less, but enough to eat: {on} {off}");
}

/// A corridor of land along r = 0 from Red's capital (-5, 0) to (9, 0), and Blue's capital just off
/// it at (5, -1), so Red's camp beyond Blue's land can only come home through it.
fn corridor_off() -> (World, u32, u32) {
    let mut w = big();
    for i in 0..w.tiles.len() {
        let t = &w.tiles[i];
        if !((t.r == 0 && (-5..=9).contains(&t.q)) || (t.q, t.r) == (5, -1)) {
            w.tiles[i].terrain = SEA;
        }
    }
    let ms = [open(&mut w, at_big(-5, 0)), open(&mut w, at_big(5, -1))];
    put(&mut w, &[at_big(-5, 0)], 8.0);
    put(&mut w, &[at_big(5, -1)], 8.0);
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    (w, ids[0], ids[1])
}

#[test]
fn a_rivals_held_tile_blocks_the_line_and_ones_own_on_rival_land_does_not() {
    let (mut w, red, blue) = corridor();
    let id = put_camp(&mut w, red, at_big(-1, 0));
    assert!(units::line(&w, id).is_some());
    put_camp(&mut w, blue, at_big(-3, 0));
    assert_eq!(civs::holder(&w, at_big(-3, 0)), Some(blue), "held land is the holder's");
    assert_eq!(civs::tile_civ(&w, at_big(-3, 0)), Some(red), "but still Red's land");
    assert_eq!(units::line(&w, id), None, "Blue's held tile on Red's own land cuts it");
    units::step(&mut w);
    assert_eq!(kinds(&w, EventKind::LineCut), 1);
    let red_cut = w.events.iter().filter(|e| e.kind == EventKind::CampCutOff && e.civ == Some(red)).count();
    assert_eq!(red_cut, 0, "cut by an army, not by land");
    let e = w.events.iter().find(|e| e.kind == EventKind::LineCut).unwrap();
    assert_eq!(e.other, Some(blue));
    assert!(e.text(&w).contains("is cut by"), "{}", e.text(&w));
    assert_eq!(w.civs[&red].army.cut_by_army, 1);
    // The line's ends never block it: Blue holding Red's capital or Red's camp's own tile.
    put_camp(&mut w, blue, at_big(-5, 0));
    assert!(units::line(&w, id).is_some(), "the capital end");
    put_camp(&mut w, blue, at_big(-1, 0));
    assert!(units::line(&w, id).is_some(), "the camp's end");

    let (mut w, red, blue) = corridor_off();
    let id = put_camp(&mut w, red, at_big(9, 0));
    let blues: Vec<usize> = (0..=9).map(|q| at_big(q, 0)).filter(|&t| civs::tile_civ(&w, t) == Some(blue)).collect();
    assert!(!blues.is_empty());
    assert_eq!(units::line(&w, id), None, "Blue's land blocks it");
    let c = camp_mut(&mut w, id);
    c.held = blues.iter().copied().chain(std::iter::once(at_big(9, 0))).collect();
    c.held.sort_unstable();
    assert!(units::line(&w, id).is_some(), "Red's own held tiles on Blue's land let it through");
}

#[test]
fn the_larger_guard_share_takes_a_contested_tile_and_a_tie_stays_with_the_holder() {
    let contest = |red_people: f64, blue_holds: bool| {
        let (mut w, ids, _) = three();
        let (red, blue) = (ids[0], ids[1]);
        let mid = at_big(1, 3);
        let r = put_camp(&mut w, red, at_big(0, 3));
        let b = put_camp(&mut w, blue, at_big(2, 3));
        camp_mut(&mut w, r).people = red_people;
        camp_mut(&mut w, r).actor.territory = vec![at_big(0, 3), mid];
        camp_mut(&mut w, b).actor.territory = vec![mid, at_big(2, 3)];
        if blue_holds {
            camp_mut(&mut w, b).held = vec![mid, at_big(2, 3)];
        }
        units::step(&mut w);
        (held(&w, red).contains(&mid), held(&w, blue).contains(&mid), units::camp(&w, red).unwrap().territory.clone())
    };
    let (r, b, zone) = contest(6.0, true);
    assert!(r && !b, "the thicker guard takes it from the holder");
    assert!(zone.contains(&at_big(1, 3)));
    let (r, b, _) = contest(3.0, true);
    assert!(!r && b, "a tie stays with the holder");
    let (r, b, _) = contest(3.0, false);
    assert!(r && !b, "a fresh tie to the lower civ id");
    let (r, b, zone) = contest(2.0, false);
    assert!(!r && b, "the thinner guard loses it");
    assert!(zone.contains(&at_big(1, 3)), "but keeps it in its zone");
}

#[test]
fn a_deployed_camp_on_rival_land_still_pillages() {
    let (mut w, ids, _) = led();
    let red = ids[0];
    let t = at_big(4, 1);
    assert_eq!(civs::tile_civ(&w, t), Some(ids[1]));
    w.tiles[t].stock = [0.0, 50.0, 0.0, 0.0, 0.0];
    let id = put_camp(&mut w, red, t);
    camp_mut(&mut w, id).store = [0.0; N];
    w.act(Action::Zone { tiles: vec![at_big(3, 1)] }).unwrap();
    units::step(&mut w);
    assert!(held(&w, red).contains(&t));
    assert!(w.civs[&red].army.pillaged > 0.0 && w.civs[&red].army.foraged == 0.0);
}

#[test]
fn a_camp_cut_by_an_army_eats_its_pack_then_forages() {
    let (mut w, red, blue) = corridor();
    let t = at_big(-1, 0);
    let id = put_camp(&mut w, red, t);
    w.tiles[t].stock = [0.0, 50.0, 0.0, 0.0, 0.0];
    put_camp(&mut w, blue, at_big(-3, 0));
    let full = total(&units::camp(&w, red).unwrap().store);
    assert!(full >= 3.0 * UNIT.food * 2.0);
    units::step(&mut w);
    let a = &w.civs[&red].army;
    assert_eq!((a.foraged, a.resupplied), (0.0, 0.0), "it eats from its pack");
    assert!(total(&units::camp(&w, red).unwrap().store) < full);
    let mut turns = 1;
    while w.civs[&red].army.foraged == 0.0 {
        units::step(&mut w);
        turns += 1;
        assert!(turns < 20);
    }
    assert!(turns >= 3, "the pack lasted {turns} turns first");
    assert_eq!(w.civs[&red].army.resupplied, 0.0);
    assert!(w.units.contains_key(&id));
}
