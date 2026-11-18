//! Signposts and settlements. A civ posts a signpost on a tile a clan can live on (`post`); its clans
//! that divide or move on weigh it as a site (`draw`); the first of its clans to live there founds a
//! settlement (`step`, `found`): a market that never fades and a store the levy at that market fills
//! (`civs::levy`), which spoils and is capped (`upkeep`). A settlement roots its civ's claim as the
//! capital does (`roots`), and a rival army that stands on it, with no army of its civ there, takes it
//! with its store (`take`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::claim;
use crate::clans::site_value;
use crate::data::*;
use crate::events::{Event, EventKind};
use crate::land::{cost_map, passable, steps};
use crate::lots::Lots;
use crate::market::{total, Basket};
use crate::rng::rand;
use crate::world::World;

/// A signpost: the tile and the turn it was posted.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signpost {
    pub tile: usize,
    pub turn: u32,
}

/// A settlement: a civ's market and store away from its capital.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settlement {
    pub id: u32,
    pub civ: u32,
    pub tile: usize,
    /// Its market; it never fades.
    pub market: u32,
    /// The turn it was founded.
    pub founded: u32,
    /// What the levy at its market has paid in, less what spoiled.
    pub store: Basket,
    /// Goods cut by the cap last turn.
    pub lost: f64,
    /// Where the wares in its store came from (`lots.rs`).
    #[serde(default, skip_serializing_if = "Lots::is_empty")]
    pub lots: Lots,
}

/// Whether civs post signposts: `Config.signposts`.
pub fn open(w: &World) -> bool {
    w.config.signposts
}

/// The settlement on `tile`.
pub fn at(w: &World, tile: usize) -> Option<&Settlement> {
    w.settlements.values().find(|s| s.tile == tile)
}

/// The settlement whose market is `mid`.
pub fn of_market(w: &World, mid: u32) -> Option<&Settlement> {
    w.settlements.values().find(|s| s.market == mid)
}

/// `civ`'s settlements, in id order.
pub fn of(w: &World, civ: u32) -> impl Iterator<Item = &Settlement> {
    w.settlements.values().filter(move |s| s.civ == civ)
}

/// The tiles `civ`'s claim is joined to: its capital and its settlements.
pub fn roots(w: &World, civ: u32) -> Vec<usize> {
    w.civs.get(&civ).map(|c| c.capital).into_iter().chain(of(w, civ).map(|s| s.tile)).collect()
}

/// `civ`'s store on `tile`: the capital's, or a settlement's of its own there.
pub fn store(w: &World, civ: u32, tile: usize) -> Option<&Basket> {
    let c = w.civs.get(&civ)?;
    if c.capital == tile {
        return Some(&c.store);
    }
    of(w, civ).find(|s| s.tile == tile).map(|s| &s.store)
}

/// As `store`, to draw on or fill.
pub fn store_mut(w: &mut World, civ: u32, tile: usize) -> Option<&mut Basket> {
    if w.civs.get(&civ)?.capital == tile {
        return w.civs.get_mut(&civ).map(|c| &mut c.store);
    }
    w.settlements.values_mut().find(|s| s.civ == civ && s.tile == tile).map(|s| &mut s.store)
}

/// The market of `civ`'s store on `tile`: the capital's, or the settlement's.
pub fn store_market(w: &World, civ: u32, tile: usize) -> Option<u32> {
    let c = w.civs.get(&civ)?;
    if c.capital == tile {
        return Some(c.market);
    }
    of(w, civ).find(|s| s.tile == tile).map(|s| s.market)
}

/// What `civ`'s store on `tile` is called: "its capital", or "its settlement at (q, r)".
pub fn store_name(w: &World, civ: u32, tile: usize) -> String {
    if w.civs.get(&civ).is_some_and(|c| c.capital == tile) {
        "its capital".to_string()
    } else {
        format!("its settlement at {}", w.at(tile))
    }
}

/// Every standing civ's capital and every settlement: the seats a signpost keeps its distance from.
fn seats(w: &World) -> Vec<usize> {
    crate::civs::alive(w).map(|c| c.capital).chain(w.settlements.values().map(|s| s.tile)).collect()
}

