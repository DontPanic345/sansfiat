//! The text view's goods: a store's wares as chains (where each lot was dug and the markets it changed hands at),
//! and a unit's pack wares.

use engine::data::{GOOD, N};
use engine::lots::Lots;
use engine::market::Basket;
use engine::{settle, Unit, World, GOODS};

/// Where a market is, for a chain: the holder's capital market, a settlement's, or a festival by its hex.
fn market_name(w: &World, civ: u32, mid: u32) -> String {
    let Some(m) = w.markets.get(&mid) else { return format!("market {mid}") };
    if w.civs.get(&civ).is_some_and(|c| c.market == mid) {
        "the capital's market".to_string()
    } else if settle::of(w, civ).any(|s| s.market == mid) {
        "a settlement's market".to_string()
    } else {
        format!("the festival at {}", w.at(m.tile))
    }
}

/// A store's wares as chains, one line a ware, its lots the largest first: `obsidian 4.2: 60% from (3, 9) via the
/// festival at (5, 7), the capital's market`. A lot never traded is dug where it stands. Nothing with no lots.
pub(crate) fn store_chains(w: &World, civ: u32, store: &Basket, lots: &Lots) -> String {
    let mut out = String::new();
    for g in (0..N).filter(|&g| !GOOD[g].eaten && store[g] > 1e-9) {
        let mut of: Vec<_> = lots.of(g).collect();
        if of.is_empty() {
            continue;
        }
        of.sort_by(|a, b| b.share.total_cmp(&a.share));
        let parts: Vec<String> = of
            .iter()
            .map(|l| {
                let from = w.at(l.source);
                if l.markets.is_empty() {
                    format!("{:.0}% dug at {from}", l.share * 100.0)
                } else {
                    let chain: Vec<String> = l.markets.iter().map(|&m| market_name(w, civ, m)).collect();
                    format!("{:.0}% from {from} via {}", l.share * 100.0, chain.join(", "))
                }
            })
            .collect();
        out += &format!("  {} {:.1}: {}\n", GOODS[g], store[g], parts.join("; "));
    }
    out
}

/// A unit's pack wares, for its line: `, wares salt 0.50 and obsidian 0.20`, or nothing.
pub(crate) fn pack_wares(u: &Unit) -> String {
    let wares: Vec<String> = (0..N)
        .filter(|&g| !GOOD[g].eaten && u.store[g] > 1e-9)
        .map(|g| format!("{} {:.2}", GOODS[g], u.store[g]))
        .collect();
    if wares.is_empty() {
        String::new()
    } else {
        format!(", wares {}", wares.join(" and "))
    }
}

/// The settlement of `civ` on `tile`, for its store's chains (`settle::panels` has no lots).
pub(crate) fn settlement_lots(w: &World, tile: usize) -> Option<&Lots> {
    w.settlements.values().find(|s| s.tile == tile).map(|s| &s.lots)
}
