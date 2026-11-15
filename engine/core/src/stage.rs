//! A stage: a world placed on a known map, given orders and read with the real engine. Tests, the
//! CLI and the wasm page build the same stage from one `StageSpec`. A staged world is not a save:
//! its placements bypass the log, so a stage is never written out; it asserts outcomes.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use serde::{Deserialize, Serialize};

use crate::clans::new_clan;
use crate::data::{size_radius, Config, Fighter, N, PLAINS, SEA};
use crate::hex::Hex;
use crate::homes;
use crate::map::{self, Tile};
use crate::over::Over;
use crate::pregame::End;
use crate::units::{self, Plan, Unit};
use crate::world::Phase;
use crate::{civs, claim, maptext, Action, EventKind, Market, Order, World};

/// The seven maps of the retro: each size with its seeds.
pub const RETRO: [(&str, &[u32]); 3] = [("small", &[2, 3, 6, 9]), ("medium", &[8, 9]), ("large", &[8])];

/// The rng state and next id a stage's world starts with: the placeholder generator's, as the
/// hand-built test worlds have always had them, so every draw and id they rely on holds.
const BASE_RNG: u32 = 4151932303;
const BASE_NEXT_ID: u32 = 4;

/// A hidden cost: a trail's cost to a tile, and the span of a bridge over it.
pub type HiddenCost = (Option<f64>, Option<u8>);

/// A world on a known map, with marks naming tiles by a character. See the module's note: a stage
/// is not a save.
pub struct Stage {
    world: World,
    marks: BTreeMap<char, usize>,
}

/// The pre-game's cache: one `Entry` per map (radius and seed), its pick computed once.
struct Entry {
    world: OnceLock<World>,
    runs: AtomicUsize,
}

static CACHE: Mutex<BTreeMap<(i32, u32), Arc<Entry>>> = Mutex::new(BTreeMap::new());

/// The config a stage's world is built with: the base's, on `radius`.
fn config(radius: i32, seed: u32) -> Config {
    Config { seed, radius, army: true, gifts: true, zone: true, ..Config::default() }
}

/// The pick of the pre-game on `radius` and `seed`, computed once per process.
fn retro_world(radius: i32, seed: u32) -> World {
    let entry = {
        let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
        cache
            .entry((radius, seed))
            .or_insert_with(|| Arc::new(Entry { world: OnceLock::new(), runs: AtomicUsize::new(0) }))
            .clone()
    };
    entry.world.get_or_init(|| {
        entry.runs.fetch_add(1, Ordering::SeqCst);
        let mut w = World::new(Config { seed, radius, ..Config::default() });
        while w.phase == Phase::Pregame {
            w.run(1);
        }
        w
    });
    entry.world.get().expect("the pick").clone()
}

/// How many pre-games have run for the map of `radius` and `seed`: at most one, since the pick is
/// cached. For the tests of the cache.
pub fn pregame_runs(radius: i32, seed: u32) -> usize {
    let cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    cache.get(&(radius, seed)).map_or(0, |e| e.runs.load(Ordering::SeqCst))
}

impl Stage {
    /// A stage of the map `text` (see `maptext::parse`): bare, with no clans, markets or layers.
    pub fn parse(text: &str) -> Result<Stage, String> {
        let parsed = maptext::parse(text)?;
        Ok(Stage::base(parsed.radius, parsed.tiles))
    }

    /// A stage of bare plains inside `radius`, sea on its ring.
    pub fn flat(radius: i32) -> Stage {
        let mut tiles = maptext::bare(radius);
        for t in &mut tiles {
            t.terrain = if t.hex().distance(Hex::ORIGIN) < radius { PLAINS } else { SEA };
            t.trail_cost = crate::land::flat_cost(t.terrain);
        }
        Stage::base(radius, tiles)
    }

    /// The pick of the retro map `size` (a name from `RETRO`) with `seed`: the pre-game run to
    /// `Phase::Choose`, computed once per process and cloned here.
    pub fn retro(size: &str, seed: u32) -> Result<Stage, String> {
        let radius = size_radius(size).ok_or_else(|| format!("unknown size {size}"))?;
        Ok(Stage { world: retro_world(radius, seed), marks: BTreeMap::new() })
    }

