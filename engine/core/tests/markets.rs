mod common;

use common::*;
use engine::actors::{walk_cost, Actor, Carrier};
use engine::clans::harvest;
use engine::data::*;
use engine::land::{is_hearth, tend};
use engine::markets::{choose, desire, gain, go, load, worths};
use engine::World;

const EPS: f64 = 1e-9;

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn free(w: &mut World, id: u32) {
    let c = w.clans.remove(&id).unwrap();
    for &t in &c.territory {
        w.tiles[t].claim = None;
    }
}

#[test]
fn held_grain_tames_into_a_hearth_and_left_grain_goes_wild() {
    let mut w = flat();
    let o = at(0, 0);
    w.tiles[o].wild_grain = 4.0;
    w.tiles[o].cap[GRAIN] = 4.0;
    let id = put(&mut w, &[o], 3.0);
    for _ in 1..TENDING.after {
        tend(&mut w);
    }
    assert_eq!((w.tiles[o].tame, w.tiles[o].cap[GRAIN]), (0, 4.0));
    let mut cap = 4.0;
    for k in 1..=5 {
        tend(&mut w);
        let t = &w.tiles[o];
        assert_eq!(t.tame, k);
        let want = round1(4.0 * (1.0 + (TENDING.domestic - 1.0) * k as f64 / TENDING.tame_turns as f64));
        assert_eq!(t.cap[GRAIN], want);
        assert!(t.cap[GRAIN] >= cap);
        cap = t.cap[GRAIN];
    }
    assert!(cap > 4.0);
    for _ in 5..TENDING.tame_turns {
        tend(&mut w);
    }
    assert!(is_hearth(&w.tiles[o]));
    assert_eq!(w.tiles[o].cap[GRAIN], 10.0);
    // Past a hearth taming goes on, and the grain keeps rising, more slowly.
    for _ in 0..TENDING.tame_turns {
        tend(&mut w);
    }
    assert_eq!(w.tiles[o].tame, 2 * TENDING.tame_turns, "taming has no ceiling");
    assert!((w.tiles[o].cap[GRAIN] - 4.0 * (TENDING.domestic + TENDING.beyond)).abs() < 1e-9);

    free(&mut w, id);
    for _ in 0..TENDING.tame_turns {
        tend(&mut w);
    }
    assert_eq!(w.tiles[o].tame, TENDING.tame_turns, "unheld, it tames back a step a turn");
    assert!(is_hearth(&w.tiles[o]));
    tend(&mut w);
    tend(&mut w);
    assert_eq!(w.tiles[o].tame, TENDING.tame_turns - 2);
    assert!(!is_hearth(&w.tiles[o]));
    assert!(w.tiles[o].cap[GRAIN] < 10.0);
}

/// A clan of 5 at the middle with 10 grain from last turn, `store` of it kept, 40 grain at home;
/// a market `k` plains steps away where fish costs `fish` grain and both were traded.
fn grain_clan(k: i32, store: f64, fish: f64) -> (World, u32, u32) {
    let mut w = flat();
    let o = at(0, 0);
    w.tiles[o].stock[GRAIN] = 40.0;
    let id = put(&mut w, &[o], 5.0);
    let c = w.clans.get_mut(&id).unwrap();
    c.store[GRAIN] = store;
    c.harvest[GRAIN] = 10.0 - store;
    let mid = open(&mut w, out(0, k));
    let m = w.markets.get_mut(&mid).unwrap();
    m.prices[FISH] = fish;
    m.goods = vec![GRAIN, FISH];
    (w, id, mid)
}

#[test]
fn a_festival_is_worth_its_gain_and_desire_less_the_walk() {
    let (w, id, mid) = grain_clan(1, 0.0, 0.4);
    let m = &w.markets[&mid];
    let c = &w.clans[&id];
    let l = load(m, &[0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0], 5.0, walk_cost(1.0 * DAY, 1.0));
    let g = l.gain * CLAN.harvest_labour;
    assert!(g > 0.0);
    assert_eq!(l.walkers, 1.0, "the second load does not pay its walk");
    assert_eq!(worths(&w, c), vec![(mid, g + MARKET.desire - walk_cost(1.0 * DAY, 1.0), 2.0 * DAY)]);
    // Company: more for each clan that came last turn, up to the cap.
    let mut m = m.clone();
    m.traders = MARKET.company_cap + 3;
    let most = MARKET.desire + MARKET.company * MARKET.company_cap as f64;
    assert_eq!(desire(&m, false), most);
    assert_eq!(desire(&m, true), most + MARKET.gather_bonus);
}

