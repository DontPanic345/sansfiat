use engine::{Action, Config, Save, World, TERRAIN};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sim {
    world: World,
}

#[wasm_bindgen]
impl Sim {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, radius: i32) -> Sim {
        Sim { world: World::new(Config { seed, radius }) }
    }

    /// Rebuilds the world from a save's JSON.
    pub fn load(save: &str) -> Result<Sim, String> {
        let save: Save = serde_json::from_str(save).map_err(|e| e.to_string())?;
        Ok(Sim { world: World::replay(&save)? })
    }

    /// Applies an action given as JSON, such as `{"type":"endTurn"}`. `Err` says why not.
    pub fn act(&mut self, action: &str) -> Result<(), String> {
        let action: Action = serde_json::from_str(action).map_err(|e| e.to_string())?;
        self.world.act(action)
    }

    /// The whole world as JSON.
    pub fn state(&self) -> String {
        serde_json::to_string(&self.world).unwrap()
    }

    /// `{config, log}` as JSON: enough to replay this world.
    pub fn save(&self) -> String {
        serde_json::to_string(&self.world.save()).unwrap()
    }
}

/// `[{name, color, land}]` as JSON.
#[wasm_bindgen]
pub fn terrain() -> String {
    let rows: Vec<_> =
        TERRAIN.iter().map(|t| serde_json::json!({ "name": t.name, "color": t.color, "land": t.land })).collect();
    serde_json::to_string(&rows).unwrap()
}
