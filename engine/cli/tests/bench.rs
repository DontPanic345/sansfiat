//! `bench --after N`: one line per seed, its world, pre-game and N turns past the pick; and the
//! refusal of `--turns` with `--after`.

use std::process::Command;

#[test]
fn bench_after_prints_its_line_for_a_small_map() {
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat"))
        .args(["bench", "--size", "small", "--after", "3", "2"])
        .output()
        .expect("the sansfiat binary runs");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.starts_with("seed 2: world "), "{text}");
    assert!(text.contains(" ms, pre-game ") && text.contains(" ms, 3 turns past the pick "), "{text}");
    assert!(text.trim_end().ends_with(" ms a turn)"), "{text}");
}

#[test]
fn bench_turns_and_after_together_is_an_error() {
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat"))
        .args(["bench", "--size", "small", "--turns", "3", "--after", "3", "2"])
        .output()
        .expect("the sansfiat binary runs");
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("give one"));
}
