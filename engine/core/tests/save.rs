//! The save's version: written first, kept through a round trip, and refused with a reason when it
//! isn't this build's.

use engine::world::SAVE_VERSION;
use engine::{Config, Save, World};

#[test]
fn a_save_carries_its_version_and_round_trips() {
    let mut w = World::new(Config::default());
    w.run(12);
    let save = w.save();
    assert_eq!(save.version, SAVE_VERSION);
    let json = serde_json::to_string(&save).unwrap();
    assert!(json.starts_with("{\"version\":1,"), "the version comes first: {json}");
    let back: Save = serde_json::from_str(&json).unwrap();
    assert_eq!(World::replay(&back).unwrap(), w);
}

#[test]
fn a_save_without_a_version_is_refused_naming_it() {
    let mut json = serde_json::to_value(World::new(Config::default()).save()).unwrap();
    json.as_object_mut().unwrap().remove("version");
    let save: Save = serde_json::from_value(json).unwrap();
    assert_eq!(save.version, 0, "a save without a version reads as 0");
    let err = World::replay(&save).unwrap_err();
    assert_eq!(err, "this save is from an older build (no version); this build reads version 1");
}

#[test]
fn a_save_of_another_version_is_refused_naming_both() {
    let mut save = World::new(Config::default()).save();
    save.version = 2;
    let err = World::replay(&save).unwrap_err();
    assert_eq!(err, "this save is version 2; this build reads version 1");
}
