use engine::{Config, World, GOODS, TERRAIN};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sim {
    world: World,
}

#[wasm_bindgen]
impl Sim {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, radius: i32, clans: usize, markets: usize, reach: i32) -> Sim {
        Sim { world: World::new(Config { seed, radius, clans, markets, reach }) }
    }

    pub fn step(&mut self, turns: u32) {
        self.world.run(turns);
    }

    /// The whole world as JSON.
    pub fn state(&self) -> String {
        serde_json::to_string(&self.world).unwrap()
    }
}

#[wasm_bindgen]
pub fn goods() -> Vec<String> {
    GOODS.iter().map(|g| g.to_string()).collect()
}

/// `[{name, color, land, yields}]` as JSON.
#[wasm_bindgen]
pub fn terrain() -> String {
    let rows: Vec<_> = TERRAIN
        .iter()
        .map(|t| serde_json::json!({ "name": t.name, "color": t.color, "land": t.land, "yields": t.yields }))
        .collect();
    serde_json::to_string(&rows).unwrap()
}
