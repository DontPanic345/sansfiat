//! A clan's labour, split into jobs. After its routes are paid, its days go to gathering over the
//! tiles it claims, to fields on its river ground (`fields.rs`), to herding over its grass
//! (`herds.rs`), and to a source's pan or hill (`sources.rs`). Each job divides its days evenly over
//! its tiles, every tile paying its walk first (`actors::split`).

use crate::actors::{self, Route};
use crate::data::*;
use crate::fields;
use crate::herds;
use crate::market::Basket;
use crate::world::World;

/// What the jobs of a clan's labour come to over its territory.
pub struct Jobs {
    /// Each tile's gathering work, 0 where `gathers` is false.
    pub gather: Vec<f64>,
    /// The field job's work on each field tile.
    pub field: Vec<f64>,
    /// The field tiles' indices into `tiles`.
    pub fk: Vec<usize>,
    /// What the source job digs, by good.
    pub dug: Basket,
    /// The herding job's days on each herding tile.
    pub herd: Vec<f64>,
    /// The herding tiles' indices into `tiles`.
    pub hk: Vec<usize>,
}

/// The jobs of a clan's labour over `tiles` (its territory, with `yearly` walks): gathering over the
/// tiles `gathers` marks, the field job at `fields` over the tiles with ground, and the herding job at
/// `herds` over the tiles with grass (`herds::graze`), each tile paying its walk. A field tile is a
/// tile with ground that yields: days are not sown on grain still wild. A job with no tile gives
/// nothing: `fields` and `herds` then go to gathering. A clan that gathers a tile with a source gives
/// `SOURCE.share` of its days to the source job, taken from the others by their shares; `dug` is what it
/// digs, by good. With `dig` off it has no source job. A herder walks its land at `HERD.walk` of a
/// gatherer's weight. Herding has a share only above 0 and once herds are open.
#[allow(clippy::too_many_arguments)]
pub fn jobs(
    w: &World,
    people: f64,
    routes: &[Route],
    tiles: &[usize],
    yearly: &[f64],
    fields: f64,
    herds: f64,
    gathers: &[bool],
    dig: bool,
) -> Jobs {
    let fk: Vec<usize> = (0..tiles.len())
        .filter(|&k| fields::ground(w, tiles[k]) > 0.0 && fields::yield_a_day(w, tiles[k]) > 0.0)
        .collect();
    let share = if fk.is_empty() { 0.0 } else { fields };
    let gk: Vec<usize> = (0..tiles.len()).filter(|&k| gathers[k]).collect();
    let gy: Vec<f64> = gk.iter().map(|&k| yearly[k]).collect();
    let fy: Vec<f64> = fk.iter().map(|&k| yearly[k]).collect();
    // The source job: the tiles with a source the clan gathers, at `SOURCE.share` of its days.
    let sk: Vec<usize> =
        (0..tiles.len()).filter(|&k| dig && gathers[k] && w.tiles[tiles[k]].source.is_some()).collect();
    let dig = if sk.is_empty() { 0.0 } else { SOURCE.share };
    let sy: Vec<f64> = sk.iter().map(|&k| yearly[k]).collect();
    // The herding job: the grass in the territory, walked at the herder's weight.
    let hk: Vec<usize> = if herds > 0.0 && herds::open(w) {
        (0..tiles.len()).filter(|&k| herds::graze(w, tiles[k]) > 0.0).collect()
    } else {
        Vec::new()
    };
    let herd = if hk.is_empty() { 0.0 } else { herds };
    let weight = if herd >= HERD.herder - 1e-9 { HERD.walk } else { 1.0 };
    let hy: Vec<f64> = hk.iter().map(|&k| yearly[k] * weight).collect();
    let (mut parts, herd_part) = if herd > 0.0 {
        let shares = [1.0 - share - herd - dig * (1.0 - share - herd), share - dig * share, dig, herd - dig * herd];
        let mut parts = actors::split(people, people * CLAN.work, routes, &shares, &[gy, fy, sy, hy]);
        let h = parts.pop().unwrap();
        (parts, h)
    } else {
        let shares = [1.0 - share - dig * (1.0 - share), share - dig * share, dig];
        (actors::split(people, people * CLAN.work, routes, &shares, &[gy, fy, sy]), Vec::new())
    };
    let mut dug = [0.0; N];
    for (&k, days) in sk.iter().zip(parts.pop().unwrap()) {
        if let Some(g) = w.tiles[tiles[k]].source {
            dug[g] += days * SOURCE.a_day;
        }
    }
    let field = parts.pop().unwrap();
    let gathered = parts.pop().unwrap();
    let mut gather = vec![0.0; tiles.len()];
    for (&k, g) in gk.iter().zip(gathered) {
        gather[k] = g;
    }
    Jobs { gather, field, fk, dug, herd: herd_part, hk }
}
