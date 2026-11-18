//! Trade in the sweep: each map's run read beside its exchange-off control (`Config.exchange` off, which is
//! main's world): army-turns fed at a market not their civ's own, and each good's share of the trades, by count
//! and by worth, at +200 and over the watch.

use engine::judge::Scores;
use engine::judge_trade::TradeScores;
use serde::Serialize;

use crate::sweep_judge::{MapJudged, Report};

/// One map's trade read against its control.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct MapTrade {
    pub(crate) trade: TradeScores,
    pub(crate) control: TradeScores,
}

/// The trade totals over the judged maps.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct TotalTrade {
    pub(crate) trades: u32,
    pub(crate) trades_200: u32,
    pub(crate) army_turns_foreign: u32,
    pub(crate) food_foreign: f64,
    pub(crate) need_foreign: f64,
    pub(crate) army_turns_own: u32,
    pub(crate) unit_turns: u32,
    pub(crate) fills: u32,
    pub(crate) control_army_turns_foreign: u32,
    pub(crate) control_army_turns_own: u32,
    /// The largest share any one good has at +200 on any map, by count and by worth, and the map's good.
    pub(crate) largest_count_200: f64,
    pub(crate) largest_count_good: String,
    pub(crate) largest_worth_200: f64,
    pub(crate) largest_worth_good: String,
}

/// One map's trade read against its exchange-off control.
pub(crate) fn map_trade(run: &Scores, control: &Scores) -> MapTrade {
    MapTrade { trade: run.trade.clone(), control: control.trade.clone() }
}

/// The trade totals over the judged maps.
pub(crate) fn total_trade(maps: &[MapJudged]) -> TotalTrade {
    let mut t = TotalTrade::default();
    for m in maps {
        let (r, c) = (&m.trade_vs_control.trade, &m.trade_vs_control.control);
        t.trades += r.trades;
        t.trades_200 += r.trades_200;
        t.army_turns_foreign += r.army_turns_foreign;
        t.food_foreign += r.food_foreign;
        t.need_foreign += r.need_foreign;
        t.army_turns_own += r.army_turns_own;
        t.unit_turns += r.unit_turns;
        t.fills += r.fills;
        t.control_army_turns_foreign += c.army_turns_foreign;
        t.control_army_turns_own += c.army_turns_own;
        if r.largest_count_200 > t.largest_count_200 {
            t.largest_count_200 = r.largest_count_200;
            t.largest_count_good = format!("{} ({})", r.largest_count_good, m.size);
        }
        if r.largest_worth_200 > t.largest_worth_200 {
            t.largest_worth_200 = r.largest_worth_200;
            t.largest_worth_good = format!("{} ({})", r.largest_worth_good, m.size);
        }
    }
    t
}

/// The trade section of the text report: a line a map, and the totals.
pub(crate) fn print_trade(r: &Report) {
    println!("trade (control: the same map with exchange off)");
    for m in &r.maps {
        let t = &m.trade_vs_control.trade;
        println!(
            "  {} {}: fills {}, army-turns fed at a foreign market {} (food {:.1} of {:.1} needed), at its own {}, unit-turns trading {}; trades {} ({} at +200); control: foreign {}, own {}; largest count share at +200 {} {:.1}%, by worth {} {:.1}%",
            m.size,
            m.seed,
            t.fills,
            t.army_turns_foreign,
            t.food_foreign,
            t.need_foreign,
            t.army_turns_own,
            t.unit_turns,
            t.trades,
            t.trades_200,
            m.trade_vs_control.control.army_turns_foreign,
            m.trade_vs_control.control.army_turns_own,
            t.largest_count_good,
            100.0 * t.largest_count_200,
            t.largest_worth_good,
            100.0 * t.largest_worth_200,
        );
        for g in &t.goods {
            println!(
                "    {}: count {:.1}% at +200 ({:.1}% over the watch), worth {:.1}% ({:.1}%)",
                g.good,
                100.0 * g.count_200,
                100.0 * g.count_watch,
                100.0 * g.worth_200,
                100.0 * g.worth_watch
            );
        }
    }
    let t = &r.total.trade_vs_control;
    println!(
        "total trade: fills {}, army-turns fed at a foreign market {} (control {}), at its own {} (control {}), food {:.1} of {:.1} needed, unit-turns trading {}; trades {} ({} at +200); largest count share at +200 {} {:.1}%, by worth {} {:.1}%",
        t.fills,
        t.army_turns_foreign,
        t.control_army_turns_foreign,
        t.army_turns_own,
        t.control_army_turns_own,
        t.food_foreign,
        t.need_foreign,
        t.unit_turns,
        t.trades,
        t.trades_200,
        t.largest_count_good,
        100.0 * t.largest_count_200,
        t.largest_worth_good,
        100.0 * t.largest_worth_200,
    );
}
