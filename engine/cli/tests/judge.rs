//! `sweep --judge --json --solo`: the report has the groups and keys the judge's land and claim
//! measures are read from, one map per retro map, each with its no-claims control and three solo runs.

use std::process::Command;

#[test]
fn sweep_judge_json_has_the_claim_groups_and_keys() {
    let out = Command::new(env!("CARGO_BIN_EXE_sansfiat"))
        .args(["sweep", "--judge", "--json", "--solo"])
        .output()
        .expect("the sansfiat binary runs");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).expect("the report is JSON");
    let maps = v["maps"].as_array().expect("a list of maps");
    assert_eq!(maps.len(), 7, "the retro maps");
    for m in maps {
        assert!(m["no_claims"]["doubt"]["lead_changes"].is_u64(), "{m}");
        assert!(m["standing"]["on_land_200"].is_f64() || m["standing"]["on_land_200"].is_null(), "{m}");
        assert!(m["motion"]["clans_left"].is_u64() && m["motion"]["claim_taken"].is_u64(), "{m}");
        let civs = m["land"]["civs_200"].as_array().expect("land per civ at +200");
        assert!(civs.iter().all(|c| c["claimed"].is_u64() && c["held"].is_u64() && c["guard"].is_f64()), "{m}");
        let solo = m["solo"].as_array().expect("the solo runs, with --solo");
        assert_eq!(solo.len(), 3, "one solo run per civ");
        assert!(solo.iter().all(|s| s["gain"].is_f64() && s["clans"].is_f64() && s["land"].is_f64()), "{m}");
    }
    let t = &v["total"];
    assert!(t["solo"]["above"].is_u64() && t["solo"]["civs"].is_u64(), "{t}");
    assert!(t["doubt"]["no_claims_lead_changes"].is_u64(), "{t}");
    assert!(t["motion"]["clans_joined"].is_u64(), "{t}");
    assert!(t["land"]["claimed_200"].is_f64() && t["land"]["guard_200"].is_f64(), "{t}");
    assert!(t["standing"]["on_land_200"].is_f64(), "{t}");
    assert_eq!(t["health"]["panics"], 0, "{t}");
}
