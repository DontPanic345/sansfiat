use std::{env, fs, io::Read, process::exit, time::Instant};

use engine::data::{size_radius, DESERT, FOREST, HILLS, LEANING, MARSH, MOUNTAIN, PREGAME, SEA, SIZES, UNIT};
use engine::land::is_depleted;
use engine::map::{self, Zone};
use engine::units;
use engine::{
    civs, clear, land, pregame, stats, utility, Action, Basket, Config, Params, Phase, Save, World, GOODS, N, TERRAIN,
};

const USAGE: &str = "\
sansfiat run [--seed N] [--turns N] [--radius N | --size small|medium|large] [--every N] [--save file.json] [--json | --events-json] [--no-events] [--why ID]   (--why prints the reasons of a clan, market, civ or unit, newest first)
sansfiat map [--seed N] [--radius N | --size small|medium|large]   (the map as text, with counts)
sansfiat replay <file.json> [--json | --events-json] [--no-events]
sansfiat sweep [seed ...] [--turns N] [--radius N | --size small|medium|large|all] [--every N]   (seeds default 1..=10, size small; a series line every N turns, default 50)
sansfiat sweep --after N [--maps retro | seed ... [--size NAME]] [--no-army] [--no-gifts] [--no-zone] [--lever G]   (the camps and the clans: run to the pick, all on the AI, N turns on; per civ its camp, the clans that joined and the totals; --no-army, --no-gifts and --no-zone (AI camps paint no zone) are the controls; --lever G leads the lowest civ with gift G, its camp posted where the AI would; --maps retro is the seven maps)
sansfiat bench [seed ...] [--turns N] [--radius N | --size NAME]
sansfiat digest [--seed N] [--radius N | --size NAME] [--turns N] [--lead]   (prints turn and digest, from turn 0; --lead plays the scripted game: picks the first civ and gives orders)
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
    let (mut turns, mut save, mut every, mut why) = (PREGAME.max_turns, None, 0u32, None);
    // An explicit `--turns` runs on past the pre-game's end; by default the run stops there.
    let mut explicit = false;
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
            "--turns" => {
                turns = value.parse().map_err(bad)?;
                explicit = true;
            }
            "--radius" => config.radius = value.parse().map_err(bad)?,
            "--size" => config.radius = size(value)?,
            "--every" => every = value.parse().map_err(bad)?,
            "--save" => save = Some(value.clone()),
            "--why" => why = Some(value.parse().map_err(bad)?),
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let mut w = World::new(config);
    let mut shown = 0;
    while w.turn < turns && (explicit || w.end.is_none()) {
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
    if let (Some(id), true) = (why, out.text()) {
        print_notes(&w, id);
    }
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

/// Prints the events from index `*shown` on as `turn  text`, an alert marked `!`.
fn print_events(w: &World, shown: &mut usize) {
    for e in &w.events[*shown..] {
        let mark = if e.alert { "!" } else { " " };
        println!("{:>6} {mark} {}", e.turn, e.text(w));
    }
    *shown = w.events.len();
}

/// Prints the reasons of subject `id`, newest first, as `turn  text`.
fn print_notes(w: &World, id: u32) {
    match w.notes.get(&id).filter(|list| !list.is_empty()) {
        Some(list) => {
            for n in list.iter().rev() {
                println!("{:>6}  {}", n.turn, n.text);
            }
        }
        None => println!("no reasons for {id}"),
    }
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
    let mut lead = false;
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        if flag == "--lead" {
            lead = true;
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
    println!("{} {}", w.turn, stats::digest(&w));
    let mut pick = None;
    for _ in 0..turns {
        if lead {
            lead_orders(&mut w, &mut pick);
            w.act(Action::EndTurn).expect("ending a turn is allowed once the orders are given");
        } else {
            w.run(1);
        }
        println!("{} {}", w.turn, stats::digest(&w));
    }
    Ok(())
}

/// The scripted game of `digest --lead`, between turns and before each `EndTurn`; `cross-build.mjs`
/// plays the same rule. `pick` is the turn `ChooseCiv` was acted, and `n` the turns since.
/// 1. At `Choose`: lead the lowest civ id.
/// 2. In `Main`, when `n > 0 && n.is_multiple_of(20)`: `Raise { n: 1 }` (a refusal is ignored).
/// 3. When `n == 30`: `SetSwitches` both off; when `n == 40`: both on. When `n > 0 &&
///    n.is_multiple_of(25)`: `Resupply` (a refusal is ignored).
/// 4. If the player's camp has no `dest`: `MoveCamp` to the first accepted of the other civs'
///    capitals in civ id order, then its own capital.
/// 5. When `n == 60`: `Disband { n: Some(1) }`, if there is a camp.
/// 6. When `n == 10`: `SetGift` twice the default (`2 × LEANING.gift_default`); when `n == 50`:
///    `SetKeep { food: Some(5.0) }`; when `n == 70`: `SetKeep { food: None }`.
/// 7. When `n == 2`: `SetRoute` for the scout from the capital to the first forest or hills tile by
///    index seven steps out (ignored when there is none).
/// 8. When `n == 45`, if there is a camp: `MoveCamp` to its own tile (it halts), then `Zone` over the
///    first `people − 1` tiles of its line (the capital left out; a refusal is ignored). Rule 4 rests
///    while `45 <= n < 55`, so the zone spreads and is held.
fn lead_orders(w: &mut World, pick: &mut Option<u32>) {
    if w.phase == Phase::Choose {
        let first = w.civs.keys().next().copied();
        w.act(Action::ChooseCiv { civ: first }).expect("a civ can be chosen at the pick");
        *pick = Some(w.turn);
    }
    let (Some(pick), Some(me)) = (*pick, w.player) else { return };
    if w.phase != Phase::Main {
        return;
    }
    let n = w.turn - pick;
    if n > 0 && n.is_multiple_of(20) {
        let _ = w.act(Action::Raise { n: 1 });
    }
    if n == 30 {
        let _ = w.act(Action::SetSwitches { forage: false, supply: false });
    }
    if n == 40 {
        let _ = w.act(Action::SetSwitches { forage: true, supply: true });
    }
    if n > 0 && n.is_multiple_of(25) {
        let _ = w.act(Action::Resupply);
    }
    let targets: Vec<usize> =
        w.civs.values().filter(|c| c.id != me).map(|c| c.capital).chain(std::iter::once(w.civs[&me].capital)).collect();
    if n == 2 {
        // The scout searches for a rough tile seven steps out: the first forest or hills tile by index.
        let cap = w.civs[&me].capital;
        let target = (0..w.tiles.len()).find(|&i| {
            matches!(w.tiles[i].terrain, FOREST | HILLS) && w.tiles[i].hex().distance(w.tiles[cap].hex()) == 7
        });
        if let Some(to) = target {
            let _ = w.act(Action::SetRoute { from: cap, to });
        }
    }
    if !(45..55).contains(&n)
        && w.units.values().any(|u| u.civ == me && u.dest.is_none() && u.kind == units::UnitKind::Camp)
    {
        for &to in &targets {
            if w.act(Action::MoveCamp { to }).is_ok() {
                break;
            }
        }
    }
    if n == 45 {
        if let Some((id, home, people)) = units::camp(w, me).map(|u| (u.id, u.home, u.people)) {
            let _ = w.act(Action::MoveCamp { to: home });
            if let Some((path, _, _)) = units::line(w, id) {
                let capital = w.civs[&me].capital;
                let k = (people as usize).saturating_sub(1);
                let tiles: Vec<usize> = path.into_iter().filter(|&t| t != capital).take(k).collect();
                let _ = w.act(Action::Zone { tiles });
            }
        }
    }
    if n == 60 {
        let _ = w.act(Action::Disband { n: Some(1) });
    }
    if n == 10 {
        let _ = w.act(Action::SetGift { food: 2.0 * LEANING.gift_default });
    }
    if n == 50 {
        let _ = w.act(Action::SetKeep { food: Some(5.0) });
    }
    if n == 70 {
        let _ = w.act(Action::SetKeep { food: None });
    }
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
    /// How many of those were clans' homes, existing markets, raised on tended land, raised on any
    /// grain land (told apart by the site's clan, when the market was founded and whether its tile
    /// is tamed).
    tiers: [usize; 4],
    cover: f64,
    clans: usize,
    people: f64,
    /// Tiles depleted at the end.
    depleted: usize,
    markets: usize,
    hearths: usize,
    /// Valleys, sealed valleys, necks.
    closed: [usize; 3],
    ms: f64,
    /// The series: a point every `--every` turns from turn 0, and the last turn.
    series: Vec<Point>,
    /// The change in people over the last `LEVEL_TURNS` turns before the end, as a share; `None`
    /// on a run shorter than that.
    level: Option<f64>,
}

/// One point of a run's series. Later columns (people moved) go here.
struct Point {
    turn: u32,
    clans: usize,
    people: f64,
    depleted: usize,
    /// People moved at festivals, running total.
    moved: f64,
}

impl Point {
    fn of(w: &World) -> Point {
        Point {
            turn: w.turn,
            clans: w.clans.len(),
            people: w.clans.values().map(|c| c.people).sum(),
            depleted: w.tiles.iter().filter(|t| is_depleted(t)).count(),
            moved: w.moved,
        }
    }
}

/// Turns before the end over which people should have levelled off.
const LEVEL_TURNS: usize = 100;

fn sweep_run(seed: u32, radius: i32, turns: u32, every: u32) -> Run {
    let start = Instant::now();
    let mut w = World::new(Config { seed, radius, army: true, gifts: true, zone: true });
    let mut series = vec![Point::of(&w)];
    // People after each turn, from turn 0.
    let mut people = vec![series[0].people];
    while w.turn < turns && w.end.is_none() {
        w.run(1);
        let p = Point::of(&w);
        people.push(p.people);
        if w.turn.is_multiple_of(every) || w.turn >= turns || w.end.is_some() {
            series.push(p);
        }
    }
    let level = people.len().checked_sub(LEVEL_TURNS + 1).map(|k| {
        let (then, now) = (people[k], people[people.len() - 1]);
        if then > 0.0 {
            (now - then) / then
        } else {
            0.0
        }
    });
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
    let mut tiers = [0; 4];
    if let Some(e) = &w.end {
        for id in &e.topped_up {
            let m = &w.markets[id];
            let k = e.centres.iter().position(|c| c == id).unwrap();
            tiers[if e.clans.get(k).copied().flatten().is_some() {
                0
            } else if m.founded < e.turn {
                1
            } else if w.tiles[m.tile].tame > 0 {
                2
            } else {
                3
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
        depleted: s.depleted,
        markets: s.markets,
        hearths: s.hearths,
        closed: [zones(Zone::Valley), zones(Zone::Sealed), zones(Zone::Neck)],
        ms,
        series,
        level,
    }
}

/// Runs seeds to their end (or `turns`) on each size asked for, one line each, then a summary per
/// size. Seeds run in parallel; the lines come out in seed order.
fn sweep(args: &[String]) -> Result<(), String> {
    let mut turns = PREGAME.max_turns;
    let mut every = 50u32;
    let mut seeds: Vec<u32> = Vec::new();
    let mut sizes: Vec<(String, i32)> = vec![(SIZES[0].0.to_string(), SIZES[0].1)];
    let (mut after, mut army, mut retro) = (None, true, false);
    let (mut gifts, mut lever): (bool, Option<f64>) = (true, None);
    let mut zone = true;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--after" => after = Some(it.next().and_then(|v| v.parse().ok()).ok_or("--after needs a number")?),
            "--no-army" => army = false,
            "--no-gifts" => gifts = false,
            "--no-zone" => zone = false,
            "--lever" => {
                lever = Some(
                    it.next()
                        .and_then(|v| v.parse().ok())
                        .filter(|g: &f64| g.is_finite())
                        .ok_or("--lever needs a number")?,
                )
            }
            "--maps" => match it.next().map(String::as_str) {
                Some("retro") => retro = true,
                _ => return Err(format!("--maps needs retro\n{USAGE}")),
            },
            "--turns" => turns = it.next().and_then(|v| v.parse().ok()).ok_or("--turns needs a number")?,
            "--every" => {
                every = it.next().and_then(|v| v.parse().ok()).filter(|&n| n > 0).ok_or("--every needs a number")?
            }
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
    if let Some(after) = after {
        let maps = if retro {
            if !seeds.is_empty() {
                return Err(format!("--maps retro names its own seeds\n{USAGE}"));
            }
            retro_maps()?
        } else {
            if seeds.is_empty() {
                seeds = (1..=10).collect();
            }
            sizes.iter().flat_map(|(name, radius)| seeds.iter().map(move |&s| (name.clone(), *radius, s))).collect()
        };
        return sweep_after(&maps, after, army, gifts, zone, lever);
    }
    if !army || !gifts || !zone || lever.is_some() || retro {
        return Err(format!("--no-army, --no-gifts, --no-zone, --lever and --maps retro need --after N\n{USAGE}"));
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
                            done.push((k, sweep_run(seed, radius, turns, every)));
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
                "{name:<6} seed {:>3}: ended {:>3} by {:<11} centres {} topped up {} ({}/{}/{}/{})  cover {:.2}  clans {:>3}  people {:>6.1}  depleted {:>4}  markets {:>2}  hearths {:>3}  closed {}/{}/{}  {:.1} ms a turn",
                r.seed, r.ended, r.how, r.centres, r.topped_up, r.tiers[0], r.tiers[1], r.tiers[2], r.tiers[3], r.cover, r.clans, r.people, r.depleted, r.markets, r.hearths,
                r.closed[0], r.closed[1], r.closed[2], r.ms
            );
            let mut before = 0.0;
            for p in &r.series {
                println!(
                    "{name:<6}          turn {:>4}  clans {:>3}  people {:>6.1}  depleted {:>4}  moved {:>6.1}",
                    p.turn,
                    p.clans,
                    p.people,
                    p.depleted,
                    p.moved - before
                );
                before = p.moved;
            }
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
            "{name}: the top-up took {} clans' homes, {} existing markets, {} tended tiles, {} other grain tiles",
            tier(0),
            tier(1),
            tier(2),
            tier(3)
        );
        println!(
            "{name}: {} of {} ended on their own, {} needed the top-up, {cover_short} ended on the limit with three centres already (cover short or people not levelled off){}, median end turn {}",
            on_their_own,
            runs.len(),
            topped,
            if short > 0 { format!(", {short} with fewer than three centres") } else { String::new() },
            ends[ends.len() / 2]
        );
        let levels: Vec<f64> = runs.iter().filter_map(|r| r.level).collect();
        let flat = levels.iter().filter(|l| l.abs() < 0.1).count();
        println!(
            "{name}: people over the last {LEVEL_TURNS} turns before the end changed by {}: {flat} of {} levelled off (under 10%)",
            levels.iter().map(|l| format!("{:+.0}%", l * 100.0)).collect::<Vec<_>>().join(" "),
            levels.len()
        );
    }
    Ok(())
}

/// The seven maps of the retro: each size with its seeds.
const RETRO: [(&str, &[u32]); 3] = [("small", &[2, 3, 6, 9]), ("medium", &[8, 9]), ("large", &[8])];

/// The retro's maps as `(size name, radius, seed)`.
fn retro_maps() -> Result<Vec<(String, i32, u32)>, String> {
    let mut maps = Vec::new();
    for (name, seeds) in RETRO {
        let radius = size(name)?;
        maps.extend(seeds.iter().map(|&s| (name.to_string(), radius, s)));
    }
    Ok(maps)
}

/// What the camp sweep reports for one civ at the end: its line, whether it kept its first camp, and
/// its clans' people.
struct CivArmy {
    kept: bool,
    people: f64,
    joined: u32,
    lead: bool,
    hungry: u32,
    starved: u32,
    held: u32,
    cut_by_army: u32,
    text: String,
}

/// One map of the camp sweep: its civs, and the unaligned clans and all clans at the end.
struct MapRun {
    civs: Vec<CivArmy>,
    unaligned: usize,
    clans: usize,
}

/// One map of the camp sweep: runs to the pick with every civ on the AI, then `after` turns on, and
/// reports each civ (`army`, `gifts` or `zone` off is the control). With `lever` the lowest civ id is led
/// with that gift, and its camp is raised and posted where the AI would.
#[allow(clippy::too_many_arguments)]
fn after_run(
    name: &str,
    radius: i32,
    seed: u32,
    after: u32,
    army: bool,
    gifts: bool,
    zone: bool,
    lever: Option<f64>,
) -> MapRun {
    let mut w = World::new(Config { seed, radius, army, gifts, zone });
    while w.phase == Phase::Pregame {
        w.run(1);
    }
    let led = lever.and_then(|_| w.civs.keys().next().copied());
    w.act(Action::ChooseCiv { civ: led }).expect("a civ can be chosen at the pick");
    if let Some(g) = lever {
        w.act(Action::SetGift { food: g }).map_err(|e| format!("--lever {g}: {e}")).unwrap();
    }
    let pick = w.turn;
    while w.turn < pick + after {
        if let Some(civ) = led.filter(|_| army) {
            if units::camp(&w, civ).is_none() {
                let _ = w.act(Action::Raise { n: UNIT.start });
            }
            if let Some(u) = units::camp(&w, civ).filter(|u| u.dest.is_none()) {
                let at = units::post(&w, civ);
                if at != u.home {
                    let _ = w.act(Action::MoveCamp { to: at });
                }
            }
            w.act(Action::EndTurn).expect("ending a turn is allowed");
        } else {
            w.run(1);
        }
    }
    let unaligned = w.clans.values().filter(|k| k.civ.is_none()).count();
    let clans = w.clans.len();
    let civs: Vec<u32> = w.civs.keys().copied().collect();
    let civs = civs
        .into_iter()
        .map(|civ| {
            let camp = units::camp(&w, civ);
            let a = &w.civs[&civ].army;
            let people: f64 = w.clans.values().filter(|k| k.civ == Some(civ)).map(|k| k.people).sum();
            let warriors = camp.map_or(0.0, |u| u.people);
            let pack: f64 = camp.map_or(0.0, |u| u.store.iter().sum());
            let line = match camp.and_then(|u| units::line(&w, u.id)) {
                Some((_, cost, carry)) if carry.is_finite() => format!("cost {cost:.2} carry {carry:.2}"),
                Some((_, cost, _)) => format!("cost {cost:.2} carry unlimited"),
                None => "no line".to_string(),
            };
            let kept = a.lost == 0 && warriors >= f64::from(UNIT.start);
            let civ_state = &w.civs[&civ];
            let leaning = w
                .clans
                .values()
                .filter(|k| k.civ.is_none() && k.leaning.get(&civ).is_some_and(|&l| l >= LEANING.join / 2.0))
                .count();
            let lead = led == Some(civ);
            let text = format!(
                "{name:<6} seed {seed:>3} civ {civ}: camp {} warriors {warriors:>4.1} pack {pack:>5.2} hungry {:>4} starved {:>3} lost {} line {line} cut off {:>4} held {:>4} cut by army {:>3} foraged {:>3} tiles pillaged {:>3} tiles clans' people {people:>6.1}  kept {}  joined {} gifted {:.1} leaning {leaning}{}",
                match (camp.is_some(), a.lost > 0) {
                    (true, _) => "alive",
                    (false, true) => "lost",
                    (false, false) => "none",
                },
                a.hungry,
                a.starved,
                a.lost,
                a.cut_off,
                a.held,
                a.cut_by_army,
                a.foraged_tiles,
                a.pillaged_tiles,
                if kept { "yes" } else { "no" },
                civ_state.joined,
                civ_state.gifted,
                if lead { "  lead" } else { "" },
            );
            CivArmy {
                kept,
                people,
                joined: civ_state.joined,
                lead,
                hungry: a.hungry,
                starved: a.starved,
                held: a.held,
                cut_by_army: a.cut_by_army,
                text,
            }
        })
        .collect();
    MapRun { civs, unaligned, clans }
}

/// Runs the camp sweep over `maps` (size name, radius, seed) in parallel, one line per civ, then the
/// totals over all of them.
fn sweep_after(
    maps: &[(String, i32, u32)],
    after: u32,
    army: bool,
    gifts: bool,
    zone: bool,
    lever: Option<f64>,
) -> Result<(), String> {
    let runs: Vec<MapRun> = std::thread::scope(|scope| {
        let handles: Vec<_> = maps
            .iter()
            .map(|(name, radius, seed)| {
                scope.spawn(move || after_run(name, *radius, *seed, after, army, gifts, zone, lever))
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let all: Vec<&CivArmy> = runs.iter().flat_map(|r| &r.civs).collect();
    for civ in &all {
        println!("{}", civ.text);
    }
    let kept = all.iter().filter(|c| c.kept).count();
    let people: f64 = all.iter().map(|c| c.people).sum();
    let mode = match (army, gifts) {
        (true, true) if !zone => "control, no zone",
        (true, true) => "army",
        (false, true) => "control, no army",
        (true, false) => "control, no gifts",
        (false, false) => "control, no army, no gifts",
    };
    println!(
        "civs that kept their first camp: {kept} of {} (no camp lost, at least {} warriors at the end)",
        all.len(),
        UNIT.start
    );
    let sum = |f: fn(&CivArmy) -> u32| all.iter().map(|c| f(c)).sum::<u32>();
    println!(
        "camps' hungry turns {}, warriors starved {}, tile-turns held {}, turns cut by an army {}",
        sum(|c| c.hungry),
        sum(|c| c.starved),
        sum(|c| c.held),
        sum(|c| c.cut_by_army)
    );
    let mut joins: Vec<u32> = all.iter().map(|c| c.joined).collect();
    joins.sort_unstable();
    println!(
        "clans joined: {} (per civ, min/median/max {}/{}/{})",
        joins.iter().sum::<u32>(),
        joins.first().copied().unwrap_or(0),
        joins.get(joins.len() / 2).copied().unwrap_or(0),
        joins.last().copied().unwrap_or(0)
    );
    println!(
        "unaligned clans at the end: {} of {}",
        runs.iter().map(|r| r.unaligned).sum::<usize>(),
        runs.iter().map(|r| r.clans).sum::<usize>()
    );
    println!("clans' people at the end, all civs ({mode}): {people:.1}");
    if lever.is_some() {
        let mean = |lead: bool| {
            let v: Vec<u32> = all.iter().filter(|c| c.lead == lead).map(|c| c.joined).collect();
            f64::from(v.iter().sum::<u32>()) / v.len().max(1) as f64
        };
        println!("led civ joined {:.1}, its rivals {:.1} on average", mean(true), mean(false));
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
    let people: f64 = w.clans.values().map(|c| c.people).sum();
    let fed = w.clans.values().map(|c| c.fed_ratio).sum::<f64>() / w.clans.len().max(1) as f64;
    let held = w.tiles.iter().filter(|t| t.claim.is_some()).count();
    let n = w.clans.len().max(1) as f64;
    let tiles = w.clans.values().map(|c| c.territory.len() as f64).sum::<f64>() / n;
    let walk = w.clans.values().map(|c| c.walk).sum::<f64>() / n;
    let trip = w.clans.values().map(|c| c.trip()).sum::<f64>() / n;
    println!(
        "{} clans, {people:.1} people, mean fed ratio {fed:.2}, {held} tiles held, mean territory {tiles:.1} tiles walked for {walk:.1}, mean route {trip:.1}",
        w.clans.len()
    );
    let trading = w.clans.values().filter(|c| c.market.is_some()).count();
    let hearths = w.tiles.iter().filter(|t| land::is_hearth(t)).count();
    let trails = w.tiles.iter().filter(|t| t.trail).count();
    let cover = stats::summary(w).cover;
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
        None => (
            pregame::centres(w).iter().filter_map(|id| w.clans[id].market).collect(),
            "not ended, centre clans so far, by their festivals",
        ),
    };
    println!("{what}: {}", ids.len());
    print!("{}", centres_text(w, &ids));
    print!("{}", civs_text(w));
}

/// One line per civ: name, capital, market, clans, people, store, hearths on its land and units.
fn civs_text(w: &World) -> String {
    let mut out = String::new();
    for c in civs::cards(w) {
        let civ = &w.civs[&c.civ];
        let store: f64 = civ.store.iter().sum();
        out += &format!(
            "  civ {} at ({}, {}), market {}: {} clans, {:.1} people, store {store:.1}, {} hearths on its land, camp {}\n",
            c.name,
            c.q,
            c.r,
            civ.market,
            c.clans,
            c.people,
            c.hearths,
            engine::units::camp(w, c.civ).map_or("none".to_string(), |u| format!("{:.0} warriors", u.people))
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
