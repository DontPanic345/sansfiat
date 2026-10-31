use engine::data::{LEANING, UNIT};

use engine::units;
use engine::{Action, Config, Order, Phase, World};

use crate::size;

/// The seven maps of the retro: each size with its seeds.
pub(crate) const RETRO: [(&str, &[u32]); 3] = [("small", &[2, 3, 6, 9]), ("medium", &[8, 9]), ("large", &[8])];

/// The retro's maps as `(size name, radius, seed)`.
pub(crate) fn retro_maps() -> Result<Vec<(String, i32, u32)>, String> {
    let mut maps = Vec::new();
    for (name, seeds) in RETRO {
        let radius = size(name)?;
        maps.extend(seeds.iter().map(|&s| (name.to_string(), radius, s)));
    }
    Ok(maps)
}

/// What the camp sweep reports for one civ at the end: its line, whether it kept its first camp, and
/// its clans' people.
pub(crate) struct CivArmy {
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
pub(crate) struct MapRun {
    civs: Vec<CivArmy>,
    unaligned: usize,
    clans: usize,
}

/// One map of the camp sweep: runs to the pick with every civ on the AI, then `after` turns on, and
/// reports each civ (`army`, `gifts` or `zone` off is the control). With `lever` the lowest civ id is led
/// with that gift, and its camp is raised and posted where the AI would.
#[allow(clippy::too_many_arguments)]
pub(crate) fn after_run(
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
                let _ = w.act(Action::Order { civ, order: Order::Raise { n: UNIT.start } });
            }
            if let Some(u) = units::camp(&w, civ).filter(|u| u.camp().is_some_and(|c| c.dest.is_none())) {
                let at = units::post(&w, civ);
                if at != u.home {
                    let unit = u.id;
                    let _ = w.act(Action::Order { civ, order: Order::Move { unit, to: at } });
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
pub(crate) fn sweep_after(
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
