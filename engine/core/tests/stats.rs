use engine::data::*;
use engine::land::is_hearth;
use engine::stats::{centre_report, digest, summary};
use engine::{Config, World};

#[test]
fn summary_counts_match_the_world() {
    let mut w = World::new(Config { seed: 2, radius: 12 });
    w.run(100);
    let s = summary(&w);
    assert_eq!(s.turn, w.turn);
    assert_eq!(s.clans, w.clans.len());
    let people: f64 = w.clans.values().map(|c| c.people).sum();
    assert!((s.people - people).abs() < 1e-9);
    assert_eq!(s.markets, w.markets.len());
    assert_eq!(s.trails, w.tiles.iter().filter(|t| t.trail).count());
    assert_eq!(s.hearths, w.tiles.iter().filter(|t| is_hearth(t)).count());
    assert_eq!(s.held, w.tiles.iter().filter(|t| t.claim.is_some()).count());
    assert_eq!(s.trading, w.clans.values().filter(|c| c.market.is_some()).count());
    assert_eq!(s.starving, w.clans.values().filter(|c| c.starving).count());
    let n = w.clans.len().max(1) as f64;
    assert!((s.territory * n - s.held as f64).abs() < 1e-9, "territories are disjoint");
    assert!((s.walk_cost - w.clans.values().map(|c| c.walk).sum::<f64>() / n).abs() < 1e-9);
    assert_eq!(s.end, w.end);
}

#[test]
fn centre_report_compares_with_the_mean_over_land() {
    let mut w = World::new(Config { seed: 1, radius: 12 });
    w.run(100);
    let land: Vec<f64> = w.tiles.iter().filter(|t| TERRAIN[t.terrain].land).map(|t| t.fertility).collect();
    let mean = land.iter().sum::<f64>() / land.len() as f64;
    // Any markets stand in when none is a centre yet.
    w.markets.insert(
        99,
        engine::Market {
            id: 99,
            tile: land_tile(&w),
            founded: 0,
            prices: [1.0; N],
            goods: vec![],
            activity: 0.0,
            volume: 0.0,
            traders: 0,
            moved: 0.0,
        },
    );
    let ids: Vec<u32> = w.markets.keys().copied().collect();
    let report = centre_report(&w, &ids);
    assert_eq!(report.len(), ids.len());
    for r in report {
        assert!((r.fertility_land - mean).abs() < 1e-9);
        assert!(r.fertility_near >= FERTILITY.0 && r.fertility_near <= FERTILITY.1);
    }
}

fn land_tile(w: &World) -> usize {
    w.tiles.iter().position(|t| TERRAIN[t.terrain].land).unwrap()
}

#[test]
fn digest_is_stable_16_hex_and_moves_with_the_world() {
    let run = |turns| {
        let mut w = World::new(Config { seed: 3, radius: 10 });
        w.run(turns);
        w
    };
    let (a, b) = (run(20), run(20));
    assert_eq!(digest(&a), digest(&b));
    let d = digest(&a);
    assert_eq!(d.len(), 16);
    assert!(d.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')));
    assert_ne!(d, digest(&run(21)));
}
