mod common;

use common::*;
use engine::civs::{self, upkeep};
use engine::data::*;
use engine::land::{cost_map, neighbours, passable, path, step_cost};
use engine::units::{self, Unit, UnitKind};
use engine::{Action, Config, End, EventKind, Phase, World};

/// Three markets on the big world, 10 or more steps apart.
fn markets3(w: &mut World) -> [u32; 3] {
    [open(w, at_big(-5, 0)), open(w, at_big(5, 0)), open(w, at_big(0, -10))]
}

/// Ends the pre-game on `centres` by hand and founds the civs.
fn found(w: &mut World, centres: &[u32]) {
    w.end = Some(End { turn: w.turn, centres: centres.to_vec(), cover: 1.0, by_limit: false, topped_up: Vec::new() });
    civs::found(w);
}

/// A big world founded with no clans and led by the first civ (Red), its starting warriors
/// cleared away; returns the world and the civ ids.
fn led() -> (World, [u32; 3]) {
    let mut w = big();
    let ms = markets3(&mut w);
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    w.act(Action::ChooseCiv { civ: Some(ids[0]) }).unwrap();
    w.units.clear();
    w.events.clear();
    (w, [ids[0], ids[1], ids[2]])
}

/// Adds a clan of `pop` on `round` to `civ`.
fn civ_clan(w: &mut World, civ: u32, round: &[usize], pop: f64) -> u32 {
    let id = put(w, round, pop);
    w.clans.get_mut(&id).unwrap().civ = Some(civ);
    id
}

/// Puts a warrior of `civ` on `tile`.
fn unit(w: &mut World, civ: u32, tile: usize) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    w.units.insert(id, Unit { id, civ, kind: UnitKind::Warrior, tile, dest: None, banked: 0.0, hungry: 0 });
    id
}

/// Puts a warrior of `civ` on its capital.
fn capital_unit(w: &mut World, civ: u32) -> u32 {
    let tile = w.civs[&civ].capital;
    unit(w, civ, tile)
}

/// Makes every tile not in `land` sea.
fn isolate(w: &mut World, land: &[usize]) {
    for i in 0..w.tiles.len() {
        if !land.contains(&i) {
            w.tiles[i].terrain = SEA;
        }
    }
}

fn kinds(w: &World, kind: EventKind) -> Vec<&engine::Event> {
    w.events.iter().filter(|e| e.kind == kind).collect()
}

fn end_turn(w: &mut World) {
    w.act(Action::EndTurn).unwrap();
}

#[test]
fn founding_gives_each_civ_a_warrior() {
    let mut w = big();
    let ms = markets3(&mut w);
    // Red: two near clans (the bigger gives), a third that can't spare one.
    let a = put(&mut w, &[at_big(-5, 1)], 4.0);
    let b = put(&mut w, &[at_big(-5, -1)], 6.0);
    let c = put(&mut w, &[at_big(-4, 0)], 1.5);
    // Blue: a near clan of 1.5 can't spare one; the clan 3 steps away, in reach, gives.
    let d = put(&mut w, &[at_big(5, 1)], 1.5);
    let e = put(&mut w, &[at_big(8, 0)], 8.0);
    // Gold: no clan at all.
    found(&mut w, &ms);
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    assert_eq!(w.units.len(), 3);
    let by_civ = |civ: u32| w.units.values().find(|u| u.civ == civ).unwrap();
    for id in &ids {
        let u = by_civ(*id);
        assert_eq!((u.kind, u.tile, u.dest, u.banked, u.hungry), (UnitKind::Warrior, w.civs[id].capital, None, 0.0, 0));
    }
    assert_eq!([w.clans[&a].pop, w.clans[&b].pop, w.clans[&c].pop], [4.0, 5.0, 1.5]);
    assert_eq!([w.clans[&d].pop, w.clans[&e].pop], [1.5, 7.0]);
    let made = kinds(&w, EventKind::UnitMade);
    assert_eq!(made.len(), 3);
    for (ev, (civ, clan)) in made.iter().zip([(ids[0], Some(b)), (ids[1], Some(e)), (ids[2], None)]) {
        let u = by_civ(civ);
        assert_eq!((ev.civ, ev.unit, ev.clan, ev.tile), (Some(civ), Some(u.id), clan, u.tile));
    }
    assert_eq!(made[0].text(&w), format!("Red makes warrior {} from clan {b}", made[0].unit.unwrap()));
    assert_eq!(made[2].text(&w), format!("Gold makes warrior {}", made[2].unit.unwrap()));
}