/// Whether a signpost of `civ` can stand on `tile`: `Err` says why not.
fn stands(w: &World, civ: u32, tile: usize) -> Result<(), String> {
    if tile >= w.tiles.len() {
        return Err(format!("{tile} is not a tile"));
    }
    if !TERRAIN[w.tiles[tile].terrain].land || !passable(w, tile) {
        return Err("a signpost stands where a clan can live: on land it can walk".to_string());
    }
    if claim::tile_civ(w, tile).is_some_and(|o| o != civ) {
        return Err("a signpost can't stand on a rival's land".to_string());
    }
    if seats(w).iter().any(|&s| steps(w, s, tile) < SETTLE.spacing) {
        return Err(format!("a signpost stands at least {} steps from every capital and settlement", SETTLE.spacing));
    }
    Ok(())
}

/// Checks a signpost order: signposts are on, the tile can carry one (`stands`), nobody lives there
/// yet, the civ has none there and keeps fewer than `SETTLE.signposts`.
pub fn check_post(w: &World, civ: u32, tile: usize) -> Result<(), String> {
    let c = w.civs.get(&civ).ok_or_else(|| format!("{civ} is not a civ"))?;
    if !open(w) {
        return Err("signposts are off: the control".to_string());
    }
    stands(w, civ, tile)?;
    if w.clans.values().any(|k| k.home == tile) {
        return Err("a clan already lives there: a signpost names a place to settle".to_string());
    }
    if c.signposts.iter().any(|s| s.tile == tile) {
        return Err("a signpost already stands there".to_string());
    }
    if c.signposts.len() >= SETTLE.signposts {
        return Err(format!("a civ keeps {} signposts at a time: remove one first", SETTLE.signposts));
    }
    Ok(())
}

/// Checks an order removing the signpost on `tile`.
pub fn check_unpost(w: &World, civ: u32, tile: usize) -> Result<(), String> {
    let c = w.civs.get(&civ).ok_or_else(|| format!("{civ} is not a civ"))?;
    if c.signposts.iter().any(|s| s.tile == tile) {
        Ok(())
    } else {
        Err("no signpost of yours stands there".to_string())
    }
}

/// Posts `civ`'s signpost on `tile`, with a note of what the tile is worth to a daughter.
pub fn post(w: &mut World, civ: u32, tile: usize) {
    let turn = w.turn;
    let c = w.civs.get_mut(&civ).unwrap();
    c.signposts.push(Signpost { tile, turn });
    c.signposts.sort_by_key(|s| s.tile);
    let (at, worth) = (w.at(tile), worth(w, tile));
    w.note(civ, format!("posted a signpost at {at}: the tile would feed a daughter {:.0}% of her need", worth * 100.0));
}

/// Removes `civ`'s signpost on `tile`.
pub fn unpost(w: &mut World, civ: u32, tile: usize) {
    w.civs.get_mut(&civ).unwrap().signposts.retain(|s| s.tile != tile);
    let at = w.at(tile);
    w.note(civ, format!("removed its signpost at {at}"));
}

/// What `tile` is worth as a site: how well it alone would feed a daughter of `CLAN.split_at / 2`
/// people with an empty store (`clans::site_value`), 1 feeding her in full.
pub fn worth(w: &World, tile: usize) -> f64 {
    site_value(w, 0, tile, CLAN.split_at / 2.0, &[0.0; N])
}

/// The chance a clan of `people` at `home` goes to a signpost whose walk costs `cost` days and whose
/// tile would feed it `worth`: `SETTLE.draw × min(worth, 1) × exp(−cost / SETTLE.walk)`.
pub fn chance(cost: f64, worth: f64) -> f64 {
    SETTLE.draw * worth.min(1.0) * libm::exp(-cost / SETTLE.walk)
}

