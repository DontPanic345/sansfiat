//! `story`: each story plays a few turns on small seed 2, and its save replays to the digest it ended on
//! (the save is a normal save: `digest --game FILE` replays it).

use std::process::Command;

use engine::{stats, Save, World};

fn run(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat")).args(args).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn each_story_saves_a_game_that_replays_to_the_same_digest() {
    let dir = std::env::temp_dir().join("sansfiat-story");
    std::fs::create_dir_all(&dir).unwrap();
    for story in ["skirmish", "stack", "surrender"] {
        for fight in [true, false] {
            let file = dir.join(format!("{story}-{fight}.json"));
            let file = file.to_str().unwrap();
            let mut args =
                vec!["story", story, "--seed", "2", "--size", "small", "--civ", "1", "--turns", "12", "--save", file];
            if !fight {
                args.push("--no-fight");
            }
            let said = run(&args);
            assert!(said.contains(story), "{said}");
            let save: Save = serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
            assert_eq!(save.config.fight, fight, "{story}: --no-fight turns fighting off");
            let w = World::replay(&save).unwrap();
            let line = run(&["digest", "--game", file]);
            assert_eq!(line.trim(), format!("{} {}", w.turn, stats::digest(&w)), "{story} fight {fight}");
            // The save is scored by `judge` and resumed by `play`.
            assert!(run(&["judge", file]).contains("fight (game / control)"), "{story}: judge prints the fight group");
        }
    }
}