#[test]
fn make_unit_takes_a_person_from_the_biggest_near_clan() {
    let (mut w, [red, ..]) = led();
    let cap = w.civs[&red].capital;
    let a = civ_clan(&mut w, red, &[at_big(-5, 1)], 5.0);
    let b = civ_clan(&mut w, red, &[at_big(-5, -1)], 5.0);
    let small = civ_clan(&mut w, red, &[at_big(-4, 0)], 1.5);
    // Three steps from the capital: not near.
    let far = civ_clan(&mut w, red, &[at_big(-2, 0)], 20.0);
    w.act(Action::MakeUnit).unwrap();
    assert_eq!(
        [w.clans[&a].pop, w.clans[&b].pop, w.clans[&small].pop, w.clans[&far].pop],
        [4.0, 5.0, 1.5, 20.0],
        "a and b tie on 5: the lower id gives"
    );
    let u = w.units.values().next().unwrap();
    assert_eq!((u.civ, u.tile, u.dest, u.banked, u.hungry), (red, cap, None, 0.0, 0));
    assert_eq!(w.civs[&red].store, [0.0, 30.0, 0.0, 0.0, 0.0], "making costs no food");
    let made = kinds(&w, EventKind::UnitMade);
    assert_eq!((made.len(), made[0].clan, made[0].tile, made[0].civ), (1, Some(a), cap, Some(red)));
    // b has 5 against a's 4: it gives next (5 -> 4); then 4 and 4 tie to a.
    w.act(Action::MakeUnit).unwrap();
    assert_eq!([w.clans[&a].pop, w.clans[&b].pop], [4.0, 4.0]);
    w.act(Action::MakeUnit).unwrap();
    assert_eq!([w.clans[&a].pop, w.clans[&b].pop], [3.0, 4.0]);
    assert_eq!(w.units.len(), 3);
}

#[test]
fn make_unit_is_refused_without_changing_the_world() {
    let (mut w, [red, blue, _]) = led();
    civ_clan(&mut w, red, &[at_big(-5, 1)], 1.5);
    civ_clan(&mut w, red, &[at_big(-5, -1)], 1.9);
    // Another civ's clan near Red's capital doesn't count; nor does a clan with no civ.
    civ_clan(&mut w, blue, &[at_big(-4, 0)], 9.0);
    put(&mut w, &[at_big(-6, 0)], 9.0);
    let before = w.clone();
    let err = w.act(Action::MakeUnit).unwrap_err();
    assert_eq!(err, "no clan near the capital has a person to spare");
    assert_eq!(w, before);

    // Watching.
    let mut v = big();
    let ms = markets3(&mut v);
    found(&mut v, &ms);
    let before = v.clone();
    assert!(v.act(Action::MakeUnit).is_err(), "before the pick");
    assert_eq!(v, before);
    v.act(Action::ChooseCiv { civ: None }).unwrap();
    let before = v.clone();
    assert_eq!(v.act(Action::MakeUnit).unwrap_err(), "you're watching: you lead no civ");
    assert_eq!(v, before);
    // Not the main game at all.
    let mut p = big();
    assert!(p.act(Action::MakeUnit).is_err());
}

#[test]
fn units_eat_in_proportion_from_the_store() {
    let (mut w, [red, ..]) = led();
    let a = capital_unit(&mut w, red);
    let b = capital_unit(&mut w, red);
    w.civs.get_mut(&red).unwrap().store = [0.0, 3.0, 0.0, 1.0, 0.0];
    upkeep(&mut w);
    // Spoiled by a tenth: 2.7 and 0.9; two units eat 1 each of the 3.6, in proportion.
    // The first eats 1 of 3.6, the second 1 of 2.6.
    let k = (1.0 - 1.0 / 3.6) * (1.0 - 1.0 / 2.6);
    let want = [0.0, 2.7 * k, 0.0, 0.9 * k, 0.0];
    for (g, v) in w.civs[&red].store.iter().zip(want) {
        assert!((g - v).abs() < 1e-9, "{g} vs {v}");
    }
    assert_eq!([w.units[&a].hungry, w.units[&b].hungry], [0, 0]);
    assert!(kinds(&w, EventKind::UnitHungry).is_empty());
}

