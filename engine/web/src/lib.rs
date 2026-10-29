use engine::data::{CIV, PREGAME, SIZES, TENDING, UNIT};
use engine::map::Tile;
use engine::{civs, stats, units, Action, Basket, Config, Phase, Save, World, FOODS, TERRAIN};
use std::collections::BTreeMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sim {
    world: World,
}

#[wasm_bindgen]
impl Sim {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, radius: i32) -> Sim {
        Sim { world: World::new(Config { seed, radius, army: true }) }
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

    /// What changes each turn, as JSON: `{turn, end, phase, player, clans, markets, trails, tame, claims,
    /// wear, civs, units, clan_civ, summary}`. `trails` lists the tiles with one, `tame` is `[tile, turns]`
    /// where above zero, `claims` is `[tile, clan]` for every tile in a territory, `wear` is `[tile, worn,
    /// unworn]` for every tile with some food worn (each food's wear as a share, `0..=1`, and its cap before
    /// wear, in `goods()` order). `phase` is as `phase()`;
    /// `player` is the civ id the player leads, else null. `civs` is `{id: {id, name, color, market,
    /// capital, founded, store (a basket, in `goods()` order), lost, people, clans, territory (the
    /// capital's tiles), work (its work tiles), army (its `units::Counters`)}}`. `units` is the camps,
    /// `{id: camp}`, a camp being its engine fields (`kind`, `people` (warriors), `home`, `store` (the
    /// pack, a basket), `dest`, `banked`, `hungry`, `auto_forage`, `auto_supply`, `routes`, ...) and
    /// `tile`, `pack_cap` (the pack's food when full), `food_turns` (turns the pack feeds it), `line`
    /// (`{path, cost, carry}`: the path after its tile to the capital, its cost, and the food it carries
    /// a turn with all its labour; carry null is without limit, at the capital; `line` itself null when
    /// there is no line) and `cut_off`. `clan_civ` is `[clan, civ]` for every clan with a civ.
    pub fn view(&self) -> String {
        let w = &self.world;
        let trails: Vec<usize> = (0..w.tiles.len()).filter(|&i| w.tiles[i].trail).collect();
        let tame: Vec<(usize, u32)> =
            w.tiles.iter().enumerate().filter(|(_, t)| t.tame > 0).map(|(i, t)| (i, t.tame)).collect();
        let claims: Vec<(usize, u32)> =
            w.tiles.iter().enumerate().filter_map(|(i, t)| t.claim.map(|c| (i, c))).collect();
        let wear: Vec<(usize, Basket, Basket)> = w
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| t.wear.iter().any(|&x| x > 0.0))
            .map(|(i, t)| (i, t.wear, t.unworn))
            .collect();
        let civ_rows: BTreeMap<u32, serde_json::Value> = w
            .civs
            .values()
            .map(|c| {
                let mine = w.clans.values().filter(|k| k.civ == Some(c.id));
                let row = serde_json::json!({
                    "id": c.id,
                    "name": c.name,
                    "color": c.color,
                    "market": c.market,
                    "capital": c.capital,
                    "founded": c.founded,
                    "store": c.store,
                    "lost": c.lost,
                    "people": mine.clone().map(|k| k.people).sum::<f64>(),
                    "clans": mine.count(),
                    "territory": civs::territory(w, c.id),
                    "work": civs::work_tiles(w, c.id),
                    "army": c.army,
                });
                (c.id, row)
            })
            .collect();
        // Each camp as it is, with its tile, its line to the capital (path, cost, carry a turn; null
        // carry is without limit) and whether it is cut off.
        let units_json: BTreeMap<u32, serde_json::Value> = w
            .units
            .values()
            .map(|u| {
                let mut row = serde_json::to_value(u).unwrap();
                let need = u.people * UNIT.food;
                let held: f64 = u.store.iter().sum();
                row["tile"] = u.tile().into();
                row["pack_cap"] = units::pack_cap(u.people).into();
                row["food_turns"] = if need > 0.0 { held / need } else { 0.0 }.into();
                row["line"] = units::line(w, u.id)
                    .map(|(path, cost, carry)| serde_json::json!({ "path": path, "cost": cost, "carry": carry }))
                    .into();
                row["cut_off"] = units::cut_off(w, u.id).into();
                (u.id, row)
            })
            .collect();
        let clan_civ: Vec<(u32, u32)> = w.clans.values().filter_map(|c| c.civ.map(|civ| (c.id, civ))).collect();
        serde_json::json!({
            "turn": w.turn,
            "end": w.end,
            "phase": self.phase(),
            "player": w.player,
            "clans": w.clans,
            "markets": w.markets,
            "trails": trails,
            "tame": tame,
            "claims": claims,
            "wear": wear,
            "civs": civ_rows,
            "units": units_json,
            "clan_civ": clan_civ,
            "summary": stats::summary(w),
        })
        .to_string()
    }

    /// Whether `action` (JSON, as for `act`) would go through, without applying it: `""` if so,
    /// else why not.
    pub fn check(&self, action: &str) -> String {
        match serde_json::from_str::<Action>(action) {
            Ok(action) => self.world.validate(&action).err().unwrap_or_default(),
            Err(e) => e.to_string(),
        }
    }

    /// The cards for the pick, one per civ, as JSON: `[{civ, name, color, capital, q, r, clans, people,
    /// hearths, terrain: [[name, share]], rival_steps (null with one civ), rival_by_land}]`.
    pub fn pick(&self) -> String {
        serde_json::to_string(&civs::cards(&self.world)).unwrap()
    }

    /// A unit's route to its `dest`, as JSON `{path: [tiles after its own], cost, turns}`; `null` when
    /// it has no destination (or no route).
    pub fn route(&self, unit: u32) -> String {
        let w = &self.world;
        let json = units::route(w, unit).map(|(path, cost)| {
            let u = &w.units[&unit];
            let turns = units::turns(w, &path, u.tile(), u.banked);
            serde_json::json!({ "path": path, "cost": cost, "turns": turns })
        });
        serde_json::to_string(&json).unwrap()
    }

    /// The cheapest route a unit could take from tile `from` to tile `to`, for a hover preview, as
    /// `route` gives it; `null` when there is none.
    pub fn path(&self, from: usize, to: usize) -> String {
        let w = &self.world;
        let n = w.tiles.len();
        let json = if from < n && to < n { units::path_between(w, from, to) } else { None }.map(|(path, cost)| {
            let turns = units::turns(w, &path, from, 0.0);
            serde_json::json!({ "path": path, "cost": cost, "turns": turns })
        });
        serde_json::to_string(&json).unwrap()
    }

    /// The civ whose land tile `i` is (see `civs::tile_civ`), or -1 for none.
    pub fn tile_civ(&self, i: usize) -> i32 {
        if i >= self.world.tiles.len() {
            return -1;
        }
        civs::tile_civ(&self.world, i).map_or(-1, |c| c as i32)
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

/// Rules the page needs to read the world, as JSON: `{tame_turns, players, unit_labour, unit_food,
/// hungry_turns, store_cap, territory}`.
#[wasm_bindgen]
pub fn rules() -> String {
    serde_json::json!({
        "tame_turns": TENDING.tame_turns,
        "players": PREGAME.players,
        "unit_labour": UNIT.labour,
        "unit_food": UNIT.food,
        "hungry_turns": UNIT.hungry_turns,
        "store_cap": CIV.store_cap,
        "territory": CIV.territory,
    })
    .to_string()
}
