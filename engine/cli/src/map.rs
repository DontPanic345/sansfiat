use crate::{size, USAGE};
use engine::data::{DESERT, FOREST, HILLS, MARSH, MOUNTAIN, SEA};
use engine::map::{self, Country};
use engine::{Config, World, TERRAIN};

/// Prints the map as text, a legend and counts.
pub(crate) fn map(args: &[String]) -> Result<(), String> {
    let mut config = Config::default();
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let value = it.next().ok_or(format!("{flag} needs a value\n{USAGE}"))?;
        let bad = |_| format!("{flag}: not a number: {value}");
        match flag.as_str() {
            "--seed" => config.seed = value.parse().map_err(bad)?,
            "--radius" => config.radius = value.parse().map_err(bad)?,
            "--size" => config.radius = size(value)?,
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let w = World::new(config);
    print!("{}", map::ascii(&w.tiles, config.radius));
    let legend = [('~', "sea"), ('=', "lake"), (',', "coast"), ('.', "plains"), ('f', "forest"), ('h', "hills")]
        .into_iter()
        .chain([('^', "mountain"), (':', "desert"), ('%', "marsh"), ('r', "river"), ('z', "floodplain")])
        .chain([('V', "valley"), ('P', "pass"), ('S', "sealed valley"), ('N', "neck")])
        .map(|(c, n)| format!("{c} {n}"))
        .collect::<Vec<_>>()
        .join("  ");
    println!("{legend}");
    let count = |terrain: usize| w.tiles.iter().filter(|t| t.terrain == terrain).count();
    let land = w.tiles.iter().filter(|t| TERRAIN[t.terrain].land).count();
    let lakes = w.tiles.iter().filter(|t| t.lake).count();
    let masses = map::landmasses(&w.tiles, config.radius).len();
    let rivers = w.tiles.iter().filter(|t| t.river > 0).count();
    let mouths = w.tiles.iter().filter(|t| t.flow.is_some_and(|j| w.tiles[j].terrain == SEA)).count();
    let flood = w.tiles.iter().filter(|t| t.flood && t.terrain != MARSH).count();
    let deltas = w.tiles.iter().filter(|t| t.flood && t.terrain == MARSH).count();
    let longest = w.tiles.iter().map(|t| t.river).max().unwrap_or(0);
    println!(
        "seed {} radius {}: {} tiles, {land} land, {} water ({lakes} lake), {} mountains, {} hills, {} deserts, {} forests, {masses} landmasses, \
         {rivers} river tiles ({mouths} mouths, longest {longest}), {flood} floodplain, {deltas} delta",
        config.seed,
        config.radius,
        w.tiles.len(),
        count(SEA),
        count(MOUNTAIN),
        count(HILLS),
        count(DESERT),
        count(FOREST)
    );
    let zones = |zone: Country| map::zone_groups(&w.tiles, config.radius, zone).len();
    let passes = w.tiles.iter().filter(|t| t.pass).count();
    let sites = map::open_sites(&w.tiles, config.radius);
    println!(
        "closed country: {} valleys ({passes} passes), {} sealed valleys, {} necks; open sites at {sites:?}",
        zones(Country::Valley),
        zones(Country::Sealed),
        zones(Country::Neck)
    );
    Ok(())
}