#[test]
fn a_hungry_unit_is_announced_once_and_disbands_on_the_third_turn() {
    let (mut w, [red, ..]) = led();
    let cap = w.civs[&red].capital;
    // Clans 1 and 2 steps from the capital's tile; the nearer one takes the person back.
    let near = civ_clan(&mut w, red, &[at_big(-5, 1)], 5.0);
    let other = civ_clan(&mut w, red, &[at_big(-5, 2)], 5.0);
    let u = unit(&mut w, red, cap);
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    for want in [1, 2] {
        upkeep(&mut w);
        assert_eq!(w.units[&u].hungry, want);
    }
    assert_eq!(kinds(&w, EventKind::UnitHungry).len(), 1, "once, not every hungry turn");
    upkeep(&mut w);
    assert!(w.units.is_empty());
    assert_eq!([w.clans[&near].pop, w.clans[&other].pop], [6.0, 5.0]);
    let starved = kinds(&w, EventKind::UnitStarved);
    assert_eq!(starved.len(), 1);
    assert_eq!(
        (starved[0].civ, starved[0].unit, starved[0].clan, starved[0].tile),
        (Some(red), Some(u), Some(near), cap)
    );
    assert_eq!(starved[0].text(&w), format!("Red's warrior {u} disbands from hunger; its person rejoins clan {near}"));
    assert_eq!(kinds(&w, EventKind::UnitHungry)[0].text(&w), format!("Red's warrior {u} goes hungry"));
    assert!(kinds(&w, EventKind::UnitDisbanded).is_empty());
}

#[test]
fn a_fed_unit_starts_its_count_again() {
    let (mut w, [red, ..]) = led();
    let u = capital_unit(&mut w, red);
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    upkeep(&mut w);
    upkeep(&mut w);
    assert_eq!(w.units[&u].hungry, 2);
    w.civs.get_mut(&red).unwrap().store = [0.0, 30.0, 0.0, 0.0, 0.0];
    upkeep(&mut w);
    assert_eq!(w.units[&u].hungry, 0);
    w.civs.get_mut(&red).unwrap().store = [0.0; N];
    upkeep(&mut w);
    upkeep(&mut w);
    assert_eq!(w.units[&u].hungry, 2, "survived: the count began again");
    assert_eq!(kinds(&w, EventKind::UnitHungry).len(), 2, "hungry again is announced again");
    // Less than a unit's food is hungry too.
    let (mut w, [red, ..]) = led();
    let u = capital_unit(&mut w, red);
    w.civs.get_mut(&red).unwrap().store = [0.0, 0.5, 0.0, 0.0, 0.0];
    upkeep(&mut w);
    assert_eq!(w.units[&u].hungry, 1);
    assert_eq!(w.civs[&red].store, [0.0; N]);
}

#[test]
fn disbanding_returns_the_person_to_the_nearest_civ_clan() {
    let (mut w, [red, blue, _]) = led();
    let tile = at_big(-5, 3);
    // Homes 2 steps from the unit: ids tie, the lower wins. A closer clan of no civ doesn't count.
    let high = civ_clan(&mut w, red, &[at_big(-3, 3)], 5.0);
    let low = civ_clan(&mut w, red, &[at_big(-5, 5)], 5.0);
    let (low, high) = (low.min(high), low.max(high));
    put(&mut w, &[at_big(-5, 4)], 5.0);
    // Blue's clan is nearer than either of Red's, and still isn't the unit's own civ.
    let rival = civ_clan(&mut w, blue, &[at_big(-4, 3)], 5.0);
    let u = unit(&mut w, red, tile);
    w.act(Action::DisbandUnit { unit: u }).unwrap();
    assert!(w.units.is_empty());
    assert_eq!([w.clans[&low].pop, w.clans[&high].pop, w.clans[&rival].pop], [6.0, 5.0, 5.0]);
    let ev = kinds(&w, EventKind::UnitDisbanded);
    assert_eq!((ev.len(), ev[0].clan, ev[0].civ, ev[0].unit, ev[0].tile), (1, Some(low), Some(red), Some(u), tile));
    assert_eq!(ev[0].text(&w), format!("Red's warrior {u} disbands; its person rejoins clan {low}"));
    assert_eq!(w.log.last(), Some(&Action::DisbandUnit { unit: u }));
}

