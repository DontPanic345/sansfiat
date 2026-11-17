//! Sources: the tiles salt and obsidian come from (`docs/design/goods.md`). A source is a feature of
//! its tile, fixed when the map is made: salt on some coast and marsh tiles, obsidian on a few hills.
//! A clan that gathers a source's tile gives the source job a share of its days (`fields::jobs`).

use crate::data::*;
use crate::map::Tile;
use crate::rng::rand_index;
use crate::world::World;

/// Mixed into the seed for the sources' own stream, so placing them moves nothing else on the map.
const SOURCE_SALT: u32 = 0x9e3779b9;

/// The terrains a good's sources lie on, and how many a small map has. A good with no row here has no
/// source.
pub fn ground(good: usize) -> Option<(&'static [usize], usize)> {
    match good {
        SALT => Some((&[COAST, MARSH], SOURCE.salt)),
        OBSIDIAN => Some((&[HILLS], SOURCE.obsidian)),
        _ => None,
    }
}

/// Places every good's sources on `tiles`: its count for the map's size (`scaled`), each on a tile of
/// its terrain picked from the sources' stream, one source to a tile.
pub fn place(tiles: &mut [Tile], seed: u32) {
    let mut rng = seed ^ SOURCE_SALT;
    for good in 0..N {
        let Some((terrains, per_small)) = ground(good) else { continue };
        let mut free: Vec<usize> =
            (0..tiles.len()).filter(|&i| terrains.contains(&tiles[i].terrain) && tiles[i].source.is_none()).collect();
        for _ in 0..scaled(per_small, tiles.len()) {
            if free.is_empty() {
                break;
            }
            let i = free.swap_remove(rand_index(&mut rng, free.len()));
            tiles[i].source = Some(good);
        }
    }
}

/// The tiles with a source of `good`, in tile order.
pub fn of(w: &World, good: usize) -> Vec<usize> {
    (0..w.tiles.len()).filter(|&i| w.tiles[i].source == Some(good)).collect()
}

/// Hex steps from `tile` to the nearest source of `good`; `None` on a map with none.
pub fn steps_from(w: &World, good: usize, tile: usize) -> Option<i32> {
    let h = w.tiles[tile].hex();
    of(w, good).into_iter().map(|s| w.tiles[s].hex().distance(h)).min()
}
