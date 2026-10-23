use std::{env, fs, io::Read, process::exit, time::Instant};

use engine::{clear, utility, Basket, Config, Params, Save, World, GOODS, N, TERRAIN};

const USAGE: &str = "\
sansfiat run [--seed N] [--turns N] [--radius N] [--save file.json] [--json]
sansfiat replay <file.json> [--json]
sansfiat bench [seed ...] [--turns N] [--radius N]
sansfiat clear <file.json | ->   {\"baskets\": [{\"grain\": 4}, {\"fish\": 2}], \"prices\"?: {\"grain\": 1}}
sansfiat goods";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("run") => run(&args[1..]),
        Some("replay") => match args.get(1) {
            Some(path) => replay(path, args.iter().any(|a| a == "--json")),
            None => Err(USAGE.to_string()),
        },
        Some("bench") => bench(&args[1..]),
        Some("clear") => clear_file(args.get(1).map(String::as_str).unwrap_or("-")),
        Some("goods") => {
            println!("{}", GOODS.join(" "));
            Ok(())
        }
        _ => Err(USAGE.to_string()),
    };
    if let Err(e) = result {
        eprintln!("{e}");
        exit(2);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let mut config = Config::default();
    let (mut turns, mut json, mut save) = (50u32, false, None);
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        if flag == "--json" {
            json = true;
            continue;
        }
        let value = it.next().ok_or(format!("{flag} needs a value\n{USAGE}"))?;
        let bad = |_| format!("{flag}: not a number: {value}");
        match flag.as_str() {
            "--seed" => config.seed = value.parse().map_err(bad)?,
            "--turns" => turns = value.parse().map_err(bad)?,
            "--radius" => config.radius = value.parse().map_err(bad)?,
            "--save" => save = Some(value.clone()),
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let mut w = World::new(config);
    w.run(turns);
    if let Some(path) = save {
        fs::write(&path, serde_json::to_string(&w.save()).unwrap()).map_err(|e| format!("{path}: {e}"))?;
    }
    report(&w, json);
    Ok(())
}

fn replay(path: &str, json: bool) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let save: Save = serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
    report(&World::replay(&save)?, json);
    Ok(())
}

fn report(w: &World, json: bool) {
    if json {
        println!("{}", serde_json::to_string(w).unwrap());
        return;
    }
    let land = w.tiles.iter().filter(|t| TERRAIN[t.terrain].land).count();
    println!(
        "seed {} radius {}: turn {}, {} tiles ({} land), {} actions logged",
        w.config.seed,
        w.config.radius,
        w.turn,
        w.tiles.len(),
        land,
        w.log.len()
    );
    let people: f64 = w.clans.values().map(|c| c.pop).sum();
    let fed = w.clans.values().map(|c| c.fed_ratio).sum::<f64>() / w.clans.len().max(1) as f64;
    let held = w.tiles.iter().filter(|t| t.claim.is_some()).count();
    println!("{} clans, {people:.1} people, mean fed ratio {fed:.2}, {held} tiles held", w.clans.len());
}

/// Times `turns` turns from a fresh world for each seed.
fn bench(args: &[String]) -> Result<(), String> {
    let mut turns = 200u32;
    let mut seeds: Vec<u32> = Vec::new();
    let mut radius = Config::default().radius;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--turns" => turns = it.next().and_then(|v| v.parse().ok()).ok_or("--turns needs a number")?,
            "--radius" => radius = it.next().and_then(|v| v.parse().ok()).ok_or("--radius needs a number")?,
            s => seeds.push(s.parse().map_err(|_| format!("not a seed: {s}\n{USAGE}"))?),
        }
    }
    if seeds.is_empty() {
        seeds = vec![1, 2, 3];
    }
    for seed in seeds {
        let start = Instant::now();
        let mut w = World::new(Config { seed, radius });
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

fn basket_of(v: &serde_json::Value, what: &str) -> Result<Basket, String> {
    let obj = v.as_object().ok_or(format!("{what} must be an object of good -> amount"))?;
    let mut b = [0.0; N];
    for (k, amount) in obj {
        let g = GOODS.iter().position(|g| g == k).ok_or(format!("unknown good {k}; goods: {}", GOODS.join(" ")))?;
        b[g] = amount.as_f64().ok_or(format!("{what}.{k} must be a number"))?;
    }
    Ok(b)
}

fn clear_file(path: &str) -> Result<(), String> {
    let mut text = String::new();
    if path == "-" {
        std::io::stdin().read_to_string(&mut text).map_err(|e| e.to_string())?;
    } else {
        text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    }
    let input: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let baskets = input["baskets"]
        .as_array()
        .ok_or("expected {\"baskets\": [...]}")?
        .iter()
        .enumerate()
        .map(|(i, b)| basket_of(b, &format!("baskets[{i}]")))
        .collect::<Result<Vec<_>, _>>()?;
    let start = match input.get("prices") {
        Some(p) => basket_of(p, "prices")?.map(|v| if v > 0.0 { v } else { 1.0 }),
        None => [1.0; N],
    };

    let c = clear(&baskets, &start, &Params::default());
    let name = |g: usize| GOODS[g];
    println!("cleared {} after {} iterations", c.cleared, c.iterations);
    println!("{:>8}  {}", "", c.goods.iter().map(|&g| format!("{:>7}", name(g))).collect::<Vec<_>>().join(" "));
    let pick = |b: &Basket| c.goods.iter().map(|&g| format!("{:>7.3}", b[g])).collect::<Vec<_>>().join(" ");
    println!("{:>8}  {}", "prices", pick(&c.prices));
    println!("{:>8}  {}", "volume", pick(&c.volume));
    for (i, (b, a)) in baskets.iter().zip(&c.after).enumerate() {
        println!("{:>8}  {}  ->  {}  utility {:.3} -> {:.3}", i, pick(b), pick(a), utility(b), utility(a));
    }
    Ok(())
}
