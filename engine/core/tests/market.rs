use engine::{clear, rng::rand, utility, Basket, Params, N};

const P: Params = Params { iterations: 300, step: 0.5, tolerance: 1e-5 };

fn total(bs: &[Basket]) -> Basket {
    let mut t = [0.0; N];
    for b in bs {
        for g in 0..N {
            t[g] += b[g];
        }
    }
    t
}

#[test]
fn rng_matches_the_js_prototypes() {
    let mut s = 1;
    assert_eq!(rand(&mut s), 0.6270739405881613);
    assert_eq!(rand(&mut s), 0.002735721180215478);
    assert_eq!(rand(&mut s), 0.5274470399599522);
    assert_eq!(s, 1199730144);
}

#[test]
fn symmetric_traders_split_evenly() {
    let c = clear(&[[4.0, 0.0, 0.0, 0.0, 0.0], [0.0, 4.0, 0.0, 0.0, 0.0]], &[1.0; N], &P);
    assert!(c.cleared);
    assert_eq!(c.goods, vec![0, 1]);
    for b in &c.after {
        assert!((b[0] - 2.0).abs() < 1e-4 && (b[1] - 2.0).abs() < 1e-4, "{b:?}");
    }
}

#[test]
fn the_scarce_good_is_dearer() {
    // Σ sqrt utility: aggregate demand ∝ 1/p², so p_fish / p_grain = sqrt(9 / 1).
    let c = clear(&[[9.0, 0.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0, 0.0]], &[1.0; N], &P);
    assert!((c.prices[1] / c.prices[0] - 3.0).abs() < 1e-3, "{:?}", c.prices);
}

#[test]
fn goods_are_conserved_and_nobody_loses() {
    let before =
        [[5.0, 1.0, 0.0, 2.0, 0.0], [0.0, 6.0, 1.0, 0.0, 0.0], [1.0, 0.0, 7.0, 0.5, 0.0], [2.0, 2.0, 2.0, 2.0, 0.0]];
    let c = clear(&before, &[1.0; N], &P);
    let (a, b) = (total(&before), total(&c.after));
    for g in 0..N {
        assert!((a[g] - b[g]).abs() < 1e-9, "good {g}: {} -> {}", a[g], b[g]);
    }
    for (x, y) in before.iter().zip(&c.after) {
        assert!(utility(y) >= utility(x) - 1e-3, "{x:?} -> {y:?}");
    }
    let mean = c.goods.iter().map(|&g| c.prices[g].ln()).sum::<f64>() / c.goods.len() as f64;
    assert!(mean.abs() < 1e-12);
}

#[test]
fn one_trader_does_not_clear() {
    let start = [2.0, 0.5, 1.0, 1.0, 1.0];
    let c = clear(&[[1.0, 1.0, 1.0, 1.0, 0.0]], &start, &P);
    assert!(!c.cleared);
    assert_eq!(c.prices, start);
}
