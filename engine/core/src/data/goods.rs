//! The goods table and the rules that read it: what a good is, its source, its keeping and its arms.

/// What a good is for (`goods.md`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Use {
    /// Eaten.
    Food,
    /// Slows the spoiling of the store it lies in, and is used up doing it (`KEEPING`).
    Keeps,
    /// Raises the strength of the warriors raised from the store it lies in, and is used up at the
    /// raising (`ARMS`).
    Arms,
}

/// A row of the goods table. Eating, spoiling, `market::total` and the market's worth read it. The
/// table has no row for a good without a use.
pub struct Good {
    pub name: &'static str,
    /// What a unit of it weighs against a unit of food: a carrier's load (`Actor::carry`) and a clan's
    /// festival basket (`MARKET.carry`) are counted in weight.
    pub weight: f64,
    /// The share of it a store loses a turn.
    pub spoil: f64,
    /// Whether people eat it: only eaten goods feed, fill a pack and count in a store's food.
    pub eaten: bool,
    pub uses: Use,
}

const fn food(name: &'static str) -> Good {
    Good { name, weight: 1.0, spoil: 0.1, eaten: true, uses: Use::Food }
}

/// Salt and obsidian are light for what they do, and keep: a clan's carriers bring them to a festival
/// at a fifth of food's weight, and a store loses none of either but by using it.
pub const GOOD: [Good; 7] = [
    food("game"),
    food("grain"),
    food("fruit"),
    food("fish"),
    food("shellfish"),
    Good { name: "salt", weight: 0.2, spoil: 0.0, eaten: false, uses: Use::Keeps },
    Good { name: "obsidian", weight: 0.2, spoil: 0.0, eaten: false, uses: Use::Arms },
];
pub const N: usize = GOOD.len();
/// The goods' names, in the table's order.
pub const GOODS: [&str; N] = {
    let mut out = [""; N];
    let mut g = 0;
    while g < N {
        out[g] = GOOD[g].name;
        g += 1;
    }
    out
};
/// How many goods are eaten: the first rows of `GOOD`, as the const check below holds.
pub const FOOD_N: usize = 5;
const _: () = {
    let mut g = 0;
    while g < N {
        assert!(GOOD[g].eaten == (g < FOOD_N), "the eaten goods come first, and number FOOD_N");
        g += 1;
    }
};
/// The foods' names: the eaten goods, which come first in the table.
pub const FOODS: [&str; FOOD_N] = {
    let mut out = [""; FOOD_N];
    let mut g = 0;
    while g < FOOD_N {
        out[g] = GOOD[g].name;
        g += 1;
    }
    out
};
/// The rate a food spoils at a turn (`actors::worths` reads it): every food keeps alike, `GOOD[GAME]`'s.
pub const FOOD_SPOIL: f64 = GOOD[GAME].spoil;

pub const GAME: usize = 0;
pub const GRAIN: usize = 1;
pub const FRUIT: usize = 2;
pub const FISH: usize = 3;
pub const SHELLFISH: usize = 4;
pub const SALT: usize = 5;
pub const OBSIDIAN: usize = 6;

/// Where the goods that aren't eaten come from, and how a clan gathers them (`map.rs`, `clans::reap`).
pub struct SourceRules {
    /// Salt sources per `SMALL_TILES` tiles, on coast and marsh tiles.
    pub salt: usize,
    /// Obsidian sources per `SMALL_TILES` tiles, on hills.
    pub obsidian: usize,
    /// The share of its days a clan gives the gathering of a source it holds: a job in its split.
    pub share: f64,
    /// Units of obsidian a day of work at its source gives.
    pub a_day: f64,
    /// Units of salt a day of work at a pan gives.
    pub salt_a_day: f64,
    /// A clan gathers a source's good only while it holds less than this of it a person.
    pub hold: f64,
    /// What a clan reckons a unit of a source's good at, in food, when it weighs holding the tile.
    pub worth: f64,
}

