//! The reach guard: what a unit, a clan or a market can reach on staged maps, pinned so the day's
//! scale of every cost and labour (`DAY`) changes none of it. Counts and tile lists are exact; a
//! labour or a cost is read over `DAY`, to 1e-9.

mod common;

use common::{led_order, led_scout};
use engine::data::*;
use engine::markets;
use engine::stage::{open_market, Stage};
use engine::units;
use engine::{Order, World};

const EPS: f64 = 1e-9;

fn close(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() < EPS, "{what}: {a} against {b}");
}

/// A fingerprint of a tile list: its count and a hash of its tiles in order.
fn fp(tiles: impl IntoIterator<Item = usize>) -> (usize, u64) {
    let (mut n, mut h) = (0, 1469598103934665603u64);
    for t in tiles {
        n += 1;
        h = (h ^ t as u64).wrapping_mul(1099511628211);
    }
    (n, h)
}

/// The retro pick of small 2, led by its first civ, as bare plains.
fn plain() -> Stage {
    let mut s = Stage::retro("small", 2).unwrap();
    let first = s.world().civs.keys().next().copied().expect("a civ");
    s.lead(first).unwrap();
    s.plain();
    s
}

/// Tiles by their hex offset from the capital, picked by a pattern that gives forest and hills with
/// hidden trail costs (some trails built), a river and a worn plains path. Costs set by hand are in
/// days, so they carry `DAY`.
fn rough() -> Stage {
    let mut s = plain();
    let w = s.world();
    let cap = w.civs[&w.player.unwrap()].capital;
    let ch = w.tiles[cap].hex();
    let near: Vec<usize> = (0..w.tiles.len()).filter(|&i| i != cap && w.tiles[i].hex().distance(ch) <= 7).collect();
    for i in near {
        let h = s.world().tiles[i].hex();
        let (dq, dr) = (h.q - ch.q, h.r - ch.r);
        if s.world().tiles[i].terrain == SEA {
            continue;
        }
        match (dq * 3 + dr * 5).rem_euclid(9) {
            0 | 1 => s.paint(i, FOREST, Some(1.0 * DAY), None),
            2 => s.paint(i, HILLS, Some(2.0 * DAY), None),
            3 => s.paint(i, HILLS, Some(1.0 * DAY), None),
            _ => {}
        }
        let t = &mut s.world_mut().tiles[i];
        match (dq * 3 + dr * 5).rem_euclid(9) {
            1 | 3 => t.trail = true,
            4 => t.river = 1,
            5 if dq > 0 => t.trail = true,
            _ => {}
        }
    }
    s
}

#[derive(Debug, PartialEq)]
struct Reach {
    /// Tiles within 6 steps a camp can walk to, by the turns the walk takes: `(turns, tiles)`.
    turns: Vec<(u32, usize)>,
    tiles: (usize, u64),
    /// The sum over those tiles of the walk's cost (days), and of the line's carry over the eating.
    cost: f64,
    carry: f64,
}

/// The camp's turns from the capital to every tile within 6 steps, and `line_from` there.
fn camp_reach(mut s: Stage) -> Reach {
    let me = s.world().player.unwrap();
    let cap = s.world().civs[&me].capital;
    let id = s.army(me, cap, 3.0);
    let w = s.world();
    let eats = w.units[&id].people * UNIT.food;
    let (mut hist, mut tiles, mut cost, mut carry) = (std::collections::BTreeMap::new(), Vec::new(), 0.0, 0.0);
    for i in 0..w.tiles.len() {
        if i == cap || w.tiles[i].hex().distance(w.tiles[cap].hex()) > 6 {
            continue;
        }
        let Some((path, c)) = units::path_between(w, cap, i) else { continue };
        *hist.entry(units::turns(w, &path, cap, 0.0)).or_insert(0usize) += 1;
        tiles.push(i);
        cost += c / DAY;
        if let Some((_, carried)) = units::line_from(w, id, i) {
            carry += carried / eats;
        }
    }
    Reach { turns: hist.into_iter().collect(), tiles: fp(tiles), cost, carry }
}

#[test]
fn the_camps_turns_and_line_on_plains() {
    let r = camp_reach(plain());
    assert_eq!(r.turns, [(1, 23), (2, 36)]);
    assert_eq!(r.tiles, (59, 6943691052843469787));
    close(r.cost, 231.0, "cost");
    close(r.carry, 100.53749999999998, "carry");
}

