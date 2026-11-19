//! Herds: what a tile's grass carries, how a herd breeds, and how a clan comes to keep one (`herds.rs`).

/// The herd job (`herds.rs`, `jobs.rs`): a clan's animals, held in its store, breed toward what its
/// herding days and its grass carry, and the clan eats the increase. The glaze tunes these.
pub struct HerdRules {
    /// Animals a tile carries by terrain (`TERRAIN`'s order), before its fertility. Grass is plains and hills.
    pub graze: [f64; 8],
    /// Animals one herding day keeps a turn.
    pub tend: f64,
    /// The herd's logistic rate a turn, as `FOOD[GAME].regen` is game's.
    pub breed: f64,
    /// Animals a person a clan takes up herding with.
    pub start: f64,
    /// The most of its herd a clan short of food eats a turn, beyond the increase.
    pub lean: f64,
    /// The share of its days a clan's herding moves a turn.
    pub notch: f64,
    /// Food a person a turn by which the better share must beat the current one.
    pub margin: f64,
    /// The share from which a clan is a herder, with wide land.
    pub herder: f64,
    /// A herder's territory radius (a gatherer's is `CLAN.territory_radius`).
    pub radius: i32,
    /// The weight of a herder's walks inside its territory, against `CLAN.inside`: the camp moves with
    /// the herd, so it walks out and back less.
    pub walk: f64,
    /// Why these numbers: where they came from, and what tuning them did (relay/specs/342/).
    pub note: &'static str,
}

pub const HERD: HerdRules = HerdRules {
    graze: [0.0, 1.0, 6.0, 1.0, 4.0, 0.0, 1.0, 2.0],
    tend: 3.0,
    breed: 0.3,
    start: 0.5,
    lean: 0.25,
    notch: 0.1,
    margin: 0.01,
    herder: 0.2,
    radius: 3,
    walk: 0.5,
    note: "The clay's start numbers (relay/specs/342/probe.md, p4): plains carry six animals and hills four, \
           a herding day tends three, a herd breeds at game's rate and starts at half an animal a person. \
           A clan takes all the increase, and a notch reckons the herd's steady yield, breed × K ÷ 4. \
           Untuned: the glaze finds the offtake and the tend that make herds feed.",
};
