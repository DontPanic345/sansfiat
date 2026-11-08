//! The save version guards the rules: these digests are the rules' fingerprint. A change that moves
//! them changes what a save replays into, so it bumps `SAVE_VERSION` and these constants together.

use std::process::Command;

use engine::world::SAVE_VERSION;

/// The version these digests are for.
const VERSION: u32 = 15;

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
    // Seeds 1-3 watched from their picks (`play` gives 271, 260, 289) to turns 290, 279, 308, and seed 2
    // led by the scripted game of `digest --lead` to turn 279: the turns stay where the flat trail's picks put them.
    assert_eq!(digest("1", "290", false), "51330d17c09fea27");
    assert_eq!(digest("2", "279", false), "9ad46f328e7c8814");
    assert_eq!(digest("3", "308", false), "980fc476cb9eb247");
    assert_eq!(digest("2", "279", true), "326e8ccac6a0228c");
}

#[test]
fn the_led_game_state_digest_is_its_baseline() {
    // The led game's world without its log: the rules' fingerprint, which the log's shape does not move.
    assert_eq!(digest_with("2", "279", true, true), "50753d38dda1bf3d");
}
