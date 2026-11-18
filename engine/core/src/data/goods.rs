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
    /// Units of the good a day of work at a source gives.
    pub a_day: f64,
    /// A clan gathers a source's good only while it holds less than this of it a person.
    pub hold: f64,
    /// What a clan reckons a unit of a source's good at, in food, when it weighs holding the tile.
    pub worth: f64,
}

/// Twelve salt pans and five obsidian hills on a small map: salt is common along the shore and obsidian
/// rare, so most civs trade for obsidian or go without. A twentieth of a holder's days gives a clan of eight
/// about 0.45 of the good a turn: the share a holder can give without starving more than the control (0.1 did).
pub const SOURCE: SourceRules = SourceRules { salt: 12, obsidian: 5, share: 0.05, a_day: 0.25, hold: 0.5, worth: 0.5 };

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
    /// Salt a civ's store (capital or settlement) keeps back from its keeping, for the packs to fill from
    /// (`Order::Fill`, `exchange.rs`). Only with `Config.exchange`.
    pub packs: f64,
    /// The share of a clan's salt a store's levy takes (against `CIV.levy` of its food), so the packs have
    /// some to fill from. Only with `Config.exchange`.
    pub levy: f64,
}

/// A unit of salt keeps 10 food and spares it half its spoiling: half a food saved for a unit of salt,
/// so salt is worth about half its measure in food to a full store and nothing to an empty one. A clan
/// at its surplus target loses 0.2 food a person a turn, and wants salt as it wants a food. With exchange
/// on, a store keeps 10 units of salt back from its keeping and its levy takes half a clan's salt: a pack's
/// salt keeps its own food within a march or two, so a pack carries more than it will sell, and the thin
/// salt a capital's clans bring (about 0.2 a turn) fills a reserve only if the levy takes a good share.
pub const KEEPING: KeepingRules =
    KeepingRules { keeps: 10.0, slows: 0.5, want_at: 0.2, civ_want: 1.0, packs: 10.0, levy: 0.5 };

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
/// stand in for numbers. A store holds blades for twenty.
pub const ARMS: ArmsRules = ArmsRules { each: 0.5, edge: 1.25, hold: 10.0 };

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

/// How a clan passes goods on to a second festival (`exchange::carry_route`).
pub struct PassRules {
    /// The share of a second walk's cost (in labour) that the wares' worth at the festival's last prices must
    /// cover for a clan to make it.
    pub cover: f64,
}

/// A clan's unsold pile is a few units of salt, a third of a food a unit, and a walk costs a person's days:
/// at the full cost no clan ever walks (probe: the worth is a hundredth of the cost), so a clan walks when
/// the wares pay a hundredth of it. At this the sweep carries a few hundred units a map, and health holds.
pub const PASS: PassRules = PassRules { cover: 0.01 };
