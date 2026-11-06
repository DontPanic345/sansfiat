//! `judge --compare A.json B.json`: the metrics of two reports, their changes, and the floors. Each
//! case writes small reports to a directory of its own and runs the binary on them.

use std::path::PathBuf;
use std::process::{Command, Output};

fn dir(case: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sansfiat-compare-{case}"));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn compare(case: &str, a: &str, b: &str, all: bool) -> Output {
    let dir = dir(case);
    let (fa, fb) = (dir.join("a.json"), dir.join("b.json"));
    std::fs::write(&fa, a).unwrap();
    std::fs::write(&fb, b).unwrap();
    let mut args = vec!["judge", "--compare"];
    let (sa, sb) = (fa.to_str().unwrap(), fb.to_str().unwrap());
    args.push(sa);
    args.push(sb);
    if all {
        args.push("--all");
    }
    Command::new(env!("CARGO_BIN_EXE_sansfiat")).args(&args).output().expect("the sansfiat binary runs")
}

fn stdout(out: &Output) -> String {
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout.clone()).unwrap()
}

/// The table row of `path`, if it is printed.
fn row<'a>(text: &'a str, path: &str) -> Option<&'a str> {
    text.lines().find(|l| l.trim_start().strip_prefix(path).is_some_and(|r| r.starts_with(' ')))
}

#[test]
fn flatten_keys_arrays_by_id_or_index_and_maps_by_size_and_seed() {
    let a = r#"{
        "maps": [{"size": "small", "seed": 2, "x": 1.0}, {"size": "medium", "seed": 8, "x": 2.0}],
        "total": {"list": [{"id": 7, "v": 1}, {"v": 2}], "flag": true, "name": "s", "gone": null},
        "seconds": 1.0, "panics": [], "floor": {}
    }"#;
    let text = stdout(&compare("keys", a, a, true));
    for path in ["maps.small 2.x", "maps.medium 8.x", "maps.small 2.seed", "total.list.7.v", "total.list.1.v"] {
        assert!(row(&text, path).is_some(), "{path} is listed:\n{text}");
    }
    for path in ["total.flag", "total.name", "total.gone", "seconds", "panics", "floor"] {
        assert!(row(&text, path).is_none(), "{path} is not a metric:\n{text}");
    }
}

#[test]
fn a_change_within_the_floor_is_not_flagged_and_one_past_it_is() {
    let a = r#"{"total": {"x": 1.0, "y": 1.0}, "floor": {"total.x": 0.5, "total.y": 0.5}}"#;
    let b = r#"{"total": {"x": 1.4, "y": 1.6}, "floor": {"total.x": 0.2, "total.y": 0.2}}"#;
    let text = stdout(&compare("floor", a, b, false));
    assert!(!row(&text, "total.x").unwrap().ends_with('*'), "{text}");
    assert!(row(&text, "total.y").unwrap().ends_with('*'), "{text}");
    assert!(text.contains("flagged 1"), "{text}");
}

#[test]
fn a_metric_in_one_report_only_is_listed_and_never_flagged() {
    let a = r#"{"total": {"x": 1.0, "only_a": 3.0}, "floor": {"total.x": 0.1}}"#;
    let b = r#"{"total": {"x": 1.0, "only_b": 9.0}, "floor": {"total.x": 0.1}}"#;
    let text = stdout(&compare("only", a, b, false));
    assert!(row(&text, "total.only_a").unwrap().contains("only in A"), "{text}");
    assert!(row(&text, "total.only_b").unwrap().contains("only in B"), "{text}");
    assert!(!row(&text, "total.only_a").unwrap().contains('*'), "{text}");
    assert!(!row(&text, "total.only_b").unwrap().contains('*'), "{text}");
    assert!(text.contains("only in A 1, only in B 1"), "{text}");
}

#[test]
fn with_no_floor_nothing_is_flagged() {
    let a = r#"{"total": {"x": 1.0}}"#;
    let b = r#"{"total": {"x": 5.0}}"#;
    let text = stdout(&compare("nofloor", a, b, false));
    assert!(!row(&text, "total.x").unwrap().contains('*'), "{text}");
    assert!(text.contains("changed 1, flagged 0"), "{text}");
    assert_eq!(text.lines().last(), Some("no floor in either report: nothing is flagged"), "{text}");
}

#[test]
fn a_report_against_itself_has_no_changes() {
    let a = r#"{"total": {"x": 1.0, "y": 2.5}, "floor": {"total.x": 0.0, "total.y": 0.0}}"#;
    let text = stdout(&compare("same", a, a, false));
    assert!(text.contains("listed 2, changed 0, flagged 0, only in A 0, only in B 0"), "{text}");
}

#[test]
fn a_missing_or_bad_file_is_an_error_naming_it() {
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat"))
        .args(["judge", "--compare", "/nonexistent/a.json", "/nonexistent/b.json"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("/nonexistent/a.json"));
    let out = compare("bad", "{not json", "{}", false);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("a.json"), "{}", String::from_utf8_lossy(&out.stderr));
}
