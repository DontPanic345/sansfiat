//! The sweep's led games: each civ of each retro map led by the automatic player (`script::Auto`,
//! the planner through `act`) beside the idle control (the same civ led and given no order), each
//! played to its end. Doubt is read here: whether the led civ's place in Standing changes after the
//! pick, whether games end before the limit, and what leading is worth over idling.

use engine::data::END;
use engine::judge::{rank_changes, rank_of};
use engine::script::Auto;
use engine::{rng, Action, Config, Phase, Why};
use serde::Serialize;

use crate::retro::to_pick;
use crate::sweep_judge::{mean, MapJudged};

/// One led run, read at its end.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct LedRun {
    /// The led civ's Standing and place at the end.
    pub(crate) standing: f64,
    pub(crate) rank: usize,
    /// How often its place changed after the pick (`judge::rank_changes`), and the game's lead.
    pub(crate) rank_changes: u32,
    pub(crate) lead_changes: u32,
    /// How it ended: `limit`, `conquest` or `fell`, and the turns after the pick.
    pub(crate) why: String,
    pub(crate) turn: u32,
    pub(crate) early: bool,
    /// The led civ's capital fell.
    pub(crate) fell: bool,
    /// The led civ won.
    pub(crate) won: bool,
}

/// One civ led on one map: by the automatic player, and idle.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct LedGame {
    pub(crate) civ: u32,
    pub(crate) auto: LedRun,
    pub(crate) idle: LedRun,
}

/// The led games over the maps.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct TotalLed {
    pub(crate) games: u32,
    /// Games in which the led civ's place changed after the pick, and the changes in all.
    pub(crate) rank_changed: u32,
    pub(crate) rank_changes: u32,
    pub(crate) idle_rank_changed: u32,
    pub(crate) idle_rank_changes: u32,
    pub(crate) lead_changes: u32,
    pub(crate) idle_lead_changes: u32,
    /// Games over before the limit, and those the led civ's fall ended.
    pub(crate) early: u32,
    pub(crate) fell: u32,
    pub(crate) idle_early: u32,
    pub(crate) idle_fell: u32,
    /// Games the led civ won.
    pub(crate) won: u32,
    pub(crate) idle_won: u32,
    /// The led civ's Standing at the end, mean over the games, played and idle, and the games the
    /// played civ ended above its idle self.
    pub(crate) standing: f64,
    pub(crate) idle_standing: f64,
    pub(crate) over_idle: f64,
    pub(crate) ahead_of_idle: u32,
}

/// The lead's changes over `standing`, as `judge::Watch` counts them: the first place changes hands
/// only to a civ ahead by more than the margin, which `rank_changes` of the leader reads as its fall.
fn lead_changes(standing: &[std::collections::BTreeMap<u32, f64>]) -> u32 {
    let Some(first) = standing.first() else { return 0 };
    let margin = engine::data::JUDGE.lead_margin;
    let top = |s: &std::collections::BTreeMap<u32, f64>| {
        s.iter()
            .fold(None, |b: Option<(u32, f64)>, (&k, &v)| if b.is_none_or(|(_, bv)| v > bv) { Some((k, v)) } else { b })
    };
    let Some((mut leader, _)) = top(first) else { return 0 };
    let mut changes = 0;
    for s in &standing[1..] {
        if let Some((k, v)) = top(s) {
            if k != leader && v > s.get(&leader).copied().unwrap_or(0.0) + margin {
                leader = k;
                changes += 1;
            }
        }
    }
    changes
}

/// One led run of `config` with civ `led`: `auto` plays it with the planner, else it gives no order.
/// `draws` extra rolls of the world's rng are taken after the pick (the null's difference).
fn led_run(config: Config, led: u32, auto: bool, draws: u32) -> LedRun {
    let mut w = to_pick(config);
    w.act(Action::ChooseCiv { civ: Some(led) }).expect("a civ can be led at the pick");
    for _ in 0..draws {
        rng::rand(&mut w.rng);
    }
    let backstop = END.limit.iter().map(|l| l.1).max().unwrap_or(0) + 10;
    let mut turns = 0;
    let over = loop {
        if let Phase::Over(o) = w.phase {
            break o;
        }
        assert!(turns < backstop, "a led game never ended");
        if auto {
            Auto::orders(&mut w);
        }
        w.act(Action::EndTurn).expect("ending a turn is allowed");
        turns += 1;
    };
    let seen = &w.standing[..=(over.turn as usize).min(w.standing.len() - 1)];
    let last = seen.last().expect("the pick's Standing");
    LedRun {
        standing: last.get(&led).copied().unwrap_or(0.0),
        rank: rank_of(last, led),
        rank_changes: rank_changes(seen, led),
        lead_changes: lead_changes(seen),
        why: match over.why {
            Why::Conquest => "conquest",
            Why::Limit => "limit",
            Why::Fell { .. } => "fell",
        }
        .to_string(),
        turn: over.turn,
        early: over.why != Why::Limit,
        fell: matches!(over.why, Why::Fell { .. }),
        won: over.winner == led,
    }
}

