//! `play`: a led civ played a line at a time. The game is a save (`--game FILE`), and every call
//! resumes it: the world is replayed from the save, each line read from stdin is applied through
//! `World::act` (the page's door, so `validate` checks it), and after each line the save is written
//! again and the line is appended to the lines file (`FILE` with `.lines.jsonl`).

use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use engine::{civs, Action, Config, Order, Phase, Save, World};
use serde::{Deserialize, Serialize};

use crate::run::print_notes;
use crate::{size, view, USAGE};

/// The grammar a refused or unknown line is answered with.
const GRAMMAR: &str = "lines: an order as JSON ({\"type\":\"move\",\"unit\":12,\"to\":5}), an action \
    ({\"type\":\"setGift\",\"food\":2}), end [N], why ID, view, map, note TEXT; an empty line or # is skipped";

/// One line read: the turn after it, the line, and its result (`ok`, `refused: why`, `noted`, `unknown`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Entry {
    pub(crate) turn: u32,
    pub(crate) line: String,
    pub(crate) result: String,
}

/// The lines file beside a game: `g.json` is `g.lines.jsonl`.
pub(crate) fn lines_path(game: &Path) -> PathBuf {
    game.with_extension("lines.jsonl")
}

/// The entries of a lines file; none when there is no file.
pub(crate) fn read_entries(game: &Path) -> Result<Vec<Entry>, String> {
    let path = lines_path(game);
    let Ok(text) = fs::read_to_string(&path) else { return Ok(Vec::new()) };
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(|e| format!("{}: {e}", path.display())))
        .collect()
}

/// The civ the `k`-th (1-based) in id order.
fn civ_id(w: &World, k: u32) -> Result<u32, String> {
    let ids: Vec<u32> = w.civs.keys().copied().collect();
    ids.get((k as usize).wrapping_sub(1))
        .copied()
        .ok_or_else(|| format!("--civ {k}: there are {} civs, 1 to {}", ids.len(), ids.len()))
}

/// Writes the game's save to `path`.
fn write_game(path: &Path, w: &World) -> Result<(), String> {
    fs::write(path, serde_json::to_string(&w.save()).unwrap()).map_err(|e| format!("{}: {e}", path.display()))
}

/// Appends one entry to the lines file.
fn append(path: &Path, entry: &Entry) -> Result<(), String> {
    let lines = lines_path(path);
    let mut f =
        OpenOptions::new().create(true).append(true).open(&lines).map_err(|e| format!("{}: {e}", lines.display()))?;
    writeln!(f, "{}", serde_json::to_string(entry).unwrap()).map_err(|e| format!("{}: {e}", lines.display()))
}

pub(crate) fn play(args: &[String]) -> Result<(), String> {
    let (mut game, mut seed, mut radius, mut civ) = (None, None, None, None);
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let value = it.next().ok_or(format!("{flag} needs a value\n{USAGE}"))?;
        let bad = |_| format!("{flag}: not a number: {value}");
        match flag.as_str() {
            "--game" => game = Some(value.clone()),
            "--seed" => seed = Some(value.parse::<u32>().map_err(bad)?),
            "--size" => radius = Some(size(value)?),
            "--civ" => civ = Some(value.parse::<u32>().map_err(bad)?),
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let game = game.ok_or(format!("play needs --game FILE\n{USAGE}"))?;
    let path = Path::new(&game);

    let mut w = if path.exists() {
        let text = fs::read_to_string(path).map_err(|e| format!("{game}: {e}"))?;
        let save: Save = serde_json::from_str(&text).map_err(|e| format!("{game}: {e}"))?;
        let w = World::replay(&save)?;
        if seed.is_some_and(|s| s != w.config.seed) || radius.is_some_and(|r| r != w.config.radius) {
            return Err(format!(
                "{game} is seed {} radius {}; the flags say otherwise",
                w.config.seed, w.config.radius
            ));
        }
        if let Some(k) = civ {
            let want = civ_id(&w, k)?;
            if w.player != Some(want) {
                return Err(format!(
                    "{game} leads civ {}, not {k}",
                    w.player.map_or("none".to_string(), |c| c.to_string())
                ));
            }
        }
        w
    } else {
        let seed = seed.ok_or(format!("a new game needs --seed\n{USAGE}"))?;
        let radius = radius.ok_or(format!("a new game needs --size\n{USAGE}"))?;
        let mut w = World::new(Config { seed, radius, ..Config::default() });
        while w.phase == Phase::Pregame {
            w.act(Action::EndTurn)?;
        }
        match civ {
            None => {
                print!("{}", cards(&w));
                println!("no civ chosen: pick one with --civ K to make {game}");
                return Ok(());
            }
            Some(k) => {
                let id = civ_id(&w, k)?;
                w.act(Action::ChooseCiv { civ: Some(id) })?;
            }
        }
        write_game(path, &w)?;
        w
    };

    let entries = read_entries(path)?;
    let from = entries.last().map_or(w.picked.unwrap_or(w.turn), |e| e.turn);
    let since = w.events.partition_point(|e| e.turn < from);
    print!("{}", view::text(&w, since));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let text = line.trim();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        let result = apply(&mut w, text, since);
        write_game(path, &w)?;
        append(path, &Entry { turn: w.turn, line: text.to_string(), result })?;
    }
    Ok(())
}

