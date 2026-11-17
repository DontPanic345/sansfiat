//! Goods in the sweep: each map's run read against its no-sources control (`Config.goods` off): the
//! share of festivals where salt or obsidian trades, how far each is found from its source at +200,
//! and the world's people and each civ's Standing beside the control's.

use engine::judge::Scores;
use engine::judge_goods::GoodsScores;
use serde::Serialize;

use crate::sweep_judge::{MapJudged, Report};

/// One map's goods read against its control.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct MapGoods {
    pub(crate) goods: GoodsScores,
    /// The control's festival-turns where salt or obsidian traded: none, with no sources.
    pub(crate) control_festivals_wares: u32,
    /// The people of the world at the end of the watch, and the control's.
    pub(crate) people: f64,
    pub(crate) control_people: f64,
    /// Starving clan-turns over clan-turns after +100, and the control's.
    pub(crate) starving: f64,
    pub(crate) control_starving: f64,
}

/// One good over the seven maps.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct TotalGood {
    pub(crate) good: String,
    pub(crate) sources: u32,
    pub(crate) sources_held_200: u32,
    pub(crate) festivals: u32,
    pub(crate) held_200: f64,
    pub(crate) stores_200: u32,
    pub(crate) stores_away_200: u32,
    /// The mean over every store holding it at +200 of its steps from the nearest source, and the
    /// furthest on any map.
    pub(crate) mean_steps_200: f64,
    pub(crate) max_steps_200: i32,
    pub(crate) civ_stores_200: u32,
    /// Maps where it is found in a store that holds no source of it at +200.
    pub(crate) maps_away_200: u32,
}

/// The seven maps' goods.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct TotalGoods {
    pub(crate) goods: Vec<TotalGood>,
    pub(crate) festivals: u32,
    pub(crate) festivals_wares: u32,
    /// `festivals_wares` over `festivals`, and the control's (0 with no sources).
    pub(crate) share: f64,
    pub(crate) control_share: f64,
    pub(crate) armies_armed: u32,
    /// The people of the seven worlds over the control's.
    pub(crate) people_vs_control: f64,
    pub(crate) starving: f64,
    pub(crate) control_starving: f64,
}

fn starving(s: &Scores) -> f64 {
    s.towns.total.all.starving_share()
}

/// One map's goods read against its control; `people` are the worlds' at the watch's end.
pub(crate) fn map_goods(run: &Scores, control: &Scores, people: f64, control_people: f64) -> MapGoods {
    MapGoods {
        goods: run.goods.clone(),
        control_festivals_wares: control.goods.festivals_wares,
        people,
        control_people,
        starving: starving(run),
        control_starving: starving(control),
    }
}

/// The goods totals over the judged maps.
pub(crate) fn total_goods(maps: &[MapJudged]) -> TotalGoods {
    let mut t = TotalGoods::default();
    let (mut people, mut control, mut control_wares, mut control_festivals) = (0.0, 0.0, 0, 0);
    let mut steps: Vec<f64> = Vec::new();
    for m in maps {
        let g = &m.goods_vs_control;
        t.festivals += g.goods.festivals;
        t.festivals_wares += g.goods.festivals_wares;
        t.armies_armed += g.goods.armies_armed;
        control_wares += g.control_festivals_wares;
        control_festivals += m.no_goods.goods.festivals;
        people += g.people;
        control += g.control_people;
        t.starving += g.starving / maps.len() as f64;
        t.control_starving += g.control_starving / maps.len() as f64;
        for (k, s) in g.goods.goods.iter().enumerate() {
            if t.goods.len() <= k {
                t.goods.push(TotalGood { good: s.good.clone(), ..TotalGood::default() });
                steps.push(0.0);
            }
            let o = &mut t.goods[k];
            o.sources += s.sources;
            o.sources_held_200 += s.sources_held_200;
            o.festivals += s.festivals;
            o.held_200 += s.held_200;
            o.stores_200 += s.stores_200;
            o.stores_away_200 += s.stores_away_200;
            o.civ_stores_200 += s.civ_stores_200;
            o.max_steps_200 = o.max_steps_200.max(s.max_steps_200);
            o.maps_away_200 += (s.stores_away_200 > 0) as u32;
            steps[k] += s.mean_steps_200 * f64::from(s.stores_200);
        }
    }
    for (o, s) in t.goods.iter_mut().zip(steps) {
        o.mean_steps_200 = if o.stores_200 > 0 { s / f64::from(o.stores_200) } else { 0.0 };
    }
    let over = |a: u32, b: u32| if b > 0 { f64::from(a) / f64::from(b) } else { 0.0 };
    t.share = over(t.festivals_wares, t.festivals);
    t.control_share = over(control_wares, control_festivals);
    t.people_vs_control = if control > 0.0 { people / control } else { 0.0 };
    t
}

/// The goods section of the text report: a line a map and a good, and the totals.
pub(crate) fn print_goods(r: &Report) {
    println!("goods (control: the same map with no sources)");
    for m in &r.maps {
        let g = &m.goods_vs_control;
        println!(
            "  {} {}: festivals with salt or obsidian traded {} of {} ({:.1}%; control {}), armies armed {}, people {:.0} ({:.0}), starving {:.3} ({:.3})",
            m.size,
            m.seed,
            g.goods.festivals_wares,
            g.goods.festivals,
            100.0 * f64::from(g.goods.festivals_wares) / f64::from(g.goods.festivals.max(1)),
            g.control_festivals_wares,
            g.goods.armies_armed,
            g.people,
            g.control_people,
            g.starving,
            g.control_starving
        );
        for s in &g.goods.goods {
            println!(
                "    {}: sources {} ({} held at +200), traded at {} festival-turns; at +200 held {:.1} in {} stores, {} of them away from any source, mean {:.1} steps from a source, furthest {}; civ stores holding it {}, most a civ store held {:.1}",
                s.good, s.sources, s.sources_held_200, s.festivals, s.held_200, s.stores_200, s.stores_away_200,
                s.mean_steps_200, s.max_steps_200, s.civ_stores_200, s.civ_most
            );
        }
    }
    let t = &r.total.goods_vs_control;
    println!(
        "total goods: festivals with salt or obsidian traded {} of {} ({:.1}%; control {:.1}%), armies armed {}, people over control {:.3}, starving {:.3} ({:.3})",
        t.festivals_wares,
        t.festivals,
        t.share * 100.0,
        t.control_share * 100.0,
        t.armies_armed,
        t.people_vs_control,
        t.starving,
        t.control_starving
    );
    for o in &t.goods {
        println!(
            "total {}: sources {} ({} held at +200), traded at {} festival-turns; at +200 held {:.1} in {} stores, {} away from any source, mean {:.1} steps from a source, furthest {}; civ stores {}; maps where it left its source {} of {}",
            o.good, o.sources, o.sources_held_200, o.festivals, o.held_200, o.stores_200, o.stores_away_200,
            o.mean_steps_200, o.max_steps_200, o.civ_stores_200, o.maps_away_200, r.maps.len()
        );
    }
}