#[test]
fn the_camps_turns_and_line_on_rough_country() {
    let r = camp_reach(rough());
    assert_eq!(r.turns, [(1, 15), (2, 39), (3, 5)]);
    assert_eq!(r.tiles, (59, 6943691052843469787));
    close(r.cost, 275.25, "cost");
    close(r.carry, 96.34854364584056, "carry");
}

/// The land tile furthest from the capital by a walk (lowest index on a tie): the end of a long path.
fn far(w: &World) -> usize {
    let cap = w.civs[&w.player.unwrap()].capital;
    let by = |i: usize| units::path_between(w, cap, i).map(|(p, _)| p.len());
    (0..w.tiles.len())
        .filter(|&i| i != cap)
        .filter_map(|i| by(i).map(|n| (n, std::cmp::Reverse(i))))
        .max()
        .map(|(_, r)| r.0)
        .unwrap()
}

/// A civ's territory, the clan's home and the people, after 20 turns from one clan of 6.
#[test]
fn a_clans_territory_after_twenty_turns() {
    let mut s = Stage::flat(common::BIG);
    let home = common::at_big(0, 0);
    let h = s.world().tiles[home].hex();
    let near: Vec<usize> = (0..s.world().tiles.len()).filter(|&i| s.world().tiles[i].hex().distance(h) <= 3).collect();
    for &i in &near {
        let k = s.world().tiles[i].hex();
        s.food(
            i,
            [GRAIN, GAME, FRUIT][((k.q - k.r).rem_euclid(3)) as usize],
            2.0 + (k.q + 2 * k.r).rem_euclid(3) as f64,
        );
    }
    s.clan(home, 6.0);
    s.turn(20).unwrap();
    let w = s.world();
    let mut lands: Vec<(usize, usize, Vec<usize>)> =
        w.clans.values().map(|c| (c.home, c.people.round() as usize, c.territory.clone())).collect();
    lands.sort();
    let want = vec![(186, 7, vec![162, 163, 164, 165, 184, 185, 186, 187, 188, 208, 209, 210, 211, 233, 234, 235])];
    assert_eq!(lands, want);
}

/// A 2-person scout sent along a long path for one turn: the steps from the capital it stands at.
fn march(s: Stage) -> (i32, usize) {
    let mut w = s.into_world();
    let me = w.player.unwrap();
    w.civs.get_mut(&me).unwrap().store = [100.0; N];
    let to = far(&w);
    let id = led_scout(&w);
    w.units.get_mut(&id).unwrap().store = [1.0; N];
    assert_eq!(w.units[&id].people, SCOUT.people as f64);
    w.act(led_order(&w, Order::Move { unit: id, to })).unwrap();
    w.act(engine::Action::EndTurn).unwrap();
    let cap = w.civs[&me].capital;
    (common::steps(&w, cap, w.units[&id].home), units::path_between(&w, cap, to).unwrap().0.len())
}

#[test]
fn a_scouts_march_over_one_turn() {
    assert_eq!(march(plain()), (7, 32));
    assert_eq!(march(rough()), (5, 32));
}

/// Markets at the capital's neighbours at distance 2, 4 and 6, on land.
fn markets_on(s: &mut Stage) -> Vec<u32> {
    let w = s.world();
    let cap = w.civs[&w.player.unwrap()].capital;
    let ch = w.tiles[cap].hex();
    let mut at = Vec::new();
    for d in [2, 4, 6] {
        let t = (0..w.tiles.len()).find(|&i| {
            w.tiles[i].terrain != SEA && w.tiles[i].hex().distance(ch) == d && !w.markets.values().any(|m| m.tile == i)
        });
        at.extend(t);
    }
    let w = s.world_mut();
    at.into_iter().map(|t| open_market(w, t)).collect()
}

fn festivals(mut s: Stage) -> Vec<(u32, f64, f64)> {
    let ms = markets_on(&mut s);
    let w = s.world();
    let cap = w.civs[&w.player.unwrap()].capital;
    let home = w.tiles[cap].hex();
    let h = (0..w.tiles.len()).find(|&i| w.tiles[i].terrain != SEA && w.tiles[i].hex().distance(home) == 1).unwrap();
    let id = s.clan(h, 8.0);
    let w = s.world_mut();
    for (k, m) in ms.iter().enumerate() {
        w.markets.get_mut(m).unwrap().traders = k * 2;
    }
    let c = w.clans.get_mut(&id).unwrap();
    c.store = [0.5, 1.0, 0.0, 0.2, 0.0];
    c.harvest = [0.0, 2.0, 0.0, 0.0, 0.0];
    let c = c.clone();
    markets::worths(w, &c).into_iter().map(|(m, worth, cost)| (m, worth / DAY, cost / DAY)).collect()
}

