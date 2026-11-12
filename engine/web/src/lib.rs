use engine::data::{CIV, FIELD, FIGHT, FIGHTERS, LEANING, NOTES, PREGAME, SIZES, TENDING, TRAIL, UNIT};
use engine::map::Tile;
use engine::over::{self, Why};
use engine::script::Lead;
use engine::stage::{Stage, StageSpec};
use engine::units::UnitKind;
use engine::{civs, fields, markets, stats, units, Action, Basket, Config, Phase, Save, World, FOODS, TERRAIN};
use std::collections::BTreeMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sim {
    world: World,
    lead: Lead,
    staged: bool,
}

#[wasm_bindgen]
impl Sim {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32, radius: i32) -> Sim {
        Sim { world: World::new(Config { seed, radius, ..Config::default() }), lead: Lead::default(), staged: false }
    }

    /// Rebuilds the world from a save's JSON.
    pub fn load(save: &str) -> Result<Sim, String> {
        let save: Save = serde_json::from_str(save).map_err(|e| e.to_string())?;
        Ok(Sim { world: World::replay(&save)?, lead: Lead::default(), staged: false })
    }

    /// The stage `spec` (JSON, as `engine::stage::StageSpec` reads it) describes: a world placed on a
    /// known map, led by its lead. A staged game is not a save, so the page does not autosave it.
    pub fn stage(spec: &str) -> Result<Sim, String> {
        let spec: StageSpec = serde_json::from_str(spec).map_err(|e| e.to_string())?;
        let world = Stage::from_spec(&spec)?.into_world();
        Ok(Sim { world, lead: Lead::default(), staged: true })
    }

    /// Whether this game was built by `stage`, not played from a seed or loaded from a save.
    pub fn staged(&self) -> bool {
        self.staged
    }

    /// Applies an action given as JSON, such as `{"type":"endTurn"}`, or a unit's order, such as
    /// `{"type":"order","civ":1,"order":{"type":"move","unit":12,"to":5}}`. `Err` says why not.
    pub fn act(&mut self, action: &str) -> Result<(), String> {
        let action: Action = serde_json::from_str(action).map_err(|e| e.to_string())?;
        self.world.act(action)
    }

    /// The scripted game, for the cross-build check and scripts: gives the led civ's orders for the
    /// turn, as `core/src/script.rs` does (the caller then ends the turn).
    pub fn lead_orders(&mut self) {
        self.lead.orders(&mut self.world);
    }

    /// The whole world as JSON.
    pub fn state(&self) -> String {
        serde_json::to_string(&self.world).unwrap()
    }

    /// What never changes after generation, as columns (one array per field, indexed by tile):
    /// `{radius, q, r, terrain, fertility, river, flow (-1 for none), lake, flood, zone
    /// (0 open, 1 valley, 2 sealed, 3 neck), pass, wild_grain, trail_cost, span, ground (the field days
    /// the tile takes a turn, `fields::ground`)}`. The page asks once per map.
    pub fn layout(&self) -> String {
        let t = &self.world.tiles;
        let col = |f: &dyn Fn(&Tile) -> serde_json::Value| -> Vec<serde_json::Value> { t.iter().map(f).collect() };
        let ground: Vec<f64> = (0..t.len()).map(|i| fields::ground(&self.world, i)).collect();
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
            "zone": col(&|t| (t.country as u8).into()),
            "pass": col(&|t| u8::from(t.pass).into()),
            "wild_grain": col(&|t| t.wild_grain.into()),
            "trail_cost": col(&|t| t.trail_cost.into()),
            "span": col(&|t| t.span.into()),
            "ground": ground,
        })
        .to_string()
    }

    /// What changes each turn, as JSON: `{turn, end, phase, player, clans, markets, trails, trail_work,
    /// revealed, tame, claims, wear, civs, units, clan_civ, summary}`. `trails` lists the tiles with one,
    /// `trail_work` is `[tile, labour, need]` for each tile with a trail under way (`need` its labour to
    /// finish), `revealed` the led civ's revealed tiles (empty when it leads none), `tame` is `[tile, turns]`
    /// where above zero, `claims` is `[tile, clan]` for every tile in a clan's territory, `wear` is `[tile, worn,
    /// unworn]` for every tile with some food worn (each food's wear as a share, `0..=1`, and its cap before
    /// wear, in `goods()` order). `phase` is as `phase()`;
    /// `player` is the civ id the player leads, else null. `civs` is `{id: {id, name, color, market,
    /// capital, founded, store (a basket, in `goods()` order), lost, people, clans, claim (the civ's claimed
    /// tiles, sorted, the capital among them), land (its held tiles: its whole claim),
    /// army (its `units::Counters`), keep (the food it keeps
    /// in force, `civs::keep`), keep_set
    /// (the food the player set it to, null for the default), gift
    /// (the most it gives a turn), posted (food posted to its market last turn), given (food given
    /// last turn), joined (clans joined since founding), gifted (food given since founding), living
    /// (`{gathering, farming, army}`: its people by what they do, gathering over their land (Σ people ×
    /// (1 − fields) over its clans), working fields (Σ people × fields over its clans), or in its units
    /// (the people of all its units: camps and scouts))}}`. Each
    /// clan in `clans` carries `leaning` (civ id to leaning; empty for a clan with a civ), `routes`
    /// (its festival route: `[{to, path, cost}]`, the path from home, so its walk is home then the path),
    /// `fields` (its share of its days on fields) and `plots` (`[[tile, days, grain]]`, its field tiles
    /// this turn, from `fields::plots`).
    /// `markets` is `{id: market}`, each market with `clans` (the clans whose festival it is).
    /// `rules()` gives `join`, the leaning a clan joins a civ at, for the tints. `units` is the camps,
    /// `{id: camp}`, a camp being its engine fields (`kind`, `people` (warriors), `home`, `store` (the
    /// pack, a basket), `dest`, `banked`, `hungry`, `auto_forage`, `auto_supply`, `routes`, ...) and
    /// `tile`, `pack_cap` (the pack's food when full), `food_turns` (turns the pack feeds it), `line`
    /// (`{path, cost, carry}`: the path after its tile to the capital, its cost, and the food it carries
    /// a turn with all its labour; carry null is without limit, at the capital; `line` itself null when
    /// there is no line), `cut_off`, `territory` (its zone, sorted, its tile in it), `held` (the zone's
    /// tiles its warriors hold) with `guard` (each one's guard share after its walk), `zone_cap` (the
    /// most tiles its zone may have) and `cut_at` (`{path, at, by}` when a rival's held ground cuts the
    /// line: the line it would have without held ground, the first tile on it a rival holds and that
    /// civ; else null). A scout also has `plan` when it has a route: `{known, possible, candidates}`,
    /// where `known` and `possible` are `{path, cost}` (path from the route's start, null
    /// when there is none) and `candidates` the unrevealed tiles the search would reveal next. A scout's
    /// `search` is null without a route, else `{next, left, known}`: `next` the tiles the search will try
    /// in its order, `left` the unrevealed mountains it leaves (`{tile, through}`, cheapest first, ten at
    /// most), and `known` the known route's cost. A scout's `dest` is the tile it marches to with no route.
    /// `clan_civ` is `[clan, civ]` for every clan with a civ. `over` is null in a game still going, else
    /// `{winner, why: "conquest" | "limit" | "fell", by, turn}`: `by` the civ that took the led civ's capital
    /// (for "fell", else null), `turn` the world's turn the game ended on. `limit` is `{turns, at}`: the turn
    /// limit after the pick and the world's turn it falls on (null before the pick). `standing` is
    /// `[[turn, {civ: share}]]`, each civ's Standing at the pick and after each turn since, by the world's turn.
    /// Events are not in the view: they come from `events`, each with `alert` and `amount` as the engine gives them.
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
        let parts = stats::standing_parts(w);
        let civ_rows: BTreeMap<u32, serde_json::Value> = w
            .civs
            .values()
            .map(|c| {
                let part = parts.get(&c.id).copied().unwrap_or_default();
                let mine = w.clans.values().filter(|k| k.civ == Some(c.id));
                let (gathering, farming) = mine
                    .clone()
                    .fold((0.0, 0.0), |(g, f), k| (g + k.people * (1.0 - k.fields), f + k.people * k.fields));
                let army: f64 = w.units.values().filter(|u| u.civ == c.id).map(|u| u.people).sum();
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
                    "claim": c.claim,
                    "land": civs::land(w, c.id),
                    "army": c.army,
                    "keep": civs::keep(w, c.id),
                    "keep_set": c.keep,
                    "gift": c.gift,
                    "posted": c.posted,
                    "given": c.given,
                    "joined": c.joined,
                    "gifted": c.gifted,
                    "standing": part.clans + part.land,
                    "standing_clans": part.clans,
                    "standing_land": part.land,
                    "pressed": c.pressed,
                    "besieged": c.besieged,
                    "fallen": c.fallen,
                    "living": { "gathering": gathering, "farming": farming, "army": army },
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
                row["zone_cap"] = units::zone_cap(u).into();
                row["cut_at"] = units::cut_at(w, u.id)
                    .map(|(path, at, by)| serde_json::json!({ "path": path, "at": at, "by": by }))
                    .into();
                if let (UnitKind::Scout, Some((from, to))) = (u.kind(), u.scout().and_then(|s| s.route)) {
                    let plan = units::route_plan(w, u.civ, from, to);
                    let way = |p: &Option<(Vec<usize>, f64)>| p.as_ref().map(|(path, cost)| serde_json::json!({ "path": path, "cost": cost }));
                    row["plan"] = serde_json::json!({ "known": way(&plan.known), "possible": way(&plan.possible), "candidates": plan.candidates });
                }
                if u.kind() == UnitKind::Scout {
                    let sc = u.scout();
                    row["wander"] = sc.and_then(|s| s.wander).into();
                    row["hunger"] = sc
                        .and_then(|s| s.hunger)
                        .map(|h| serde_json::json!({ "forage": h.forage, "line": h.line, "eats": h.eats, "cost": h.cost, "carry": h.carry }))
                        .into();
                    row["search"] = units::search_view(w, u.id)
                        .map(|v| {
                            let left: Vec<_> = v.left.iter().map(|&(tile, through)| serde_json::json!({ "tile": tile, "through": through })).collect();
                            serde_json::json!({ "next": v.next, "left": left, "known": v.known })
                        })
                        .into();
                }
                (u.id, row)
            })
            .collect();
        // The led civ's revealed tiles, and the trails under way (labour in, labour needed).
        let revealed: Vec<usize> =
            w.player.and_then(|p| w.civs.get(&p)).map_or(Vec::new(), |c| c.revealed.iter().copied().collect());
        let trail_work: Vec<(usize, f64, f64)> = w
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| t.trail_work > 0.0 && !t.trail)
            .filter_map(|(i, t)| {
                TRAIL.work.iter().find(|&&(k, _)| k == t.terrain).map(|&(_, need)| (i, t.trail_work, need))
            })
            .collect();
        let picked = w.picked.unwrap_or(0);
        let over = match w.phase {
            Phase::Over(o) => {
                let (why, by) = match o.why {
                    Why::Conquest => ("conquest", None),
                    Why::Limit => ("limit", None),
                    Why::Fell { by } => ("fell", Some(by)),
                };
                serde_json::json!({ "winner": o.winner, "why": why, "by": by, "turn": picked + o.turn })
            }
            _ => serde_json::Value::Null,
        };
        let limit = over::ends_at(w).map(|at| serde_json::json!({ "turns": over::limit(w), "at": at }));
        let standing: Vec<(u32, &BTreeMap<u32, f64>)> =
            w.standing.iter().enumerate().map(|(k, s)| (picked + k as u32, s)).collect();
        let clan_civ: Vec<(u32, u32)> = w.clans.values().filter_map(|c| c.civ.map(|civ| (c.id, civ))).collect();
        let clans: BTreeMap<u32, serde_json::Value> = w
            .clans
            .values()
            .map(|k| {
                let mut row = serde_json::to_value(k).unwrap();
                row["fields"] = k.fields.into();
                row["plots"] = serde_json::to_value(fields::plots(w, k)).unwrap();
                (k.id, row)
            })
            .collect();
        // Each festival with the clans that go to it.
        let by_market = markets::by_market(w);
        let markets: BTreeMap<u32, serde_json::Value> = w
            .markets
            .values()
            .map(|m| {
                let mut row = serde_json::to_value(m).unwrap();
                row["clans"] = by_market.get(&m.id).cloned().unwrap_or_default().into();
                (m.id, row)
            })
            .collect();
        // Each town: its festival's tile and market, and the clans living on the tile.
        let towns: Vec<serde_json::Value> = engine::towns::towns(w)
            .iter()
            .map(|t| serde_json::json!({ "tile": t.tile, "market": t.market, "clans": t.clans }))
            .collect();
        serde_json::json!({
            "turn": w.turn,
            "end": w.end,
            "phase": self.phase(),
            "player": w.player,
            "clans": clans,
            "markets": markets,
            "towns": towns,
            "trails": trails,
            "trail_work": trail_work,
            "revealed": revealed,
            "tame": tame,
            "claims": claims,
            "wear": wear,
            "civs": civ_rows,
            "units": units_json,
            "clan_civ": clan_civ,
            "over": over,
            "limit": limit,
            "standing": standing,
            "contested": w.contested,
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

    /// A unit's line as if it stood on `tile`, as JSON `{cost, carry, eats}`: the cost of the walk to the
    /// capital from there, the food the line carries a turn (null without limit, at the capital), and
    /// what the unit's people eat a turn. `null` when no line reaches the capital from `tile`.
    #[wasm_bindgen(js_name = lineFrom)]
    pub fn line_from(&self, unit: u32, tile: usize) -> String {
        let w = &self.world;
        let eats = w.units.get(&unit).map_or(0.0, |u| u.people * UNIT.food);
        let json = units::line_from(w, unit, tile)
            .map(|(cost, carry)| serde_json::json!({ "cost": cost, "carry": carry, "eats": eats }));
        serde_json::to_string(&json).unwrap()
    }

    /// A civ's ring: every land tile its line can feed an army from, as JSON `[[tile, share], …]`, the share
    /// the part of an army's days its line takes to feed it there (`units::ring`); empty for no civ or a
    /// fallen one.
    pub fn ring(&self, civ: u32) -> String {
        let pairs: Vec<(usize, f64)> = units::ring(&self.world, civ).into_iter().collect();
        serde_json::to_string(&pairs).unwrap()
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

    /// The phase: `"pregame"`, `"choose"` (founding is done, the player has yet to pick), `"main"` or
    /// `"over"` (the game has ended; the world runs on, and the player's orders are refused).
    pub fn phase(&self) -> String {
        match self.world.phase {
            Phase::Pregame => "pregame",
            Phase::Choose => "choose",
            Phase::Main => "main",
            Phase::Over(_) => "over",
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

    /// The events from index `from` on, as a JSON array; each has its wording in `text`, its `alert`
    /// (a trouble of the led civ that has just started) and, for `peopleLeft`, its `amount`.
    pub fn events(&self, from: usize) -> String {
        let w = &self.world;
        let rows: Vec<_> = w.events.iter().skip(from).map(|e| e.json(w)).collect();
        serde_json::to_string(&rows).unwrap()
    }

    /// The reasons of subject `id` (a clan, market, civ or unit), as JSON `[{turn, text}]`, newest
    /// first; empty for an id with none.
    pub fn notes(&self, id: u32) -> String {
        let rows: Vec<_> = self
            .world
            .notes
            .get(&id)
            .into_iter()
            .flatten()
            .rev()
            .map(|n| serde_json::json!({ "turn": n.turn, "text": n.text }))
            .collect();
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
/// hungry_turns, store_cap, join, gift_max, notes_shown, save_version, fighters, fight}`. `join` is the
/// leaning at which a clan joins a civ; `gift_max` is the most a civ can give a turn (its store cap);
/// `notes_shown` is how many reasons the panel shows. `fighters` is `[{name, attack, defence}]` in
/// `Fighter` order (strength per person attacking and defending); `fight` is `{entrench, entrench_cap,
/// hungry, surrender}`: a defender's strength gained per turn still and its cap, the factor on an army
/// hungry last turn, and the hungry turns that surrender it. `field` is `{farmer}`: the share of its days from
/// which a clan is a farmer (`FIELD.farmer`).
#[wasm_bindgen]
pub fn rules() -> String {
    serde_json::json!({
        "tame_turns": TENDING.tame_turns,
        "players": PREGAME.players,
        "unit_labour": UNIT.labour,
        "unit_food": UNIT.food,
        "hungry_turns": UNIT.hungry_turns,
        "store_cap": CIV.store_cap,
        "join": LEANING.join,
        "gift_max": CIV.store_cap,
        "notes_shown": NOTES.shown,
        "save_version": engine::world::SAVE_VERSION,
        "fighters": FIGHTERS.iter().map(|f| serde_json::json!({ "name": f.name, "attack": f.attack, "defence": f.defence })).collect::<Vec<_>>(),
        "fight": { "entrench": FIGHT.entrench, "entrench_cap": FIGHT.entrench_cap, "hungry": FIGHT.hungry, "surrender": FIGHT.surrender },
        "field": { "farmer": FIELD.farmer },
    })
    .to_string()
}
