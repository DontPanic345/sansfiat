//! Units: scouts, the planner, the unit kinds, the fighters and the fight.

use super::*;
use serde::{Deserialize, Serialize};

pub struct ScoutRules {
    /// People a scout is made with, from the civ's clans, one each.
    pub people: u32,
    /// Days to reveal a forest or hills tile (a mountain takes a whole turn's search labour).
    pub reveal: f64,
    /// Turns of eating a scout keeps over the walk to its store when it turns for it: the turn it
    /// spends working before it looks again, and the turn it arrives on. With a pack of 36 (`CARRY.scout`)
    /// no scout starves at 1 to 4 on the retro maps (`relay/specs/337/errand.md`); 3 leaves both turns.
    pub spare: f64,
    /// The share of a full pack a scout with an errand waits for at a store before it sets out. A civ
    /// whose store gains about a food a turn (medium 9) holds a scout at the store for tens of turns
    /// at a half; a quarter builds as many routes as a tenth, with fewer turns on the way.
    pub out: f64,
}

pub const SCOUT: ScoutRules = ScoutRules { people: 2, reveal: 1.0 * DAY, spare: 3.0, out: 0.25 };

/// The planner (`units/ai.rs`): what the AI's civs, and the automatic player, weigh each turn.
pub struct PlanRules {
    /// A share: the levy a turn must cover the civ's mouths and one more this many times over before
    /// it raises that one.
    pub feed: f64,
    /// A share of the store's cap: a store this full has its levy cut to the room left, so the levy
    /// counts as feeding one more whenever it covers the mouths at all.
    pub full: f64,
    /// Turns of its mouths' food the store keeps beyond a new warrior's pack.
    pub reserve: f64,
    /// Turns of its food a pack holds under which an army with salt goes to a market to buy food.
    pub trade_at: f64,
    /// The most of a civ's people its army may be, a share.
    pub army: f64,
    /// Food a turn it gives while a clan at its festival is a rival's or leans to one.
    pub gift_contested: f64,
    /// Tiles a turn its claim widens by, while its armies are enough to hold the wider claim.
    pub widen: usize,
    /// Tiles of its claim a person of its armies holds: the planner claims no more.
    pub hold: f64,
    /// Hex steps: the claim widens only toward a clan's home this near its edge.
    pub widen_reach: i32,
    /// How many times a defender's strength an army's must be before it marches onto it.
    pub edge: f64,
    /// Turns: a capital the army outweighs falls to it within this many, by the fight's own rule.
    pub siege: u32,
    /// The share of its people an army must have left when the capital falls.
    pub left: f64,
    /// Turns' walk: the farthest capital an army larger than a party marches on, on its pack.
    pub march: u32,
    /// People: the smallest army that leaves its post to pillage or to cut a line.
    pub raid: f64,
    /// A share of an army's days: the most its line may take where it pillages (`units::ring`).
    pub reach: f64,
    /// The share of its pack an army must hold to leave its own land.
    pub packed: f64,
    /// Turns of food in its pack under which an army abroad halts to forage.
    pub low: f64,
    /// People: the largest army that goes past its line's reach, living off the land.
    pub party: f64,
    /// How many of the nearest rival tiles an army weighs when it picks one to pillage.
    pub looks: usize,
    pub note: &'static str,
}

pub const PLAN: PlanRules = PlanRules {
    feed: 1.5,
    full: 0.75,
    reserve: 2.0,
    trade_at: 2.0,
    army: 0.25,
    gift_contested: 4.0,
    widen: 1,
    hold: 15.0,
    widen_reach: 3,
    edge: 1.5,
    siege: 8,
    left: 0.3,
    march: 4,
    raid: 3.0,
    reach: 0.5,
    packed: 0.8,
    low: 1.0,
    party: 4.0,
    looks: 12,
    note: "Tuned on the seven retro maps against the judge's led games (relay/specs/157/canvas.md): feed 1.5 fills \
           a pack past spoilage where 1.25 never did; march 3 or 4 and a party of 4 keep an army where its pack \
           and the land feed it (a stack of 14 sent 16 steps starved every time); low 1 turns a party back \
           before it starves and still reaches a rival (2 kept every warrior fed but took 5 tiles against 17). hold 15 (relay/specs/336/scores.md): at 6 a party of 4 never widens the claim it is \
           founded with and 8 tiles change hands on the retro maps; at 15, 36 do and the lead changes 13 times; \
           at 30 the claims cover the map and one capital falls where two did.",
};

pub struct UnitRules {
    /// Days a person a turn: one person's work: what a camp spends on its steps, its foraging and its line (`CLAN.work`).
    pub labour: f64,
    /// Food a person a turn a warrior eats from the pack: one person's need.
    pub food: f64,
    /// Hungry turns in a row before the camp loses a warrior (and one more each turn after).
    pub hungry_turns: u32,
    /// The fewest people a clan must have to give one to the camp.
    pub min_clan: f64,
    /// Warriors raised at founding. 0: a civ starts with its scout and nothing else, and raises its
    /// army itself (`founding.md`, The start).
    pub start: u32,
    /// The warriors of a first army, where a script or the judge needs a number (`Lead`, the stories,
    /// the camp sweep's lever, the economy group's pack).
    pub first: u32,
    /// Food a day a unit takes from a tile it forages (own or unclaimed land).
    pub forage: f64,
    /// Food a day a unit takes from a rival's tile it pillages.
    pub pillage: f64,
    /// Turns a stripped tile yields nothing to anyone and regrows nothing.
    pub fallow: u32,
    /// Hex steps from the capital the AI posts its camp.
    pub post: usize,
    /// Days a person a turn a warrior sets aside to guard a deployed camp's zone, divided over its held tiles.
    pub guarding: f64,
    /// A share: the guard share a tile must keep after its walk to stay held (`divide`).
    pub guard: f64,
    /// The weight of a guard's walk inside the zone (`walk_cost`), as a clan's inside its territory.
    pub inside: f64,
    /// Tiles (hex steps) of its line the AI's zone covers past the camp's own, at most `people − 1`.
    pub zone: usize,
    /// Salt a person's pack takes from a store of its civ (`Order::Fill`): the pack's salt room.
    pub salt: f64,
}

