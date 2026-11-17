use serde::Serialize;

use crate::data::{GOOD, N};

pub type Basket = [f64; N];

/// The food in a basket: its eaten goods (`GOOD`).
pub fn total(b: &Basket) -> f64 {
    (0..N).filter(|&g| GOOD[g].eaten).map(|g| b[g]).sum()
}

/// A basket of `food` shared evenly over the foods, and no salt or obsidian.
pub fn spread(food: f64) -> Basket {
    let n = GOOD.iter().filter(|g| g.eaten).count() as f64;
    std::array::from_fn(|g| if GOOD[g].eaten { food / n } else { 0.0 })
}

/// What a basket weighs: each good at its `GOOD` weight.
pub fn weight(b: &Basket) -> f64 {
    (0..N).map(|g| b[g] * GOOD[g].weight).sum()
}

/// A basket's goods that aren't eaten, summed.
pub fn wares(b: &Basket) -> f64 {
    (0..N).filter(|&g| !GOOD[g].eaten).map(|g| b[g]).sum()
}

/// Every trader's worth of a food, and of nothing else: what a trader with no use for salt or
/// obsidian wants.
pub const FOOD_WORTH: Basket = {
    let mut out = [0.0; N];
    let mut g = 0;
    while g < N {
        if GOOD[g].eaten {
            out[g] = 1.0;
        }
        g += 1;
    }
    out
};

/// Takes up to `amount` of food from `from`, in proportion to the food it holds. What isn't eaten
/// stays.
pub fn take(from: &mut Basket, amount: f64) -> Basket {
    let held = total(from);
    if held <= 0.0 || amount <= 0.0 {
        return [0.0; N];
    }
    let share = (amount / held).min(1.0);
    let out: Basket = std::array::from_fn(|g| if GOOD[g].eaten { from[g] * share } else { 0.0 });
    for f in 0..N {
        from[f] -= out[f];
    }
    out
}

/// Adds `b` into `to`, good by good.
pub fn add(to: &mut Basket, b: &Basket) {
    for f in 0..N {
        to[f] += b[f];
    }
}

/// The sum of the takes, good by good.
pub fn sum(takes: &[Basket]) -> Basket {
    std::array::from_fn(|f| takes.iter().map(|t| t[f]).sum())
}

/// Each trader values a basket at Σ worth × sqrt(amount), so given exchange ratios `prices` it wants
/// amount_g ∝ (worth_g / p_g)² of each good, spending exactly what its own basket is worth. A food's
/// worth is 1; a good the trader has no use for is worth 0 to it, and it wants none. A trader that
/// wants none of `goods` keeps what it brought.
pub fn demand(basket: &Basket, prices: &Basket, goods: &[usize], worth: &Basket) -> Basket {
    let wealth: f64 = goods.iter().map(|&g| prices[g] * basket[g]).sum();
    let denom: f64 = goods.iter().map(|&g| worth[g] * worth[g] / prices[g]).sum();
    let mut out = [0.0; N];
    for &g in goods {
        out[g] = if denom > 0.0 { wealth * worth[g] * worth[g] / (prices[g] * prices[g] * denom) } else { basket[g] };
    }
    out
}

pub fn utility(basket: &Basket) -> f64 {
    basket.iter().map(|v| v.max(0.0).sqrt()).sum()
}

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub iterations: usize,
    pub step: f64,
    pub tolerance: f64,
}

impl Default for Params {
    fn default() -> Self {
        Params { iterations: 300, step: 0.5, tolerance: 1e-5 }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Clearing {
    /// Geometric mean 1 over `goods`; any other good keeps its start price.
    pub prices: Basket,
    pub after: Vec<Basket>,
    pub volume: Basket,
    /// The goods anyone brought.
    pub goods: Vec<usize>,
    /// False with fewer than 2 traders or goods: nothing changed hands and `prices` are the start prices.
    pub cleared: bool,
    pub iterations: usize,
}

/// Finds exchange ratios at which supply meets demand by tâtonnement, then hands out the goods.
/// `worths` is each trader's worth of each good (`demand`). A good nobody there wants is not traded:
/// it stays with whoever brought it.
pub fn clear(baskets: &[Basket], worths: &[Basket], start: &Basket, p: &Params) -> Clearing {
    let mut supply = [0.0; N];
    for b in baskets {
        for g in 0..N {
            supply[g] += b[g];
        }
    }
    let goods: Vec<usize> = (0..N).filter(|&g| supply[g] > 1e-9 && worths.iter().any(|w| w[g] > 0.0)).collect();
    let mut prices = *start;
    if baskets.len() < 2 || goods.len() < 2 {
        let after = baskets.to_vec();
        return Clearing { prices, after, volume: [0.0; N], goods, cleared: false, iterations: 0 };
    }

    let mut iterations = 0;
    while iterations < p.iterations {
        iterations += 1;
        let mut want = [0.0; N];
        for (b, w) in baskets.iter().zip(worths) {
            let d = demand(b, &prices, &goods, w);
            for &g in &goods {
                want[g] += d[g];
            }
        }
        let mut worst = 0.0f64;
        for &g in &goods {
            let excess = (want[g] - supply[g]) / supply[g];
            worst = worst.max(excess.abs());
            prices[g] *= libm::exp(p.step * excess.clamp(-1.0, 1.0));
        }
        let mean = libm::exp(goods.iter().map(|&g| libm::log(prices[g])).sum::<f64>() / goods.len() as f64);
        for &g in &goods {
            prices[g] /= mean;
        }
        if worst < p.tolerance {
            break;
        }
    }

    // Ration whichever side of each good is long, so goods are conserved exactly.
    let net: Vec<Basket> = baskets
        .iter()
        .zip(worths)
        .map(|(b, w)| {
            let d = demand(b, &prices, &goods, w);
            let mut z = [0.0; N];
            for &g in &goods {
                z[g] = d[g] - b[g];
            }
            z
        })
        .collect();
    let mut after = baskets.to_vec();
    let mut volume = [0.0; N];
    for &g in &goods {
        let buys: f64 = net.iter().map(|z| z[g].max(0.0)).sum();
        let sells: f64 = net.iter().map(|z| (-z[g]).max(0.0)).sum();
        let v = buys.min(sells);
        if v <= 0.0 {
            continue;
        }
        volume[g] = v;
        for (a, z) in after.iter_mut().zip(&net) {
            a[g] += if z[g] > 0.0 { z[g] * (v / buys) } else { z[g] * (v / sells) };
        }
    }
    Clearing { prices, after, volume, goods, cleared: true, iterations }
}