/// The signpost a clan of `civ` goes to instead of its own best site, and what the tile is worth to
/// it: a daughter, or a clan moving on, named `id`, of `people` with `store`, setting out from `home`.
/// Each of its civ's signposts it can walk to within `SETTLE.reach`, where nobody lives yet and which
/// would feed it at least `min`, is drawn with its `chance`; the chances are scaled to sum to at most 1.
/// One `rand` call when any signpost is in the draw, none otherwise.
pub fn draw(
    w: &mut World,
    civ: Option<u32>,
    id: u32,
    home: usize,
    people: f64,
    store: &Basket,
    min: f64,
) -> Option<(usize, f64)> {
    let c = civ.and_then(|k| w.civs.get(&k)).filter(|k| !k.is_out())?;
    if !open(w) || c.signposts.is_empty() {
        return None;
    }
    let near = cost_map(w, &[home], SETTLE.reach, |i| passable(w, i));
    let mut odds: Vec<(usize, f64, f64)> = Vec::new();
    for s in &c.signposts {
        let Some(&cost) = near.get(&s.tile) else { continue };
        if s.tile == home || w.clans.values().any(|k| k.home == s.tile) {
            continue;
        }
        let worth = site_value(w, id, s.tile, people, store);
        if worth > min {
            odds.push((s.tile, worth, chance(cost, worth)));
        }
    }
    if odds.is_empty() {
        return None;
    }
    let sum: f64 = odds.iter().map(|o| o.2).sum();
    let mut r = rand(&mut w.rng) * sum.max(1.0);
    for (tile, worth, p) in odds {
        if r < p {
            return Some((tile, worth));
        }
        r -= p;
    }
    None
}

/// The `settle` step, after `eat`: each civ's signposts that can no longer stand go (a rival's land, a
/// settlement or capital too near), and where a clan of the civ lives on one, the clan (the lowest id)
/// founds a settlement there (`found`).
pub fn step(w: &mut World) {
    let ids: Vec<u32> = crate::civs::alive(w).map(|c| c.id).collect();
    for civ in ids {
        let posts: Vec<usize> = w.civs[&civ].signposts.iter().map(|s| s.tile).collect();
        for tile in posts {
            if let Err(why) = stands(w, civ, tile) {
                w.civs.get_mut(&civ).unwrap().signposts.retain(|s| s.tile != tile);
                let at = w.at(tile);
                w.note(civ, format!("its signpost at {at} is gone: {why}"));
                continue;
            }
            let settler = w.clans.values().find(|k| k.home == tile && k.civ == Some(civ)).map(|k| k.id);
            if let Some(clan) = settler {
                found(w, civ, tile, clan);
            }
        }
    }
}

/// Founds `civ`'s settlement on `tile`, where its clan `clan` lives: the festival there, or a new
/// market, is the settlement's; the signpost goes; the tile joins the civ's claim where it may claim;
/// the clan goes to the market at its door. Emits `SettlementFounded`.
pub fn found(w: &mut World, civ: u32, tile: usize, clan: u32) -> u32 {
    let market = w.markets.values().find(|m| m.tile == tile).map(|m| m.id);
    let market = market.unwrap_or_else(|| crate::markets::raise(w, tile));
    let id = w.next_id;
    w.next_id += 1;
    w.settlements.insert(
        id,
        Settlement { id, civ, tile, market, founded: w.turn, store: [0.0; N], lost: 0.0, lots: Lots::default() },
    );
    w.civs.get_mut(&civ).unwrap().signposts.retain(|s| s.tile != tile);
    if claim::may_claim(w, civ) {
        claim::join(w, civ, tile);
    }
    let k = w.clans.get_mut(&clan).unwrap();
    k.market = Some(market);
    k.market_turns = 0;
    k.routes.clear();
    w.emit(Event::new(EventKind::SettlementFounded, tile).civ(civ).clan(clan).market(market));
    let (at, name) = (w.at(tile), w.civs[&civ].name.clone());
    w.note(civ, format!("clan {clan} settled at its signpost at {at}: a settlement is founded"));
    w.note(clan, format!("founded {name}'s settlement at {at}: it settled at the signpost"));
    w.note(id, format!("founded by clan {clan} of {name}"));
    crate::leaning::land(w, false);
    id
}