#[test]
fn a_person_with_no_civ_clan_is_lost() {
    let (mut w, [red, ..]) = led();
    let u = capital_unit(&mut w, red);
    let free = put(&mut w, &[at_big(-5, 1)], 5.0);
    w.act(Action::DisbandUnit { unit: u }).unwrap();
    assert_eq!(w.clans[&free].pop, 5.0);
    let ev = kinds(&w, EventKind::UnitDisbanded);
    assert_eq!(ev[0].clan, None);
    assert_eq!(ev[0].text(&w), format!("Red's warrior {u} disbands; its person is lost"));
}

#[test]
fn disbanding_is_refused_for_what_is_not_yours() {
    let (mut w, [red, blue, _]) = led();
    let mine = capital_unit(&mut w, red);
    let theirs = capital_unit(&mut w, blue);
    let before = w.clone();
    assert_eq!(w.act(Action::DisbandUnit { unit: theirs }).unwrap_err(), "that unit isn't yours");
    assert!(w.act(Action::DisbandUnit { unit: 9999 }).is_err());
    assert_eq!(w, before);

    let mut v = big();
    let ms = markets3(&mut v);
    found(&mut v, &ms);
    let any = *v.units.keys().next().unwrap();
    assert!(v.act(Action::DisbandUnit { unit: any }).is_err(), "before the pick");
    v.act(Action::ChooseCiv { civ: None }).unwrap();
    let before = v.clone();
    assert_eq!(v.act(Action::DisbandUnit { unit: any }).unwrap_err(), "you're watching: you lead no civ");
    assert_eq!(v, before);
    w.act(Action::DisbandUnit { unit: mine }).unwrap();
}

#[test]
fn a_unit_takes_the_cheaper_detour() {
    let (mut w, [red, ..]) = led();
    // A wall of forest three tiles thick, q -3..=-1, with a gap on the row r = 1.
    for t in &mut w.tiles {
        let h = t.hex();
        if (-3..=-1).contains(&h.q) && h.r != 1 && h.distance(engine::hex::Hex::ORIGIN) < BIG {
            t.terrain = FOREST;
        }
    }
    let from = at_big(-5, 0);
    let to = at_big(1, 0);
    let u = unit(&mut w, red, from);
    w.act(Action::MoveUnit { unit: u, to }).unwrap();
    assert_eq!(w.units[&u].tile, from, "an order doesn't move it");
    let (tiles, cost) = units::route(&w, u).unwrap();
    assert!(tiles.iter().all(|&t| w.tiles[t].terrain == PLAINS), "round the wall, not through it");
    assert_eq!((tiles.len(), cost, *tiles.last().unwrap()), (7, 7.0, to));
    // Straight through the forest would cost 9.
    let straight: f64 = (-4..=1).map(|q| if (-3..=-1).contains(&q) { 2.0 } else { 1.0 }).sum();
    assert_eq!(straight, 9.0);
    for _ in 0..2 {
        end_turn(&mut w);
    }
    assert_eq!((w.units[&u].tile, w.units[&u].dest), (to, None));
}

#[test]
fn a_unit_spends_a_turns_labour_a_turn() {
    let (mut w, [red, ..]) = led();
    let from = at_big(-5, 0);
    let to = at_big(2, 0);
    let u = unit(&mut w, red, from);
    w.act(Action::MoveUnit { unit: u, to }).unwrap();
    let (tiles, cost) = units::route(&w, u).unwrap();
    assert_eq!((tiles.len(), cost), (7, 7.0));
    assert_eq!(units::turns(&w, &tiles, from, 0.0), 2);
    assert_eq!(units::turns(&w, &tiles, from, 0.5), 2);
    assert_eq!(units::turns(&w, &tiles[..3], from, 0.0), 1);
    assert_eq!(units::turns(&w, &tiles[..4], from, 0.0), 2);
    assert_eq!(units::turns(&w, &[], from, 0.0), 0);
    end_turn(&mut w);
    // Three plains steps with 3.5, and the half left over.
    let x = &w.units[&u];
    assert_eq!((x.tile, x.dest, x.banked), (tiles[2], Some(to), 0.5));
    assert_eq!(units::turns(&w, &tiles[3..], x.tile, x.banked), 1);
    end_turn(&mut w);
    let x = &w.units[&u];
    assert_eq!((x.tile, x.dest, x.banked), (to, None, 0.0), "4.0 pays the last four steps");
    end_turn(&mut w);
    assert_eq!(w.units[&u].tile, to);
}