#[test]
fn a_clan_goes_where_the_trade_beats_the_walk_and_its_route_is_its_trip() {
    let (mut w, id, mid) = grain_clan(1, 0.0, 0.4);
    go(&mut w);
    let c = &w.clans[&id];
    assert_eq!(c.market, Some(mid));
    assert_eq!(c.routes.len(), 1);
    let r = &c.routes[0];
    assert_eq!((r.to, r.path.clone(), r.cost), (out(0, 1), vec![out(0, 1)], 2.0 * DAY));
    assert_eq!(c.trip(), r.cost);

    // Far enough that the walk costs more than the gain and the desire.
    let (mut w, id, _) = grain_clan(1, 0.0, 0.4);
    let m = w.markets.values().next().unwrap().clone();
    let g = gain(&m, &[0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0], 5.0, 0.2);
    let k = ((g * CLAN.harvest_labour + MARKET.desire) / (2.0 * DAY)).floor() as i32 + 1;
    let (mut far, fid, _) = grain_clan(k, 0.0, 0.4);
    assert!(k < R);
    go(&mut far);
    assert_eq!((far.clans[&fid].market, far.clans[&fid].trip()), (None, 0.0), "{k} steps away");
    assert!(far.clans[&fid].routes.is_empty());

    // A surplus adds the desire to gather.
    let store = 0.6;
    let (mut rich, rid, rmid) = grain_clan(k, store, 0.4);
    assert!(store >= MARKET.surplus_at * 5.0 && 2.0 * k as f64 * DAY <= MARKET.reach_walk);
    go(&mut rich);
    assert_eq!(rich.clans[&rid].market, Some(rmid));
    assert!((rich.clans[&rid].trip() - 2.0 * k as f64 * DAY).abs() < EPS);

    // It harvests with the route's labour paid first: left 3 of it, it takes 3 food.
    go(&mut w);
    let c = w.clans.get_mut(&id).unwrap();
    c.routes[0].cost = 5.0 * CLAN.work - 3.0 * DAY;
    harvest(&mut w);
    let got: f64 = w.clans[&id].harvest.iter().sum();
    assert!((got - 3.0 * DAY / CLAN.harvest_labour).abs() < EPS, "harvested {got}");
    assert!(w.tiles[out(0, 1)].traffic > 0.0, "the route is walked");

    // Nothing to gain (only grain ever traded there), no surplus, and the walk beats the desire.
    let k = (MARKET.desire / (2.0 * DAY)).floor() as i32 + 1;
    let (mut w, id, mid) = grain_clan(k, 0.0, 0.4);
    assert!(k < R);
    w.markets.get_mut(&mid).unwrap().goods = vec![GRAIN];
    go(&mut w);
    assert_eq!(w.clans[&id].market, None);
}

#[test]
fn a_clan_goes_to_the_festival_worth_most() {
    let (mut w, id, far) = grain_clan(2, 0.0, 0.4);
    let near = open(&mut w, out(3, 1));
    w.markets.get_mut(&near).unwrap().goods = vec![GRAIN, FISH];
    w.markets.get_mut(&near).unwrap().prices[FISH] = 0.4;
    go(&mut w);
    assert_eq!((w.clans[&id].market, w.clans[&id].trip()), (Some(near), 2.0 * DAY));
    // Company at the far one outweighs its longer walk.
    w.markets.get_mut(&far).unwrap().traders = MARKET.company_cap;
    w.clans.get_mut(&id).unwrap().market = None;
    assert!(MARKET.company * MARKET.company_cap as f64 > 2.0);
    go(&mut w);
    assert_eq!(w.clans[&id].market, Some(far));
}

#[test]
fn a_clan_keeps_its_festival_unless_another_beats_it_by_the_margin() {
    // Two festivals the same walk away, the same in all but company.
    let (mut w, id, a) = grain_clan(1, 0.0, 0.4);
    let b = open(&mut w, out(3, 1));
    let m = w.markets[&a].clone();
    let mb = w.markets.get_mut(&b).unwrap();
    (mb.goods, mb.prices) = (m.goods, m.prices);
    w.clans.get_mut(&id).unwrap().market = Some(b);
    let pick = |w: &World| choose(&worths(w, &w.clans[&id]), w.clans[&id].market, MARKET.keep_turns);
    assert_eq!(choose(&worths(&w, &w.clans[&id]), None, 0), Some(a), "ties to the lower id");
    assert_eq!(pick(&w), Some(b), "equal: it keeps its own");
    // One clan's company is within the margin; enough of them is not.
    let enough = (MARKET.switch_margin / MARKET.company).floor() as usize + 1;
    assert!(enough <= MARKET.company_cap);
    w.markets.get_mut(&a).unwrap().traders = enough - 1;
    let x = worths(&w, &w.clans[&id]);
    assert!((x[0].1 - x[1].1 - MARKET.company * (enough - 1) as f64).abs() < EPS);
    let keep = MARKET.keep_turns;
    assert_eq!(choose(&[(a, 1.0 + MARKET.switch_margin, 2.0), (b, 1.0, 2.0)], Some(b), keep), Some(b));
    w.markets.get_mut(&a).unwrap().traders = enough;
    assert_eq!(pick(&w), Some(a));
    // A current festival worth a little under nothing is kept; worth less than the margin it is left.
    assert_eq!(choose(&[(a, 0.5, 2.0), (b, -1.0, 2.0)], Some(b), keep), Some(a));
    assert_eq!(choose(&[(a, -0.5, 2.0), (b, -1.0, 2.0)], Some(b), keep), Some(b));
    assert_eq!(choose(&[(a, -0.5, 2.0), (b, -MARKET.stay_margin - 0.1, 2.0)], Some(b), keep), None);
}

