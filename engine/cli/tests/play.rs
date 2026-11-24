//! `play` and `judge` on small seed 2: a game made and played a line at a time, resumed by a second
//! call, and judged. Each call runs the binary, as a shell's calls would.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use engine::{stats, Save, World};

/// Runs the binary with `args` and `input` on stdin; its stdout, which must succeed.
/// A civ starts with its scout and no army: its first warriors, raised at the pick (one warrior's pack
/// is what the endowment less the scout's pack fills).
const RAISE: &str = "{\"type\":\"raise\",\"n\":1,\"kind\":\"camp\"}\n";

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
        .find_map(|l| l.trim_start().strip_prefix("army "))
        .and_then(|r| r.split(' ').next())
        .and_then(|n| n.parse().ok())
        .expect("the view names the army");
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
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], RAISE);
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
    // The raise at the pick is the first line, then the five of the session.
    assert_eq!(results.len(), 6);
    assert!(results[1].starts_with("refused:"), "{results:?}");
    assert_eq!(results[2], "ok");
    assert_eq!(results[3], "noted");
    assert_eq!(results[5], "ok");
}

#[test]
fn a_second_call_resumes_the_same_game() {
    let (_dir, game) = game_in("resume");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], RAISE);
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
    // The raise at the pick, the move and the turns of the first call, then the view.
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[3], ("view".to_string(), "ok".to_string()));
}

#[test]
fn the_view_at_the_pick_is_under_80_lines() {
    let (_dir, game) = game_in("view");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], "");
    assert!(made.lines().count() < 90, "{} lines", made.lines().count());
    assert!(made.lines().any(|l| l.starts_with("  civ ") && l.ends_with(" you")), "the led civ is marked");
    assert!(made.contains("legal orders:"), "{made}");
}

#[test]
fn judge_prints_a_score_line_and_the_control_for_the_led_civ() {
    let (_dir, game) = game_in("judge");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], RAISE);
    let (camp, post) = camp_and_post(&made);
    sansfiat(&["play", "--game", &game], &format!("{{\"type\":\"move\",\"unit\":{camp},\"to\":{post}}}\nend 10\n"));

    let out = sansfiat(&["judge", &game], "");
    assert!(out.lines().any(|l| l.starts_with("turn ") && l.contains("leader")), "{out}");
    let led = out.lines().find(|l| l.starts_with("led civ ")).unwrap_or_else(|| panic!("no led civ line in:\n{out}"));
    assert!(led.contains("with every civ on the AI"), "{led}");
    // The raise at the pick and the move.
    assert!(out.contains("player orders: 2 (move 1, raise 1)"), "{out}");
}

#[test]
fn the_view_says_whether_a_gift_reaches_each_clan_in_reach() {
    // On small seed 2, 25 turns after the pick, an unaligned clan is in reach of the led civ's market
    // but trades at another, so the civ's gift never reaches it: the view says so, and `why` for that
    // clan gives its reasons. The seed and turn follow the rules: with the feeding (`feeding.md`) the
    // first of seeds 1 to 24 that shows one at this turn is this.
    let (_dir, game) = game_in("reach");
    sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], RAISE);
    let out = sansfiat(&["play", "--game", &game], "end 25\n");
    assert!(out.contains("a gift reaches only those that trade there"), "{out}");
    let clan = out
        .lines()
        .find(|l| l.starts_with("  clan ") && l.contains("so your gift doesn't reach it"))
        .unwrap_or_else(|| panic!("a clan in reach trades elsewhere:\n{out}"));
    let id = clan["  clan ".len()..].split(' ').next().unwrap();
    let out = sansfiat(&["play", "--game", &game], &format!("why {id}\n"));
    assert!(out.contains(&format!("why {id}: its reasons, newest first")), "{out}");
}

