use engine::{Action, Config, Save, World, TERRAIN};

#[test]
fn same_seed_same_world() {
    let run = || {
        let mut w = World::new(Config::default());
        w.run(30);
        w
    };
    assert_eq!(run(), run());
}

#[test]
fn replaying_a_save_rebuilds_the_world() {
    for seed in 1..=5 {
        let mut w = World::new(Config { seed, radius: 6 });
        w.run(12);
        let json = serde_json::to_string(&w.save()).unwrap();
        let save: Save = serde_json::from_str(&json).unwrap();
        assert_eq!(World::replay(&save).unwrap(), w);
    }
}

#[test]
fn every_action_is_logged_and_counts_turns() {
    let mut w = World::new(Config::default());
    w.act(Action::EndTurn).unwrap();
    w.run(2);
    assert_eq!(w.turn, 3);
    assert_eq!(w.log, vec![Action::EndTurn; 3]);
}

#[test]
fn actions_read_and_write_as_tagged_json() {
    assert_eq!(serde_json::to_string(&Action::EndTurn).unwrap(), r#"{"type":"endTurn"}"#);
}

#[test]
fn the_map_has_sea_at_the_rim_and_land_inside() {
    let w = World::new(Config::default());
    let r = w.config.radius;
    assert_eq!(w.tiles.len() as i32, 3 * r * (r + 1) + 1);
    assert!(w.tiles.iter().filter(|t| t.hex().distance(engine::hex::Hex::ORIGIN) == r).all(|t| t.terrain == 0));
    assert!(w.tiles.iter().any(|t| TERRAIN[t.terrain].land));
}
