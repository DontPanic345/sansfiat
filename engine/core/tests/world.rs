use engine::{Config, World, TERRAIN};

#[test]
fn same_seed_same_world() {
    let run = || {
        let mut w = World::new(Config::default());
        w.run(30);
        serde_json::to_string(&w).unwrap()
    };
    assert_eq!(run(), run());
}

#[test]
fn clans_live_on_land_and_trade_within_reach() {
    let w = World::new(Config::default());
    assert_eq!(w.clans.len(), Config::default().clans);
    for c in &w.clans {
        assert!(TERRAIN[w.tiles[c.tile].terrain].land);
        if let Some(m) = c.market {
            assert!(w.tiles[c.tile].hex().distance(w.tiles[w.markets[m].tile].hex()) <= w.config.reach);
        }
    }
}

#[test]
fn trade_never_lowers_total_welfare() {
    for seed in 1..=5 {
        let mut w = World::new(Config { seed, ..Config::default() });
        w.run(20);
        for t in &w.history {
            assert!(t.traded >= t.autarky - 1e-6, "seed {seed} turn {}: {} < {}", t.turn, t.traded, t.autarky);
        }
    }
}
