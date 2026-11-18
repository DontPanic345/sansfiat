//! A stage test: a stage written as data (`stage/NAME.json`), the orders it gives at turns, and the
//! holds that must hold as it runs. `Run` plays it a turn at a time, so `cargo test` and the page
//! run the same code, and the verdict is core's.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::data::N;
use crate::stage::{Stage, StageSpec, TileRef};
use crate::units::{self, Doing};
use crate::{Action, EventKind, Order, World};

/// Every test in `core/stage/`: its name and its file's text. A file there is listed here.
pub const LIST: &[(&str, &str)] = &[
    ("scout-errand", include_str!("../stage/scout-errand.json")),
    ("scout-killed", include_str!("../stage/scout-killed.json")),
    ("scout-trapped", include_str!("../stage/scout-trapped.json")),
    ("scout-walks-round", include_str!("../stage/scout-walks-round.json")),
];

/// A stage test. Its stage is `StageSpec`'s fields beside its own; `orders` are given before the
/// turn after `turn` turns have ended; `holds` are what must hold.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageTest {
    pub name: String,
    pub claim: String,
    #[serde(flatten)]
    pub stage: StageSpec,
    #[serde(default)]
    pub orders: Vec<TestOrder>,
    pub turns: u32,
    pub holds: Vec<Hold>,
}

/// An order given by civ `civ` (an index into `civs`) before the turn after `turn` turns have
/// ended. `order` is an `Order` as JSON, with its refs resolved when it is given: `{"mark": c}` and
/// `{"tile": i}` are tiles, `{"scout": k}` and `{"camp": k}` the ids of civ `k`'s scout and camp.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestOrder {
    pub turn: u32,
    pub civ: usize,
    pub order: Value,
}

/// What must hold. `every`: after every turn; `by`: after some turn up to `by`; `at`: after turn
/// `at`; `end`: after the last turn; `count`: the turns `count` becomes true, from false the turn
/// before, until `until` first holds (or the last turn), exactly `eq` of them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Hold {
    Every {
        every: Fact,
    },
    By {
        by: u32,
        fact: Fact,
    },
    At {
        at: u32,
        fact: Fact,
    },
    End {
        end: Fact,
    },
    Count {
        count: Fact,
        #[serde(default)]
        until: Option<Fact>,
        eq: u32,
    },
}

/// A fact the world holds or not. `civ` is an index into `civs`; a fact on a unit that is gone is
/// false.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Fact {
    Not(Box<Fact>),
    /// The civ's scout stands on the tile.
    ScoutAt {
        civ: usize,
        at: TileRef,
    },
    ScoutAlive {
        civ: usize,
    },
    /// Its hungry turns in a row are above none.
    ScoutHungry {
        civ: usize,
    },
    ScoutHoming {
        civ: usize,
    },
    /// Its scout's `Doing` is this variant, in lower camel case (`back`, `trapped`, `toStore`, …).
    ScoutDoing {
        civ: usize,
        doing: String,
    },
    /// Its scout's line in the view (`units::doing_text`) starts with this text.
    ScoutDoingText {
        civ: usize,
        starts: String,
    },
    ScoutBuilt {
        civ: usize,
    },
    /// The tile the scout stands on is shut to the civ's units by a rival (`units::shut`).
    ScoutShut {
        civ: usize,
    },
    /// The tile is shut to the civ's units by a rival.
    Shut {
        civ: usize,
        at: TileRef,
    },
    Trail(TileRef),
    /// The world holds exactly `eq` events of `kind`.
    Events {
        kind: EventKind,
        eq: usize,
    },
}

/// Where a hold stands: pending, held at a turn, or failed at a turn with why.
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Pending,
    Held(u32),
    Failed(u32, String),
}

/// The `Doing` variants by their lower camel names, as a fact writes them.
const DOINGS: &[&str] = &["idle", "searching", "toStore", "filling", "tooFar", "back", "trapped"];

fn doing_name(d: Doing) -> &'static str {
    match d {
        Doing::Idle => "idle",
        Doing::Searching => "searching",
        Doing::ToStore(..) => "toStore",
        Doing::Filling(_) => "filling",
        Doing::TooFar(_) => "tooFar",
        Doing::Back(_) => "back",
        Doing::Trapped(_) => "trapped",
    }
}

