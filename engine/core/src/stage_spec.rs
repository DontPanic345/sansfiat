//! The data form of a stage: one JSON object builds the same stage in the CLI, the tests and the
//! page. `Stage::from_spec` builds it; `stage_test` runs a test written in it.

use serde::{Deserialize, Deserializer, Serialize};

use crate::data::N;
use crate::stage::Stage;
use crate::units;

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
    /// The civ (by its place in `civs`) the clan belongs to, or none.
    #[serde(default)]
    pub civ: Option<usize>,
    /// The civ (by its place in `civs`) whose capital market the clan trades at, or none.
    #[serde(default)]
    pub market: Option<usize>,
}

/// A signpost of civ `civ` (by its place in `civs`) on `at`, posted without the order's checks.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignpostSpec {
    pub civ: usize,
    pub at: TileRef,
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

/// A scout moved onto `at`, the civ's own (`civ` is an index into `civs`): its pack holds `pack`
/// grain, and `dest` is the tile it walks toward with no route (as a rival's scout is set).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoutSpec {
    pub civ: usize,
    pub at: TileRef,
    pub pack: f64,
    #[serde(default)]
    pub dest: Option<TileRef>,
}

/// A civ's store set to `food` grain when the stage is built (its other goods are dropped). With
/// `every_turn`, the stage's hand keeps it at `food` before every turn: the runner reads that, not
/// `from_spec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreSpec {
    pub civ: usize,
    pub food: f64,
    #[serde(default)]
    pub every_turn: bool,
}

/// A stage's data form: one JSON object builds the same stage in the CLI, the tests and the page.
/// Exactly one of `map` (the map's text) or `retro` is given. `map` and `marks` are text, or an
/// array of rows joined with newlines; `marks` is a grid (see `Stage::marks`). `civs` are the
/// tiles their civs found on, `people` each one's clan; `lead` is the index of the civ led. Every
/// `civ` in the spec (in `armies`, `scouts`, `stores`, a clan, a signpost) is an index into `civs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageSpec {
    #[serde(default, deserialize_with = "text_or_rows")]
    pub map: Option<String>,
    #[serde(default)]
    pub retro: Option<RetroSpec>,
    #[serde(default, deserialize_with = "text_or_rows")]
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
    /// Signposts, posted after the clans.
    #[serde(default)]
    pub signposts: Vec<SignpostSpec>,
    /// The turn limit after the pick, which replaces the size's (`Config.limit`); `None` is the size's.
    #[serde(default)]
    pub limit: Option<u32>,
    /// Scouts moved onto their tiles, after the armies.
    #[serde(default)]
    pub scouts: Vec<ScoutSpec>,
    /// Stores set to their food, after the scouts.
    #[serde(default)]
    pub stores: Vec<StoreSpec>,
}

/// Reads a map's or a grid's text: a string as it is, or an array of rows joined with newlines.
fn text_or_rows<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Text {
        One(String),
        Rows(Vec<String>),
    }
    Ok(Option::<Text>::deserialize(d)?.map(|t| match t {
        Text::One(s) => s,
        Text::Rows(rows) => rows.join("\n"),
    }))
}

impl Stage {
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
            let id = stage.living_clan(home, &tiles, k.people);
            if let Some(n) = k.civ {
                let civ = *ids.get(n).ok_or(format!("clans: there is no civ {n}"))?;
                stage.world.clans.get_mut(&id).unwrap().civ = Some(civ);
            }
            if let Some(n) = k.market {
                let civ = *ids.get(n).ok_or(format!("clans: there is no civ {n}"))?;
                let market = stage.world.civs[&civ].market;
                stage.world.clans.get_mut(&id).unwrap().market = Some(market);
            }
        }
        for p in &spec.signposts {
            let civ = *ids.get(p.civ).ok_or(format!("signposts: there is no civ {}", p.civ))?;
            let tile = stage.tile(&p.at)?;
            crate::settle::post(&mut stage.world, civ, tile);
        }
        for a in &spec.armies {
            let civ = *ids.get(a.civ).ok_or(format!("armies: there is no civ {}", a.civ))?;
            let tile = stage.tile(&a.at)?;
            stage.army(civ, tile, a.people);
        }
        for sc in &spec.scouts {
            let civ = *ids.get(sc.civ).ok_or(format!("scouts: there is no civ {}", sc.civ))?;
            let tile = stage.tile(&sc.at)?;
            let dest = sc.dest.as_ref().map(|d| stage.tile(d)).transpose()?;
            let id = units::scout(&stage.world, civ).ok_or(format!("scouts: civ {} has no scout", sc.civ))?.id;
            let u = stage.world.units.get_mut(&id).expect("the scout");
            u.actor.home = tile;
            u.actor.territory = vec![tile];
            u.actor.store = [0.0; N];
            u.actor.store[1] = sc.pack;
            if let Some(d) = dest {
                u.scout_mut().expect("a scout").dest = Some(d);
            }
        }
        for st in &spec.stores {
            let civ = *ids.get(st.civ).ok_or(format!("stores: there is no civ {}", st.civ))?;
            let c = stage.world.civs.get_mut(&civ).expect("the civ");
            c.actor.store = [0.0; N];
            c.actor.store[1] = st.food;
        }
        Ok(stage)
    }
}