/// `by`'s army `unit` takes the settlement `id`, which it stands on: the settlement and its store are
/// `by`'s, its tile leaves its civ's claim with the tiles that joined the claim only through it, and
/// joins `by`'s. Emits `SettlementTaken` and notes both civs.
pub fn take(w: &mut World, id: u32, by: u32, unit: u32) {
    let (owner, tile, held) = {
        let s = w.settlements.get_mut(&id).unwrap();
        let owner = std::mem::replace(&mut s.civ, by);
        (owner, s.tile, total(&s.store))
    };
    let claim = w.civs[&owner].claim.clone();
    let without: Vec<usize> = claim.iter().copied().filter(|&t| t != tile).collect();
    let kept = claim::connected_part(w, owner, &without);
    let lost = claim.len() - kept.len();
    claim::set(w, owner, kept);
    if claim::may_claim(w, by) {
        claim::join(w, by, tile);
    }
    w.emit(Event::new(EventKind::SettlementTaken, tile).civ(owner).other(by).unit(unit).amount(lost as f64));
    let at = w.at(tile);
    let name = |c: u32| w.civs.get(&c).map_or_else(|| "a rival".to_string(), |c| c.name.clone());
    let (taker, loser) = (name(by), name(owner));
    w.note(owner, format!("lost its settlement at {at} and {lost} tile(s) of land: {taker}'s army stands on it"));
    w.note(by, format!("its army took {loser}'s settlement at {at}, with {held:.0} food in its store"));
    w.note(id, format!("taken by {taker}'s army from {loser}"));
}

/// A fallen civ's signposts go, and its settlements with their stores: their markets are ordinary
/// festivals from then on.
pub fn fallen(w: &mut World, civ: u32) {
    w.civs.get_mut(&civ).unwrap().signposts.clear();
    w.settlements.retain(|_, s| s.civ != civ);
}

/// The civs' part of the `civs` step for settlements: each store spoils and what is over
/// `SETTLE.store_cap` is lost.
pub fn upkeep(w: &mut World) {
    for s in w.settlements.values_mut() {
        crate::actors::spoil(&mut s.store);
        let all = total(&s.store);
        s.lost = (all - SETTLE.store_cap).max(0.0);
        if s.lost > 0.0 {
            crate::market::take(&mut s.store, s.lost);
        }
    }
}

/// What the page and the text view show of a settlement: its store, the clans that live on its tile
/// and that come to its market, and the tiles of its civ's claim nearer it than any other root.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Panel {
    pub id: u32,
    pub civ: u32,
    pub tile: usize,
    pub market: u32,
    pub founded: u32,
    pub store: Basket,
    pub cap: f64,
    /// The levy paid into its store this turn.
    pub levied: f64,
    /// The clans living on its tile, ids ascending.
    pub clans: Vec<u32>,
    /// The clans that come to its market, ids ascending.
    pub traders: Vec<u32>,
    /// Its land: the tiles of its civ's claim nearer it than the capital or another settlement.
    pub land: Vec<usize>,
}

/// Each settlement's panel, in id order.
pub fn panels(w: &World) -> Vec<Panel> {
    let mut near: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for c in crate::civs::alive(w) {
        let roots = roots(w, c.id);
        for &t in &c.claim {
            let root = roots.iter().copied().min_by_key(|&r| (steps(w, r, t), r != c.capital, r));
            if let Some(s) = root.and_then(|r| at(w, r)).filter(|s| s.civ == c.id) {
                near.entry(s.id).or_default().push(t);
            }
        }
    }
    w.settlements
        .values()
        .map(|s| Panel {
            id: s.id,
            civ: s.civ,
            tile: s.tile,
            market: s.market,
            founded: s.founded,
            store: s.store,
            cap: SETTLE.store_cap,
            levied: w.settle_levied.get(&s.id).copied().unwrap_or(0.0),
            clans: w.clans.values().filter(|k| k.home == s.tile).map(|k| k.id).collect(),
            traders: w.clans.values().filter(|k| k.market == Some(s.market)).map(|k| k.id).collect(),
            land: near.remove(&s.id).unwrap_or_default(),
        })
        .collect()
}