/// Every test in the list, parsed. A file that does not parse, or whose `name` is not its entry's,
/// panics with its name.
pub fn list() -> Vec<StageTest> {
    LIST.iter()
        .map(|(name, text)| {
            let test: StageTest = serde_json::from_str(text).unwrap_or_else(|e| panic!("{name}: {e}"));
            assert_eq!(&test.name, name, "the file of {name} is named {}", test.name);
            test
        })
        .collect()
}

/// The list as the page reads it: each test's name, claim, turns, its kept stores and holds as
/// words, and `json`, the file's own text, which `Sim.stage_test` builds the test from.
pub fn catalogue() -> Result<Value, String> {
    let mut out = Vec::new();
    for (name, text) in LIST {
        let test: StageTest = serde_json::from_str(text).map_err(|e| format!("{name}: {e}"))?;
        let (w, run) = Run::new(&test)?;
        out.push(json!({
            "name": test.name,
            "claim": test.claim,
            "turns": test.turns,
            "kept": run.kept_words(&w),
            "holds": (0..test.holds.len()).map(|i| run.hold_words(i, &w)).collect::<Vec<_>>(),
            "json": text,
        }));
    }
    Ok(Value::Array(out))
}

/// A test being played: the civ ids its indices stand for, the marks it names, its turn, and each
/// hold's state.
pub struct Run {
    test: StageTest,
    civs: Vec<u32>,
    marks: BTreeMap<char, usize>,
    turn: u32,
    states: Vec<State>,
    /// For a `count` hold: whether its fact held the turn before, and how many turns it began.
    prev: Vec<bool>,
    counts: Vec<u32>,
}

impl Run {
    /// The world `test`'s stage builds, and the run that plays it. Every fact is read once at turn
    /// 0, so a fact that names no civ or mark, or a doing that is no variant, fails here.
    pub fn new(test: &StageTest) -> Result<(World, Run), String> {
        let stage = Stage::from_spec(&test.stage)?;
        let marks = stage.marks.clone();
        let w = stage.into_world();
        let mut civs = Vec::new();
        for r in &test.stage.civs {
            let tile = match *r {
                TileRef::Mark { mark } => *marks.get(&mark).ok_or(format!("no tile is marked {mark:?}"))?,
                TileRef::Tile { tile } => tile,
            };
            let id = w.civs.values().find(|c| c.capital == tile).map(|c| c.id).ok_or("a civ has no capital there")?;
            civs.push(id);
        }
        let n = test.holds.len();
        let mut run = Run {
            test: test.clone(),
            civs,
            marks,
            turn: 0,
            states: vec![State::Pending; n],
            prev: vec![false; n],
            counts: vec![0; n],
        };
        for (i, h) in test.holds.iter().enumerate() {
            for f in h.facts() {
                run.eval(f, &w)?;
            }
            if let Hold::Count { count, .. } = h {
                run.prev[i] = run.eval(count, &w)?;
            }
        }
        for o in &test.orders {
            run.civ(o.civ)?;
        }
        Ok((w, run))
    }

    /// The test's name, claim and turns.
    pub fn name(&self) -> &str {
        &self.test.name
    }

    /// Whether every turn is played.
    pub fn done(&self) -> bool {
        self.turn >= self.test.turns
    }

    /// Whether the test is done and every hold held.
    pub fn passed(&self) -> bool {
        self.done() && self.states.iter().all(|s| matches!(s, State::Held(_)))
    }

    /// Each hold's state, in the test's order.
    pub fn states(&self) -> &[State] {
        &self.states
    }

    /// Plays one turn: keeps the stores kept each turn, gives the orders due, ends the turn and
    /// updates the holds. `Err` is a refused order's reason, or a fact that cannot be read.
    pub fn step(&mut self, w: &mut World) -> Result<(), String> {
        if self.done() {
            return Err(format!("{} is over", self.test.name));
        }
        for st in self.test.stage.stores.iter().filter(|s| s.every_turn) {
            let id = self.civ(st.civ)?;
            let c = w.civs.get_mut(&id).ok_or("a kept store's civ is gone")?;
            c.actor.store = [0.0; N];
            c.actor.store[1] = st.food;
        }
        for o in self.test.orders.iter().filter(|o| o.turn == self.turn) {
            let id = self.civ(o.civ)?;
            let order: Order = serde_json::from_value(self.resolve(&o.order, w)?)
                .map_err(|e| format!("turn {}: order: {e}", self.turn))?;
            if w.player == Some(id) {
                w.act(Action::Order { civ: id, order })?;
            } else {
                w.order(id, order)?;
            }
        }
        w.act(Action::EndTurn)?;
        self.turn += 1;
        self.update(w)
    }

