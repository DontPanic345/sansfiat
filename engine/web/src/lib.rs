use engine::data::{PREGAME, SIZES, TENDING};
use engine::map::Tile;
use engine::{stats, Action, Config, Phase, Save, World, FOODS, TERRAIN};
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

    /// What never changes after generation, as columns (one array per field, indexed by tile):
    /// `{radius, q, r, terrain, fertility, river, flow (-1 for none), lake, flood, zone
    /// (0 open, 1 valley, 2 sealed, 3 neck), pass, wild_grain, trail_cost, span}`. The page asks once per map.
    pub fn layout(&self) -> String {
        let t = &self.world.tiles;
        let col = |f: &dyn Fn(&Tile) -> serde_json::Value| -> Vec<serde_json::Value> { t.iter().map(f).collect() };
        serde_json::json!({
            "radius": self.world.config.radius,
            "q": col(&|t| t.q.into()),
            "r": col(&|t| t.r.into()),
            "terrain": col(&|t| t.terrain.into()),
            "fertility": col(&|t| t.fertility.into()),
            "river": col(&|t| t.river.into()),
            "flow": col(&|t| t.flow.map_or(-1, |f| f as i64).into()),
            "lake": col(&|t| u8::from(t.lake).into()),
            "flood": col(&|t| u8::from(t.flood).into()),
            "zone": col(&|t| (t.zone as u8).into()),
            "pass": col(&|t| u8::from(t.pass).into()),
            "wild_grain": col(&|t| t.wild_grain.into()),
            "trail_cost": col(&|t| t.trail_cost.into()),
            "span": col(&|t| t.span.into()),
        })
        .to_string()
    }

    /// What changes each turn, as JSON: `{turn, end, clans, markets, trails, tame, claims, summary}`.
    /// `trails` lists the tiles with one, `tame` is `[tile, turns]` where above zero, `claims` is
    /// `[tile, clan]` for every tile on a round.
    pub fn view(&self) -> String {
        let w = &self.world;
        let trails: Vec<usize> = (0..w.tiles.len()).filter(|&i| w.tiles[i].trail).collect();
        let tame: Vec<(usize, u32)> =
            w.tiles.iter().enumerate().filter(|(_, t)| t.tame > 0).map(|(i, t)| (i, t.tame)).collect();
        let claims: Vec<(usize, u32)> =
            w.tiles.iter().enumerate().filter_map(|(i, t)| t.claim.map(|c| (i, c))).collect();
        serde_json::json!({
            "turn": w.turn,
            "end": w.end,
            "clans": w.clans,
            "markets": w.markets,
            "trails": trails,
            "tame": tame,
            "claims": claims,
            "summary": stats::summary(w),
        })
        .to_string()
    }

    /// The phase: `"pregame"`, `"choose"` (founding is done, the player has yet to pick) or `"main"`.
    pub fn phase(&self) -> String {
        match self.world.phase {
            Phase::Pregame => "pregame",
            Phase::Choose => "choose",
            Phase::Main => "main",
        }
        .to_string()
    }

    /// Whether the pre-game has ended.
    pub fn ended(&self) -> bool {
        self.world.end.is_some()
    }

    /// One tile in full (stocks and caps included) as JSON.
    pub fn tile(&self, i: usize) -> String {
        serde_json::to_string(&self.world.tiles[i]).unwrap()
    }

    /// The `stats::Summary` as JSON.
    pub fn summary(&self) -> String {
        serde_json::to_string(&stats::summary(&self.world)).unwrap()
    }

    /// `stats::digest` of the world: 16 hex digits that match the native build's for the same seed.
    pub fn digest(&self) -> String {
        stats::digest(&self.world)
    }

    /// The events from index `from` on, as a JSON array; each has its wording in `text`.
    pub fn events(&self, from: usize) -> String {
        let w = &self.world;
        let rows: Vec<_> = w.events.iter().skip(from).map(|e| e.json(w)).collect();
        serde_json::to_string(&rows).unwrap()
    }

    /// `{config, log}` as JSON: enough to replay this world.
    pub fn save(&self) -> String {
        serde_json::to_string(&self.world.save()).unwrap()
    }
}

/// `[{name, color, land, move_cost}]` as JSON; `move_cost` is null where impassable.
#[wasm_bindgen]
pub fn terrain() -> String {
    let rows: Vec<_> = TERRAIN
        .iter()
        .map(|t| serde_json::json!({ "name": t.name, "color": t.color, "land": t.land, "move_cost": t.move_cost }))
        .collect();
    serde_json::to_string(&rows).unwrap()
}

/// `[{name, radius}]` as JSON: the map sizes a player can pick.
#[wasm_bindgen]
pub fn sizes() -> String {
    let rows: Vec<_> =
        SIZES.iter().map(|(name, radius)| serde_json::json!({ "name": name, "radius": radius })).collect();
    serde_json::to_string(&rows).unwrap()
}

/// The food names, in the order of every basket, as JSON.
#[wasm_bindgen]
pub fn goods() -> String {
    serde_json::to_string(&FOODS).unwrap()
}

/// Rules the page needs to read the world, as JSON: `{tame_turns, players}`.
#[wasm_bindgen]
pub fn rules() -> String {
    serde_json::json!({ "tame_turns": TENDING.tame_turns, "players": PREGAME.players }).to_string()
}
