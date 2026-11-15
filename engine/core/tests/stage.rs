//! The stage: the map text, the layers, the placements, the orders and reads, the retro cache and
//! the spec form.

use engine::data::{size_radius, GRAIN};
use engine::hex::Hex;
use engine::land;
use engine::map::{self, index};
use engine::stage::{pregame_runs, ArmySpec, Stage, StageSpec, TileRef, RETRO};
use engine::{stats, Config, Order, World};

/// The text of a grid of `radius`: every tile a `.` but those in `marks`, drawn as `map::ascii`
/// draws a map.
fn grid_text(radius: i32, marks: &[(Hex, char)]) -> String {
    let mut out = String::new();
    for r in -radius..=radius {
        let (lo, hi) = ((-radius).max(-r - radius), radius.min(-r + radius));
        out += &" ".repeat((2 * lo + r + 2 * radius) as usize);
        let row: Vec<String> = (lo..=hi)
            .map(|q| {
                let h = Hex { q, r };
                marks.iter().find(|m| m.0 == h).map_or('.', |m| m.1).to_string()
            })
            .collect();
        out += &row.join(" ");
        out.push('\n');
    }
    out
}

/// Every glyph of the map text, on a map of radius 2 (19 tiles).
const GOOD: &str = "  ~ = ,\n . f h ^\n: % r z V\n S N P ~\n  . f ,\n";

fn at(radius: i32, q: i32, r: i32) -> usize {
    index(radius, Hex { q, r }).unwrap()
}

#[test]
fn round_trip_of_every_glyph() {
    let text = GOOD;
    let parsed = engine::map::parse(text).unwrap();
    assert_eq!(parsed.radius, 2);
    assert_eq!(map::ascii(&parsed.tiles, 2), text);
}

#[test]
fn retro_maps_round_trip_at_the_pick() {
    for (size, seeds) in RETRO {
        for &seed in seeds {
            let s = Stage::retro(size, seed).unwrap();
            let w = s.world();
            let radius = w.config.radius;
            let text = map::ascii(&w.tiles, radius);
            let parsed = engine::map::parse(&text).unwrap();
            assert_eq!(map::ascii(&parsed.tiles, radius), text, "{size} {seed}");
        }
    }
}

#[test]
fn a_bad_text_is_refused_with_its_place() {
    let good = GOOD;
    let unknown = good.replace(": % r", ": x r");
    let err = engine::map::parse(&unknown).err().unwrap();
    assert!(err.contains("row 3, column 2") && err.contains("'x'"), "{err}");
    let short = good.replace(" . f h ^", " . f h");
    let err = engine::map::parse(&short).err().unwrap();
    assert!(err.contains("row 2") && err.contains("column 4 is missing"), "{err}");
    let few: String = good.lines().take(4).map(|l| format!("{l}\n")).collect();
    let err = engine::map::parse(&few).err().unwrap();
    assert!(err.contains("5 rows") && err.contains("has 4"), "{err}");
}

#[test]
fn the_map_command_output_parses_as_printed() {
    let w = World::new(Config { seed: 2, radius: 3, ..Config::default() });
    let mut printed = map::ascii(&w.tiles, 3);
    printed += "~ sea  . plains  f forest  h hills\n";
    printed += "tiles 37, land 20\n";
    let parsed = engine::map::parse(&printed).unwrap();
    assert_eq!(parsed.radius, 3);
    assert_eq!(map::ascii(&parsed.tiles, 3), map::ascii(&w.tiles, 3));
}

#[test]
fn a_parsed_map_has_the_bases_rng_and_no_clans_or_markets() {
    let s = Stage::flat(4);
    let parsed = Stage::parse(&s.ascii()).unwrap();
    let w = parsed.world();
    assert_eq!(w.rng, 4151932303);
    assert_eq!(w.next_id, 4);
    assert!(w.clans.is_empty() && w.markets.is_empty());
    assert_eq!(map::ascii(&w.tiles, 4), s.ascii());
    assert_eq!(stats::digest(w), stats::digest(s.world()));
}

#[test]
fn marks_hidden_and_food_set_their_tiles() {
    let mut s = Stage::flat(3);
    s.marks(&grid_text(3, &[(Hex { q: 0, r: 0 }, 'A'), (Hex { q: 1, r: -1 }, 'B')])).unwrap();
    assert_eq!(s.mark('A'), at(3, 0, 0));
    assert_eq!(s.mark('B'), at(3, 1, -1));
    assert!(s.ascii().contains('A'));
    s.hidden(
        &grid_text(3, &[(Hex { q: 1, r: 0 }, 'h'), (Hex { q: -1, r: 0 }, '^')]),
        &[('h', (Some(4.0), None)), ('^', (Some(9.0), Some(2)))],
    )
    .unwrap();
    assert_eq!(s.world().tiles[at(3, 1, 0)].trail_cost, Some(4.0));
    assert_eq!(s.world().tiles[at(3, -1, 0)].span, Some(2));
    let t = s.mark('A');
    s.food(t, GRAIN, 5.0);
    let tile = &s.world().tiles[t];
    assert_eq!((tile.cap[GRAIN], tile.unworn[GRAIN], tile.stock[GRAIN]), (5.0, 5.0, 5.0));
}

