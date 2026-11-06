use crate::retro::pregame;
use crate::{size, USAGE};
use engine::{Action, Config, World};
use std::time::Instant;

/// Times `turns` turns from a fresh world for each seed; with `after`, times that many turns past
/// the pick instead, the pre-game run and timed on its own.
pub(crate) fn bench(args: &[String]) -> Result<(), String> {
    let mut turns: Option<u32> = None;
    let mut after: Option<u32> = None;
    let mut seeds: Vec<u32> = Vec::new();
    let mut radius = Config::default().radius;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--turns" => turns = Some(it.next().and_then(|v| v.parse().ok()).ok_or("--turns needs a number")?),
            "--after" => after = Some(it.next().and_then(|v| v.parse().ok()).ok_or("--after needs a number")?),
            "--radius" => radius = it.next().and_then(|v| v.parse().ok()).ok_or("--radius needs a number")?,
            "--size" => radius = size(it.next().ok_or("--size needs a name")?)?,
            s => seeds.push(s.parse().map_err(|_| format!("not a seed: {s}\n{USAGE}"))?),
        }
    }
    if turns.is_some() && after.is_some() {
        return Err(format!("--turns and --after are two benches: give one\n{USAGE}"));
    }
    if seeds.is_empty() {
        seeds = vec![1, 2, 3];
    }
    for seed in seeds {
        let config = Config { seed, radius, army: true, gifts: true, zone: true, ..Config::default() };
        if let Some(after) = after {
            bench_after(config, after)?;
            continue;
        }
        let turns = turns.unwrap_or(200);
        let start = Instant::now();
        let mut w = World::new(config);
        let built = start.elapsed();
        w.run(turns);
        let total = start.elapsed();
        println!(
            "seed {seed}: world {:.1} ms, {turns} turns {:.1} ms ({:.3} ms a turn)",
            built.as_secs_f64() * 1e3,
            (total - built).as_secs_f64() * 1e3,
            (total - built).as_secs_f64() * 1e3 / turns.max(1) as f64
        );
    }
    Ok(())
}

/// One seed: the world built, the pre-game run to the pick, no civ picked, then `after` turns timed.
fn bench_after(config: Config, after: u32) -> Result<(), String> {
    let seed = config.seed;
    let start = Instant::now();
    let world = World::new(config);
    let built = start.elapsed();
    let mut w = pregame(world);
    let pregame_time = start.elapsed() - built;
    w.act(Action::ChooseCiv { civ: None }).map_err(|e| format!("seed {seed}: {e}"))?;
    let timed = Instant::now();
    w.run(after);
    let took = timed.elapsed();
    println!(
        "seed {seed}: world {:.1} ms, pre-game {:.1} ms, {after} turns past the pick {:.1} ms ({:.3} ms a turn)",
        built.as_secs_f64() * 1e3,
        pregame_time.as_secs_f64() * 1e3,
        took.as_secs_f64() * 1e3,
        took.as_secs_f64() * 1e3 / after.max(1) as f64
    );
    Ok(())
}
