//! `sansfiat inspect`: a save read at a turn, a target by id or tile, a diff between turns, a list.

use std::process::Command;

use serde_json::Value;

fn sansfiat(args: &[&str]) -> Result<Value, String> {
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat"))
        .arg("inspect")
        .args(args)
        .output()
        .expect("the sansfiat binary runs");
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    Ok(serde_json::from_slice(&out.stdout).expect("inspect prints JSON"))
}

/// A watched game on the small map, saved at turn 310 (past the pick, with armies out).
fn save(name: &str) -> String {
    let path = std::env::temp_dir().join(format!("sansfiat-inspect-{name}-{}.json", std::process::id()));
    let path = path.to_str().unwrap().to_string();
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat"))
        .args(["run", "--seed", "2", "--size", "small", "--turns", "310", "--no-events", "--save", &path])
        .output()
        .unwrap();
    assert!(out.status.success());
    path
}

#[test]
fn a_unit_by_id_at_a_turn_and_its_changes_since() {
    let g = save("unit");
    let list = sansfiat(&[&g, "--list", "units"]).unwrap();
    assert_eq!(list["turn"], 310);
    let camp = list["rows"].as_array().unwrap().iter().find(|r| r["kind"] == "camp").expect("an army is out");
    let id = format!("#{}", camp["id"]);
    let now = sansfiat(&[&g, &id]).unwrap();
    assert_eq!(now["at"], camp["at"]);
    assert_eq!(sansfiat(&[&g, "--id", &camp["id"].to_string()]).unwrap()["at"], camp["at"]);

    let early = sansfiat(&[&g, &id, "--turn", "305"]).unwrap();
    assert_eq!(early["turn"], 305);
    let d = sansfiat(&[&g, &id, "--turn", "305", "--diff", "310"]).unwrap();
    assert_eq!((d["from"].as_u64(), d["to"].as_u64()), (Some(305), Some(310)));
    for c in d["changes"].as_array().unwrap() {
        // Each change is the two readings' values at its path.
        let ptr = format!("/{}", c["path"].as_str().unwrap().replace('.', "/"));
        assert_eq!(early.pointer(&ptr).unwrap_or(&Value::Null), &c["from"], "{c}");
        assert_eq!(now.pointer(&ptr).unwrap_or(&Value::Null), &c["to"], "{c}");
    }
    for row in d["notes"].as_array().unwrap().iter().chain(d["events"].as_array().unwrap()) {
        assert!(row["turn"].as_u64().unwrap() > 305, "only what is newer than the first turn: {row}");
    }
    let _ = std::fs::remove_file(g);
}

#[test]
fn a_tile_by_hex_or_index_and_a_new_world() {
    let g = save("tile");
    let civ = &sansfiat(&[&g, "--list", "civs"]).unwrap()["rows"][0];
    let (q, r, i) = (&civ["at"]["q"], &civ["at"]["r"], &civ["at"]["tile"]);
    let by_hex = sansfiat(&[&g, &format!("{q},{r}")]).unwrap();
    let by_index = sansfiat(&[&g, &i.to_string()]).unwrap();
    assert_eq!(by_hex, by_index);
    assert_eq!(by_hex["here"]["capitalOf"], civ["id"]);

    let fresh = sansfiat(&["--seed", "2", "--size", "small", "--turn", "3", "0,0"]).unwrap();
    assert_eq!(fresh["turn"], 3);
    assert!(sansfiat(&["--seed", "2", "0,0"]).unwrap_err().contains("--turn"));
    assert!(sansfiat(&[&g, "#999999"]).unwrap_err().contains("nothing has id"));
    assert!(sansfiat(&[&g]).unwrap_err().contains("needs a target"));
    let _ = std::fs::remove_file(g);
}
