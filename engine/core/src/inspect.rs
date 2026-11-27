//! One thing in the world as JSON: a unit, clan, market, civ, settlement or tile, with where it
//! stands, its state as saved, its reasons and the events about it. The CLI's `inspect` and the
//! page's `Sim.inspect` both answer from here. A reading: it never changes the world.

use serde_json::{json, Map, Value};

use crate::data::{INSPECT, TERRAIN};
use crate::events::Event;
use crate::hex::Hex;
use crate::{civs, map, World};

/// What a query asks about: a subject by id (every id comes from `World::next_id`, so an id names
/// one thing), or a tile by index.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    Id(u32),
    Tile(usize),
}

impl Target {
    /// `#N` is the subject with id N, `Q,R` the tile at that hex, and a bare `N` the tile with index N
    /// (the text view's and the page's tile numbers).
    pub fn parse(w: &World, text: &str) -> Result<Target, String> {
        let text = text.trim();
        if let Some(id) = text.strip_prefix('#') {
            return id.parse().map(Target::Id).map_err(|_| format!("not an id: {text}"));
        }
        if let Some((q, r)) = text.split_once(',') {
            let (q, r) = (q.trim().parse(), r.trim().parse());
            let (Ok(q), Ok(r)) = (q, r) else { return Err(format!("not a hex: {text}")) };
            return map::index(w.config.radius, Hex { q, r })
                .map(Target::Tile)
                .ok_or_else(|| format!("no tile at {text}"));
        }
        match text.parse::<usize>() {
            Ok(i) if i < w.tiles.len() => Ok(Target::Tile(i)),
            Ok(i) => Err(format!("no tile {i}: the map has {}", w.tiles.len())),
            Err(_) => Err(format!("not a target: {text} (#ID, Q,R or a tile index)")),
        }
    }
}

/// Where tile `i` is: its index, hex and terrain.
pub fn at(w: &World, i: usize) -> Value {
    let t = &w.tiles[i];
    json!({ "tile": i, "q": t.q, "r": t.r, "terrain": TERRAIN[t.terrain].name })
}

/// The thing `target` names at this turn, or `Err` when there is none.
pub fn inspect(w: &World, target: Target) -> Result<Value, String> {
    let mut v = match target {
        Target::Tile(i) if i < w.tiles.len() => tile(w, i),
        Target::Tile(i) => return Err(format!("no tile {i}: the map has {}", w.tiles.len())),
        Target::Id(id) => subject(w, id).ok_or_else(|| format!("nothing has id {id} at turn {}", w.turn))?,
    };
    v["turn"] = w.turn.into();
    v["phase"] = serde_json::to_value(w.phase).unwrap();
    Ok(v)
}

/// A subject by id: its kind, where it stands, its state, its reasons (newest first) and its events.
fn subject(w: &World, id: u32) -> Option<Value> {
    let (kind, home, state, extra) = if let Some(u) = w.units.get(&id) {
        let civ = w.civs.get(&u.civ).map(|c| c.name.clone());
        (u.kind().name(), u.actor.home, json!(u), json!({ "civ": u.civ, "civName": civ }))
    } else if let Some(c) = w.clans.get(&id) {
        ("clan", c.actor.home, json!(c), json!({ "civ": civs::tile_civ(w, c.actor.home) }))
    } else if let Some(c) = w.civs.get(&id) {
        let units: Vec<u32> = w.units.values().filter(|u| u.civ == id).map(|u| u.id).collect();
        ("civ", c.capital, json!(c), json!({ "name": c.name, "units": units }))
    } else if let Some(m) = w.markets.get(&id) {
        ("market", m.tile, json!(m), json!({}))
    } else {
        let s = w.settlements.get(&id)?;
        ("settlement", s.tile, json!(s), json!({ "civ": s.civ }))
    };
    let mut v = json!({ "kind": kind, "id": id, "at": at(w, home), "state": state });
    merge(&mut v, extra);
    v["notes"] = notes(w, id);
    v["events"] = events(w, |e| [e.clan, e.market, e.other, e.civ, e.unit].contains(&Some(id)));
    Some(v)
}

