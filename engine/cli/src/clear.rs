use std::fs;
use std::io::Read;

use engine::{clear, utility, Basket, Params, GOODS, N};

pub(crate) fn basket_of(v: &serde_json::Value, what: &str) -> Result<Basket, String> {
    let obj = v.as_object().ok_or(format!("{what} must be an object of good -> amount"))?;
    let mut b = [0.0; N];
    for (k, amount) in obj {
        let g = GOODS.iter().position(|g| g == k).ok_or(format!("unknown good {k}; goods: {}", GOODS.join(" ")))?;
        b[g] = amount.as_f64().ok_or(format!("{what}.{k} must be a number"))?;
    }
    Ok(b)
}

pub(crate) fn clear_file(path: &str) -> Result<(), String> {
    let mut text = String::new();
    if path == "-" {
        std::io::stdin().read_to_string(&mut text).map_err(|e| e.to_string())?;
    } else {
        text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    }
    let input: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let baskets = input["baskets"]
        .as_array()
        .ok_or("expected {\"baskets\": [...]}")?
        .iter()
        .enumerate()
        .map(|(i, b)| basket_of(b, &format!("baskets[{i}]")))
        .collect::<Result<Vec<_>, _>>()?;
    let start = match input.get("prices") {
        Some(p) => basket_of(p, "prices")?.map(|v| if v > 0.0 { v } else { 1.0 }),
        None => [1.0; N],
    };

    let c = clear(&baskets, &start, &Params::default());
    let name = |g: usize| GOODS[g];
    println!("cleared {} after {} iterations", c.cleared, c.iterations);
    println!("{:>8}  {}", "", c.goods.iter().map(|&g| format!("{:>7}", name(g))).collect::<Vec<_>>().join(" "));
    let pick = |b: &Basket| c.goods.iter().map(|&g| format!("{:>7.3}", b[g])).collect::<Vec<_>>().join(" ");
    println!("{:>8}  {}", "prices", pick(&c.prices));
    println!("{:>8}  {}", "volume", pick(&c.volume));
    for (i, (b, a)) in baskets.iter().zip(&c.after).enumerate() {
        println!("{:>8}  {}  ->  {}  utility {:.3} -> {:.3}", i, pick(b), pick(a), utility(b), utility(a));
    }
    Ok(())
}
