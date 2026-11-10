//! Towns: a festival whose tile has `TOWN.homes` or more homes on it reads as a town. The reading
//! changes only `Market.town`, which the `markets` step keeps and which no rule reads; `TownFormed`
//! goes to the feed when a town forms, and the market gets a note when it stops being one.

use serde::Serialize;

use crate::data::TOWN;
use crate::events::{Event, EventKind};
use crate::homes;
use crate::world::World;

/// A festival whose tile reads as a town: the tile, the market and the clans living on the tile, ids
/// ascending.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Town {
    pub tile: usize,
    pub market: u32,
    pub clans: Vec<u32>,
}

/// Serde's skip for `Market.town` while it is false.
pub fn is_false(b: &bool) -> bool {
    !*b
}

/// Each festival whose tile has `TOWN.homes` or more homes, in market id order.
pub fn towns(w: &World) -> Vec<Town> {
    let index = homes::homes(w);
    w.markets
        .values()
        .filter_map(|m| {
            let clans = index.get(&m.tile)?;
            (clans.len() >= TOWN.homes).then(|| Town { tile: m.tile, market: m.id, clans: clans.clone() })
        })
        .collect()
}

/// The `markets` step's reading: a market whose tile first reads as a town turns `Market.town` on
/// and emits `TownFormed` with the clans there; one that stops reading as one turns it off, with a
/// note on the market.
pub fn check(w: &mut World) {
    let now = towns(w);
    let mids: Vec<u32> = w.markets.keys().copied().collect();
    for mid in mids {
        let was = w.markets[&mid].town;
        let town = now.iter().find(|t| t.market == mid);
        match (was, town) {
            (false, Some(t)) => {
                let (tile, count) = (t.tile, t.clans.len());
                w.markets.get_mut(&mid).unwrap().town = true;
                w.emit(Event::new(EventKind::TownFormed, tile).market(mid).amount(count as f64));
            }
            (true, None) => {
                let tile = w.markets[&mid].tile;
                let count = homes::homes(w).get(&tile).map_or(0, Vec::len);
                w.markets.get_mut(&mid).unwrap().town = false;
                w.note(mid, format!("is no longer a town: {count} clans live here"));
            }
            _ => {}
        }
    }
}