/// A clan of `people` at the middle with `grain` grain from last turn, and a festival `k` steps away
/// where fish is cheap.
fn carrying(people: f64, grain: f64, k: i32) -> (World, u32, u32) {
    let mut w = flat();
    let o = at(0, 0);
    w.tiles[o].stock[GRAIN] = 40.0;
    let id = put(&mut w, &[o], people);
    let c = w.clans.get_mut(&id).unwrap();
    c.basket[GRAIN] = grain;
    c.harvest[GRAIN] = grain;
    let mid = open(&mut w, out(0, k));
    let m = w.markets.get_mut(&mid).unwrap();
    m.prices[FISH] = 0.2;
    m.goods = vec![GRAIN, FISH];
    (w, id, mid)
}

#[test]
fn a_clan_brings_each_load_whose_trade_beats_its_walkers_days_and_stops_at_the_first_that_does_not() {
    let (w, _, mid) = carrying(20.0, 40.0, 1);
    let m = &w.markets[&mid];
    let b = [0.0, 40.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let unit = Actor::carry(Carrier::Clan).load;
    let mut last = 0.5;
    for walk in [0.0, 0.05, 0.2, 0.5, 1.0, 2.0, 6.0].map(|d| d * DAY) {
        let l = load(m, &b, 20.0, walk);
        let brought = l.share * 40.0;
        assert!(l.share <= last + EPS, "a longer walk brings no more: {walk} brings {brought}");
        last = l.share;
        assert!(brought >= unit - EPS, "the first load goes with the walker who goes anyway");
        assert!((l.walkers - (brought / unit).max(1.0)).abs() < EPS, "a walker for each 2 food");
        assert!((l.gain - gain(m, &b, 20.0, l.share)).abs() < EPS);
        if walk == 0.0 {
            assert_eq!(l.share, 0.5, "a clan on its festival's tile walks nothing and brings its whole surplus");
            continue;
        }
        // Every load it brought paid its walker's days; the next does not.
        let at = |k: f64| gain(m, &b, 20.0, (k * unit / 40.0).min(0.5));
        let n = (brought / unit).round();
        for k in 2..=n as usize {
            let k = k as f64;
            assert!((at(k) - at(k - 1.0)) * CLAN.harvest_labour > walk, "load {k} at {walk}");
        }
        if l.share < 0.5 {
            assert!((at(n + 1.0) - at(n)) * CLAN.harvest_labour <= walk, "load {} at {walk}", n + 1.0);
        }
    }
    assert!(load(m, &b, 20.0, 0.05 * DAY).share > load(m, &b, 20.0, 6.0 * DAY).share, "the door brings more");
    assert_eq!(load(m, &b, 20.0, 6.0 * DAY).share * 40.0, unit, "a long walk brings one load");
    // Only the food beyond a food a person goes: a clan with none brings none.
    assert_eq!(load(m, &[0.0, 20.0, 0.0, 0.0, 0.0, 0.0, 0.0], 20.0, 0.0).share, 0.0);
}

#[test]
fn a_clan_that_carries_more_pays_more_days_to_the_same_festival() {
    let one_walker = 2.0 * 3.0 * DAY;
    let (few, a, mid) = carrying(2.0, 0.4, 3);
    let (mut many, b, _) = carrying(20.0, 200.0, 3);
    // A near festival is worth its loads; at 3 steps a clan of twenty with 200 grain brings what pays.
    let cost = |w: &World, id: u32, mid: u32| worths(w, &w.clans[&id]).iter().find(|x| x.0 == mid).unwrap().2;
    let l = load(&many.markets[&mid], &many.clans[&b].basket, 20.0, one_walker);
    assert!((cost(&few, a, mid) - one_walker).abs() < EPS, "a light clan walks with one");
    assert!((cost(&many, b, mid) - one_walker * l.walkers).abs() < EPS);
    many.clans.get_mut(&b).unwrap().store[GRAIN] = 50.0;
    go(&mut many);
    let c = &many.clans[&b];
    let l = load(&many.markets[&mid], &[0.0, 250.0, 0.0, 0.0, 0.0, 0.0, 0.0], 20.0, one_walker);
    assert_eq!(c.market, Some(mid));
    assert!((c.routes[0].cost - one_walker * l.walkers).abs() < EPS && (c.walkers - l.walkers).abs() < EPS);
    // A clan of twenty that brings nothing walks it alone, as a clan of two does.
    let (empty, c, mid) = carrying(20.0, 0.0, 3);
    assert!((cost(&empty, c, mid) - one_walker).abs() < EPS);
}