    /// The stage `spec` describes: its map or retro pick, then its layers, its civs, its lead and
    /// its armies, in that order.
    pub fn from_spec(spec: &StageSpec) -> Result<Stage, String> {
        let mut stage = match (&spec.map, &spec.retro) {
            (Some(text), None) => Stage::parse(text)?,
            (None, Some(r)) => Stage::retro(&r.size, r.seed)?,
            _ => return Err("a stage needs one of map or retro".to_string()),
        };
        stage.world.config.limit = spec.limit;
        if let Some(text) = &spec.marks {
            stage.marks(text)?;
        }
        for h in &spec.hidden {
            let t = stage.tile(&h.at)?;
            stage.hidden_at(t, h.trail_cost, h.span);
        }
        for f in &spec.food {
            let t = stage.tile(&f.at)?;
            if f.good >= N {
                return Err(format!("good {} is not one of {N}", f.good));
            }
            stage.food(t, f.good, f.amount);
        }
        let mut ids = Vec::new();
        if !spec.civs.is_empty() {
            let people = spec.people.ok_or("civs need people")?;
            let tiles = spec.civs.iter().map(|r| stage.tile(r)).collect::<Result<Vec<_>, _>>()?;
            ids = stage.civs(&tiles, people);
        }
        if let Some(k) = spec.lead {
            let civ = *ids.get(k).ok_or(format!("lead: there is no civ {k}"))?;
            stage.lead(civ)?;
        }
        for k in &spec.clans {
            let home = stage.tile(&k.home)?;
            let tiles = k.tiles.iter().map(|r| stage.tile(r)).collect::<Result<Vec<_>, _>>()?;
            stage.living_clan(home, &tiles, k.people);
        }
        for a in &spec.armies {
            let civ = *ids.get(a.civ).ok_or(format!("armies: there is no civ {}", a.civ))?;
            let tile = stage.tile(&a.at)?;
            stage.army(civ, tile, a.people);
        }
        Ok(stage)
    }

    /// A stage of `tiles` on a map of `radius`, with the world a bare map has.
    fn base(radius: i32, tiles: Vec<Tile>) -> Stage {
        let mut world = World::new(config(radius, 1));
        world.tiles = tiles;
        world.clans.clear();
        world.rng = BASE_RNG;
        world.next_id = BASE_NEXT_ID;
        world.markets.clear();
        Stage { world, marks: BTreeMap::new() }
    }

    /// The end of the game, once it has ended (`Phase::Over`).
    pub fn over(&self) -> Option<Over> {
        match self.world.phase {
            Phase::Over(o) => Some(o),
            _ => None,
        }
    }

    /// The world, for a read the stage has no method for.
    pub fn world(&self) -> &World {
        &self.world
    }

    /// The world, mutable. It bypasses the log: a painting here is not an action, so a replay does
    /// not show it. Only for a setup a test needs and no builder gives.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// The world, consumed.
    pub fn into_world(self) -> World {
        self.world
    }

    /// The map as text, with each mark drawn as its character (`map::ascii_marked`).
    pub fn ascii(&self) -> String {
        let marks: Vec<(usize, char)> = self.marks.iter().map(|(&c, &i)| (i, c)).collect();
        map::ascii_marked(&self.world.tiles, self.world.config.radius, &marks)
    }

    /// Names the tiles of a grid (see `maptext::grid`) by their characters: each character but `.`
    /// is a mark for its tile. The grid's radius must be the map's.
    pub fn marks(&mut self, text: &str) -> Result<(), String> {
        let (radius, cells) = maptext::grid(text)?;
        self.check_radius(radius)?;
        for (i, c) in cells.into_iter().enumerate() {
            if c != '.' {
                self.marks.insert(c, i);
            }
        }
        Ok(())
    }

    /// The tile marked `c`. Panics when no tile is marked so: a test names a mark it has set.
    pub fn mark(&self, c: char) -> usize {
        *self.marks.get(&c).unwrap_or_else(|| panic!("no tile is marked {c:?}"))
    }

