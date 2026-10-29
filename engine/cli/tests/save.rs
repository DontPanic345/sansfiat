//! The save version guards the rules: these digests are the rules' fingerprint. A change that moves
//! them changes what a save replays into, so it bumps `SAVE_VERSION` and these constants together.

use std::process::Command;

use engine::world::SAVE_VERSION;

/// The version these digests are for.
const VERSION: u32 = 1;

/// `sansfiat digest` for `seed` on the small map, from turn 0 to `turn`, and the digest it prints
/// for that turn (the last line).
fn digest(seed: &str, turn: &str, lead: bool) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sansfiat"));
    cmd.args(["digest", "--seed", seed, "--size", "small", "--turns", turn]);
    if lead {
        cmd.arg("--lead");
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
    // Seeds 1-3 watched from the pick (turns 316, 283, 219) for 20 turns, and seed 2 led by the
    // scripted game of `digest --lead` from its pick (283) for the same 20 turns.
    assert_eq!(digest("1", "336", false), "be1197ab970402b9");
    assert_eq!(digest("2", "303", false), "8b866f497fb07b8d");
    assert_eq!(digest("3", "239", false), "85468f7177213aab");
    assert_eq!(digest("2", "303", true), "ed8f01e25fd6433b");
}