    /// Plays the test to its last turn and returns the first failed hold, its words and its turn.
    /// A hold still pending at the end fails there too.
    pub fn run_all(test: &StageTest) -> Result<(), String> {
        let (mut w, mut run) = Run::new(test)?;
        while !run.done() {
            run.step(&mut w).map_err(|e| format!("{}: {e}", test.name))?;
        }
        if run.passed() {
            return Ok(());
        }
        let (i, (turn, why)) = run
            .states
            .iter()
            .enumerate()
            .find_map(|(i, s)| match s {
                State::Failed(t, why) => Some((i, (*t, why.clone()))),
                _ => None,
            })
            .ok_or_else(|| format!("{}: a hold is still pending", test.name))?;
        Err(format!("{}: failed on turn {turn}: {}: {why}", test.name, run.hold_words(i, &w)))
    }

    /// The stores kept each turn, as words: "the store of Red is kept at 50".
    pub fn kept_words(&self, w: &World) -> Vec<String> {
        self.test
            .stage
            .stores
            .iter()
            .filter(|s| s.every_turn)
            .map(|s| format!("the store of {} is kept at {}", self.civ_name(s.civ, w), s.food))
            .collect()
    }

    /// Hold `i` as a line the page shows: "by turn 10: Red's scout has built its trails".
    pub fn hold_words(&self, i: usize, w: &World) -> String {
        let h = &self.test.holds[i];
        match h {
            Hold::Every { every } => format!("every turn: {}", self.fact_words(every, w)),
            Hold::By { by, fact } => format!("by turn {by}: {}", self.fact_words(fact, w)),
            Hold::At { at, fact } => format!("on turn {at}: {}", self.fact_words(fact, w)),
            Hold::End { end } => format!("on the last turn: {}", self.fact_words(end, w)),
            Hold::Count { count, until, eq } => {
                let until = match until {
                    Some(u) => format!(" until {}", self.fact_words(u, w)),
                    None => String::new(),
                };
                format!("{} begins on {eq} turn(s){until}", self.fact_words(count, w))
            }
        }
    }