/// The three civs' cards for the pick, one line each.
fn cards(w: &World) -> String {
    let mut out = String::new();
    for c in civs::cards(w) {
        let terrain: Vec<String> = c.terrain.iter().map(|(n, s)| format!("{n} {:.0}%", s * 100.0)).collect();
        let rival = c.rival_steps.map_or("no rival".to_string(), |s| format!("rival capital {s} steps away"));
        out += &format!(
            "civ {} {} at ({}, {}): {} clans, {:.1} people, {} hearths, {}; {rival}\n",
            c.civ,
            c.name,
            c.q,
            c.r,
            c.clans,
            c.people,
            c.hearths,
            terrain.join(", ")
        );
    }
    out
}

/// Applies one line to the world and prints what it answers; returns the line's result.
fn apply(w: &mut World, text: &str, since: usize) -> String {
    if text.starts_with('{') {
        return match serde_json::from_str::<Action>(text) {
            Ok(action) => act(w, action, since),
            Err(_) => match (serde_json::from_str::<Order>(text), w.player) {
                (Ok(order), Some(civ)) => act(w, Action::Order { civ, order }, since),
                (Ok(_), None) => refuse("you lead no civ".to_string()),
                (Err(_), _) => unknown(text),
            },
        };
    }
    let (cmd, rest) = text.split_once(' ').map_or((text, ""), |(c, r)| (c, r.trim()));
    match cmd {
        "end" => {
            let n: u32 = if rest.is_empty() {
                1
            } else {
                match rest.parse() {
                    Ok(n) => n,
                    Err(_) => return unknown(text),
                }
            };
            for _ in 0..n {
                if w.phase != Phase::Main {
                    break;
                }
                if let Err(e) = w.act(Action::EndTurn) {
                    return refuse(e);
                }
            }
            print!("{}", view::text(w, since));
            "ok".to_string()
        }
        "why" => match rest.parse::<u32>() {
            Ok(id) => {
                print_notes(w, id);
                "ok".to_string()
            }
            Err(_) => unknown(text),
        },
        "view" => {
            print!("{}", view::text(w, since));
            "ok".to_string()
        }
        "map" => {
            print!("{}", map_text(w));
            "ok".to_string()
        }
        "note" => {
            println!("noted");
            "noted".to_string()
        }
        _ => unknown(text),
    }
}

/// Applies an action through `act`: a refusal prints and changes nothing; an accepted one prints the view.
fn act(w: &mut World, action: Action, since: usize) -> String {
    match w.act(action) {
        Ok(()) => {
            print!("{}", view::text(w, since));
            "ok".to_string()
        }
        Err(e) => refuse(e),
    }
}

fn refuse(why: String) -> String {
    println!("refused: {why}");
    format!("refused: {why}")
}

fn unknown(text: &str) -> String {
    println!("unknown: {text}\n{GRAMMAR}");
    "unknown".to_string()
}

/// The map as `sansfiat map` prints it, with the led civ's capital `C` and its units `U`.
fn map_text(w: &World) -> String {
    let mut marks: Vec<(usize, char)> = Vec::new();
    if let Some(me) = w.player {
        marks.push((w.civs[&me].capital, 'C'));
        marks.extend(w.units.values().filter(|u| u.civ == me).map(|u| (u.tile(), 'U')));
    }
    let mut out = engine::map::ascii_marked(&w.tiles, w.config.radius, &marks);
    out += "C your capital  U your units\n";
    out
}
