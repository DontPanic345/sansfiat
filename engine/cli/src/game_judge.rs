//! `judge FILE [--json]`: a played game read back, scored turn by turn beside the same map with every
//! civ on the AI, and beside the same map with the led civ given no orders (the idle control).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use engine::data::JUDGE;
use engine::judge::{civ_lands, rank_of, CivLand, EndScores, Scores, Watch};
use engine::world::SAVE_VERSION;
use engine::{Action, Config, Save, World};
use serde::Serialize;

use crate::play::{read_entries, Entry};
use crate::retro::to_pick;
use crate::USAGE;

/// A played game read back: the led civ's Standing beside the control's and the idle control's, the
/// scores of all three, the led civ's land beside the control's, and the player's own decisions from
/// the log and the lines file.
#[derive(Serialize)]
struct GameJudged {
    pick: u32,
    turns: u32,
    led: u32,
    game: Scores,
    control: Scores,
    /// The led civ given no orders: the same map, the led civ chosen and only `EndTurn`.
    idle: Scores,
    standing: Ending,
    land: Lands,
    player: Player,
}

/// The led civ's Standing and rank at the end, in the game, in the control and in the idle control,
/// and how each ended.
#[derive(Serialize)]
struct Ending {
    game: f64,
    game_rank: usize,
    control: f64,
    control_rank: usize,
    idle: f64,
    idle_rank: usize,
    game_end: EndAt,
    control_end: EndAt,
    idle_end: EndAt,
}

/// How a run ended: `why +turn` (`fell +88`, `limit +200`), or `goes on +turn` when it had not ended by
/// the game's turns; and the led civ's rank at the end (`ranks` in the run's end group).
#[derive(Serialize)]
struct EndAt {
    how: String,
    rank: Option<usize>,
}

impl EndAt {
    fn of(e: &EndScores, led: u32, turns: u32) -> EndAt {
        let how = match &e.over {
            Some(o) => format!("{} +{}", o.why, o.turn),
            None => format!("goes on +{turns}"),
        };
        EndAt { how, rank: e.ranks.get(&led).copied() }
    }
}

/// The led civ's land at the end, in the game and in the control.
#[derive(Serialize)]
struct Lands {
    game: CivLand,
    control: CivLand,
}

/// What the player did: accepted orders and actions by kind (from the log), how often a turn had
/// none, and the refusals and notes of the lines file.
#[derive(Serialize)]
struct Player {
    orders_by_kind: BTreeMap<String, u32>,
    orders_per_100: f64,
    idle_turns: u32,
    longest_idle: u32,
    refused: Vec<Entry>,
    noted: usize,
}

/// The idle control: the same config to the same pick, the led civ `led` chosen there, and `EndTurn`
/// for `turns` turns with no order, every other civ on the AI. Its watch is its own.
fn idle_run(config: Config, led: u32, turns: u32) -> Result<(World, Watch), String> {
    let mut w = to_pick(config);
    w.act(Action::ChooseCiv { civ: Some(led) })?;
    let mut watch = Watch::new(&w);
    for _ in 0..turns {
        w.act(Action::EndTurn)?;
        watch.turn(&w);
    }
    Ok((w, watch))
}

