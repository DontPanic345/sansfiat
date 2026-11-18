//! Lots (`lots.rs`): where a ware in a store came from. `Lots::mix` reweights by amount; in a real world
//! every store's wares carry lots that sum to 1 and start at a source of the good, every turn.

mod common;

use engine::data::*;
use engine::lots::{cleared, Lot, Lots};
use engine::market::Basket;
use engine::stage::Stage;
use engine::{Action, Config, Phase, World};

fn basket(g: usize, amount: f64) -> Basket {
    let mut b = [0.0; N];
    b[g] = amount;
    b
}

#[test]
fn a_mix_reweights_shares_by_amount_and_merges_equal_chains() {
    let mut mine = Lots::dug(SALT, 3);
    mine.mix(SALT, 1.0, 3.0, &Lots::dug(SALT, 9));
    let shares: Vec<(usize, f64)> = mine.of(SALT).map(|l| (l.source, l.share)).collect();
    assert_eq!(shares, vec![(9, 0.75), (3, 0.25)], "3 of 4 came from tile 9");
    mine.mix(SALT, 4.0, 4.0, &Lots::dug(SALT, 3));
    let shares: Vec<(usize, f64)> = mine.of(SALT).map(|l| (l.source, l.share)).collect();
    assert_eq!(shares, vec![(3, 0.625), (9, 0.375)], "the same chain adds to its lot");
    mine.mix(OBSIDIAN, 0.0, 2.0, &Lots::dug(OBSIDIAN, 5));
    assert_eq!(mine.of(SALT).count(), 2, "another good leaves salt's lots alone");
    assert_eq!(mine.of(OBSIDIAN).count(), 1);
}

#[test]
fn a_store_holding_none_takes_only_what_arrives() {
    let mut mine = Lots::dug(SALT, 3);
    mine.mix(SALT, 0.0, 2.0, &Lots::dug(SALT, 7));
    let all: Vec<&Lot> = mine.of(SALT).collect();
    assert_eq!((all.len(), all[0].source, all[0].share), (1, 7, 1.0));
}

#[test]
fn lots_are_capped_by_the_smallest_and_chains_by_the_oldest_market() {
    let mut mine = Lots::default();
    for source in 0..(LOTS.most + 3) {
        mine.mix(SALT, source as f64, 1.0 + source as f64, &Lots::dug(SALT, source));
    }
    assert_eq!(mine.of(SALT).count(), LOTS.most);
    let sum: f64 = mine.of(SALT).map(|l| l.share).sum();
    assert!((sum - 1.0).abs() < 1e-9, "renormalised: {sum}");
    assert!(mine.of(SALT).all(|l| l.source >= 3), "the smallest, the oldest sources, went");
    let mut chain = Lots::dug(SALT, 1);
    for m in 0..(LOTS.chain as u32 + 3) {
        chain = chain.traded_at(SALT, m);
    }
    let l = chain.of(SALT).next().unwrap();
    assert_eq!((l.source, l.markets.len()), (1, LOTS.chain));
    assert_eq!(l.markets[0], 3, "the oldest markets went, never the source");
}

#[test]
fn a_clearing_hands_every_buyer_the_sellers_pooled_with_the_market_added() {
    // Row 0 and row 1 sell salt, 1 and 3 of it; row 2 buys all 4.
    let rows = vec![Lots::dug(SALT, 10), Lots::dug(SALT, 20), Lots::default()];
    let brought = vec![basket(SALT, 1.0), basket(SALT, 3.0), basket(GRAIN, 4.0)];
    let mut after = vec![[0.0; N]; 3];
    after[2][SALT] = 4.0;
    after[0][GRAIN] = 1.0;
    after[1][GRAIN] = 3.0;
    let kept = vec![[0.0; N]; 3];
    let out = cleared(&rows, &kept, &brought, &after, 77);
    let got: Vec<(usize, Vec<u32>, f64)> = out[2].of(SALT).map(|l| (l.source, l.markets.clone(), l.share)).collect();
    assert_eq!(got, vec![(20, vec![77], 0.75), (10, vec![77], 0.25)]);
    assert_eq!(out[0], rows[0], "a seller's shares stay as they are");
}

/// Every store's wares carry lots that sum to 1 and start at a source of the good.
fn check(w: &World) {
    let stores = |label: String, store: &Basket, lots: &Lots| {
        for g in (0..N).filter(|&g| !GOOD[g].eaten && store[g] > 1e-9) {
            let sum: f64 = lots.of(g).map(|l| l.share).sum();
            assert!(
                (sum - 1.0).abs() < 1e-9,
                "turn {}: {label} holds {} {} with lots summing {sum}",
                w.turn,
                store[g],
                GOODS[g]
            );
            for l in lots.of(g) {
                assert_eq!(w.tiles[l.source].source, Some(g), "turn {}: {label}'s lot starts at a source", w.turn);
                assert!(l.markets.len() <= LOTS.chain);
            }
        }
        assert!(lots.0.iter().filter(|l| l.good == 0).count() == 0, "only wares have lots");
        for g in 0..N {
            assert!(lots.of(g).count() <= LOTS.most, "{label}: at most LOTS.most lots a good");
        }
    };
    for c in w.clans.values() {
        stores(format!("clan {} store", c.id), &c.store, &c.lots);
        stores(format!("clan {} basket", c.id), &c.basket, &c.lots);
    }
    for c in w.civs.values() {
        stores(format!("civ {}", c.id), &c.store, &c.lots);
    }
    for s in w.settlements.values() {
        stores(format!("settlement {}", s.id), &s.store, &s.lots);
    }
    for u in w.units.values() {
        stores(format!("unit {}", u.id), &u.store, &u.lots);
    }
}

#[test]
fn every_store_keeps_lots_that_sum_to_one_and_start_at_a_source() {
    let mut w = Stage::retro("small", 2).unwrap().into_world();
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    let (mut held, mut traded) = (0, 0);
    let turns: u32 = std::env::var("LOTS_TURNS").ok().and_then(|t| t.parse().ok()).unwrap_or(60);
    for _ in 0..turns {
        w.run(1);
        check(&w);
        held += w.clans.values().filter(|c| !c.lots.is_empty()).count();
        let all = w.clans.values().map(|c| &c.lots).chain(w.civs.values().map(|c| &c.actor.lots));
        traded += all.flat_map(|l| l.0.iter()).filter(|l| !l.markets.is_empty()).count();
    }
    assert!(held > 0, "clans dug salt or obsidian and recorded it");
    assert!(traded > 0, "wares changed hands at a market and the lots say where");
}

#[test]
fn with_exchange_off_no_lot_is_written() {
    let mut w = World::new(Config { seed: 2, radius: 16, exchange: false, ..Config::default() });
    while w.phase == Phase::Pregame {
        w.run(1);
    }
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    w.run(40);
    assert!(w.clans.values().all(|c| c.lots.is_empty()));
    assert!(w.civs.values().all(|c| c.actor.lots.is_empty()));
    assert!(w.settlements.values().all(|s| s.lots.is_empty()));
}
