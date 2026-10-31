//! `play` and `judge` on small seed 2: a game made and played a line at a time, resumed by a second
//! call, and judged. Each call runs the binary, as a shell's calls would.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use engine::{stats, Save, World};

/// Runs the binary with `args` and `input` on stdin; its stdout, which must succeed.
fn sansfiat(args: &[&str], input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sansfiat"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the sansfiat binary runs");
    // A call that reads no stdin may exit before this write lands; that is not a failure.
    let _ = child.stdin.take().unwrap().write_all(input.as_bytes());
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "sansfiat {args:?} failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}

/// A fresh directory for one test, with its game file.
fn game_in(test: &str) -> (PathBuf, String) {
    let dir = std::env::temp_dir().join(format!("sansfiat-play-{test}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let game = dir.join("g.json").to_string_lossy().into_owned();
    (dir, game)
}

/// The turn of the last view in `out`: its `turn 283, ...` line.
fn turn_of(out: &str) -> u32 {
    let line = out.lines().rfind(|l| l.starts_with("turn ")).expect("a view has its turn");
    line["turn ".len()..].split(',').next().unwrap().parse().unwrap()
}

/// The led civ's first camp's id, and the AI's post for it, from a view.
fn camp_and_post(view: &str) -> (u32, usize) {
    let camp = view
        .lines()
        .find_map(|l| l.trim_start().strip_prefix("camp "))
        .and_then(|r| r.split(' ').next())
        .and_then(|n| n.parse().ok())
        .expect("the view names the camp");
    let post = view
        .lines()
        .find_map(|l| l.split("would post your camp at ").nth(1))
        .and_then(|r| r.split(' ').next())
        .and_then(|n| n.parse().ok())
        .expect("the view names the AI's post");
    (camp, post)
}

/// The number of actions the save at `game` logs.
fn log_len(game: &str) -> usize {
    let save: Save = serde_json::from_str(&fs::read_to_string(game).unwrap()).unwrap();
    save.log.len()
}

/// The lines file's entries: their lines and results.
fn entries(game: &str) -> Vec<(String, String)> {
    let path = game.replace(".json", ".lines.jsonl");
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let v: serde_json::Value = serde_json::from_str(l).unwrap();
            (v["line"].as_str().unwrap().to_string(), v["result"].as_str().unwrap().to_string())
        })
        .collect()
}

#[test]
fn play_without_civ_prints_the_cards_and_writes_no_file() {
    let (_dir, game) = game_in("cards");
    let out = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small"], "");
    let cards = out.lines().filter(|l| l.starts_with("civ ")).count();
    assert_eq!(cards, 3, "{out}");
    assert!(!std::path::Path::new(&game).exists(), "no civ chosen, so no game");
}

#[test]
fn a_piped_session_is_logged_and_refusals_change_nothing() {
    let (_dir, game) = game_in("session");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], "");
    let (camp, post) = camp_and_post(&made);
    let pick = turn_of(&made);
    let before = log_len(&game);

    let session = format!(
        "{{\"type\":\"move\",\"unit\":999,\"to\":5}}\n\
         {{\"type\":\"move\",\"unit\":{camp},\"to\":{post}}}\n\
         note test\n\
         why {camp}\n\
         end 3\n"
    );
    let out = sansfiat(&["play", "--game", &game], &session);
    assert!(out.contains("refused: 999 is not a unit"), "{out}");
    assert!(out.contains("noted"), "{out}");
    assert_eq!(turn_of(&out), pick + 3);
    // The refusal logs nothing; the move and the three turns do.
    assert_eq!(log_len(&game), before + 4);

    let lines = entries(&game);
    let results: Vec<&str> = lines.iter().map(|(_, r)| r.as_str()).collect();
    assert_eq!(results.len(), 5);
    assert!(results[0].starts_with("refused:"), "{results:?}");
    assert_eq!(results[1], "ok");
    assert_eq!(results[2], "noted");
    assert_eq!(results[4], "ok");
}

#[test]
fn a_second_call_resumes_the_same_game() {
    let (_dir, game) = game_in("resume");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], "");
    let (camp, post) = camp_and_post(&made);
    sansfiat(&["play", "--game", &game], &format!("{{\"type\":\"move\",\"unit\":{camp},\"to\":{post}}}\nend 3\n"));
    let world = |game: &str| {
        let save: Save = serde_json::from_str(&fs::read_to_string(game).unwrap()).unwrap();
        World::replay(&save).unwrap()
    };
    let before = world(&game);
    let digest = stats::digest(&before);
    let turn = before.turn;

    let view = sansfiat(&["play", "--game", &game], "view\n");
    assert_eq!(turn_of(&view), turn, "the call resumes at the turn it stopped");
    assert_eq!(stats::digest(&world(&game)), digest, "the replayed world is the same world");

    // Every line of both calls is in the lines file, the last one `view`, with its result.
    let lines = entries(&game);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[2], ("view".to_string(), "ok".to_string()));
}

#[test]
fn the_view_at_the_pick_is_under_80_lines() {
    let (_dir, game) = game_in("view");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], "");
    assert!(made.lines().count() < 80, "{} lines", made.lines().count());
    assert!(made.contains("leader, you"), "the led civ is marked");
    assert!(made.contains("legal orders:"), "{made}");
}

#[test]
fn judge_prints_a_score_line_and_the_control_for_the_led_civ() {
    let (_dir, game) = game_in("judge");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], "");
    let (camp, post) = camp_and_post(&made);
    sansfiat(&["play", "--game", &game], &format!("{{\"type\":\"move\",\"unit\":{camp},\"to\":{post}}}\nend 10\n"));

    let out = sansfiat(&["judge", &game], "");
    assert!(out.lines().any(|l| l.starts_with("turn ") && l.contains("leader")), "{out}");
    let led = out.lines().find(|l| l.starts_with("led civ ")).unwrap_or_else(|| panic!("no led civ line in:\n{out}"));
    assert!(led.contains("with every civ on the AI"), "{led}");
    assert!(out.contains("player orders: 1 (move 1)"), "{out}");
}

#[test]
fn the_view_says_whether_a_gift_reaches_each_clan_in_reach() {
    // The trial game (relay/specs/101/play.md): at its last turn clan 321 is in reach of Red's
    // market but trades at another, so Red's gift never reaches it.
    let (_dir, game) = game_in("reach");
    let trial = concat!(env!("CARGO_MANIFEST_DIR"), "/../../relay/specs/101/trial/g.json");
    fs::copy(trial, &game).unwrap();
    let out = sansfiat(&["play", "--game", &game], "why 321\n");
    assert!(out.contains("a gift reaches only those that trade there"), "{out}");
    let clan = out.lines().find(|l| l.starts_with("  clan 321 at")).expect("clan 321 is in reach");
    assert!(clan.contains("so your gift doesn't reach it"), "{clan}");
    assert!(out.contains("why 321: its reasons, newest first"), "{out}");
}
