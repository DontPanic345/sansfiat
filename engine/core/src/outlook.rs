//! A civ's outlook, for the led civ's panel and the text view: what it can order now that brings a
//! clan or a rival's land (the reach line), and who could take its capital (the capital line).
//! Derived from the world each time it is read, never saved.
//!
//! In reach is any of (a) its gift reaches a clan (`civs::gift_reach`), (b) a claim joins the home of
//! a clan not its own (`claim::check` accepts it, and `leaning::land` brings the clan at once), and
//! (c) a rival's tile lies inside the civ's ring (`units::ring`: an army's line feeds it there,
//! whatever its size), so an army that ends its turn on it takes it.

use std::collections::VecDeque;

use crate::civs;
use crate::claim;
use crate::data::*;
use crate::land::{neighbours, steps};
use crate::units::{self, UnitKind};
use crate::world::World;

/// The fewest new claim tiles that join a clan's home (b).
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTo {
    /// The clan whose home the claim ends on, and that home.
    pub clan: u32,
    pub home: usize,
    /// The new tiles, from the civ's land out to the home.
    pub tiles: Vec<usize>,
    /// The clans not the civ's whose homes the new tiles cover, and their people.
    pub clans: usize,
    pub people: f64,
}

/// The rival tile an army could take (c): a rival capital when one is in reach, else the tile whose
/// line is cheapest.
#[derive(Clone, Debug, PartialEq)]
pub struct ArmyTo {
    pub tile: usize,
    pub rival: u32,
    pub capital: bool,
    /// The civ's army nearest the tile by steps; `None` when it has no army.
    pub army: Option<u32>,
}

/// What a civ can order now that brings a clan or a rival's land, and the nearest of each kind.
#[derive(Clone, Debug, PartialEq)]
pub struct Reach {
    pub claim: Option<ClaimTo>,
    /// The clans its gift reaches (a).
    pub gift: usize,
    pub army: Option<ArmyTo>,
    /// The nearest clan not its own to its capital by steps: (clan, home).
    pub near_clan: Option<(u32, usize)>,
    /// The nearest rival tile to its capital by steps: (tile, rival).
    pub near_rival: Option<(usize, u32)>,
}

impl Reach {
    /// Whether anything the civ can order now brings a clan or a rival's land.
    pub fn any(&self) -> bool {
        self.claim.is_some() || self.gift > 0 || self.army.is_some()
    }
}

/// `civ`'s reach now.
pub fn reach(w: &World, civ: u32) -> Reach {
    let Some(c) = w.civs.get(&civ).filter(|c| !c.is_out()) else {
        return Reach { claim: None, gift: 0, army: None, near_clan: None, near_rival: None };
    };
    let cap = c.capital;
    let near_clan = w
        .clans
        .values()
        .filter(|k| k.civ != Some(civ))
        .min_by_key(|k| (steps(w, cap, k.home), k.home, k.id))
        .map(|k| (k.id, k.home));
    let near_rival = rival_tiles(w, civ).into_iter().min_by_key(|&(t, _)| (steps(w, cap, t), t));
    Reach { claim: claim_to(w, civ), gift: civs::gift_reach(w, civ), army: army_to(w, civ), near_clan, near_rival }
}

/// Every tile of a standing rival's land, its capital among them, with that rival.
fn rival_tiles(w: &World, civ: u32) -> Vec<(usize, u32)> {
    civs::alive(w).filter(|o| o.id != civ).flat_map(|o| o.claim.iter().map(move |&t| (t, o.id))).collect()
}

/// A tile a claim of `civ`'s may cover: land, no rival's land, and no rival's capital.
fn claimable(w: &World, civ: u32, i: usize) -> bool {
    TERRAIN[w.tiles[i].terrain].land
        && claim::tile_civ(w, i).is_none_or(|o| o == civ)
        && !civs::alive(w).any(|o| o.id != civ && o.capital == i)
}

/// The fewest new claim tiles that join the home of a clan not `civ`'s (b): a walk out from its land
/// over claimable tiles, each new tile one. Ties go to the bigger clan, then the lower tile. A path
/// `claim::check` refuses is passed over for the next.
pub fn claim_to(w: &World, civ: u32) -> Option<ClaimTo> {
    let c = w.civs.get(&civ)?;
    if !claim::may_claim(w, civ) {
        return None;
    }
    let n = w.tiles.len();
    let mut own = vec![false; n];
    for t in c.claim.iter().copied().chain(crate::settle::roots(w, civ)) {
        own[t] = true;
    }
    // 0-1 walk: a tile of its land costs nothing to cross, a new one costs one.
    let mut cost = vec![u32::MAX; n];
    let mut before = vec![usize::MAX; n];
    let mut queue = VecDeque::new();
    for t in (0..n).filter(|&t| own[t]) {
        cost[t] = 0;
        queue.push_back(t);
    }
    while let Some(i) = queue.pop_front() {
        for j in neighbours(w, i) {
            if !claimable(w, civ, j) {
                continue;
            }
            let step = u32::from(!own[j]);
            if cost[i] + step < cost[j] {
                cost[j] = cost[i] + step;
                before[j] = i;
                if step == 0 {
                    queue.push_front(j);
                } else {
                    queue.push_back(j);
                }
            }
        }
    }
    let mut homes: Vec<(u32, f64, usize, u32)> = w
        .clans
        .values()
        .filter(|k| k.civ != Some(civ) && cost[k.home] != u32::MAX && cost[k.home] > 0)
        .map(|k| (cost[k.home], k.people, k.home, k.id))
        .collect();
    homes.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.total_cmp(&a.1)).then(a.2.cmp(&b.2)));
    homes.into_iter().find_map(|(_, _, home, clan)| {
        let mut tiles = Vec::new();
        let mut t = home;
        while !own[t] {
            tiles.push(t);
            t = before[t];
        }
        tiles.reverse();
        let mut all = c.claim.clone();
        all.extend(&tiles);
        claim::check(w, civ, &all).ok()?;
        let covered: Vec<&crate::clans::Clan> =
            w.clans.values().filter(|k| k.civ != Some(civ) && tiles.contains(&k.home)).collect();
        Some(ClaimTo { clan, home, clans: covered.len(), people: covered.iter().map(|k| k.people).sum(), tiles })
    })
}

