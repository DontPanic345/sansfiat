//! `digest --game FILE` prints a save's turn and digest, the line the page's `Sim.digest()` is matched against.

use std::process::Command;

use engine::{stats, Config, World};

#[test]
fn digest_game_prints_the_saved_worlds_turn_and_digest() {
    let mut w = World::new(Config { seed: 2, radius: 12, ..Config::default() });
    w.run(5);
    let dir = std::env::temp_dir().join("sansfiat-digest-game");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("g.json");
    std::fs::write(&file, serde_json::to_string(&w.save()).unwrap()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat")).args(["digest", "--game"]).arg(&file).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(String::from_utf8(out.stdout).unwrap().trim(), format!("{} {}", w.turn, stats::digest(&w)));
}