#[test]
fn turns_is_what_march_does() {
    let (mut w, [red, ..]) = led();
    // Mixed ground: forest, hills, a marsh over a river, a trail.
    for (k, t) in [FOREST, PLAINS, HILLS, MARSH, PLAINS, FOREST, PLAINS, PLAINS, MARSH, PLAINS].iter().enumerate() {
        w.tiles[at_big(-4 + k as i32, 0)].terrain = *t;
    }
    w.tiles[at_big(-1, 0)].river = 1;
    w.tiles[at_big(4, 0)].river = 1;
    w.tiles[at_big(2, 0)].trail = true;
    let from = at_big(-5, 0);
    for end in 1..=10 {
        for banked in [0.0, 0.75] {
            let to = at_big(-5 + end, 0);
            // Keep the unit on the row: only the row's tiles can be walked.
            let mut v = w.clone();
            for i in 0..v.tiles.len() {
                let h = v.tiles[i].hex();
                if h.r != 0 && v.tiles[i].terrain != SEA {
                    v.tiles[i].terrain = SEA;
                }
            }
            let u = unit(&mut v, red, from);
            let (tiles, _) = units::path_between(&v, from, to).unwrap();
            let want = units::turns(&v, &tiles, from, banked);
            v.act(Action::MoveUnit { unit: u, to }).unwrap();
            v.units.get_mut(&u).unwrap().banked = banked;
            let mut n = 0;
            while v.units[&u].dest.is_some() {
                units::march(&mut v);
                n += 1;
                assert!(n < 50);
            }
            assert_eq!((v.units[&u].tile, n), (to, want), "to {end} with {banked} banked");
        }
    }
}

#[test]
fn a_step_dearer_than_a_turn_is_taken_the_turn_after() {
    let (mut w, [red, ..]) = led();
    let from = at_big(-5, 0);
    let to = at_big(-4, 0);
    isolate(&mut w, &[from, to]);
    w.tiles[to].terrain = MARSH;
    w.tiles[to].river = 1;
    assert_eq!(step_cost(&w, from, to), Some(3.0 + RIVER.cross));
    let u = unit(&mut w, red, from);
    w.act(Action::MoveUnit { unit: u, to }).unwrap();
    let (tiles, _) = units::route(&w, u).unwrap();
    assert_eq!(units::turns(&w, &tiles, from, 0.0), 2);
    end_turn(&mut w);
    let x = &w.units[&u];
    assert_eq!((x.tile, x.dest, x.banked), (from, Some(to), UNIT.labour));
    end_turn(&mut w);
    let x = &w.units[&u];
    assert_eq!((x.tile, x.dest, x.banked), (to, None, 0.0));
}

#[test]
fn rivers_cost_to_cross_and_a_trail_bridges_them() {
    let (mut w, [red, ..]) = led();
    let row: Vec<usize> = (0..3).map(|q| at_big(q, 0)).collect();
    isolate(&mut w, &row);
    let u = unit(&mut w, red, row[0]);
    let cost = |w: &World| units::path_between(w, row[0], row[2]).unwrap().1;
    assert_eq!(cost(&w), 2.0);
    w.tiles[row[1]].river = 1;
    assert_eq!(cost(&w), 2.0 + RIVER.cross, "a river off its course");
    w.tiles[row[0]].flow = Some(row[1]);
    assert_eq!(cost(&w), 2.0, "along its course");
    w.tiles[row[0]].flow = None;
    w.tiles[row[1]].trail = true;
    assert_eq!(cost(&w), TRAIL.cost.min(1.0) + 1.0, "a trail bridges it");
    // The unit's own route agrees.
    w.act(Action::MoveUnit { unit: u, to: row[2] }).unwrap();
    assert_eq!(units::route(&w, u).unwrap().1, cost(&w));
}

#[test]
fn hidden_trail_costs_are_never_read() {
    let (mut w, [red, ..]) = led();
    for (k, t) in w.tiles.iter_mut().enumerate() {
        if t.hex().q == -2 && t.terrain == PLAINS && t.hex().r != 0 {
            t.terrain = FOREST;
        }
        if t.terrain == FOREST && k % 2 == 0 {
            t.terrain = HILLS;
        }
    }
    let from = at_big(-5, 0);
    let to = at_big(4, 3);
    let u = unit(&mut w, red, from);
    w.act(Action::MoveUnit { unit: u, to }).unwrap();
    let before = units::route(&w, u).unwrap();
    for t in &mut w.tiles {
        t.trail_cost = Some(0.1);
        t.span = Some(1);
    }
    assert_eq!(units::route(&w, u).unwrap(), before);
    assert_eq!(path(&w, from, to, |i| passable(&w, i)).unwrap(), before);
    w.act(Action::MoveUnit { unit: u, to }).unwrap();
    for _ in 0..4 {
        end_turn(&mut w);
    }
    assert_eq!(w.units[&u].tile, to);
}

