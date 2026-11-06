//! `judge --compare A.json B.json`: two `sweep --judge --json` reports side by side. A metric is a
//! numeric leaf of a report, by its path; its change is B less A, and it is flagged when the change
//! is past its noise floor (the `floor` a report carries from `sweep --judge --json --floor`).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use serde_json::Value;

use crate::USAGE;

/// The report's top-level keys that are not metrics.
const NOT_METRICS: [&str; 3] = ["seconds", "floor", "panics"];

/// The groups, in the order the table prints them. A metric in any other group comes after them,
/// in name order.
const GROUPS: [&str; 9] = ["standing", "doubt", "decisions", "motion", "land", "economy", "armies", "fight", "health"];

/// Every numeric leaf of a report, by its path (`a.b.c`). An array's element is keyed by its `id`,
/// or by `size seed` for a map, else by its index. Booleans and strings are not metrics.
pub(crate) fn flatten(v: &Value) -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    if let Value::Object(map) = v {
        for (k, x) in map.iter().filter(|(k, _)| !NOT_METRICS.contains(&k.as_str())) {
            walk(k, x, &mut out);
        }
    }
    out
}

fn walk(path: &str, v: &Value, out: &mut BTreeMap<String, f64>) {
    match v {
        Value::Number(n) => {
            if let Some(x) = n.as_f64() {
                out.insert(path.to_string(), x);
            }
        }
        Value::Object(map) => {
            for (k, x) in map {
                walk(&format!("{path}.{k}"), x, out);
            }
        }
        Value::Array(items) => {
            for (i, x) in items.iter().enumerate() {
                walk(&format!("{path}.{}", element_key(x, i)), x, out);
            }
        }
        _ => {}
    }
}

/// An array element's key: its `id`, or `size seed` for a map, else its index.
fn element_key(v: &Value, index: usize) -> String {
    let plain = |v: &Value| v.as_str().map_or_else(|| v.to_string(), str::to_string);
    if let Value::Object(o) = v {
        if let Some(id) = o.get("id") {
            return plain(id);
        }
        if let (Some(size), Some(seed)) = (o.get("size"), o.get("seed")) {
            return format!("{} {}", plain(size), plain(seed));
        }
    }
    index.to_string()
}

/// The report's floors, if it carries any: per path, its noise floor.
fn floors(v: &Value) -> Option<BTreeMap<String, f64>> {
    let map = v.get("floor")?.as_object()?;
    Some(map.iter().filter_map(|(k, x)| x.as_f64().map(|f| (k.clone(), f))).collect())
}

/// One metric in both reports or one of them.
pub(crate) struct Row {
    pub(crate) path: String,
    pub(crate) a: Option<f64>,
    pub(crate) b: Option<f64>,
    /// The larger of A's and B's floor for the path; `None` when neither report has floors.
    pub(crate) floor: Option<f64>,
    pub(crate) flagged: bool,
}

impl Row {
    /// B less A, when the metric is in both reports.
    fn change(&self) -> Option<f64> {
        Some(self.b? - self.a?)
    }
}

/// The metrics of two reports, by path in name order.
pub(crate) struct Comparison {
    pub(crate) rows: Vec<Row>,
    /// Whether either report carries floors.
    pub(crate) floored: bool,
}

/// Compares two reports: each metric in either, its change, and whether the change is past its floor.
pub(crate) fn compare(a: &Value, b: &Value) -> Comparison {
    let (fa, fb) = (flatten(a), flatten(b));
    let (ka, kb) = (floors(a), floors(b));
    let floored = ka.is_some() || kb.is_some();
    let paths: BTreeSet<&String> = fa.keys().chain(fb.keys()).collect();
    let rows = paths
        .into_iter()
        .map(|path| {
            let (x, y) = (fa.get(path).copied(), fb.get(path).copied());
            let floor = floored.then(|| {
                [&ka, &kb].iter().filter_map(|k| k.as_ref().and_then(|m| m.get(path)).copied()).fold(0.0, f64::max)
            });
            let flagged = match (x, y, floor) {
                (Some(x), Some(y), Some(f)) => (y - x).abs() > f,
                _ => false,
            };
            Row { path: path.clone(), a: x, b: y, floor, flagged }
        })
        .collect();
    Comparison { rows, floored }
}

