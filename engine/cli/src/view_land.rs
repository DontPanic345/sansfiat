//! The land and the settlements in the text view: the led civ's claim and its order, its signposts
//! and settlements, and the rivals' settlements. Every number is read off the world.

use engine::market::total;
use engine::{claim, outlook, settle, World};

use crate::view::{tile, tiles};

/// The led civ's land: its claim's size and tiles, what holds it, its posture, and what it does to
/// the clans whose homes are on it.
pub(crate) fn claim_lines(w: &World, me: u32) -> String {
    let c = &w.civs[&me];
    let posture = [(c.pressed, "pressed"), (c.besieged, "besieged")]
        .iter()
        .filter(|(on, _)| *on)
        .map(|(_, name)| format!(", {name}"))
        .collect::<String>();
    let names: Vec<String> = c.claim.iter().map(|&t| tile(w, t)).collect();
    let mut out = format!(
        "land: {} tiles, your claim{posture}: {}\n  nobody stands on it: a rival army that ends its turn on a tile takes it, \
         with any tiles that cuts off from your capital and settlements, unless an army of yours holds that tile\n",
        c.claim.len(),
        names.join(" ")
    );
    if let Some(n) = w.notes.get(&me).and_then(|l| l.iter().rev().find(|n| n.text.starts_with("the claim "))) {
        out += &format!("  last claim order, turn {}: {}\n", n.turn, n.text);
    }
    out
}

/// The led civ's signposts and settlements, and the rivals' settlements: a signpost's tile, age and
/// worth; a settlement's store, the clans living on it and coming to its market, and its land.
pub(crate) fn settle_lines(w: &World, me: u32) -> String {
    let mut out = String::new();
    for s in &w.civs[&me].signposts {
        out += &format!(
            "signpost at {}: posted {} turn(s) ago, the tile would feed a daughter {:.0}% of her need; your clans that \
             divide or move on may go there, and the first to settle founds a settlement\n",
            tile(w, s.tile),
            w.turn - s.turn,
            settle::worth(w, s.tile) * 100.0
        );
        if let Some(why) = settle::idle(w, s.tile) {
            out += &format!("  it draws nobody: {why}\n");
        }
    }
    for p in settle::panels(w) {
        if p.civ != me {
            let name = w.civs.get(&p.civ).map_or("a rival", |c| c.name.as_str());
            out += &format!("{name}'s settlement at {}: store {:.0}\n", tile(w, p.tile), total(&p.store));
            continue;
        }
        let people: f64 = p.clans.iter().map(|id| w.clans[id].people).sum();
        out += &format!(
            "settlement at {} (market {}), founded turn {}: store {:.1} of {:.0} (levy {:.1} this turn), {} clan(s) of {:.0} \
             people live here, {} come to its market, land {} tile(s): {}\n  your claim holds where it joins it; a rival \
             army that ends its turn on it takes it with its store, unless an army of yours is there\n",
            tile(w, p.tile),
            p.market,
            p.founded,
            total(&p.store),
            p.cap,
            p.levied,
            p.clans.len(),
            people,
            p.traders.len(),
            p.land.len(),
            tiles(w, &p.land)
        );
        if let Some(lots) = crate::view_goods::settlement_lots(w, p.tile) {
            out += &crate::view_goods::store_chains(w, me, &p.store, lots);
        }
    }
    out
}

/// The claim order as the legal list offers it: the current claim's tiles, ready to copy, with what the
/// order does to the land; and, when the reach holds a claim, the whole order that brings a clan.
pub(crate) fn claim_orders(w: &World, me: u32) -> String {
    let c = &w.civs[&me];
    let ids = |list: &[usize]| list.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",");
    let homes_held = w.clans.values().any(|k| k.civ == Some(me) && claim::tile_civ(w, k.home) == Some(me));
    let clans = if homes_held {
        "; a tile you leave out is dropped, and a clan whose home is on it is no longer yours"
    } else {
        "; a tile you leave out is dropped"
    };
    let mut out = format!(
        "  {{\"type\":\"claim\",\"tiles\":[{}]}}  sets your land to exactly the tiles you list, with your capital and settlements: these are your {} tiles{clans}\n",
        ids(&c.claim),
        c.claim.len()
    );
    if let Some(to) = outlook::reach(w, me).claim {
        let mut all = c.claim.clone();
        all.extend(&to.tiles);
        all.sort_unstable();
        all.dedup();
        let (n, unit) = if to.tiles.len() == 1 { (1, "tile") } else { (to.tiles.len(), "tiles") };
        let more = if to.clans > 1 { format!(" and {} more", to.clans - 1) } else { String::new() };
        out += &format!(
            "  {{\"type\":\"claim\",\"tiles\":[{}]}}  the claim in reach: adds {n} {unit}, {}, and brings clan {}{more}\n",
            ids(&all),
            tiles(w, &to.tiles),
            to.clan
        );
    }
    out
}
