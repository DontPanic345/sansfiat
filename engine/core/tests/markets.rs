mod common;

use common::*;
use engine::clans::{harvest, nutrition};
use engine::data::*;
use engine::land::{hold, is_hearth, step_cost};
use engine::markets::{fade, form, gain, go, trade};
use engine::round::cost;
use engine::{utility, Basket, World};

const EPS: f64 = 1e-9;

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn free(w: &mut World, id: u32) {
    let c = w.clans.remove(&id).unwrap();
    for t in c.round {
        w.tiles[t].claim = None;
    }
}

#[test]
fn a_round_wears_a_trail_and_an_unwalked_one_wears_away() {
    let mut w = flat();
    let (o, f) = (at(0, 0), out(0, 1));
    w.tiles[f].terrain = FOREST;
    let id = put(&mut w, &[o, f], 3.0);
    let before = cost(&w, &w.clans[&id].round);
    assert_eq!(before, 3.0);
    for _ in 1..TRAIL.after {
        hold(&mut w);
    }
    assert!(!w.tiles[f].trail);
    hold(&mut w);
    assert!(w.tiles[f].trail, "held {} turns", w.tiles[f].held);
    assert_eq!(step_cost(&w, f), Some(TRAIL.cost));
    assert_eq!(cost(&w, &w.clans[&id].round), 2.0);

    free(&mut w, id);
    for _ in 1..TRAIL.fade {
        hold(&mut w);
    }
    assert!(w.tiles[f].trail);
    hold(&mut w);
    assert!(!w.tiles[f].trail, "unheld {} turns", w.tiles[f].unheld);
    assert_eq!(step_cost(&w, f), Some(2.0));
}

#[test]
fn held_grain_tames_into_a_hearth_and_left_grain_goes_wild() {
    let mut w = flat();
    let o = at(0, 0);
    w.tiles[o].wild_grain = 4.0;
    w.tiles[o].cap[GRAIN] = 4.0;
    let id = put(&mut w, &[o], 3.0);
    for _ in 1..TENDING.after {
        hold(&mut w);
    }
    assert_eq!((w.tiles[o].tame, w.tiles[o].cap[GRAIN]), (0, 4.0));
    let mut cap = 4.0;
    for k in 1..=5 {
        hold(&mut w);
        let t = &w.tiles[o];
        assert_eq!(t.tame, k);
        let want = round1(4.0 * (1.0 + (TENDING.domestic - 1.0) * k as f64 / TENDING.tame_turns as f64));
        assert_eq!(t.cap[GRAIN], want);
        assert!(t.cap[GRAIN] >= cap);
        cap = t.cap[GRAIN];
    }
    assert!(cap > 4.0);
    for _ in 5..TENDING.tame_turns {
        hold(&mut w);
    }
    assert!(is_hearth(&w.tiles[o]));
    assert_eq!(w.tiles[o].cap[GRAIN], 10.0);
    hold(&mut w);
    assert_eq!(w.tiles[o].tame, TENDING.tame_turns, "tame stops at a hearth");

    free(&mut w, id);
    hold(&mut w);
    assert_eq!(w.tiles[o].tame, TENDING.tame_turns - 1);
    assert!(!is_hearth(&w.tiles[o]));
    hold(&mut w);
    assert_eq!(w.tiles[o].tame, TENDING.tame_turns - 2);
    assert!(w.tiles[o].cap[GRAIN] < 10.0);
}

/// A clan of `pop` at `home` held `form_after` turns, and three more clans two steps away.
fn camp(w: &mut World, home: usize, pop: f64, others: usize) {
    put(w, &[home], pop);
    w.tiles[home].held = MARKET.form_after;
    let h = w.tiles[home].hex();
    for d in [0, 2, 4].into_iter().take(others) {
        let (dq, dr) = engine::hex::DIRS[d];
        let t = engine::map::index(R, engine::hex::Hex { q: h.q + 2 * dq, r: h.r + 2 * dr }).unwrap();
        put(w, &[t], 3.0);
    }
}

#[test]
fn a_market_forms_on_a_long_held_home_with_four_clans_in_reach() {
    let mut w = flat();
    let o = at(0, 0);
    camp(&mut w, o, 5.0, 3);
    w.turn = 7;
    form(&mut w);
    assert_eq!(w.markets.len(), 1);
    let m = w.markets.values().next().unwrap();
    assert_eq!((m.tile, m.founded, m.prices, m.activity), (o, 7, [1.0; N], 2.0 * MARKET.min_volume));

    let mut w = flat();
    camp(&mut w, o, 5.0, 2);
    form(&mut w);
    assert!(w.markets.is_empty(), "three clans in reach");

    let mut w = flat();
    camp(&mut w, o, 5.0, 3);
    w.tiles[o].held = MARKET.form_after - 1;
    form(&mut w);
    assert!(w.markets.is_empty(), "not held long enough");

    let mut w = flat();
    camp(&mut w, o, 5.0, 3);
    open(&mut w, out(3, 4));
    form(&mut w);
    assert_eq!(w.markets.len(), 1, "four steps from a market");
}

