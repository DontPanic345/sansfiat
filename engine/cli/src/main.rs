use std::{env, fs, io::Read, process::exit, time::Instant};

use engine::data::{size_radius, DESERT, FOREST, HILLS, MARSH, MOUNTAIN, PREGAME, SEA, SIZES};
use engine::map::{self, Zone};
use engine::{
    civs, clear, land, pregame, round, stats, utility, Basket, Config, Params, Save, World, GOODS, N, TERRAIN,
};

const USAGE: &str = "\
sansfiat run [--seed N] [--turns N] [--radius N | --size small|medium|large] [--every N] [--save file.json] [--json | --events-json] [--no-events]
sansfiat map [--seed N] [--radius N | --size small|medium|large]   (the map as text, with counts)
sansfiat replay <file.json> [--json | --events-json] [--no-events]
sansfiat sweep [seed ...] [--turns N] [--radius N | --size small|medium|large|all]   (seeds default 1..=10, size small)
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
            Some(path) => replay(path, &args[2..]),
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
    let (mut turns, mut save, mut every) = (PREGAME.max_turns, None, 0u32);
    let mut out = Output::default();
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        if out.flag(flag) {
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
    let mut shown = 0;
    while w.turn < turns && w.end.is_none() {
        w.run(1);
        if out.events_text() {
            print_events(&w, &mut shown);
        }
        if every > 0 && w.turn.is_multiple_of(every) && out.text() {
            println!("{}", line(&stats::summary(&w)));
        }
    }
    if let Some(path) = save {
        fs::write(&path, serde_json::to_string(&w.save()).unwrap()).map_err(|e| format!("{path}: {e}"))?;
    }
    report(&w, &out);
    Ok(())
}

/// What `run` and `replay` print: the report, or the world as JSON, or the events as JSON; the
/// events in text between the summary lines unless `--no-events`.
#[derive(Default)]
struct Output {
    json: bool,
    events_json: bool,
    no_events: bool,
}

impl Output {
    /// Takes the flag if it is one of ours.
    fn flag(&mut self, flag: &str) -> bool {
        match flag {
            "--json" => self.json = true,
            "--events-json" => self.events_json = true,
            "--no-events" => self.no_events = true,
            _ => return false,
        }
        true
    }

    /// Whether the plain-text lines (summaries, the report) are printed.
    fn text(&self) -> bool {
        !self.json && !self.events_json
    }

    fn events_text(&self) -> bool {
        self.text() && !self.no_events
    }
}

