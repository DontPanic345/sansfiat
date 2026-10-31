use engine::data::{PREGAME, SIZES};
use engine::land::is_depleted;
use engine::map::{self, Country};
use engine::{stats, Config, World};
use std::time::Instant;

use crate::retro::{retro_maps, sweep_after};
use crate::{size, USAGE};

/// One run of the sweep.
pub(crate) struct Run {
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
pub(crate) struct Point {
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
pub(crate) const LEVEL_TURNS: usize = 100;

pub(crate) fn sweep_run(seed: u32, radius: i32, turns: u32, every: u32) -> Run {
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
    let zones = |zone: Country| map::zone_groups(&w.tiles, radius, zone).len();
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
        closed: [zones(Country::Valley), zones(Country::Sealed), zones(Country::Neck)],
        ms,
        series,
        level,
    }
}

/// Runs seeds to their end (or `turns`) on each size asked for, one line each, then a summary per
/// size. Seeds run in parallel; the lines come out in seed order.
pub(crate) fn sweep(args: &[String]) -> Result<(), String> {
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
