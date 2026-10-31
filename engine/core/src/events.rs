//! What happened in a turn: the events, their wording, and how one is made. A rule makes an event
//! with `Event::new(kind, tile)` and its subject's setters, and records it with `World::emit`.

use serde::{Deserialize, Serialize};

use crate::world::World;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventKind {
    ClanDivided,
    ClanStarving,
    ClanMoved,
    ClanMerged,
    MarketFormed,
    MarketFaded,
    HearthTamed,
    CentreFormed,
    PregameEnded,
    CivFounded,
    CampRaised,
    WarriorsRaised,
    WarriorsDisbanded,
    CampResupplied,
    CampCutOff,
    CampHungry,
    WarriorsStarved,
    CampForaged,
    CampPillaged,
    ScoutRouteFound,
    ScoutNoRoute,
    ScoutTrailBuilt,
    ScoutPassFound,
    ScoutBlocked,
    ScoutHungry,
    ScoutStarved,
    ClanJoined,
    Deployed,
    Held,
    LineCut,
    PeopleLeft,
}

/// Something that happened in a turn. `tile` is where; `other` is the daughter of a division, a
/// zone's size (`Deployed`, `Held`) or the civ whose army cut a line (`LineCut`); `unit` is for units.
/// `alert` marks the events the led civ must hear of as they start (`flag_alerts`); the page and
/// the CLI read it from the JSON as they read the rest.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub turn: u32,
    pub kind: EventKind,
    pub tile: usize,
    pub clan: Option<u32>,
    pub market: Option<u32>,
    pub other: Option<u32>,
    pub civ: Option<u32>,
    pub unit: Option<u32>,
    /// A route's cost, for the scout's route events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    /// A number of people, for `PeopleLeft`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default)]
    pub alert: bool,
}

impl Event {
    /// The one wording of an event, for the CLI and the page.
    pub fn text(&self, w: &World) -> String {
        let h = w.tiles[self.tile].hex();
        let at = format!("({}, {})", h.q, h.r);
        let clan = self.clan.unwrap_or(0);
        let market = self.market.unwrap_or(0);
        match self.kind {
            EventKind::ClanDivided => {
                format!("Clan {clan} divides; clan {} settles at {at}", self.other.unwrap_or(0))
            }
            EventKind::ClanStarving => format!("Clan {clan} is starving"),
            EventKind::ClanMerged => format!("Clan {clan} merges into clan {}", self.other.unwrap_or(0)),
            EventKind::ClanMoved => format!("Clan {clan} moves on to {at}"),
            EventKind::MarketFormed => format!("A market forms at {at}"),
            EventKind::MarketFaded => format!("Market {market} fades"),
            EventKind::HearthTamed => format!("A hearth is tamed at {at}"),
            EventKind::CentreFormed => match self.clan {
                Some(c) => format!("Clan {c} becomes a centre"),
                None => format!("Market {market} becomes a centre"),
            },
            EventKind::CivFounded => {
                let name = self.civ.and_then(|id| w.civs.get(&id)).map_or("A civ", |c| c.name.as_str());
                format!("{name} is founded at {at}, on market {market}")
            }
            EventKind::CampRaised => format!("{} raises a camp at {at}", self.civ_name(w)),
            EventKind::WarriorsRaised => format!("{} raises more warriors for its camp", self.civ_name(w)),
            EventKind::WarriorsDisbanded | EventKind::WarriorsStarved => {
                let why = if self.kind == EventKind::WarriorsStarved { " from hunger" } else { "" };
                let person = match self.clan {
                    Some(c) => format!("they rejoin clan {c}"),
                    None => "they are lost".to_string(),
                };
                format!("{}'s warriors disband{why} at {at}; {person}", self.civ_name(w))
            }
            EventKind::CampResupplied => format!("{}'s camp at {at} is resupplied", self.civ_name(w)),
            EventKind::CampCutOff => format!("{}'s camp at {at} is cut off from its capital", self.civ_name(w)),
            EventKind::CampHungry => format!("{}'s camp at {at} goes hungry", self.civ_name(w)),
            EventKind::CampForaged => format!("{}'s camp forages at {at}", self.civ_name(w)),
            EventKind::CampPillaged => format!("{}'s camp pillages at {at}", self.civ_name(w)),
            EventKind::ScoutRouteFound => {
                format!("{}'s scout has its route, cost {:.1}", self.civ_name(w), self.cost.unwrap_or(0.0))
            }
            EventKind::ScoutNoRoute => format!("{}'s scout: no route can exist", self.civ_name(w)),
            EventKind::ScoutTrailBuilt => format!("{}'s scout builds a trail at {at}", self.civ_name(w)),
            EventKind::ScoutPassFound => format!("{}'s scout finds a pass at {at}", self.civ_name(w)),
            EventKind::ScoutBlocked => format!("{}'s scout is blocked at {at}", self.civ_name(w)),
            EventKind::ScoutHungry => format!("{}'s scout at {at} goes hungry", self.civ_name(w)),
            EventKind::ScoutStarved => format!("{}'s scout at {at} loses a person to hunger", self.civ_name(w)),
            EventKind::ClanJoined => format!("Clan {clan} joins {}", self.civ_name(w)),
            EventKind::PeopleLeft => {
                format!("{:.1} people leave {}'s clans for others", self.amount.unwrap_or(0.0), self.civ_name(w))
            }
            EventKind::Deployed => {
                format!("{}'s camp at {at} deploys over {} tiles", self.civ_name(w), self.other.unwrap_or(0))
            }
            EventKind::Held => {
                format!("{}'s camp at {at} holds its zone of {} tiles", self.civ_name(w), self.other.unwrap_or(0))
            }
            EventKind::LineCut => {
                let by = self.other.and_then(|id| w.civs.get(&id)).map_or("a rival", |c| c.name.as_str());
                format!("{}'s camp's line is cut by {by}'s army", self.civ_name(w))
            }
            EventKind::PregameEnded => match &w.end {
                Some(e) => {
                    let n = e.centres.len();
                    let centres = format!("{n} centre{}", if n == 1 { "" } else { "s" });
                    if e.topped_up.is_empty() {
                        format!("The pre-game ends: {centres}")
                    } else {
                        format!("The pre-game ends: {centres}, {} topped up", e.topped_up.len())
                    }
                }
                None => "The pre-game ends".to_string(),
            },
        }
    }
}