/// The led games of one map: each civ in id order, played and idle, all at once.
pub(crate) fn map_led(config: Config, civs: &[u32], draws: u32) -> Vec<LedGame> {
    let runs: Vec<(LedRun, LedRun)> = std::thread::scope(|scope| {
        let handles: Vec<_> = civs
            .iter()
            .map(|&c| {
                (
                    scope.spawn(move || led_run(config, c, true, draws)),
                    scope.spawn(move || led_run(config, c, false, draws)),
                )
            })
            .collect();
        handles
            .into_iter()
            .map(|(a, i)| {
                let join = |h: std::thread::ScopedJoinHandle<LedRun>| {
                    h.join().unwrap_or_else(|e| std::panic::resume_unwind(e))
                };
                (join(a), join(i))
            })
            .collect()
    });
    civs.iter().zip(runs).map(|(&civ, (auto, idle))| LedGame { civ, auto, idle }).collect()
}

/// The led games over `maps`.
pub(crate) fn total_led(maps: &[MapJudged]) -> TotalLed {
    let all: Vec<&LedGame> = maps.iter().flat_map(|m| &m.led).collect();
    let count = |f: fn(&LedGame) -> bool| all.iter().filter(|g| f(g)).count() as u32;
    let sum = |f: fn(&LedGame) -> u32| all.iter().map(|g| f(g)).sum::<u32>();
    let avg = |f: fn(&LedGame) -> f64| mean(&all.iter().map(|g| f(g)).collect::<Vec<_>>());
    TotalLed {
        games: all.len() as u32,
        rank_changed: count(|g| g.auto.rank_changes > 0),
        rank_changes: sum(|g| g.auto.rank_changes),
        idle_rank_changed: count(|g| g.idle.rank_changes > 0),
        idle_rank_changes: sum(|g| g.idle.rank_changes),
        lead_changes: sum(|g| g.auto.lead_changes),
        idle_lead_changes: sum(|g| g.idle.lead_changes),
        early: count(|g| g.auto.early),
        fell: count(|g| g.auto.fell),
        idle_early: count(|g| g.idle.early),
        idle_fell: count(|g| g.idle.fell),
        won: count(|g| g.auto.won),
        idle_won: count(|g| g.idle.won),
        standing: avg(|g| g.auto.standing),
        idle_standing: avg(|g| g.idle.standing),
        over_idle: avg(|g| g.auto.standing - g.idle.standing),
        ahead_of_idle: count(|g| g.auto.standing > g.idle.standing),
    }
}

/// The led games as text: a line a map, then the totals; with a `floor`, the gain over idle is read less it.
pub(crate) fn print_led(maps: &[MapJudged], t: &TotalLed, floor: Option<f64>) {
    println!("led games (each civ led by the planner, beside the same civ led and idle):");
    for m in maps {
        let games: Vec<String> = m
            .led
            .iter()
            .map(|g| {
                let name = m.names.get(&g.civ).map_or("?", |n| n.as_str());
                let run = |r: &LedRun| {
                    format!("{:.3} #{} moves {} {} +{}", r.standing, r.rank, r.rank_changes, r.why, r.turn)
                };
                format!("{name} {} (idle {})", run(&g.auto), run(&g.idle))
            })
            .collect();
        println!("  {:<6} seed {:>3}  {}", m.size, m.seed, games.join("; "));
    }
    println!(
        "  doubt: the led civ's place changed in {} of {} games ({} changes; idle {} games, {}); lead changes {} (idle {})",
        t.rank_changed, t.games, t.rank_changes, t.idle_rank_changed, t.idle_rank_changes, t.lead_changes, t.idle_lead_changes
    );
    println!(
        "  ends: before the limit {} of {} ({} by the led civ's fall); idle {} ({} by its fall); won {} (idle {})",
        t.early, t.games, t.fell, t.idle_early, t.idle_fell, t.won, t.idle_won
    );
    let read = match floor {
        Some(f) => format!(", less the floor {f:.4}: {}", if t.over_idle - f > 0.0 { "pass" } else { "MISS" }),
        None => String::new(),
    };
    println!(
        "  leading: Standing at the end {:.4} against idle {:.4}, gain {:+.4}{read}; ahead of its idle self in {} of {}",
        t.standing, t.idle_standing, t.over_idle, t.ahead_of_idle, t.games
    );
}