#[test]
fn civs_lead_and_army_place_the_civs_and_orders_are_checked() {
    let mut s = Stage::flat(12);
    let (a, b) = (at(12, -5, 0), at(12, 5, 0));
    let ids = s.civs(&[a, b], 14.0);
    assert_eq!(ids.len(), 2);
    s.lead(ids[0]).unwrap();
    let camp = s.army(ids[0], at(12, -4, 0), 4.0);
    assert_eq!(s.camp(ids[0]).unwrap().id, camp);
    assert_eq!(s.people(camp), 4.0);
    let legal = at(12, -3, 0);
    s.order(ids[0], Order::Move { unit: camp, to: legal }).unwrap();
    assert_eq!(s.camp(ids[0]).unwrap().camp().unwrap().dest, Some(legal));
    let sea = at(12, 12, 0);
    let err = s.order(ids[0], Order::Move { unit: camp, to: sea }).err().unwrap();
    assert!(err.contains("only go over land"), "{err}");
    let before = s.world().turn;
    s.turn(3).unwrap();
    assert_eq!(s.world().turn, before + 3);
    let start = s.world().turn;
    assert_eq!(s.until(5, |w| w.turn >= start + 2).unwrap(), Some(2));
    assert_eq!(s.until(1, |_| false).unwrap(), None);
}

#[test]
fn the_retro_cache_runs_each_pregame_once() {
    let radius = size_radius("small").unwrap();
    let a = Stage::retro("small", 2).unwrap();
    let b = Stage::retro("small", 2).unwrap();
    assert_eq!(stats::digest(a.world()), stats::digest(b.world()));
    assert_eq!(pregame_runs(radius, 2), 1);
}

#[test]
fn a_spec_round_trips_and_builds_the_builders_world() {
    let text = Stage::flat(12).ascii();
    let parsed = Stage::parse(&text).unwrap();
    assert_eq!(stats::digest(parsed.world()), stats::digest(Stage::flat(12).world()));

    let spec = StageSpec {
        map: Some(text.clone()),
        retro: None,
        marks: Some(grid_text(12, &[(Hex { q: -5, r: 0 }, 'A'), (Hex { q: 5, r: 0 }, 'B')])),
        hidden: Vec::new(),
        food: Vec::new(),
        civs: vec![TileRef::Mark { mark: 'A' }, TileRef::Mark { mark: 'B' }],
        people: Some(14.0),
        lead: Some(0),
        armies: vec![ArmySpec { civ: 0, at: TileRef::Tile { tile: at(12, -4, 0) }, people: 4.0 }],
        clans: Vec::new(),
        limit: None,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let back: StageSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(serde_json::to_string(&back).unwrap(), json);
    assert!(json.contains("\"people\":14.0") && json.contains("\"civs\""));

    let from = Stage::from_spec(&spec).unwrap();

    let mut built = Stage::parse(&text).unwrap();
    built.marks(spec.marks.as_deref().unwrap()).unwrap();
    let ids = built.civs(&[built.mark('A'), built.mark('B')], 14.0);
    built.lead(ids[0]).unwrap();
    built.army(ids[0], at(12, -4, 0), 4.0);
    assert_eq!(stats::digest(from.world()), stats::digest(built.world()));
}

#[test]
fn plain_makes_the_land_bare_and_clears_what_the_led_civ_has_seen() {
    let mut s = Stage::retro("small", 2).unwrap();
    let first = *s.world().civs.keys().next().unwrap();
    s.lead(first).unwrap();
    s.plain();
    let w = s.world();
    assert!(w.tiles.iter().all(|t| t.terrain == engine::data::SEA || t.terrain == engine::data::PLAINS));
    // Plains carry the flat trail cost (the sea none), so `plain` leaves nothing hidden but that.
    let flat = |t: &engine::map::Tile| {
        if t.terrain == engine::data::SEA {
            None
        } else {
            land::flat_cost(engine::data::PLAINS)
        }
    };
    assert!(w.tiles.iter().all(|t| !t.trail && t.river == 0 && t.trail_cost == flat(t) && t.span.is_none()));
    assert!(w.civs[&first].revealed.is_empty());
}

#[test]
fn paint_sets_a_tiles_terrain_and_hidden_cost() {
    let mut s = Stage::flat(3);
    let i = at(3, 1, 0);
    s.paint(i, engine::data::FOREST, Some(2.0), Some(3));
    let t = &s.world().tiles[i];
    assert_eq!((t.terrain, t.trail_cost, t.span), (engine::data::FOREST, Some(2.0), Some(3)));
}