#[test]
fn a_new_order_resets_the_bank_and_its_own_tile_clears_the_order() {
    let (mut w, [red, ..]) = led();
    let from = at_big(-5, 0);
    let u = unit(&mut w, red, from);
    w.act(Action::MoveUnit { unit: u, to: at_big(3, 0) }).unwrap();
    end_turn(&mut w);
    assert_eq!(w.units[&u].banked, 0.5);
    let here = w.units[&u].tile;
    w.act(Action::MoveUnit { unit: u, to: at_big(3, 1) }).unwrap();
    assert_eq!((w.units[&u].banked, w.units[&u].dest), (0.0, Some(at_big(3, 1))));
    end_turn(&mut w);
    assert_ne!(w.units[&u].tile, here);
    let here = w.units[&u].tile;
    w.act(Action::MoveUnit { unit: u, to: here }).unwrap();
    assert_eq!((w.units[&u].dest, w.units[&u].banked), (None, 0.0));
    end_turn(&mut w);
    assert_eq!(w.units[&u].tile, here);
}

#[test]
fn move_orders_are_refused_without_changing_the_world() {
    let (mut w, [red, blue, _]) = led();
    let from = at_big(-5, 0);
    let mine = unit(&mut w, red, from);
    let theirs = unit(&mut w, blue, from);
    let sea = at_big(0, 3);
    w.tiles[sea].terrain = SEA;
    let mountain = at_big(0, 4);
    w.tiles[mountain].terrain = MOUNTAIN;
    // An island the unit can't reach.
    let island = at_big(7, 3);
    for n in neighbours(&w, island).collect::<Vec<_>>() {
        w.tiles[n].terrain = SEA;
    }
    let before = w.clone();
    let cases = [
        (theirs, at_big(0, 0), "that unit isn't yours"),
        (mine, sea, "a unit can only go over land"),
        (mine, mountain, "a unit can only go over land"),
        (mine, island, "no route there"),
        (mine, w.tiles.len(), &format!("{} is not a tile", w.tiles.len())),
        (9999, at_big(0, 0), "9999 is not a unit"),
    ];
    for (unit, to, why) in cases {
        assert_eq!(w.act(Action::MoveUnit { unit, to }).unwrap_err(), why);
        assert_eq!(w, before);
    }
    w.act(Action::MoveUnit { unit: mine, to: at_big(0, 0) }).unwrap();

    // Watching, and before the pick.
    let mut v = big();
    let ms = markets3(&mut v);
    found(&mut v, &ms);
    let any = *v.units.keys().next().unwrap();
    let before = v.clone();
    assert!(v.act(Action::MoveUnit { unit: any, to: 0 }).is_err(), "before the pick");
    assert_eq!(v, before);
    v.act(Action::ChooseCiv { civ: None }).unwrap();
    let before = v.clone();
    assert_eq!(
        v.act(Action::MoveUnit { unit: any, to: at_big(0, 0) }).unwrap_err(),
        "you're watching: you lead no civ"
    );
    assert_eq!(v, before);
}

#[test]
fn path_agrees_with_cost_map_on_a_real_map() {
    let w = World::new(Config { seed: 2, radius: 16 });
    let ok = |i: usize| TERRAIN[w.tiles[i].terrain].land && passable(&w, i);
    let land: Vec<usize> = (0..w.tiles.len()).filter(|&i| ok(i)).collect();
    let mut seed = 12345u32;
    let mut pick = |n: usize| {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (seed >> 8) as usize % n
    };
    let mut cases = 0;
    for _ in 0..25 {
        let s = land[pick(land.len())];
        let costs = cost_map(&w, &[s], f64::INFINITY, ok);
        for _ in 0..12 {
            let t = land[pick(land.len())];
            let got = path(&w, s, t, ok);
            match costs.get(&t) {
                None => assert!(got.is_none()),
                Some(&c) => {
                    let (tiles, cost) = got.unwrap();
                    assert_eq!(cost, c);
                    assert_eq!(tiles.last().copied().unwrap_or(s), t);
                    let mut at = s;
                    let mut sum = 0.0;
                    for &n in &tiles {
                        assert!(neighbours(&w, at).any(|x| x == n));
                        sum += step_cost(&w, at, n).unwrap();
                        at = n;
                    }
                    assert_eq!(sum, c);
                    cases += 1;
                }
            }
        }
        assert_eq!(path(&w, s, s, ok), Some((Vec::new(), 0.0)));
    }
    assert!(cases > 100);
}