    /// Sets the hidden costs of the tiles of a grid: each character but `.` takes the pair `table`
    /// gives it, `(trail cost, span)`.
    pub fn hidden(&mut self, text: &str, table: &[(char, HiddenCost)]) -> Result<(), String> {
        let (radius, cells) = maptext::grid(text)?;
        self.check_radius(radius)?;
        for (i, c) in cells.into_iter().enumerate() {
            if c == '.' {
                continue;
            }
            let (_, (cost, span)) = *table.iter().find(|(k, _)| *k == c).ok_or(format!("no hidden cost for {c:?}"))?;
            self.hidden_at(i, cost, span);
        }
        Ok(())
    }

    /// Sets tile `i`'s hidden trail cost and bridge span.
    pub fn hidden_at(&mut self, i: usize, trail_cost: Option<f64>, span: Option<u8>) {
        let t = &mut self.world.tiles[i];
        t.trail_cost = trail_cost;
        t.span = span;
    }

    /// Sets tile `i`'s food `good` (an index into `GOODS`) to `amount`: its cap, its unworn cap and
    /// its stock, so a staged army has something to forage or pillage.
    pub fn food(&mut self, i: usize, good: usize, amount: f64) {
        let t = &mut self.world.tiles[i];
        t.cap[good] = amount;
        t.unworn[good] = amount;
        t.stock[good] = amount;
    }

    /// Puts a clan of `people` on tile `i`, holding only it. Returns its id.
    pub fn clan(&mut self, i: usize, people: f64) -> u32 {
        let id = self.fresh_id();
        self.world.tiles[i].claim = Some(id);
        self.world.clans.insert(id, new_clan(id, i, people));
        self.world.clans.get_mut(&id).expect("the clan").territory = vec![i];
        id
    }

    /// A clan of `people` living on `home` and claiming `tiles` through `homes::claim_land`, so a tile
    /// another clan holds keeps its claim: the home may be shared. Returns its id.
    pub fn living_clan(&mut self, home: usize, tiles: &[usize], people: f64) -> u32 {
        let id = self.fresh_id();
        let mut territory = tiles.to_vec();
        territory.push(home);
        territory.sort_unstable();
        territory.dedup();
        let mut c = new_clan(id, home, people);
        c.territory = territory.clone();
        homes::claim_land(&mut self.world, id, &territory);
        self.world.clans.insert(id, c);
        id
    }

    /// Founds a civ on each tile of `tiles`: a market opens there, a clan of `people` lives there,
    /// and the civs are founded as `civs::found` founds them. Returns the new civs' ids, in order.
    pub fn civs(&mut self, tiles: &[usize], people: f64) -> Vec<u32> {
        let before: Vec<u32> = self.world.civs.keys().copied().collect();
        let markets: Vec<u32> = tiles.iter().map(|&t| open_market(&mut self.world, t)).collect();
        for &t in tiles {
            self.clan(t, people);
        }
        found_civs(&mut self.world, &markets);
        self.world.civs.keys().copied().filter(|id| !before.contains(id)).collect()
    }

    /// Makes `civ` the civ the player leads, as `ChooseCiv` does (logged, so a replay has it).
    pub fn lead(&mut self, civ: u32) -> Result<(), String> {
        self.act(Action::ChooseCiv { civ: Some(civ) })?;
        self.world.events.clear();
        Ok(())
    }

    /// Makes every land tile plains and bare: no rivers, trails or lakes, the plains carry the flat
    /// trail cost and nothing hidden, and no food set on it. The led civ's revealed set is cleared, so it has seen nothing. The capital, its
    /// camp and its scout are left as they are. Not logged: a plain stage is not a save.
    pub fn plain(&mut self) {
        for t in &mut self.world.tiles {
            if t.terrain != SEA {
                t.terrain = PLAINS;
            }
            t.river = 0;
            t.flow = None;
            t.trail = false;
            t.trail_work = 0.0;
            t.trail_cost = crate::land::flat_cost(t.terrain);
            t.span = None;
        }
        if let Some(me) = self.world.player {
            if let Some(c) = self.world.civs.get_mut(&me) {
                c.revealed.clear();
            }
        }
    }

