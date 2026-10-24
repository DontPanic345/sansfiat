use std::{env, fs, io::Read, process::exit, time::Instant};

use engine::data::{size_radius, DESERT, FOREST, HILLS, MOUNTAIN, PREGAME, SEA, SIZES};
use engine::map;
use engine::{clear, land, pregame, round, stats, utility, Basket, Config, Params, Save, World, GOODS, N, TERRAIN};

const USAGE: &str = "\
sansfiat run [--seed N] [--turns N] [--radius N | --size small|medium|large] [--every N] [--save file.json] [--json]
sansfiat map [--seed N] [--radius N | --size small|medium|large]   (the map as text, with counts)
sansfiat replay <file.json> [--json]
sansfiat sweep [seed ...] [--turns N] [--radius N | --size NAME]   (seeds default 1..=10)
sansfiat bench [seed ...] [--turns N] [--radius N | --size NAME]
sansfiat digest [--seed N] [--radius N | --size NAME] [--turns N]   (prints turn and digest, from turn 0)
sansfiat clear <file.json | ->   {\"baskets\": [{\"grain\": 4}, {\"fish\": 2}], \"prices\"?: {\"grain\": 1}}
sansfiat goods";

/// The radius of the map size called `name`.
fn size(name: &str) -> Result<i32, String> {
    size_radius(name).ok_or_else(|| {
        format!("unknown size {name}; sizes: {}", SIZES.iter().map(|s| s.0).collect::<Vec<_>>().join(" "))
    })
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("run") => run(&args[1..]),
        Some("replay") => match args.get(1) {
            Some(path) => replay(path, args.iter().any(|a| a == "--json")),
            None => Err(USAGE.to_string()),
        },
        Some("sweep") => sweep(&args[1..]),
        Some("bench") => bench(&args[1..]),
        Some("digest") => digest(&args[1..]),
        Some("map") => map(&args[1..]),
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
    let (mut turns, mut json, mut save, mut every) = (PREGAME.max_turns, false, None, 0u32);
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
            "--size" => config.radius = size(value)?,
            "--every" => every = value.parse().map_err(bad)?,
            "--save" => save = Some(value.clone()),
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let mut w = World::new(config);
    while w.turn < turns && w.end.is_none() {
        w.run(1);
        if every > 0 && w.turn.is_multiple_of(every) && !json {
            println!("{}", line(&stats::summary(&w)));
        }
    }
    if let Some(path) = save {
        fs::write(&path, serde_json::to_string(&w.save()).unwrap()).map_err(|e| format!("{path}: {e}"))?;
    }
    report(&w, json);
    Ok(())
}

/// Prints the map as text, a legend and counts.
fn map(args: &[String]) -> Result<(), String> {
    let mut config = Config::default();
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let value = it.next().ok_or(format!("{flag} needs a value\n{USAGE}"))?;
        let bad = |_| format!("{flag}: not a number: {value}");
        match flag.as_str() {
            "--seed" => config.seed = value.parse().map_err(bad)?,
            "--radius" => config.radius = value.parse().map_err(bad)?,
            "--size" => config.radius = size(value)?,
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let w = World::new(config);
    print!("{}", map::ascii(&w.tiles, config.radius));
    let legend = [('~', "sea"), ('=', "lake"), (',', "coast"), ('.', "plains"), ('f', "forest"), ('h', "hills")]
        .into_iter()
        .chain([('^', "mountain"), (':', "desert"), ('%', "marsh")])
        .map(|(c, n)| format!("{c} {n}"))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{legend}");
    let count = |terrain: usize| w.tiles.iter().filter(|t| t.terrain == terrain).count();
    let land = w.tiles.iter().filter(|t| TERRAIN[t.terrain].land).count();
    let lakes = w.tiles.iter().filter(|t| t.lake).count();
    let masses = map::landmasses(&w.tiles, config.radius).len();
    println!(
        "seed {} radius {}: {} tiles, {land} land, {} water ({lakes} lake), {} mountains, {} hills, {} deserts, {} forests, {masses} landmasses",
        config.seed,
        config.radius,
        w.tiles.len(),
        count(SEA),
        count(MOUNTAIN),
        count(HILLS),
        count(DESERT),
        count(FOREST)
    );
    Ok(())
}

/// Prints `<turn> <digest>` for turn 0 and after each of `--turns` turns (400), end or not.
fn digest(args: &[String]) -> Result<(), String> {
    let mut config = Config::default();
    let mut turns = 400u32;
    let mut it = args.iter();
    while let Some(flag) = it.next() {
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
    println!("{} {}", w.turn, stats::digest(&w));
    for _ in 0..turns {
        w.run(1);
        println!("{} {}", w.turn, stats::digest(&w));
    }
    Ok(())
}

/// One summary line.
fn line(s: &stats::Summary) -> String {
    format!(
        "turn {:>4}  clans {:>3}  people {:>6.1}  fed {:.2}  held {:>3}  trails {:>3}  hearths {:>3}  markets {:>2}  trading {:>2}  centres {}  cover {:.2}",
        s.turn, s.clans, s.people, s.mean_fed, s.held, s.trails, s.hearths, s.markets, s.trading, s.centres.len(), s.cover
    )
}

/// The centres' report: one line each, then the distances between them.
fn centres_text(w: &World, ids: &[u32]) -> String {
    let mut out = String::new();
    for c in stats::centre_report(w, ids) {
        out += &format!(
            "  centre market {} at ({}, {}): {} traders, age {}, {} hearths in reach, fertility near {:.2} vs land {:.2}\n",
            c.market, c.q, c.r, c.traders, c.age, c.hearths_in_reach, c.fertility_near, c.fertility_land
        );
    }
    let d = stats::distances(w, ids);
    if !d.is_empty() {
        out +=
            &format!("  steps between centres: {}\n", d.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(", "));
    }
    out
}

/// Runs seeds to their end (or `turns`), one line each, then totals.
fn sweep(args: &[String]) -> Result<(), String> {
    let mut turns = PREGAME.max_turns;
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
        seeds = (1..=10).collect();
    }
    let (mut by_centres, mut ends, mut ratios) = (0, Vec::new(), Vec::new());
    for &seed in &seeds {
        let start = Instant::now();
        let mut w = World::new(Config { seed, radius });
        while w.turn < turns && w.end.is_none() {
            w.run(1);
        }
        let ms = start.elapsed().as_secs_f64() * 1e3 / w.turn.max(1) as f64;
        let s = stats::summary(&w);
        let (ended, how) = match &w.end {
            Some(e) => (e.turn, if e.by_limit { "limit" } else { "centres" }),
            None => (w.turn, "none"),
        };
        if how == "centres" {
            by_centres += 1;
        }
        ends.push(ended);
        let reports = stats::centre_report(&w, &s.centres);
        let ratio = stats::mean_ratio(&reports);
        ratios.extend(ratio);
        println!(
            "seed {seed:>3}: ended {ended:>3} by {how:<7}  clans {:>3}  people {:>6.1}  markets {:>2}  hearths {:>3}  cover {:.2}  fertility ratio {}  {ms:.1} ms a turn",
            s.clans,
            s.people,
            s.markets,
            s.hearths,
            s.cover,
            ratio.map_or("-".to_string(), |r| format!("{r:.2}")),
        );
    }
    ends.sort_unstable();
    let ratio = if ratios.is_empty() {
        "-".to_string()
    } else {
        format!("{:.2}", ratios.iter().sum::<f64>() / ratios.len() as f64)
    };
    println!(
        "{by_centres} of {} ended by centres, median end turn {}, mean fertility ratio {ratio}",
        seeds.len(),
        ends[ends.len() / 2]
    );
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
    let n = w.clans.len().max(1) as f64;
    let tiles = w.clans.values().map(|c| c.round.len() as f64).sum::<f64>() / n;
    let walk = w.clans.values().map(|c| round::cost(w, &c.round)).sum::<f64>() / n;
    println!(
        "{} clans, {people:.1} people, mean fed ratio {fed:.2}, {held} tiles held, mean round {tiles:.1} tiles costing {walk:.1}",
        w.clans.len()
    );
    let trading = w.clans.values().filter(|c| c.market.is_some()).count();
    let hearths = w.tiles.iter().filter(|t| land::is_hearth(t)).count();
    let trails = w.tiles.iter().filter(|t| t.trail).count();
    let cover = pregame::land_cover(w, &pregame::centres(w));
    let end = match &w.end {
        Some(e) => {
            let tiles: Vec<String> = e
                .centres
                .iter()
                .map(|id| w.markets.get(id).map_or("gone".to_string(), |m| format!("{}", m.tile)))
                .collect();
            let how = if e.by_limit { "by the turn limit" } else { "by its centres" };
            format!("ended turn {} {how}, centres at tiles [{}]", e.turn, tiles.join(", "))
        }
        None => "not ended".to_string(),
    };
    println!(
        "{} markets, {trading} clans trading, {hearths} hearths, {trails} trails, cover {cover:.2}, {end}",
        w.markets.len()
    );
    let (ids, what) = match &w.end {
        Some(e) if e.by_limit => (e.centres.clone(), "turn limit reached, centres it had"),
        Some(e) => (e.centres.clone(), "centres"),
        None => (pregame::centres(w), "not ended, centres so far"),
    };
    println!("{what}: {}", ids.len());
    print!("{}", centres_text(w, &ids));
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
            "--size" => radius = size(it.next().ok_or("--size needs a name")?)?,
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