    /// The state of every hold, and the run's, as the page reads it: `{turn, turns, done, passed,
    /// holds: [{words, state, turn?, why?}]}`.
    pub fn state_json(&self, w: &World) -> Value {
        let holds: Vec<Value> = self
            .states
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let mut o = Map::new();
                o.insert("words".into(), json!(self.hold_words(i, w)));
                match s {
                    State::Pending => o.insert("state".into(), json!("pending")),
                    State::Held(t) => {
                        o.insert("turn".into(), json!(t));
                        o.insert("state".into(), json!("held"))
                    }
                    State::Failed(t, why) => {
                        o.insert("turn".into(), json!(t));
                        o.insert("why".into(), json!(why));
                        o.insert("state".into(), json!("failed"))
                    }
                };
                Value::Object(o)
            })
            .collect();
        json!({
            "turn": self.turn,
            "turns": self.test.turns,
            "done": self.done(),
            "passed": self.passed(),
            "holds": holds,
        })
    }

    fn update(&mut self, w: &World) -> Result<(), String> {
        let t = self.turn;
        let last = self.done();
        for i in 0..self.test.holds.len() {
            if self.states[i] != State::Pending {
                continue;
            }
            let h = self.test.holds[i].clone();
            let next = match &h {
                Hold::Every { every } => {
                    if !self.eval(every, w)? {
                        Some(State::Failed(t, self.not_holding(every, w)))
                    } else if last {
                        Some(State::Held(t))
                    } else {
                        None
                    }
                }
                Hold::By { by, fact } => {
                    if self.eval(fact, w)? && t <= *by {
                        Some(State::Held(t))
                    } else if t >= *by {
                        Some(State::Failed(t, self.not_holding(fact, w)))
                    } else {
                        None
                    }
                }
                Hold::At { at, fact } => (t == *at).then(|| self.settle(self.eval(fact, w), fact, w, t)),
                Hold::End { end } => last.then(|| self.settle(self.eval(end, w), end, w, t)),
                Hold::Count { count, until, eq } => {
                    let cur = self.eval(count, w)?;
                    if !self.prev[i] && cur {
                        self.counts[i] += 1;
                    }
                    self.prev[i] = cur;
                    let until_holds = match until {
                        Some(u) => self.eval(u, w)?,
                        None => false,
                    };
                    (until_holds || last).then(|| {
                        if self.counts[i] == *eq {
                            State::Held(t)
                        } else {
                            State::Failed(t, format!("it began on {} turn(s), not {eq}", self.counts[i]))
                        }
                    })
                }
            };
            if let Some(s) = next {
                self.states[i] = s;
            }
        }
        if last {
            for s in self.states.iter_mut().filter(|s| **s == State::Pending) {
                *s = State::Failed(t, "it did not settle by the last turn".into());
            }
        }
        Ok(())
    }

    fn settle(&self, ok: Result<bool, String>, fact: &Fact, w: &World, t: u32) -> State {
        match ok {
            Ok(true) => State::Held(t),
            Ok(false) => State::Failed(t, self.not_holding(fact, w)),
            Err(e) => State::Failed(t, e),
        }
    }

    fn not_holding(&self, fact: &Fact, w: &World) -> String {
        let _ = (fact, w);
        "it does not hold".to_string()
    }

    fn civ(&self, k: usize) -> Result<u32, String> {
        self.civs.get(k).copied().ok_or(format!("there is no civ {k}"))
    }

    fn civ_name(&self, k: usize, w: &World) -> String {
        match self.civs.get(k).and_then(|id| w.civs.get(id)) {
            Some(c) => c.name.clone(),
            None => format!("civ {k}"),
        }
    }

    fn tile(&self, r: &TileRef) -> Result<usize, String> {
        match *r {
            TileRef::Mark { mark } => self.marks.get(&mark).copied().ok_or(format!("no tile is marked {mark:?}")),
            TileRef::Tile { tile } => Ok(tile),
        }
    }

    /// A tile as words: its `(q, r)`.
    fn tile_words(&self, r: &TileRef, w: &World) -> String {
        match self.tile(r).ok().and_then(|i| w.tiles.get(i)) {
            Some(t) => format!("({}, {})", t.q, t.r),
            None => "a tile that is not on the map".into(),
        }
    }

    fn scout_of<'a>(&self, k: usize, w: &'a World) -> Result<Option<&'a units::Unit>, String> {
        Ok(units::scout(w, self.civ(k)?))
    }

    /// Reads `f` in `w`.
    fn eval(&self, f: &Fact, w: &World) -> Result<bool, String> {
        Ok(match f {
            Fact::Not(inner) => !self.eval(inner, w)?,
            Fact::ScoutAt { civ, at } => {
                let t = self.tile(at)?;
                self.scout_of(*civ, w)?.is_some_and(|u| u.tile() == t)
            }
            Fact::ScoutAlive { civ } => self.scout_of(*civ, w)?.is_some(),
            Fact::ScoutHungry { civ } => self.scout_of(*civ, w)?.is_some_and(|u| u.hungry > 0),
            Fact::ScoutHoming { civ } => self.scout_of(*civ, w)?.and_then(|u| u.scout()).is_some_and(|s| s.homing),
            Fact::ScoutDoing { civ, doing } => {
                if !DOINGS.contains(&doing.as_str()) {
                    return Err(format!("no doing {doing:?}"));
                }
                self.scout_of(*civ, w)?.and_then(|u| u.scout()).is_some_and(|s| doing_name(s.doing) == doing.as_str())
            }
            Fact::ScoutDoingText { civ, starts } => {
                self.scout_of(*civ, w)?.is_some_and(|u| units::doing_text(w, u).starts_with(starts.as_str()))
            }
            Fact::ScoutBuilt { civ } => self.scout_of(*civ, w)?.and_then(|u| u.scout()).is_some_and(|s| s.built),
            Fact::ScoutShut { civ } => {
                let id = self.civ(*civ)?;
                self.scout_of(*civ, w)?.is_some_and(|u| units::shut(w, id).contains_key(&u.tile()))
            }
            Fact::Shut { civ, at } => {
                let t = self.tile(at)?;
                units::shut(w, self.civ(*civ)?).contains_key(&t)
            }
            Fact::Trail(at) => {
                let t = self.tile(at)?;
                w.tiles.get(t).ok_or(format!("{t} is not a tile"))?.trail
            }
            Fact::Events { kind, eq } => w.events.iter().filter(|e| e.kind == *kind).count() == *eq,
        })
    }

    /// A fact as words: "Red's scout is hungry", "(3, 0) has a trail".
    fn fact_words(&self, f: &Fact, w: &World) -> String {
        let scout = |civ: &usize| format!("{}'s scout", self.civ_name(*civ, w));
        match f {
            Fact::Not(inner) => match &**inner {
                Fact::ScoutHungry { civ } => format!("{} is not hungry", scout(civ)),
                Fact::ScoutShut { civ } => format!("{} is not on a shut tile", scout(civ)),
                Fact::ScoutAt { civ, at } => format!("{} is not at {}", scout(civ), self.tile_words(at, w)),
                Fact::ScoutAlive { civ } => format!("{} is gone", scout(civ)),
                other => format!("not ({})", self.fact_words(other, w)),
            },
            Fact::ScoutAt { civ, at } => format!("{} is at {}", scout(civ), self.tile_words(at, w)),
            Fact::ScoutAlive { civ } => format!("{} is alive", scout(civ)),
            Fact::ScoutHungry { civ } => format!("{} is hungry", scout(civ)),
            Fact::ScoutHoming { civ } => format!("{} is walking home to its store", scout(civ)),
            Fact::ScoutDoing { civ, doing } => format!("{} is {doing}", scout(civ)),
            Fact::ScoutDoingText { civ, starts } => format!("{} says it is {starts}", scout(civ)),
            Fact::ScoutBuilt { civ } => format!("{} has built its trails", scout(civ)),
            Fact::ScoutShut { civ } => format!("{} is on a tile a rival holds", scout(civ)),
            Fact::Shut { civ, at } => {
                format!("{} is held against {}", self.tile_words(at, w), self.civ_name(*civ, w))
            }
            Fact::Trail(at) => format!("{} has a trail", self.tile_words(at, w)),
            Fact::Events { kind, eq } => {
                let name = serde_json::to_value(kind).ok().and_then(|v| v.as_str().map(String::from));
                format!("there are {eq} {} events", name.unwrap_or_default())
            }
        }
    }

    /// Resolves the refs in an order's JSON: a one-key object `{"mark": c}` or `{"tile": i}` is a
    /// tile, `{"scout": k}` and `{"camp": k}` the ids of civ `k`'s unit. Other objects are kept.
    fn resolve(&self, v: &Value, w: &World) -> Result<Value, String> {
        Ok(match v {
            Value::Object(m) if m.len() == 1 => {
                let (key, x) = m.iter().next().expect("one key");
                match (key.as_str(), x) {
                    ("mark", Value::String(s)) => {
                        let mut chars = s.chars();
                        let c = match (chars.next(), chars.next()) {
                            (Some(c), None) => c,
                            _ => return Err(format!("mark {s:?} is not one character")),
                        };
                        json!(self.tile(&TileRef::Mark { mark: c })?)
                    }
                    ("tile", Value::Number(_)) => x.clone(),
                    ("scout", Value::Number(k)) => {
                        let k = k.as_u64().ok_or("scout: not a civ")? as usize;
                        json!(self.scout_of(k, w)?.ok_or(format!("civ {k} has no scout"))?.id)
                    }
                    ("camp", Value::Number(k)) => {
                        let k = k.as_u64().ok_or("camp: not a civ")? as usize;
                        json!(units::camp(w, self.civ(k)?).ok_or(format!("civ {k} has no camp"))?.id)
                    }
                    _ => Value::Object(
                        m.iter().map(|(k, x)| Ok((k.clone(), self.resolve(x, w)?))).collect::<Result<_, String>>()?,
                    ),
                }
            }
            Value::Object(m) => Value::Object(
                m.iter().map(|(k, x)| Ok((k.clone(), self.resolve(x, w)?))).collect::<Result<_, String>>()?,
            ),
            Value::Array(a) => Value::Array(a.iter().map(|x| self.resolve(x, w)).collect::<Result<_, String>>()?),
            other => other.clone(),
        })
    }
}

impl Hold {
    /// The facts a hold reads: its fact, and its `until` when it has one.
    fn facts(&self) -> Vec<&Fact> {
        match self {
            Hold::Every { every } => vec![every],
            Hold::By { fact, .. } | Hold::At { fact, .. } => vec![fact],
            Hold::End { end } => vec![end],
            Hold::Count { count, until, .. } => std::iter::once(count).chain(until.iter()).collect(),
        }
    }
}