    /// Sets tile `i` to `terrain` with its hidden trail cost and span. Not logged, as `plain` is not.
    pub fn paint(&mut self, i: usize, terrain: usize, trail_cost: Option<f64>, span: Option<u8>) {
        let t = &mut self.world.tiles[i];
        t.terrain = terrain;
        t.trail_cost = trail_cost;
        t.span = span;
    }

    /// Moves `civ`'s camp onto tile `i` with `people` warriors, in one stack and a full pack, and holds it
    /// there: its march is to `i` (a camp posts itself elsewhere, and a rival's capital is no zone). A civ
    /// with no camp (a civ starts with none) gets an empty one at its capital first, which takes no
    /// people from its clans. Returns the camp's unit id.
    pub fn army(&mut self, civ: u32, i: usize, people: f64) -> u32 {
        let id = match units::camp(&self.world, civ) {
            Some(u) => u.id,
            None => units::make_camp(&mut self.world, civ),
        };
        let u: &mut Unit = self.world.units.get_mut(&id).expect("the camp");
        u.actor.home = i;
        u.actor.territory = vec![i];
        u.actor.people = people;
        u.actor.store = [units::pack_cap(people) / N as f64; N];
        let c = u.camp_mut().expect("a camp");
        c.held = vec![i];
        c.stack = [0.0; 2];
        c.stack[Fighter::Warriors.index()] = people;
        c.dest = Some(i);
        id
    }

    /// Gives `order` for `civ`, as `World::order` does. `Err` is the engine's reason, and the world
    /// is unchanged.
    pub fn order(&mut self, civ: u32, order: Order) -> Result<(), String> {
        self.world.order(civ, order)
    }

    /// Takes `action`, as the player's door does: `Err` is the engine's reason.
    pub fn act(&mut self, action: Action) -> Result<(), String> {
        self.world.act(action)
    }

    /// Ends `n` turns. The first refusal's reason is returned.
    pub fn turn(&mut self, n: u32) -> Result<(), String> {
        for _ in 0..n {
            self.act(Action::EndTurn)?;
        }
        Ok(())
    }

    /// Ends turns until `pred` holds: `Some(k)` after `k` turns, or `None` if it still doesn't
    /// after `max`.
    pub fn until(&mut self, max: u32, pred: impl Fn(&World) -> bool) -> Result<Option<u32>, String> {
        for taken in 0..=max {
            if pred(&self.world) {
                return Ok(Some(taken));
            }
            if taken < max {
                self.act(Action::EndTurn)?;
            }
        }
        Ok(None)
    }

    /// Whether `civ` has revealed tile `i`.
    pub fn revealed(&self, civ: u32, i: usize) -> bool {
        self.world.civs[&civ].revealed.contains(&i)
    }

    /// Whether tile `i` has a trail.
    pub fn trail(&self, i: usize) -> bool {
        self.world.tiles[i].trail
    }

    /// `civ`'s route plan from `from` to `to`: the known and possible costs and paths.
    pub fn route(&self, civ: u32, from: usize, to: usize) -> Plan {
        units::route_plan(&self.world, civ, from, to)
    }

    /// The people of unit `id`.
    pub fn people(&self, id: u32) -> f64 {
        self.world.units[&id].actor.people
    }

    /// The food in `civ`'s store, all goods added.
    pub fn store(&self, civ: u32) -> f64 {
        self.world.civs[&civ].actor.store.iter().sum()
    }

    /// The civ holding tile `i`, if any.
    pub fn holder(&self, i: usize) -> Option<u32> {
        claim::tile_civ(&self.world, i)
    }

    /// How many events of `kind` the world holds.
    pub fn events(&self, kind: EventKind) -> usize {
        self.world.events.iter().filter(|e| e.kind == kind).count()
    }

    /// `civ`'s camp, if it has one.
    pub fn camp(&self, civ: u32) -> Option<&Unit> {
        units::camp(&self.world, civ)
    }

