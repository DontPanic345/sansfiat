use crate::{size, USAGE};
use engine::script::Lead;
use engine::{stats, Action, Config, World};

/// Prints `<turn> <digest>` for turn 0 and after each of `--turns` turns (400), end or not; `--state`
/// prints `stats::state_digest` instead.
pub(crate) fn digest(args: &[String]) -> Result<(), String> {
    let mut config = Config::default();
    let mut turns = 400u32;
    let mut lead = false;
    let mut state = false;
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        if flag == "--lead" {
            lead = true;
            continue;
        }
        if flag == "--state" {
            state = true;
            continue;
        }
        let value = it.next().ok_or(format!("{flag} needs a value\n{USAGE}"))?;
        let bad = |_| format!("{flag}: not a number: {value}");
        match flag.as_str() {
            "--seed" => config.seed = value.parse().map_err(bad)?,
            "--radius" => config.radius = value.parse().map_err(bad)?,
            "--size" => config.radius = size(value)?,
            "--turns" => turns = value.parse().map_err(bad)?,
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let mut w = World::new(config);
    let hash = |w: &World| if state { stats::state_digest(w) } else { stats::digest(w) };
    println!("{} {}", w.turn, hash(&w));
    let mut script = Lead::default();
    for _ in 0..turns {
        if lead {
            script.orders(&mut w);
            w.act(Action::EndTurn).expect("ending a turn is allowed once the orders are given");
        } else {
            w.run(1);
        }
        println!("{} {}", w.turn, hash(&w));
    }
    Ok(())
}
