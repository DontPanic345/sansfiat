use std::{env, fs, io::Read, process::exit};

use engine::{clear, utility, Basket, Config, Params, World, GOODS, N};

const USAGE: &str = "\
sansfiat run [--seed N] [--turns N] [--radius N] [--clans N] [--markets N] [--reach N] [--every N] [--json]
sansfiat clear <file.json | ->   {\"baskets\": [{\"grain\": 4}, {\"fish\": 2}], \"prices\"?: {\"grain\": 1}}
sansfiat goods";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("run") => run(&args[1..]),
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
    let (mut turns, mut every, mut json) = (50u32, 10u32, false);
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
            "--clans" => config.clans = value.parse().map_err(bad)?,
            "--markets" => config.markets = value.parse().map_err(bad)?,
            "--reach" => config.reach = value.parse().map_err(bad)?,
            "--every" => every = value.parse().map_err(bad)?,
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }

    let mut w = World::new(config);
    if json {
        w.run(turns);
        println!("{}", serde_json::to_string(&w).unwrap());
        return Ok(());
    }

    let trading = w.clans.iter().filter(|c| c.market.is_some()).count();
    println!(
        "seed {}: {} tiles, {} clans ({} in reach of a market), {} markets",
        config.seed,
        w.tiles.len(),
        w.clans.len(),
        trading,
        w.markets.len()
    );
    println!("{:>5} {:>6}  {}", "turn", "gain", header());
    for _ in 0..turns {
        w.step();
        if w.turn % every.max(1) != 0 && w.turn != turns {
            continue;
        }
        let t = w.history.last().unwrap();
        let gain = 100.0 * (t.traded - t.autarky) / t.autarky;
        for m in &w.markets {
            let lead = if m.id == 0 { format!("{:>5} {:>5.1}%", w.turn, gain) } else { " ".repeat(12) };
            let state = if m.cleared { format!("{} traders", m.traders) } else { "no clearing".into() };
            println!("{lead}  {}  market {} {state}", row(&m.prices), m.id);
        }
    }
    Ok(())
}

fn header() -> String {
    GOODS.iter().map(|g| format!("{g:>7}")).collect::<Vec<_>>().join(" ")
}

fn row(b: &Basket) -> String {
    b.iter().map(|v| format!("{v:>7.3}")).collect::<Vec<_>>().join(" ")
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
