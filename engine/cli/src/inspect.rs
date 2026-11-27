use crate::{size, USAGE};
use engine::inspect::{self, Target};
use engine::{Config, Save, World};
use serde_json::{json, Value};

/// Where the world comes from: a save replayed, or a new world on the AI.
enum Source {
    Save(Save),
    New(Config),
}

impl Source {
    /// The world at the end of turn `turn` (the save's last, if `None`). A new world needs a turn.
    fn at(&self, turn: Option<u32>) -> Result<World, String> {
        match self {
            Source::Save(save) => World::replay_to(save, turn),
            Source::New(config) => {
                let turn = turn.ok_or(format!("a new world needs --turn N\n{USAGE}"))?;
                let mut w = World::new(*config);
                while w.turn < turn {
                    w.run(1);
                }
                Ok(w)
            }
        }
    }
}

/// `sansfiat inspect`: one thing in the world as JSON (`engine::inspect`), at a turn, or what changed
/// in it between two turns, or every subject of a kind.
pub(crate) fn inspect(args: &[String]) -> Result<(), String> {
    let (mut config, mut new) = (Config::default(), false);
    let mut path: Option<String> = None;
    let (mut turn, mut diff, mut list, mut target, mut compact) = (None, None, None, None, false);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--compact" {
            compact = true;
            continue;
        }
        if !a.starts_with("--") {
            match &path {
                None if a.ends_with(".json") => path = Some(a.clone()),
                _ if target.is_none() => target = Some(a.clone()),
                _ => return Err(format!("one target only: {a}\n{USAGE}")),
            }
            continue;
        }
        let value = it.next().ok_or(format!("{a} needs a value\n{USAGE}"))?;
        let num = |v: &str| v.parse::<u32>().map_err(|_| format!("{a}: not a number: {v}"));
        new |= ["--seed", "--size", "--radius"].contains(&a.as_str());
        match a.as_str() {
            "--seed" => config.seed = num(value)?,
            "--size" => config.radius = size(value)?,
            "--radius" => config.radius = value.parse().map_err(|_| format!("{a}: not a number: {value}"))?,
            "--turn" => turn = Some(num(value)?),
            "--diff" => diff = Some(num(value)?),
            "--list" => list = Some(value.clone()),
            "--id" => target = Some(format!("#{value}")),
            "--tile" => target = Some(value.clone()),
            _ => return Err(format!("unknown flag {a}\n{USAGE}")),
        }
    }
    let source = match path {
        Some(_) if new => return Err(format!("a save or --seed/--size, not both\n{USAGE}")),
        Some(p) => {
            let text = std::fs::read_to_string(&p).map_err(|e| format!("{p}: {e}"))?;
            Source::Save(serde_json::from_str(&text).map_err(|e| format!("{p}: {e}"))?)
        }
        None if new => Source::New(config),
        None => return Err(format!("inspect needs a save or --seed\n{USAGE}")),
    };
    let w = source.at(turn)?;
    let out = if let Some(kind) = list {
        json!({ "turn": w.turn, "kind": kind, "rows": inspect::list(&w, &kind)? })
    } else {
        let text =
            target.ok_or(format!("inspect needs a target (#ID, Q,R or a tile index) or --list KIND\n{USAGE}"))?;
        let t = Target::parse(&w, &text)?;
        let now = inspect::inspect(&w, t)?;
        match diff {
            None => now,
            Some(to) => changes(&now, &inspect::inspect(&source.at(Some(to))?, t)?),
        }
    };
    let text = if compact { serde_json::to_string(&out) } else { serde_json::to_string_pretty(&out) };
    println!("{}", text.unwrap());
    Ok(())
}

/// What changed between two readings of one thing: where it stands and its state, field by field,
/// and the reasons and events the later one has that are newer than the earlier turn.
fn changes(a: &Value, b: &Value) -> Value {
    let (from, to) = (a["turn"].as_u64().unwrap_or(0), b["turn"].as_u64().unwrap_or(0));
    let newer = |v: &Value| -> Vec<Value> {
        let rows = v.as_array().into_iter().flatten();
        rows.filter(|r| r["turn"].as_u64().is_some_and(|t| t > from)).cloned().collect()
    };
    let pick = |v: &Value| json!({ "at": v["at"], "state": v["state"], "phase": v["phase"] });
    json!({
        "kind": b["kind"],
        "id": b["id"],
        "from": from,
        "to": to,
        "changes": inspect::diff(&pick(a), &pick(b)),
        "notes": newer(&b["notes"]),
        "events": newer(&b["events"]),
    })
}