/// The group a metric is printed under: `total.<group>.…`, `maps.<map>.<group>.…`, or its first part.
fn group_of(path: &str) -> &str {
    let parts: Vec<&str> = path.split('.').collect();
    match parts.first() {
        Some(&"total") => parts.get(1).copied().unwrap_or("total"),
        Some(&"maps") => parts.get(2).copied().unwrap_or("maps"),
        _ => parts[0],
    }
}

impl Comparison {
    /// The rows printed: the totals, or with `all` every metric, in group order then by path.
    fn listed(&self, all: bool) -> Vec<&Row> {
        let rank = |g: &str| GROUPS.iter().position(|&x| x == g).unwrap_or(GROUPS.len());
        let mut rows: Vec<&Row> = self.rows.iter().filter(|r| all || r.path.starts_with("total.")).collect();
        rows.sort_by(|x, y| {
            let (gx, gy) = (group_of(&x.path), group_of(&y.path));
            (rank(gx), gx, &x.path).cmp(&(rank(gy), gy, &y.path))
        });
        rows
    }
}

fn cell(v: Option<f64>) -> String {
    v.map_or_else(|| "-".to_string(), |v| format!("{v:.4}"))
}

/// The comparison as a table, the counts after it, and a last line when neither report has floors.
pub(crate) fn print(c: &Comparison, all: bool) {
    let rows = c.listed(all);
    println!("{:<56}{:>12}{:>12}{:>12}{:>12}", "metric", "A", "B", "change", "floor");
    let mut group = None;
    for r in &rows {
        let g = group_of(&r.path);
        if group != Some(g) {
            println!("{g}");
            group = Some(g);
        }
        let change = match (r.a, r.b) {
            (Some(_), Some(_)) => cell(r.change()),
            (Some(_), None) => "only in A".to_string(),
            _ => "only in B".to_string(),
        };
        let mark = if r.flagged { " *" } else { "" };
        println!("  {:<54}{:>12}{:>12}{:>12}{:>12}{mark}", r.path, cell(r.a), cell(r.b), change, cell(r.floor));
    }
    let changed = rows.iter().filter(|r| matches!((r.a, r.b), (Some(a), Some(b)) if a != b)).count();
    let flagged = rows.iter().filter(|r| r.flagged).count();
    let only_a = rows.iter().filter(|r| r.a.is_some() && r.b.is_none()).count();
    let only_b = rows.iter().filter(|r| r.a.is_none() && r.b.is_some()).count();
    println!("listed {}, changed {changed}, flagged {flagged}, only in A {only_a}, only in B {only_b}", rows.len());
    if !c.floored {
        println!("no floor in either report: nothing is flagged");
    }
}

fn read(file: &str) -> Result<Value, String> {
    let text = fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("{file}: {e}"))
}

/// `judge --compare A.json B.json [--all]`: the two reports' metrics, printed.
pub(crate) fn compare_files(args: &[String]) -> Result<(), String> {
    let usage = format!("judge --compare needs two report files: A.json B.json [--all]\n{USAGE}");
    let mut files = Vec::new();
    let mut all = false;
    for a in args {
        match a.as_str() {
            "--all" => all = true,
            f if !f.starts_with("--") => files.push(f),
            _ => return Err(usage),
        }
    }
    match files.as_slice() {
        [a, b] => {
            let (va, vb) = (read(a)?, read(b)?);
            print(&compare(&va, &vb), all);
            Ok(())
        }
        _ => Err(usage),
    }
}