impl Event {
    /// The name of the event's civ; civs never go away.
    fn civ_name<'a>(&self, w: &'a World) -> &'a str {
        self.civ.and_then(|id| w.civs.get(&id)).map_or("A civ", |c| c.name.as_str())
    }

    /// The event as JSON, with its wording in `text`.
    pub fn json(&self, w: &World) -> serde_json::Value {
        let mut v = serde_json::to_value(self).unwrap();
        v["text"] = self.text(w).into();
        v
    }
}

impl Event {
    /// An event of `kind` at `tile`, with no subject. The setters name what it is about; `emit`
    /// stamps the turn.
    pub fn new(kind: EventKind, tile: usize) -> Event {
        Event {
            turn: 0,
            kind,
            tile,
            clan: None,
            market: None,
            other: None,
            civ: None,
            unit: None,
            cost: None,
            amount: None,
            alert: false,
        }
    }

    pub fn clan(mut self, id: impl Into<Option<u32>>) -> Event {
        self.clan = id.into();
        self
    }

    pub fn market(mut self, id: impl Into<Option<u32>>) -> Event {
        self.market = id.into();
        self
    }

    pub fn civ(mut self, id: impl Into<Option<u32>>) -> Event {
        self.civ = id.into();
        self
    }

    pub fn unit(mut self, id: impl Into<Option<u32>>) -> Event {
        self.unit = id.into();
        self
    }

    /// The daughter of a division, a zone's size, or the civ whose army cut a line.
    pub fn other(mut self, n: impl Into<Option<u32>>) -> Event {
        self.other = n.into();
        self
    }

    /// A route's cost.
    pub fn cost(mut self, c: impl Into<Option<f64>>) -> Event {
        self.cost = c.into();
        self
    }

    /// A number of people.
    pub fn amount(mut self, a: impl Into<Option<f64>>) -> Event {
        self.amount = a.into();
        self
    }
}

impl World {
    /// Records `e` at the current turn.
    pub fn emit(&mut self, mut e: Event) {
        e.turn = self.turn;
        self.events.push(e);
    }
}