/// Plays the digest's scripted game for `turns` more turns on `w`, which is led by `me`.
fn play(w: &mut World, me: u32, pick: u32, turns: u32, check: bool) {
    for _ in 0..turns {
        let n = w.turn - pick;
        if n > 0 && n.is_multiple_of(20) {
            let _ = w.act(Action::MakeUnit);
        }
        if n == 5 {
            w.act(Action::MakeUnit).expect("a clan near the capital can spare a person");
        }
        let targets: Vec<usize> =
            w.civs.values().filter(|c| c.id != me).map(|c| c.capital).chain([w.civs[&me].capital]).collect();
        let mine: Vec<u32> = w.units.values().filter(|u| u.civ == me).map(|u| u.id).collect();
        for u in mine {
            if w.units[&u].dest.is_none() {
                for &to in &targets {
                    if w.act(Action::MoveUnit { unit: u, to }).is_ok() {
                        break;
                    }
                }
            }
        }
        if n == 60 {
            if let Some(&u) = w.units.values().find(|u| u.civ == me).map(|u| &u.id) {
                w.act(Action::DisbandUnit { unit: u }).unwrap();
            }
        }
        end_turn(w);
        if check {
            for c in w.civs.values() {
                assert!(c.store.iter().sum::<f64>() <= CIV.store_cap + 1e-9);
            }
            for u in w.units.values() {
                assert!(w.civs.contains_key(&u.civ));
                assert!(TERRAIN[w.tiles[u.tile].terrain].land && passable(w, u.tile), "unit {} is off land", u.id);
            }
        }
    }
}

#[test]
fn a_led_real_map_keeps_its_units_on_land_and_replays() {
    let mut w = World::new(Config { seed: 2, radius: 16 });
    while w.phase != Phase::Choose {
        w.act(Action::EndTurn).unwrap();
    }
    assert_eq!(w.end.as_ref().unwrap().turn, 276);
    assert_eq!(w.units.len(), 3, "each civ starts with a warrior");
    for u in w.units.values() {
        assert_eq!(u.tile, w.civs[&u.civ].capital);
    }
    let me = *w.civs.keys().next().unwrap();
    w.act(Action::ChooseCiv { civ: Some(me) }).unwrap();
    let pick = w.turn;
    let start: Vec<usize> = w.units.values().map(|u| u.tile).collect();
    play(&mut w, me, pick, 100, true);
    let mine: Vec<&Unit> = w.units.values().filter(|u| u.civ == me).collect();
    assert!(!mine.is_empty());
    assert!(w.units.values().zip(start).any(|(u, s)| u.tile != s) || w.units.len() != 3, "the orders did something");
    assert!(w.log.iter().any(|a| matches!(a, Action::MakeUnit)));
    assert!(w.log.iter().any(|a| matches!(a, Action::MoveUnit { .. })));
    assert!(w.log.iter().any(|a| matches!(a, Action::DisbandUnit { .. })));
    assert!(!kinds(&w, EventKind::UnitMade).is_empty());
    assert!(!kinds(&w, EventKind::UnitDisbanded).is_empty());

    let json = serde_json::to_string(&w.save()).unwrap();
    let save: engine::Save = serde_json::from_str(&json).unwrap();
    let r = World::replay(&save).unwrap();
    assert_eq!(r, w);
    assert_eq!(r.events, w.events);
    // The orders' JSON is the page's.
    assert_eq!(serde_json::to_string(&Action::MakeUnit).unwrap(), r#"{"type":"makeUnit"}"#);
    assert_eq!(
        serde_json::to_string(&Action::MoveUnit { unit: 40, to: 812 }).unwrap(),
        r#"{"type":"moveUnit","unit":40,"to":812}"#
    );
    assert_eq!(
        serde_json::to_string(&Action::DisbandUnit { unit: 40 }).unwrap(),
        r#"{"type":"disbandUnit","unit":40}"#
    );
}