/// `judge FILE [--json]`: replays a played game a turn at a time under a `Watch`, plays the same map
/// with every civ on the AI to the same turn, and reads both in the judge's scores.
pub(crate) fn judge_game(args: &[String]) -> Result<(), String> {
    let mut file = None;
    let mut json = false;
    for a in args {
        match a.as_str() {
            "--json" => json = true,
            f if file.is_none() && !f.starts_with("--") => file = Some(f.to_string()),
            _ => return Err(format!("judge takes a game file and --json\n{USAGE}")),
        }
    }
    let file = file.ok_or(format!("judge needs a game file\n{USAGE}"))?;
    let path = Path::new(&file);
    let text = fs::read_to_string(path).map_err(|e| format!("{file}: {e}"))?;
    let save: Save = serde_json::from_str(&text).map_err(|e| format!("{file}: {e}"))?;
    if save.version != SAVE_VERSION {
        return Err(format!("{file}: save version {} is not {SAVE_VERSION}", save.version));
    }

    // The game, action by action: its watch starts at the pick and is turned after each end.
    let mut w = World::new(save.config);
    let mut watch: Option<Watch> = None;
    let (mut idle, mut longest, mut run, mut acted) = (0u32, 0u32, 0u32, false);
    let mut by_kind: BTreeMap<String, u32> = BTreeMap::new();
    for a in &save.log {
        w.act(a.clone()).map_err(|e| format!("{file}: {e}"))?;
        match a {
            Action::ChooseCiv { .. } => watch = Some(Watch::new(&w)),
            Action::EndTurn => {
                let Some(wt) = watch.as_mut() else { continue };
                wt.turn(&w);
                if acted {
                    run = 0;
                } else {
                    idle += 1;
                    run += 1;
                    longest = longest.max(run);
                }
                acted = false;
            }
            Action::Order { order, .. } => {
                acted = true;
                *by_kind.entry(order.kind().to_string()).or_insert(0) += 1;
            }
            Action::SetKeep { .. } | Action::SetGift { .. } => {
                acted = true;
                let kind = if matches!(a, Action::SetKeep { .. }) { "setKeep" } else { "setGift" };
                *by_kind.entry(kind.to_string()).or_insert(0) += 1;
            }
        }
    }
    let wt = watch.ok_or(format!("{file}: the game has no pick"))?;
    let led = w.player.ok_or(format!("{file}: the game leads no civ"))?;
    let n = wt.turns();

    // The control: the same config to the same pick, every civ on the AI, the same number of turns.
    let mut control = to_pick(save.config);
    control.act(Action::ChooseCiv { civ: None })?;
    let mut cwatch = Watch::new(&control);
    if cwatch.pick() != wt.pick() {
        return Err(format!("{file}: the control's pick is {}, the game's {}", cwatch.pick(), wt.pick()));
    }
    for _ in 0..n {
        control.act(Action::EndTurn)?;
        cwatch.turn(&control);
    }

    // The idle control: the led civ at the same pick, given no orders.
    let (idle_w, idle_watch) = idle_run(save.config, led, n)?;
    if idle_watch.pick() != wt.pick() {
        return Err(format!("{file}: the idle control's pick is {}, the game's {}", idle_watch.pick(), wt.pick()));
    }

    let game_scores = wt.scores(&w, &[led]);
    let control_scores = cwatch.scores(&control, &[led]);
    let idle_scores = idle_watch.scores(&idle_w, &[led]);
    let (game_end, control_end, idle_end) = (
        EndAt::of(&game_scores.end, led, n),
        EndAt::of(&control_scores.end, led, n),
        EndAt::of(&idle_scores.end, led, n),
    );
    let g_end = wt.standing_at(n).expect("scored to the end");
    let c_end = cwatch.standing_at(n).expect("scored to the end");
    let i_end = idle_watch.standing_at(n).expect("scored to the end");
    let land_of = |w: &World| civ_lands(w).into_iter().find(|c| c.id == led).expect("the led civ is in the world");
    let entries = read_entries(path)?;
    let refused: Vec<Entry> = entries.iter().filter(|e| e.result.starts_with("refused")).cloned().collect();
    let noted = entries.iter().filter(|e| e.result == "noted").count();
    let player_orders: u32 = by_kind.values().sum();
    let judged = GameJudged {
        pick: wt.pick(),
        turns: n,
        led,
        game: game_scores,
        control: control_scores,
        idle: idle_scores,
        standing: Ending {
            game: g_end.get(&led).copied().unwrap_or(0.0),
            game_rank: rank_of(g_end, led),
            control: c_end.get(&led).copied().unwrap_or(0.0),
            control_rank: rank_of(c_end, led),
            idle: i_end.get(&led).copied().unwrap_or(0.0),
            idle_rank: rank_of(i_end, led),
            game_end,
            control_end,
            idle_end,
        },
        land: Lands { game: land_of(&w), control: land_of(&control) },
        player: Player {
            orders_per_100: if n > 0 { f64::from(player_orders) * 100.0 / f64::from(n) } else { 0.0 },
            orders_by_kind: by_kind,
            idle_turns: idle,
            longest_idle: longest,
            refused,
            noted,
        },
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&judged).expect("the judgement serialises"));
    } else {
        print_game(&judged, &wt, &cwatch);
    }
    Ok(())
}