#[test]
fn a_clans_festival_worths() {
    let same = |got: Vec<(u32, f64, f64)>, want: &[(u32, f64, f64)]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert_eq!(g.0, w.0);
            close(g.1, w.1, "worth");
            close(g.2, w.2, "route");
        }
    };
    same(
        festivals(plain()),
        &[
            (13, 9.32064688683823, 8.0),
            (14, 5.0, 12.0),
            (508, 12.0, 2.0),
            (517, 9.0, 2.0),
            (518, 7.0, 6.0),
            (519, 5.0, 10.0),
        ],
    );
    same(
        festivals(rough()),
        &[
            (13, 3.3206468868382295, 14.0),
            (14, 5.5, 11.5),
            (508, 12.0, 2.0),
            (517, 5.0, 6.0),
            (518, 3.0, 10.0),
            (519, 1.0, 14.0),
        ],
    );
}

/// The tiles within `MARKET.reach` of a market, and their costs summed over `DAY`.
fn market_reach(mut s: Stage) -> Vec<((usize, u64), f64)> {
    markets_on(&mut s);
    let w = s.world();
    w.markets
        .keys()
        .map(|m| {
            let r = markets::reach(w, w.markets[m].tile);
            (fp(r.keys().copied()), r.values().sum::<f64>() / DAY)
        })
        .collect()
}

#[test]
fn a_markets_reach() {
    let same = |got: Vec<((usize, u64), f64)>, want: &[((usize, u64), f64)]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert_eq!(g.0, w.0);
            close(g.1, w.1, "cost");
        }
    };
    same(
        market_reach(plain()),
        &[
            ((23, 1008722748888188030), 47.0),
            ((16, 381460378390794259), 30.0),
            ((31, 15554769140626353298), 67.0),
            ((10, 917889417507760880), 20.0),
            ((28, 4188226653089684065), 58.0),
            ((30, 16879009175407104367), 67.0),
            ((28, 5623740252638316715), 59.0),
            ((24, 4734643650917682870), 49.0),
            ((26, 3641566043613708604), 54.0),
            ((24, 3317082977256472500), 54.0),
            ((19, 199102649478281699), 38.0),
            ((28, 9987723903039886204), 62.0),
            ((32, 18051401851842048072), 71.0),
            ((19, 5834923054727945698), 38.0),
            ((22, 4311542380923120773), 45.0),
            ((10, 10498521296487622654), 21.0),
            ((18, 12256555938220494679), 39.0),
            ((24, 14158622821670781807), 53.0),
            ((32, 6003260564047063685), 70.0),
            ((15, 5368945674371297714), 30.0),
            ((22, 15331512086420481248), 48.0),
            ((19, 637772063271483685), 40.0),
            ((19, 1195762879796604966), 40.0),
        ],
    );
    same(
        market_reach(rough()),
        &[
            ((17, 620197658021992284), 34.0),
            ((13, 15907251064839161926), 23.25),
            ((31, 15554769140626353298), 67.0),
            ((10, 917889417507760880), 20.0),
            ((25, 2336694167748276924), 50.0),
            ((30, 16879009175407104367), 67.0),
            ((28, 5623740252638316715), 59.0),
            ((24, 4734643650917682870), 49.0),
            ((26, 3641566043613708604), 54.0),
            ((24, 3317082977256472500), 54.0),
            ((19, 199102649478281699), 38.0),
            ((28, 9987723903039886204), 62.0),
            ((32, 18051401851842048072), 71.0),
            ((19, 5834923054727945698), 38.0),
            ((22, 4311542380923120773), 45.0),
            ((10, 10498521296487622654), 21.0),
            ((18, 12256555938220494679), 39.0),
            ((16, 2104734832945321476), 31.25),
            ((32, 6003260564047063685), 70.0),
            ((15, 5368945674371297714), 30.0),
            ((17, 660009377049139268), 38.0),
            ((9, 2211903091117356237), 18.0),
            ((11, 12270350723414996958), 25.0),
        ],
    );
}