/// A tile: its state, what stands on it, whose it is, and the events on it.
fn tile(w: &World, i: usize) -> Value {
    let ids = |it: &mut dyn Iterator<Item = u32>| it.collect::<Vec<u32>>();
    let here = json!({
        "units": w.units.values().filter(|u| u.actor.home == i)
            .map(|u| json!({ "id": u.id, "kind": u.kind().name(), "civ": u.civ })).collect::<Vec<_>>(),
        "clans": ids(&mut w.clans.values().filter(|c| c.actor.home == i).map(|c| c.id)),
        "markets": ids(&mut w.markets.values().filter(|m| m.tile == i).map(|m| m.id)),
        "settlements": ids(&mut w.settlements.values().filter(|s| s.tile == i).map(|s| s.id)),
        "capitalOf": w.civs.values().find(|c| c.capital == i).map(|c| c.id),
    });
    let mut v = json!({ "kind": "tile", "at": at(w, i), "state": w.tiles[i], "here": here });
    v["claim"] = w.tiles[i].claim.into();
    v["civ"] = civs::tile_civ(w, i).into();
    v["events"] = events(w, |e| e.tile == i);
    v
}

/// The reasons of subject `id`, newest first.
fn notes(w: &World, id: u32) -> Value {
    let rows = w.notes.get(&id).into_iter().flatten().rev();
    Value::Array(rows.map(|n| json!({ "turn": n.turn, "text": n.text })).collect())
}

/// The newest `INSPECT.events` events that `about` picks, newest first, each with its wording.
fn events(w: &World, about: impl Fn(&Event) -> bool) -> Value {
    let rows = w.events.iter().rev().filter(|e| about(e)).take(INSPECT.events);
    Value::Array(rows.map(|e| e.json(w)).collect())
}

/// Every subject of `kind` (`units`, `clans`, `civs`, `markets`, `settlements`), one short row each:
/// its id, where it stands, and the few fields that tell one from another.
pub fn list(w: &World, kind: &str) -> Result<Value, String> {
    let row = |id: u32, home: usize, extra: Value| {
        let mut v = json!({ "id": id, "at": at(w, home) });
        merge(&mut v, extra);
        v
    };
    let rows: Vec<Value> = match kind {
        "units" => w
            .units
            .values()
            .map(|u| {
                row(u.id, u.actor.home, json!({ "kind": u.kind().name(), "civ": u.civ, "people": u.actor.people }))
            })
            .collect(),
        "clans" => w
            .clans
            .values()
            .map(|c| row(c.id, c.actor.home, json!({ "people": c.actor.people, "market": c.actor.market })))
            .collect(),
        "civs" => w.civs.values().map(|c| row(c.id, c.capital, json!({ "name": c.name }))).collect(),
        "markets" => w.markets.values().map(|m| row(m.id, m.tile, json!({}))).collect(),
        "settlements" => w.settlements.values().map(|s| row(s.id, s.tile, json!({ "civ": s.civ }))).collect(),
        _ => return Err(format!("unknown kind {kind}; kinds: units clans civs markets settlements")),
    };
    Ok(Value::Array(rows))
}

/// What changed from `a` to `b`: `[{path, from, to}]`, a path such as `state.actor.people`, in the
/// order the fields come. A field only one side has is `null` on the other.
pub fn diff(a: &Value, b: &Value) -> Value {
    let mut out = Vec::new();
    walk("", a, b, &mut out);
    Value::Array(out)
}

fn walk(path: &str, a: &Value, b: &Value, out: &mut Vec<Value>) {
    let join = |k: &str| if path.is_empty() { k.to_string() } else { format!("{path}.{k}") };
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let keys = x.keys().chain(y.keys().filter(|k| !x.contains_key(*k)));
            for k in keys {
                walk(&join(k), x.get(k).unwrap_or(&Value::Null), y.get(k).unwrap_or(&Value::Null), out);
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            for i in 0..x.len().max(y.len()) {
                walk(&join(&i.to_string()), x.get(i).unwrap_or(&Value::Null), y.get(i).unwrap_or(&Value::Null), out);
            }
        }
        _ if a != b => out.push(json!({ "path": path, "from": a, "to": b })),
        _ => {}
    }
}

/// Adds `extra`'s fields to the object `v`.
fn merge(v: &mut Value, extra: Value) {
    if let (Value::Object(v), Value::Object(extra)) = (v, extra) {
        let extra: Map<String, Value> = extra;
        v.extend(extra);
    }
}