/// Twelve salt pans and five obsidian hills on a small map: salt is common along the shore and obsidian
/// rare, so most civs trade for obsidian or go without. A twentieth of a holder's days gives a clan of eight
/// about 0.45 of obsidian a turn: the share a holder can give without starving more than the control (0.1 did).
/// A pan gives three times that: enough salt reaches a civ's store to pay an army abroad.
pub const SOURCE: SourceRules =
    SourceRules { salt: 12, obsidian: 5, share: 0.05, a_day: 0.25, salt_a_day: 0.75, hold: 0.5, worth: 0.5 };

/// How salt keeps a store (`actors::spoil`).
pub struct KeepingRules {
    /// Food a unit of salt keeps for a turn; the salt is used up keeping it.
    pub keeps: f64,
    /// The share of its spoiling kept food is spared.
    pub slows: f64,
    /// The spoiling a person that makes a clan want salt as much as a food (`actors::worths`).
    pub want_at: f64,
    /// What a civ's store with none wants salt and obsidian at, against a food's 1; it falls to 0 as the
    /// store fills to what it holds (`actors::civ_room`).
    pub civ_want: f64,
    /// The turns of keeping a full store that a civ's store holds salt for (`actors::civ_holds`).
    pub civ_turns: f64,
}

/// A unit of salt keeps 10 food and spares it half its spoiling: half a food saved for a unit of salt,
/// so salt is worth about half its measure in food to a full store and nothing to an empty one. A clan
/// at its surplus target loses 0.2 food a person a turn, and wants salt as it wants a food.
/// A civ's store holds four turns of it: a store of 60 holds 24 salt, enough to pay its armies abroad.
pub const KEEPING: KeepingRules = KeepingRules { keeps: 10.0, slows: 0.5, want_at: 0.2, civ_want: 1.0, civ_turns: 4.0 };

/// How obsidian arms warriors (`units/muster.rs`, `units/fight.rs`).
pub struct ArmsRules {
    /// Obsidian a warrior takes from the store he is raised from, used up at the raising.
    pub each: f64,
    /// What an armed warrior's strength is multiplied by, attacking and defending.
    pub edge: f64,
    /// The most obsidian a civ's store takes: past it the levy leaves the obsidian with the clan and
    /// the store wants none at its market.
    pub hold: f64,
}

/// A warrior with obsidian blades is worth a quarter more: enough to turn an even fight, not enough to
/// stand in for numbers. A store holds blades for eighty: a civ keeps what its levy brings.
pub const ARMS: ArmsRules = ArmsRules { each: 0.5, edge: 1.25, hold: 40.0 };

/// How obsidian blades help a clan gather (`clans::reap`, `actors::worths`).
pub struct ToolRules {
    /// Obsidian a person's tools take: a clan with this much a person gathers at the full `gain`.
    pub each: f64,
    /// What a fully tooled clan's gathering labour is multiplied by.
    pub gain: f64,
    /// The share of a clan's tools worn out a turn.
    pub wear: f64,
    /// What a clan with no tools wants obsidian at, against a food's 1; it falls to 0 as its tools fill.
    pub want: f64,
}

/// A tenth of a unit tools a person, who gathers a fifth more for it, and a tenth of the tools wear out a
/// turn, so a clan keeps wanting them. A clan wants tools as it wants a food: a hill's obsidian finds buyers
/// at every festival, and buys food there.
pub const TOOLS: ToolRules = ToolRules { each: 0.1, gain: 1.2, wear: 0.1, want: 1.0 };

/// How a store remembers where its goods came from (`lots.rs`).
pub struct LotsRules {
    /// The most lots a store keeps of one good: past it the smallest are dropped and the rest renormalised.
    pub most: usize,
    /// The most markets a lot's chain keeps: past it the oldest market goes, never the source.
    pub chain: usize,
}

/// Four lots and six markets show a good's main routes on a page without a store's list growing: the shares
/// of a mixed store spread thin fast, and a chain longer than six markets is past what a clan's festival
/// and a camp's march join up in a few turns.
pub const LOTS: LotsRules = LotsRules { most: 4, chain: 6 };