#[test]
fn two_sites_in_a_turn_get_one_market_the_more_peopled() {
    let mut w = flat();
    let (a, b) = (at(-3, 0), at(3, 0));
    camp(&mut w, a, 4.0, 3);
    camp(&mut w, b, 6.0, 3);
    form(&mut w);
    assert_eq!(w.markets.len(), 1);
    assert_eq!(w.markets.values().next().unwrap().tile, b);
    // They're 6 apart, so the other forms next turn.
    form(&mut w);
    assert_eq!(w.markets.values().map(|m| m.tile).collect::<Vec<_>>(), vec![b, a]);
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
fn a_clan_goes_when_the_trade_beats_the_walk() {
    let (mut w, id, mid) = grain_clan(1, 0.0, 0.4);
    let g = gain(&w.markets[&mid], &[0.0, 10.0, 0.0, 0.0, 0.0], 5.0);
    // Worth a walk of 2, not of 6 unless it adds the desire to gather.
    let g = g * CLAN.harvest_labour;
    assert!(2.0 < g && g < 6.0 && 6.0 < g + MARKET.gather_bonus, "gain {g}");
    go(&mut w);
    assert_eq!((w.clans[&id].market, w.clans[&id].trip), (Some(mid), 2.0));

    // At path cost 3 the walk is 6, more than the gain.
    let (mut w, id, _) = grain_clan(3, 0.0, 0.4);
    go(&mut w);
    assert_eq!((w.clans[&id].market, w.clans[&id].trip), (None, 0.0));

    // A surplus adds the desire to gather.
    let (mut w, id, mid) = grain_clan(3, 0.6, 0.4);
    assert!(0.6 >= MARKET.surplus_at * 5.0);
    go(&mut w);
    assert_eq!((w.clans[&id].market, w.clans[&id].trip), (Some(mid), 6.0));

    // It harvests with the trip less labour: 12.5 − 6, under its 7.5 − 0.6 budget.
    harvest(&mut w);
    let got: f64 = w.clans[&id].harvest.iter().sum();
    assert!((got - (5.0 * CLAN.work - 6.0)).abs() < EPS, "harvested {got}");

    // With no gain to be had (only grain ever traded there) and no surplus, it stays home.
    let (mut w, id, mid) = grain_clan(1, 0.0, 0.4);
    w.markets.get_mut(&mid).unwrap().goods = vec![GRAIN];
    go(&mut w);
    assert_eq!(w.clans[&id].market, None);
}

#[test]
fn a_clan_goes_to_the_nearest_market() {
    let (mut w, id, _) = grain_clan(2, 0.0, 0.4);
    let near = open(&mut w, out(3, 1));
    w.markets.get_mut(&near).unwrap().goods = vec![GRAIN, FISH];
    w.markets.get_mut(&near).unwrap().prices[FISH] = 0.4;
    go(&mut w);
    assert_eq!((w.clans[&id].market, w.clans[&id].trip), (Some(near), 2.0));
}

#[test]
fn trade_conserves_food_and_nobody_loses_much() {
    let mut w = flat();
    let o = at(0, 0);
    let mid = open(&mut w, o);
    let baskets: [Basket; 4] =
        [[0.0, 8.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 6.0, 2.0], [3.0, 1.0, 4.0, 0.0, 0.0], [1.0, 2.0, 0.0, 3.0, 0.0]];
    let mut ids = Vec::new();
    for (k, b) in baskets.iter().enumerate() {
        let id = put(&mut w, &[out(k, 1)], 4.0);
        let c = w.clans.get_mut(&id).unwrap();
        c.basket = *b;
        c.market = Some(mid);
        ids.push(id);
    }
    let total = |w: &World| -> Basket { std::array::from_fn(|f| ids.iter().map(|id| w.clans[id].basket[f]).sum()) };
    let before = total(&w);
    trade(&mut w);
    let after = total(&w);
    for f in 0..N {
        assert!((after[f] - before[f]).abs() < 1e-9, "{f}: {} → {}", before[f], after[f]);
    }
    for (id, b) in ids.iter().zip(&baskets) {
        let a = w.clans[id].basket;
        assert!(utility(&a) >= utility(b) - 1e-6, "clan {id}: {b:?} → {a:?}");
        assert!(nutrition(&a, 4.0) >= 0.9 * nutrition(b, 4.0), "clan {id}: {b:?} → {a:?}");
    }
    let m = &w.markets[&mid];
    assert_eq!(m.traders, 4);
    assert!(m.volume > 0.0 && m.prices != [1.0; N]);
    let want = MARKET.fade * 2.0 * MARKET.min_volume + (1.0 - MARKET.fade) * m.volume;
    assert!((m.activity - want).abs() < EPS);
}

#[test]
fn a_market_nobody_trades_at_fades() {
    let mut w = flat();
    let mid = open(&mut w, at(0, 0));
    // A lone trader trades nothing.
    let id = put(&mut w, &[out(0, 1)], 4.0);
    w.clans.get_mut(&id).unwrap().market = Some(mid);
    for turn in 1..=4 {
        trade(&mut w);
        fade(&mut w);
        // 4 → 3.2 → 2.56 → 2.05 → 1.64
        assert_eq!(w.markets.contains_key(&mid), turn < 4, "turn {turn}");
    }
    assert_eq!(w.clans[&id].market, None);
}
