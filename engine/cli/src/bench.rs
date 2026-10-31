use crate::{size, USAGE};
use engine::{Config, World};
use std::time::Instant;

/// Times `turns` turns from a fresh world for each seed.
pub(crate) fn bench(args: &[String]) -> Result<(), String> {
    let mut turns = 200u32;
    let mut seeds: Vec<u32> = Vec::new();
    let mut radius = Config::default().radius;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--turns" => turns = it.next().and_then(|v| v.parse().ok()).ok_or("--turns needs a number")?,
            "--radius" => radius = it.next().and_then(|v| v.parse().ok()).ok_or("--radius needs a number")?,
            "--size" => radius = size(it.next().ok_or("--size needs a name")?)?,
            s => seeds.push(s.parse().map_err(|_| format!("not a seed: {s}\n{USAGE}"))?),
        }
    }
    if seeds.is_empty() {
        seeds = vec![1, 2, 3];
    }
    for seed in seeds {
        let start = Instant::now();
        let mut w = World::new(Config { seed, radius, army: true, gifts: true, zone: true });
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
