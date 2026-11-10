//! The festival search (`reach::settle`, `markets::festival_costs`): on a map with trails and hills,
//! every festival in reach gets the cost and the route `cost_map` and `land::path` give, including a
//! festival further than `most ÷ DAY` hex steps along a trail, and a search reused from one home to
//! the next gives the same answers.

use engine::data::*;
use engine::hex::Hex;
use engine::land::{cost_map, flat_cost, passable, path};
use engine::map::index;
use engine::markets::{festival_costs, raise};
use engine::reach::Scratch;
use engine::stage::Stage;
use engine::World;

const RADIUS: i32 = 10;

/// Plains with hills and forest in a pattern, a trail east along row 0 and another north-west from
/// the centre, and festivals on the trails' far ends and in the middle of the open ground.
fn trailed() -> (World, Vec<usize>) {
    let mut stage = Stage::flat(RADIUS);
    let tile = |q: i32, r: i32| index(RADIUS, Hex { q, r }).unwrap();
    for q in -RADIUS + 1..RADIUS {
        for r in -RADIUS + 1..RADIUS {
            let Some(i) = index(RADIUS, Hex { q, r }) else { continue };
            let terrain = match (q * 3 + r * 5).rem_euclid(11) {
                0 | 1 => HILLS,
                2 => FOREST,
                _ => PLAINS,
            };
            stage.paint(i, terrain, None, None);
        }
    }
    let mut w = stage.into_world();
    for i in 0..w.tiles.len() {
        let terrain = w.tiles[i].terrain;
        let hidden = match terrain {
            PLAINS | COAST => flat_cost(terrain),
            _ => Some(TERRAIN[terrain].move_cost.unwrap_or(DAY) * 0.8),
        };
        w.tiles[i].trail_cost = hidden;
    }
    for q in 0..RADIUS {
        let i = tile(q, 0);
        w.tiles[i].trail = true;
    }
    for k in 1..RADIUS {
        let i = tile(-k, k);
        w.tiles[i].trail = true;
    }
    let mut festivals = vec![tile(-1, 0), tile(8, 0), tile(-8, 2), tile(3, -3)];
    festivals.dedup();
    for &f in &festivals {
        raise(&mut w, f);
    }
    (w, festivals)
}

#[test]
fn the_festival_search_gives_cost_map_costs_and_land_paths() {
    let (w, festivals) = trailed();
    let mut s = Scratch::new(&w);
    let most = MARKET.reach_walk / 2.0;
    let homes = [index(RADIUS, Hex { q: 0, r: 0 }).unwrap(), index(RADIUS, Hex { q: -3, r: 2 }).unwrap()];
    for &home in &homes {
        let near = cost_map(&w, &[home], most, |i| passable(&w, i));
        let got = festival_costs(&w, &mut s, home);
        let in_reach: Vec<usize> = festivals.iter().copied().filter(|t| near.contains_key(t)).collect();
        assert!(!in_reach.is_empty(), "some festival is in reach of {home}");
        assert_eq!(got.len(), in_reach.len(), "every festival in reach, and none else, from {home}");
        for f in &got {
            let tile = w.markets[&f.market].tile;
            assert_eq!(f.cost, near[&tile], "the cost to festival {} from {home}", f.market);
            let (route, cost) = path(&w, home, tile, |i| passable(&w, i)).unwrap();
            assert_eq!((&f.path, f.cost), (&route, cost), "the route to festival {} from {home}", f.market);
        }
    }
    let far = festivals.iter().map(|&t| engine::land::steps(&w, homes[0], t)).max().unwrap();
    assert!(
        far as f64 > most / DAY,
        "the map has a festival further than most ÷ DAY steps, which is in reach by trail"
    );
}