/// Prints the events from index `*shown` on as `turn  text`.
fn print_events(w: &World, shown: &mut usize) {
    for e in &w.events[*shown..] {
        println!("{:>6}  {}", e.turn, e.text(w));
    }
    *shown = w.events.len();
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
        .chain([('^', "mountain"), (':', "desert"), ('%', "marsh"), ('r', "river"), ('z', "floodplain")])
        .chain([('V', "valley"), ('P', "pass"), ('S', "sealed valley"), ('N', "neck")])
        .map(|(c, n)| format!("{c} {n}"))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{legend}");
    let count = |terrain: usize| w.tiles.iter().filter(|t| t.terrain == terrain).count();
    let land = w.tiles.iter().filter(|t| TERRAIN[t.terrain].land).count();
    let lakes = w.tiles.iter().filter(|t| t.lake).count();
    let masses = map::landmasses(&w.tiles, config.radius).len();
    let rivers = w.tiles.iter().filter(|t| t.river > 0).count();
    let mouths = w.tiles.iter().filter(|t| t.flow.is_some_and(|j| w.tiles[j].terrain == SEA)).count();
    let flood = w.tiles.iter().filter(|t| t.flood && t.terrain != MARSH).count();
    let deltas = w.tiles.iter().filter(|t| t.flood && t.terrain == MARSH).count();
    let longest = w.tiles.iter().map(|t| t.river).max().unwrap_or(0);
    println!(
        "seed {} radius {}: {} tiles, {land} land, {} water ({lakes} lake), {} mountains, {} hills, {} deserts, {} forests, {masses} landmasses, \
         {rivers} river tiles ({mouths} mouths, longest {longest}), {flood} floodplain, {deltas} delta",
        config.seed,
        config.radius,
        w.tiles.len(),
        count(SEA),
        count(MOUNTAIN),
        count(HILLS),
        count(DESERT),
        count(FOREST)
    );
    let zones = |zone: Zone| map::zone_groups(&w.tiles, config.radius, zone).len();
    let passes = w.tiles.iter().filter(|t| t.pass).count();
    let sites = map::open_sites(&w.tiles, config.radius);
    println!(
        "closed country: {} valleys ({passes} passes), {} sealed valleys, {} necks; open sites at {sites:?}",
        zones(Zone::Valley),
        zones(Zone::Sealed),
        zones(Zone::Neck)
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

/// One run of the sweep.
struct Run {
    seed: u32,
    ended: u32,
    /// `centres`, `limit` or `limit+topup`; `none` if `--turns` stopped it first.
    how: &'static str,
    centres: usize,
    topped_up: usize,
    /// How many of those were existing markets, raised on tended land, raised on any grain land
    /// (told apart by when the market was founded and whether its tile is tamed).
    tiers: [usize; 3],
    cover: f64,
    clans: usize,
    people: f64,
    markets: usize,
    hearths: usize,
    /// Valleys, sealed valleys, necks.
    closed: [usize; 3],
    ms: f64,
}

fn sweep_run(seed: u32, radius: i32, turns: u32) -> Run {
    let start = Instant::now();
    let mut w = World::new(Config { seed, radius });
    while w.turn < turns && w.end.is_none() {
        w.run(1);
    }
    let ms = start.elapsed().as_secs_f64() * 1e3 / w.turn.max(1) as f64;
    let s = stats::summary(&w);
    let (ended, how, topped_up) = match &w.end {
        Some(e) => (
            e.turn,
            match (e.by_limit, e.topped_up.is_empty()) {
                (false, _) => "centres",
                (true, true) => "limit",
                (true, false) => "limit+topup",
            },
            e.topped_up.len(),
        ),
        None => (w.turn, "none", 0),
    };
    let mut tiers = [0; 3];
    if let Some(e) = &w.end {
        for id in &e.topped_up {
            let m = &w.markets[id];
            tiers[if m.founded < e.turn {
                0
            } else if w.tiles[m.tile].tame > 0 {
                1
            } else {
                2
            }] += 1;
        }
    }
    let zones = |zone: Zone| map::zone_groups(&w.tiles, radius, zone).len();
    Run {
        seed,
        ended,
        how,
        centres: s.centres.len(),
        topped_up,
        tiers,
        cover: s.cover,
        clans: s.clans,
        people: s.people,
        markets: s.markets,
        hearths: s.hearths,
        closed: [zones(Zone::Valley), zones(Zone::Sealed), zones(Zone::Neck)],
        ms,
    }
}

/// Runs seeds to their end (or `turns`) on each size asked for, one line each, then a summary per
/// size. Seeds run in parallel; the lines come out in seed order.
fn sweep(args: &[String]) -> Result<(), String> {
    let mut turns = PREGAME.max_turns;
    let mut seeds: Vec<u32> = Vec::new();
    let mut sizes: Vec<(String, i32)> = vec![(SIZES[0].0.to_string(), SIZES[0].1)];
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--turns" => turns = it.next().and_then(|v| v.parse().ok()).ok_or("--turns needs a number")?,
            "--radius" => {
                let r = it.next().and_then(|v| v.parse().ok()).ok_or("--radius needs a number")?;
                sizes = vec![(format!("radius {r}"), r)];
            }
            "--size" => match it.next().ok_or("--size needs a name")?.as_str() {
                "all" => sizes = SIZES.iter().map(|&(n, r)| (n.to_string(), r)).collect(),
                name => sizes = vec![(name.to_string(), size(name)?)],
            },
            s => seeds.push(s.parse().map_err(|_| format!("not a seed: {s}\n{USAGE}"))?),
        }
    }
    if seeds.is_empty() {
        seeds = (1..=10).collect();
    }
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    for (name, radius) in sizes {
        let next = std::sync::atomic::AtomicUsize::new(0);
        let runs: Vec<Run> = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..threads.min(seeds.len()))
                .map(|_| {
                    scope.spawn(|| {
                        let mut done = Vec::new();
                        loop {
                            let k = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            let Some(&seed) = seeds.get(k) else { break };
                            done.push((k, sweep_run(seed, radius, turns)));
                        }
                        done
                    })
                })
                .collect();
            let mut all: Vec<(usize, Run)> = workers.into_iter().flat_map(|h| h.join().unwrap()).collect();
            all.sort_by_key(|&(k, _)| k);
            all.into_iter().map(|(_, r)| r).collect()
        });
        for r in &runs {
            println!(
                "{name:<6} seed {:>3}: ended {:>3} by {:<11} centres {} topped up {} ({}/{}/{})  cover {:.2}  clans {:>3}  people {:>6.1}  markets {:>2}  hearths {:>3}  closed {}/{}/{}  {:.1} ms a turn",
                r.seed, r.ended, r.how, r.centres, r.topped_up, r.tiers[0], r.tiers[1], r.tiers[2], r.cover, r.clans, r.people, r.markets, r.hearths,
                r.closed[0], r.closed[1], r.closed[2], r.ms
            );
        }
        let count = |how: &str| runs.iter().filter(|r| r.how == how).count();
        let cover_short = runs.iter().filter(|r| r.how == "limit" && r.centres == PREGAME.players).count();
        let short = runs.iter().filter(|r| r.centres < PREGAME.players).count();
        let on_their_own = count("centres");
        let topped = count("limit+topup");
        let mut ends: Vec<u32> = runs.iter().map(|r| r.ended).collect();
        ends.sort_unstable();
        let tier = |k: usize| runs.iter().map(|r| r.tiers[k]).sum::<usize>();
        println!(
            "{name}: the top-up took {} existing markets, {} tended tiles, {} other grain tiles",
            tier(0),
            tier(1),
            tier(2)
        );
        println!(
            "{name}: {} of {} ended on their own, {} needed the top-up, {cover_short} ended on the limit with three centres already (cover short){}, median end turn {}",
            on_their_own,
            runs.len(),
            topped,
            if short > 0 { format!(", {short} with fewer than three centres") } else { String::new() },
            ends[ends.len() / 2]
        );
    }
    Ok(())
}

