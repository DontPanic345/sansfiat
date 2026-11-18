//! The stage tests in `core/stage/`: every listed test holds, and every file there is listed.

use std::fs;
use std::path::Path;

use engine::stage_test::{list, Run, LIST};

#[test]
fn every_listed_test_holds() {
    let failures: Vec<String> = list().iter().filter_map(|t| Run::run_all(t).err()).collect();
    assert!(failures.is_empty(), "{} failed:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn every_file_is_listed() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("stage");
    let mut unlisted = Vec::new();
    for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.unwrap().path();
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        if path.extension().is_some_and(|x| x == "json") && !LIST.iter().any(|(n, _)| *n == name) {
            unlisted.push(name);
        }
    }
    assert!(unlisted.is_empty(), "not listed in stage_test::LIST: {unlisted:?}");
}