    /// `civ`'s scout, if it has one.
    pub fn scout(&self, civ: u32) -> Option<&Unit> {
        units::scout(&self.world, civ)
    }

    /// The tile `r` names: its mark, or its index, checked against the map.
    fn tile(&self, r: &TileRef) -> Result<usize, String> {
        match *r {
            TileRef::Mark { mark } => self.marks.get(&mark).copied().ok_or(format!("no tile is marked {mark:?}")),
            TileRef::Tile { tile } if tile < self.world.tiles.len() => Ok(tile),
            TileRef::Tile { tile } => Err(format!("{tile} is not a tile")),
        }
    }

    fn check_radius(&self, radius: i32) -> Result<(), String> {
        if radius == self.world.config.radius {
            Ok(())
        } else {
            Err(format!("the grid has radius {radius}; the map has {}", self.world.config.radius))
        }
    }

    fn fresh_id(&mut self) -> u32 {
        let id = self.world.next_id;
        self.world.next_id += 1;
        id
    }
}

/// A tile by its mark or its index.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TileRef {
    Mark { mark: char },
    Tile { tile: usize },
}

/// A clan living on `home`, claiming `tiles` (its home too, where no earlier clan claims it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClanSpec {
    pub home: TileRef,
    pub tiles: Vec<TileRef>,
    pub people: f64,
}

/// A retro pick: a size from `RETRO` and a seed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetroSpec {
    pub size: String,
    pub seed: u32,
}

/// A hidden cost on one tile.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HiddenSpec {
    pub at: TileRef,
    pub trail_cost: Option<f64>,
    pub span: Option<u8>,
}

/// One good's food on one tile.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoodSpec {
    pub at: TileRef,
    pub good: usize,
    pub amount: f64,
}

/// An army put on a civ's camp's tile, with `people` warriors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmySpec {
    pub civ: usize,
    pub at: TileRef,
    pub people: f64,
}

/// A stage's data form: one JSON object builds the same stage in the CLI, the tests and the page.
/// Exactly one of `map` (the map's text) or `retro` is given. `marks` is a grid (see
/// `Stage::marks`); `civs` are the tiles their civs found on, `people` each one's clan; `lead` is
/// the index of the civ led; `armies` are indices into `civs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageSpec {
    #[serde(default)]
    pub map: Option<String>,
    #[serde(default)]
    pub retro: Option<RetroSpec>,
    #[serde(default)]
    pub marks: Option<String>,
    #[serde(default)]
    pub hidden: Vec<HiddenSpec>,
    #[serde(default)]
    pub food: Vec<FoodSpec>,
    #[serde(default)]
    pub civs: Vec<TileRef>,
    #[serde(default)]
    pub people: Option<f64>,
    #[serde(default)]
    pub lead: Option<usize>,
    #[serde(default)]
    pub armies: Vec<ArmySpec>,
    /// Clans living on a home, placed after the civs: a home may be shared (`towns`).
    #[serde(default)]
    pub clans: Vec<ClanSpec>,
    /// The turn limit after the pick, which replaces the size's (`Config.limit`); `None` is the size's.
    #[serde(default)]
    pub limit: Option<u32>,
}

/// Opens a market on tile `tile` of `w`, founded on turn 0, as a centre's market is. Returns its id.
pub fn open_market(w: &mut World, tile: usize) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let m = Market {
        id,
        tile,
        founded: 0,
        prices: [1.0; N],
        goods: Vec::new(),
        activity: 2.0 * crate::data::MARKET.min_attendance,
        volume: 0.0,
        traders: 0,
        moved: 0.0,
        town: false,
    };
    w.markets.insert(id, m);
    id
}

/// Ends the pre-game on `centres` (the markets of the civs' tiles) and founds the civs, as the
/// pre-game's end does.
pub fn found_civs(w: &mut World, centres: &[u32]) {
    w.end = Some(End {
        turn: w.turn,
        centres: centres.to_vec(),
        clans: Vec::new(),
        cover: 1.0,
        by_limit: false,
        topped_up: Vec::new(),
    });
    civs::found(w);
}
