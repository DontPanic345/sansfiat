//! The save version guards the rules: these digests are the rules' fingerprint. A change that moves
//! them changes what a save replays into, so it bumps `SAVE_VERSION` and these constants together.

use std::process::Command;

use engine::world::SAVE_VERSION;

/// The version these digests are for.
const VERSION: u32 = 8;

/// `sansfiat digest` for `seed` on the small map, from turn 0 to `turn`, and the digest it prints
/// for that turn (the last line).
fn digest(seed: &str, turn: &str, lead: bool) -> String {
    digest_with(seed, turn, lead, false)
}

/// As `digest`, with `--state` when `state` is set (the world hashed without its log).
fn digest_with(seed: &str, turn: &str, lead: bool, state: bool) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sansfiat"));
    cmd.args(["digest", "--seed", seed, "--size", "small", "--turns", turn]);
    if lead {
        cmd.arg("--lead");
    }
    if state {
        cmd.arg("--state");
    }
    let out = cmd.output().expect("the sansfiat binary runs");
    assert!(out.status.success(), "digest failed: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8(out.stdout).unwrap();
    let last = text.lines().last().expect("digest prints lines");
    let (printed, digest) = last.split_once(' ').expect("`turn digest`");
    assert_eq!(printed, turn, "the last line is the turn asked for");
    digest.to_string()
}

#[test]
fn the_save_version_pins_the_rules() {
    assert_eq!(SAVE_VERSION, VERSION, "bump SAVE_VERSION and these digests together");
    // Seeds 1-3 watched from the pick (turns 314, 236, 244) for 20 turns, and seed 2 led by the
    // scripted game of `digest --lead` from its pick (236) for the same 20 turns.
    assert_eq!(digest("1", "334", false), "a8598b82eb51bbeb");
    assert_eq!(digest("2", "256", false), "ff4816b3dc048e55");
    assert_eq!(digest("3", "264", false), "3105f025620ed7c0");
    assert_eq!(digest("2", "256", true), "ccd11adcfe0474c8");
}

#[test]
fn the_led_game_state_digest_is_its_baseline() {
    // The led game's world without its log: the rules' fingerprint, which the log's shape does not move.
    assert_eq!(digest_with("2", "256", true, true), "1be7ec6e2e7fe358");
}