fn replay(path: &str, args: &[String]) -> Result<(), String> {
    let mut out = Output::default();
    for a in args {
        if !out.flag(a) {
            return Err(format!("unknown flag {a}\n{USAGE}"));
        }
    }
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let save: Save = serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
    let w = World::replay(&save)?;
    if out.events_text() {
        print_events(&w, &mut 0);
    }
    report(&w, &out);
    Ok(())
}

fn report(w: &World, out: &Output) {
    if out.events_json {
        let events: Vec<_> = w.events.iter().map(|e| e.json(w)).collect();
        println!("{}", serde_json::json!({ "events": events }));
        return;
    }
    if out.json {
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
    let cover = pregame::land_cover(w, &w.end.as_ref().map_or_else(|| pregame::centres(w), |e| e.centres.clone()));
    let end = match &w.end {
        Some(e) => {
            let tiles: Vec<String> = e
                .centres
                .iter()
                .map(|id| w.markets.get(id).map_or("gone".to_string(), |m| format!("{}", m.tile)))
                .collect();
            let how = if e.by_limit { "by the turn limit" } else { "by its centres" };
            let up = match e.topped_up.len() {
                0 => String::new(),
                n => format!(
                    ", {n} {} topped up (market {})",
                    if n == 1 { "centre" } else { "centres" },
                    e.topped_up.iter().map(u32::to_string).collect::<Vec<_>>().join(", market ")
                ),
            };
            format!("ended turn {} {how}{up}, centres at tiles [{}]", e.turn, tiles.join(", "))
        }
        None => "not ended".to_string(),
    };
    println!(
        "{} markets, {trading} clans trading, {hearths} hearths, {trails} trails, cover {cover:.2}, {end}",
        w.markets.len()
    );
    let (ids, what) = match &w.end {
        Some(e) if e.by_limit => (e.centres.clone(), "turn limit reached, centres (topped up included)"),
        Some(e) => (e.centres.clone(), "centres"),
        None => (pregame::centres(w), "not ended, centres so far"),
    };
    println!("{what}: {}", ids.len());
    print!("{}", centres_text(w, &ids));
    print!("{}", civs_text(w));
}

/// One line per civ: name, capital, market, clans, people, store and hearths on its land.
fn civs_text(w: &World) -> String {
    let mut out = String::new();
    for c in civs::cards(w) {
        let civ = &w.civs[&c.civ];
        let store: f64 = civ.store.iter().sum();
        out += &format!(
            "  civ {} at ({}, {}), market {}: {} clans, {:.1} people, store {store:.1}, {} hearths on its land\n",
            c.name, c.q, c.r, civ.market, c.clans, c.people, c.hearths
        );
    }
    out
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