pub const UNIT: UnitRules = UnitRules {
    labour: 3.5 * DAY,
    food: 1.0,
    hungry_turns: 3,
    min_clan: 2.0,
    start: 0,
    first: 3,
    forage: 0.5 / DAY,
    pillage: 1.0 / DAY,
    fallow: 5,
    post: 3,
    guarding: 1.0 * DAY,
    guard: 0.1 * DAY,
    inside: 0.2,
    zone: 2,
    salt: 1.0,
};

/// What one person of a carrier carries (`actors::Actor::carry`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Carry {
    /// The weight a person carries on a route, there and back once a turn, against a unit of food. A
    /// clan's walkers are what it brings over this, at least one (`actors::carriers`). A line of warriors
    /// carries `UNIT.labour × load` over each day of its cost (`units::line`).
    pub load: f64,
    /// Turns of its own eating a carrier's pack holds: what it walks and works on between stores (`pack_cap`,
    /// `cap_of`). A pack doesn't spoil. 0 where a carrier carries no pack.
    pub pack: f64,
}

/// Who carries, one `Carry` each (`actors::Carrier`).
pub struct CarryRules {
    /// A clan's people on a festival route. A load of 2 food is about nine days' eating (a person eats a
    /// food a turn of `CLAN.work`, 4.67 days), "some days' food" (`labour.md`). A clan brings at most
    /// `MARKET.carry` × people, 0.2 a person, so a load of 3 or more sends every clan with one walker and
    /// changes nothing, and 1 collapses the festivals (`walk.md`).
    pub clan: Carry,
    /// A warrior on a line, and in a pack: the pack's share of a warrior holds this many turns of its food
    /// (`UNIT.food` a turn). A line's food is labour over its cost at this load (`Carry.load`).
    pub warrior: Carry,
    /// A scout's pack: of the 42 routes between the retro maps' capitals (`relay/specs/337/errand.md`) 24
    /// builds 23, 36 builds 27 and 48 builds 27; a civ's store holds 30 turns of it (`CIV.store_cap`), so a
    /// pack is filled over several turns at the store. Its load is not read yet: a scout's walks carry its
    /// pack, not a line.
    pub scout: Carry,
}

pub const CARRY: CarryRules = CarryRules {
    clan: Carry { load: 2.0, pack: 0.0 },
    warrior: Carry { load: 3.0, pack: 5.0 },
    scout: Carry { load: 3.0, pack: 36.0 },
};

/// The kinds of fighter an army's people are (`war.md`): its stack counts its people by kind, and
/// each kind indexes `FIGHTERS`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Fighter {
    #[default]
    Warriors,
    Archers,
}

impl Fighter {
    pub const ALL: [Fighter; 2] = [Fighter::Warriors, Fighter::Archers];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn rules(self) -> &'static FighterRules {
        &FIGHTERS[self.index()]
    }
}

/// One kind of fighter's strength per person.
pub struct FighterRules {
    /// Its name, plural, for notes and the view.
    pub name: &'static str,
    /// Strength per person on a tile its side didn't hold at the turn's start.
    pub attack: f64,
    /// Strength per person on a tile its side held at the turn's start.
    pub defence: f64,
    pub note: &'static str,
}

/// The kinds, indexed by `Fighter`.
pub const FIGHTERS: [FighterRules; 2] = [
    FighterRules {
        name: "warriors",
        attack: 1.0,
        defence: 1.0,
        note: "The measure of the others: a person with a club or spear, as good taking as holding.",
    },
    FighterRules {
        name: "archers",
        attack: 0.8,
        defence: 1.4,
        note: "Bows hold ground from behind cover better than they take it; the fight is over one tile, so no range.",
    },
];

/// The fight's rule (`units/fight.rs`): Lanchester-linear losses on each contested tile.
pub struct FightRules {
    /// People a side loses a turn per unit of the others' strength there.
    pub rate: f64,
    /// A defending army's strength gained per turn it has stood still.
    pub entrench: f64,
    /// The most entrenchment adds.
    pub entrench_cap: f64,
    /// A side whose strength after the losses is under this times the others' retreats.
    pub retreat: f64,
    /// An army that went hungry last turn fights at this times its strength.
    pub hungry: f64,
    /// The fewest people a civ must have on a tile to be a side there.
    pub floor: f64,
    /// Hungry turns in a row, with its line cut by a rival, before an army surrenders (before hunger's take).
    pub surrender: u32,
    pub note: &'static str,
}

pub const FIGHT: FightRules = FightRules {
    rate: 0.2,
    entrench: 0.1,
    entrench_cap: 0.5,
    retreat: 0.5,
    hungry: 0.5,
    floor: 0.01,
    surrender: 3,
    note: "Tuned on small seed 2's stories (relay/specs/104/glaze.md): at rate 0.2 a fight moves Standing \
           and a party of 2-3 still lasts its contested turns; a beaten stack retreats a step and starves \
           or surrenders at surrender 3 (the stack's and surrender story's armies both do).",
};
