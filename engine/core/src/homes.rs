//! Where clans live. A clan's home is a tile of its territory, not a claim: a home may be held by
//! another clan or by nobody, and many clans may live on one tile. A clan gathers only the tiles it
//! claims (`Tile.claim`), and writes its claim only on tiles that are unheld or its own. `homes` is
//! the index of home tiles, derived from `World.clans` each time it is asked and never saved.

use std::collections::BTreeMap;

use crate::world::World;

/// Each home tile and the clans living on it, ids ascending.
pub fn homes(w: &World) -> BTreeMap<usize, Vec<u32>> {
    let mut index: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    for c in w.clans.values() {
        index.entry(c.home).or_default().push(c.id);
    }
    index
}

/// Writes clan `id`'s claim on each tile of `territory` that is unheld or already its own. A tile
/// another clan holds keeps its claim: a home may stand on it.
pub fn claim_land(w: &mut World, id: u32, territory: &[usize]) {
    for &t in territory {
        let tile = &mut w.tiles[t];
        if tile.claim.is_none_or(|held| held == id) {
            tile.claim = Some(id);
        }
    }
}

/// Whether clan `id` gathers tile `t`: it holds the tile's claim.
pub fn gathers(w: &World, id: u32, t: usize) -> bool {
    w.tiles[t].claim == Some(id)
}