/// The rival tile an army of `civ`'s could take (c): inside its ring, a rival capital first, else
/// the tile whose line is cheapest (then the lower tile).
pub fn army_to(w: &World, civ: u32) -> Option<ArmyTo> {
    let ring = units::ring(w, civ);
    let (tile, rival) = rival_tiles(w, civ)
        .into_iter()
        .filter_map(|(t, o)| ring.get(&t).filter(|&&s| s < 1.0).map(|&s| (t, o, s)))
        .min_by(|a, b| {
            let cap = |t: usize, o: u32| w.civs[&o].capital != t;
            cap(a.0, a.1).cmp(&cap(b.0, b.1)).then(a.2.total_cmp(&b.2)).then(a.0.cmp(&b.0))
        })
        .map(|(t, o, _)| (t, o))?;
    let army = w
        .units
        .values()
        .filter(|u| u.civ == civ && u.kind() == UnitKind::Camp && u.people > 0.0)
        .min_by_key(|u| (steps(w, u.home, tile), u.id))
        .map(|u| u.id);
    Some(ArmyTo { tile, rival, capital: w.civs[&rival].capital == tile, army })
}

/// The reach line in words: the nearest thing in reach of each kind with the order that takes it,
/// or, when none holds, that nothing the civ can order now brings a clan or a rival's land, with the
/// nearest of each and what would bring it nearer.
pub fn reach_line(w: &World, civ: u32) -> String {
    let r = reach(w, civ);
    let name = |o: u32| w.civs.get(&o).map_or_else(|| format!("civ {o}"), |c| c.name.clone());
    let clan_of = |id: u32| w.clans.get(&id).map_or(0.0, |k| k.people);
    if !r.any() {
        let clan = match r.near_clan {
            None => "no clan is left that is not yours".to_string(),
            Some((id, home)) => {
                let why = match claim::tile_civ(w, home) {
                    Some(o) if o != civ => format!("its home is {}'s land, which an army ending its turn there takes", name(o)),
                    _ => "no claim joins its home (water or a rival's land is between): a signpost toward it may draw a settlement a claim can join"
                        .to_string(),
                };
                format!(
                    "the nearest clan not yours is clan {id} of {:.1} at {}, {} steps off: {why}",
                    clan_of(id),
                    w.at(home),
                    steps(w, w.civs[&civ].capital, home)
                )
            }
        };
        let rival = match r.near_rival {
            None => "no rival has land".to_string(),
            Some((t, o)) => format!(
                "the nearest rival land is {}'s at {}, past every store's reach: a settlement or a trail toward it brings it nearer",
                name(o),
                w.at(t)
            ),
        };
        return format!("nothing you can order now brings a clan or a rival's land; {clan}; {rival}");
    }
    let mut parts = Vec::new();
    if let Some(k) = &r.claim {
        let more = if k.clans > 1 { format!(" and {} more", k.clans - 1) } else { String::new() };
        let over: Vec<String> = k.tiles.iter().map(|&t| w.at(t)).collect();
        parts.push(format!(
            "claim {} to {} ({}): brings clan {}{more}, {:.1} people",
            plural(k.tiles.len(), "tile"),
            w.at(k.home),
            over.join(", "),
            k.clan,
            k.people
        ));
    }
    if r.gift > 0 {
        parts.push(format!("the gift reaches {} at your market", plural(r.gift, "clan")));
    }
    if let Some(a) = &r.army {
        let what = if a.capital { format!("{}'s capital", name(a.rival)) } else { format!("{}'s land", name(a.rival)) };
        parts.push(match a.army {
            Some(id) => format!("army {id} can take {what} at {}: march it there", w.at(a.tile)),
            None => format!("an army can take {what} at {}: raise one and march it there", w.at(a.tile)),
        });
    }
    format!("in reach: {}", parts.join("; "))
}

/// The capital line in words: the civ's army on its capital, if any, what happens without one, and
/// the rival army nearest it by steps.
pub fn capital_line(w: &World, civ: u32) -> String {
    let Some(c) = w.civs.get(&civ).filter(|c| !c.is_out()) else { return String::new() };
    let cap = c.capital;
    let guard: f64 = w
        .units
        .values()
        .filter(|u| u.civ == civ && u.kind() == UnitKind::Camp && u.home == cap)
        .map(|u| u.people)
        .sum();
    let held = if guard > 0.0 {
        format!("held by your army, {guard:.1} warriors")
    } else {
        "no army of yours on it: a rival army that ends its turn there takes it, and the game".to_string()
    };
    let rival = w
        .units
        .values()
        .filter(|u| u.civ != civ && u.kind() == UnitKind::Camp && u.people > 0.0)
        .filter(|u| w.civs.get(&u.civ).is_some_and(|o| !o.is_out()))
        .min_by_key(|u| (steps(w, u.home, cap), u.id));
    let near = match rival {
        Some(u) => format!("{}'s army of {:.1} is {} steps away", w.civs[&u.civ].name, u.people, steps(w, u.home, cap)),
        None => "no rival army stands".to_string(),
    };
    format!("{held}; {near}")
}

fn plural(n: usize, word: &str) -> String {
    if n == 1 {
        format!("1 {word}")
    } else {
        format!("{n} {word}s")
    }
}