#[test]
fn the_view_at_the_pick_lists_the_claim_and_its_new_legal_orders() {
    let (_dir, game) = game_in("claim-view");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], RAISE);
    assert!(made.lines().any(|l| l.starts_with("land: ") && l.contains(" tiles, your claim:")), "{made}");
    assert!(made.contains("nobody stands on it:"), "{made}");
    assert!(made.lines().any(|l| l.starts_with("clans: ") && l.contains("with their home on your land")), "{made}");
    assert!(made.contains(r#"  {"type":"claim","tiles":"#), "{made}");
    assert!(made.lines().count() < 90, "{} lines", made.lines().count());
    let map = sansfiat(&["play", "--game", &game], "map\n");
    assert!(map.contains("L your land"), "{map}");
    assert!(map.lines().any(|l| l.contains(" L ") || l.contains(" L\n") || l.ends_with(" L")), "{map}");
}

#[test]
fn judge_prints_the_claim_lines_for_the_led_civ() {
    let (_dir, game) = game_in("judge-claim");
    sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], RAISE);
    sansfiat(&["play", "--game", &game], "end 10\n");
    let out = sansfiat(&["judge", &game], "");
    let claim = out
        .lines()
        .find(|l| l.starts_with("land (game / control): "))
        .unwrap_or_else(|| panic!("no land line in:\n{out}"));
    assert!(claim.contains(" tiles claimed"), "{claim}");
    assert!(
        out.lines().any(|l| l.starts_with("motion (game / control):")
            && l.contains("clans left")
            && l.contains("claimed tiles taken")),
        "{out}"
    );
}

/// The tile ids in the land line: its numeric words, after the colon.
fn land_ids(view: &str) -> Vec<usize> {
    let land = view.lines().rev().find(|l| l.starts_with("land: ")).expect("a land line");
    let names = land.rsplit_once(": ").expect("the land line names its tiles").1;
    names.split(' ').filter_map(|w| w.parse().ok()).collect()
}

/// The ids in the first `[...]` of a line of the legal list.
fn order_ids(line: &str) -> Vec<usize> {
    let list = line.split('[').nth(1).and_then(|r| r.split(']').next()).expect("an order's tiles");
    list.split(',').map(|n| n.parse().unwrap()).collect()
}

/// The led civ's Standing in the last view of `view`: its share, as `civ 1 Red 21.5%, rank 2 you`.
fn standing(view: &str) -> f64 {
    let line = view.lines().rev().find(|l| l.starts_with("  civ ") && l.contains(" you")).unwrap();
    let share = line.split_whitespace().find(|w| w.ends_with("%,") || w.ends_with('%')).unwrap();
    share.trim_end_matches(',').trim_end_matches('%').parse().unwrap()
}

#[test]
fn the_claim_order_names_every_tile_of_the_land_and_the_reach_order_is_accepted() {
    let (_dir, game) = game_in("claim-order");
    let made = sansfiat(&["play", "--game", &game, "--seed", "2", "--size", "small", "--civ", "1"], RAISE);
    let claim = made.lines().find(|l| l.contains(r#"{"type":"claim","tiles":["#) && !l.contains("the claim in reach"));
    let claim = claim.unwrap_or_else(|| panic!("no claim order in:\n{made}"));
    assert!(claim.contains("a tile you leave out is dropped"), "{claim}");
    let mine = order_ids(claim);
    let land = land_ids(&made);
    assert_eq!(land.len(), mine.len(), "the order lists the land's tiles: {claim}\n{made}");
    assert!(land.iter().all(|t| mine.contains(t)), "every id of the land line is in the claim line");

    let reach =
        made.lines().find(|l| l.contains("the claim in reach")).unwrap_or_else(|| panic!("no reach order:\n{made}"));
    let adds = reach.split(": adds ").nth(1).unwrap();
    let new_tiles: Vec<usize> = order_ids(reach).into_iter().filter(|t| !land.contains(t)).collect();
    assert!(adds.starts_with("1 tile,"), "{reach}");
    assert_eq!(new_tiles.len(), 1, "{reach}");
    let clan: u32 =
        reach.split("brings clan ").nth(1).and_then(|r| r.split_whitespace().next()).unwrap().parse().unwrap();

    let before = standing(&made);
    let order = reach.split("}  ").next().unwrap().trim_start().to_string() + "}";
    let out = sansfiat(&["play", "--game", &game], &format!("{order}\n"));
    assert!(out.contains("clan ") && !out.contains("refused"), "the order is accepted:\n{out}");
    assert!(out.to_lowercase().contains(&format!("clan {clan} joins")), "the clan joins the civ:\n{out}");
    let mut grown = land.clone();
    grown.extend(&new_tiles);
    grown.sort_unstable();
    let mut after = land_ids(&out);
    after.sort_unstable();
    assert_eq!(after, grown, "the land grows by exactly the reach's tiles");
    assert!(standing(&out) > before, "Standing grows with the clan: {before} to {}", standing(&out));
}