/// The judgement as text: the score line every `JUDGE.sample` turns, the led civ at the end beside
/// the control, its claim and land beside the control's, the doubt and motion of both, the player's
/// decisions and refusals.
fn print_game(j: &GameJudged, game: &Watch, control: &Watch) {
    let shares = |s: &BTreeMap<u32, f64>| -> String {
        s.iter().map(|(id, v)| format!("civ {id} {:.1}%", v * 100.0)).collect::<Vec<_>>().join(", ")
    };
    for k in (0..=j.turns).step_by(JUDGE.sample as usize) {
        let s = game.standing_at(k).expect("scored to the end");
        let leader = game.leader_at(k).map_or("none".to_string(), |l| format!("civ {l}"));
        println!("turn {} (+{k}): {}; leader {leader}", j.pick + k, shares(s));
    }
    let s = &j.standing;
    println!(
        "led civ {}: Standing at +{} {:.1}% (rank {}) in the game, {:.1}% (rank {}) with every civ on the AI, {:.1}% (rank {}) idle led civ",
        j.led,
        j.turns,
        s.game * 100.0,
        s.game_rank,
        s.control * 100.0,
        s.control_rank,
        s.idle * 100.0,
        s.idle_rank
    );
    let rank = |r: Option<usize>| r.map_or("-".to_string(), |r| r.to_string());
    println!(
        "end (game / control / idle): {} rank {} / {} rank {} / {} rank {}",
        s.game_end.how,
        rank(s.game_end.rank),
        s.control_end.how,
        rank(s.control_end.rank),
        s.idle_end.how,
        rank(s.idle_end.rank)
    );
    let (l, c) = (&j.land.game, &j.land.control);
    println!(
        "land (game / control): {} / {} tiles claimed",
        l.claimed, c.claimed
    );
    let (g, c) = (&j.game.doubt, &j.control.doubt);
    println!(
        "doubt (game / control): lead changes {} / {}, spread +{} {:.3} / {:.3}, swings {} / {}",
        g.lead_changes,
        c.lead_changes,
        j.turns,
        game.spread_at(j.turns).unwrap_or(0.0),
        control.spread_at(j.turns).unwrap_or(0.0),
        g.swings,
        c.swings
    );
    let (gm, cm) = (&j.game.motion, &j.control.motion);
    println!(
        "motion (game / control): clans changed {} / {}, people {:.1} / {:.1}, land tiles {} / {}, clans left {} / {}, claimed tiles taken {} / {}",
        gm.clans_changed,
        cm.clans_changed,
        gm.people_changed,
        cm.people_changed,
        gm.land_changed,
        cm.land_changed,
        gm.clans_left,
        cm.clans_left,
        gm.claim_taken,
        cm.claim_taken
    );
    let (gf, cf) = (&j.game.fight, &j.control.fight);
    let civs: Vec<String> = gf
        .civs
        .iter()
        .map(|(id, c)| format!("{id}: fallen {:.2} taken {} surrendered {}", c.fallen, c.taken, c.surrendered))
        .collect();
    println!(
        "fight (game / control): people fallen {:.2} / {:.2}, tiles taken {} / {}, armies surrendered {} / {}",
        gf.fallen, cf.fallen, gf.taken, cf.taken, gf.surrendered, cf.surrendered
    );
    println!("fight by civ (game): {}", civs.join("; "));
    let field = |s: &Scores| s.fields.civs.get(&j.led).cloned().unwrap_or_default();
    let (gl, cl) = (field(&j.game), field(&j.control));
    let at_200 = |v: String| if j.turns >= 200 { v } else { "-".to_string() };
    println!(
        "fields (game / control): farming share at +200 {} / {}, field tiles at +200 {} / {}, store turns at a pack {} / {} of {}",
        at_200(format!("{:.3}", gl.farming_share_200)),
        at_200(format!("{:.3}", cl.farming_share_200)),
        at_200(gl.field_tiles_200.to_string()),
        at_200(cl.field_tiles_200.to_string()),
        gl.store_turns_at_pack,
        cl.store_turns_at_pack,
        j.turns
    );
    let p = &j.player;
    let kinds: Vec<String> = p.orders_by_kind.iter().map(|(k, v)| format!("{k} {v}")).collect();
    println!(
        "player orders: {} ({}), {:.2} per 100 turns; turns with none {} of {}, longest run {}",
        p.orders_by_kind.values().sum::<u32>(),
        if kinds.is_empty() { "none".to_string() } else { kinds.join(", ") },
        p.orders_per_100,
        p.idle_turns,
        j.turns,
        p.longest_idle
    );
    println!("player lines: {} refused, {} noted", p.refused.len(), p.noted);
    for r in &p.refused {
        println!("  refused at turn {}: {} ({})", r.turn, r.line, r.result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::data::size_radius;

    #[test]
    fn the_idle_led_civ_is_given_no_order_and_is_the_same_on_every_run() {
        let config = Config { seed: 2, radius: size_radius("small").unwrap(), ..Config::default() };
        let led = *to_pick(config).civs.keys().next().unwrap();
        let (w, watch) = idle_run(config, led, 12).unwrap();
        assert_eq!((watch.pick(), watch.turns()), (w.turn - 12, 12));
        assert_eq!(w.player, Some(led), "the idle run leads the civ");
        let kinds: Vec<&Action> = w.log.iter().filter(|a| !matches!(a, Action::ChooseCiv { .. })).collect();
        assert!(kinds.iter().all(|a| matches!(a, Action::EndTurn)), "no order in the idle run: {kinds:?}");
        let after_pick = w.log.iter().rev().take_while(|a| matches!(a, Action::EndTurn)).count();
        assert_eq!(after_pick, 12, "one EndTurn a turn past the pick, and nothing else");
        let (_, again) = idle_run(config, led, 12).unwrap();
        assert_eq!(watch.standing_at(12), again.standing_at(12), "the same Standing on every run");
    }
}
