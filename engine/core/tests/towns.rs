//! Towns, build-1: many clans on one home tile. A home is not a claim, a clan gathers only the tiles
//! it claims, a tile's field ground is shared among the clans that sow it, and `homes` indexes the
//! clans by home. Staged on bare plains with a civ at the centre, so fields are open.

use engine::actors::{walk_cost, Route};
use engine::clans::{self, harvest, new_clan, reap, walks, Reaped};
use engine::data::{Config, CLAN, FIELD, GRAIN, MAP, TENDING};
use engine::events::EventKind;
use engine::fields;
use engine::hex::Hex;
use engine::homes;
use engine::judge::Watch;
use engine::land::neighbours;
use engine::land::{passable, path};
use engine::map::index;
use engine::markets::form_and_fade;
use engine::markets::{by_market, festival_costs, go, in_reach, raise, walkers};
use engine::reach::Scratch;
use engine::stage::Stage;
use engine::towns::towns;
use engine::World;

const RADIUS: i32 = 4;

/// Bare plains with a civ founded at the centre, and no clans: the test places its own.
fn staged() -> (World, usize) {
    let mut stage = Stage::flat(RADIUS);
    let centre = index(RADIUS, Hex { q: 0, r: 0 }).unwrap();
    stage.civs(&[centre], 5.0);
    let mut w = stage.into_world();
    w.clans.clear();
    for t in &mut w.tiles {
        t.claim = None;
    }
    (w, centre)
}

/// Makes tile `t` a river tile whose ground is `FIELD.river` and whose grain yields: tamed.
fn river(w: &mut World, t: usize) {
    w.tiles[t].river = MAP.big_river;
    w.tiles[t].tame = TENDING.tame_turns;
    // Little grain a day, so the food budget does not cut the days the clans ask.
    w.tiles[t].fertility = 0.1;
}

/// Puts a clan of `people` on `home`, holding `territory` (its home included, unclaimed tiles
/// claimed, held ones left to their holders). Returns its id.
fn clan(w: &mut World, home: usize, territory: &[usize], people: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let mut territory = territory.to_vec();
    territory.sort_unstable();
    let mut c = new_clan(id, home, people);
    c.territory = territory.clone();
    homes::claim_land(w, id, &territory);
    w.clans.insert(id, c);
    id
}

/// The field days clan `id` has on tile `t` this turn, from `Clan.sown`.
fn sown_on(w: &World, id: u32, t: usize) -> f64 {
    w.clans[&id].sown.iter().find(|x| x.0 == t).map_or(0.0, |x| x.1)
}

/// What clan `id` would reap this turn from its territory with no fields: gathering only on its claims.
fn gathered(w: &World, id: u32) -> Reaped {
    let c = &w.clans[&id];
    let (_, yearly) = walks(w, c.home, &c.territory);
    let gathers: Vec<bool> = c.territory.iter().map(|&t| homes::gathers(w, id, t)).collect();
    reap(w, id, c.people, &c.store, &c.routes, &c.territory, &yearly, 0.0, &gathers)
}

/// The days clan `id` would sow on `tile` this turn with its own split, before the shared step.
fn asked(w: &World, id: u32, tile: usize) -> f64 {
    let c = &w.clans[&id];
    let (_, yearly) = walks(w, c.home, &c.territory);
    let gathers = vec![true; c.territory.len()];
    let reaped = reap(w, id, c.people, &c.store, &c.routes, &c.territory, &yearly, c.fields, &gathers);
    let k = c.territory.iter().position(|&t| t == tile).unwrap();
    reaped.days[k]
}

#[test]
fn the_config_turns_towns_on_by_default() {
    assert!(Config::default().towns);
}

#[test]
fn each_clan_gathers_only_its_own_claim() {
    let (mut w, s) = staged();
    let ring: Vec<usize> = neighbours(&w, s).collect();
    let (a1, a2, b1, b2, c1, c2) = (ring[0], ring[1], ring[2], ring[3], ring[4], ring[5]);
    // A claims the shared home and its two tiles; B and C live on it too and claim their own.
    let a = clan(&mut w, s, &[s, a1, a2], 10.0);
    let b = clan(&mut w, s, &[s, b1, b2], 10.0);
    let c = clan(&mut w, s, &[s, c1, c2], 10.0);
    assert_eq!(w.tiles[s].claim, Some(a));
    for t in [s, a1, a2, b1, b2, c1, c2] {
        w.tiles[t].stock[GRAIN] = 10.0;
    }
    let (a_takes, b_takes, c_takes) = (gathered(&w, a).takes, gathered(&w, b).takes, gathered(&w, c).takes);
    let at = |w: &World, id: u32, t: usize| w.clans[&id].territory.iter().position(|&x| x == t).unwrap();
    let a_on_home = a_takes[at(&w, a, s)][GRAIN];
    assert!(a_on_home > 0.0, "A gathers its home");
    assert_eq!(b_takes[at(&w, b, s)][GRAIN], 0.0, "B does not gather a home it does not claim");

    harvest(&mut w);

    assert!((w.tiles[s].take[GRAIN] - a_on_home).abs() < 1e-9, "only A's take falls on the shared home");
    assert!((w.tiles[s].stock[GRAIN] - (10.0 - a_on_home)).abs() < 1e-9);
    let b_food: f64 = b_takes.iter().map(|t| t[GRAIN]).sum();
    let c_food: f64 = c_takes.iter().map(|t| t[GRAIN]).sum();
    assert!(b_food > 0.0 && c_food > 0.0);
    assert!((w.clans[&b].harvest[GRAIN] - b_food).abs() < 1e-9, "B takes from its own claimed tiles");
    assert!((w.clans[&c].harvest[GRAIN] - c_food).abs() < 1e-9, "C takes from its own claimed tiles");
}

#[test]
fn clans_sowing_one_tile_share_its_ground() {
    let (mut w, s) = staged();
    river(&mut w, s);
    let ground = fields::ground(&w, s);
    // Three clans live on the river tile and farm it; A holds it, the others claim nothing.
    let ids: Vec<u32> = [0.3, 0.4, 0.5].iter().map(|&f| clan(&mut w, s, &[s], f * ground / CLAN.work)).collect();
    for &id in &ids {
        w.clans.get_mut(&id).unwrap().fields = 1.0;
    }
    let asks: Vec<f64> = ids.iter().map(|&id| asked(&w, id, s)).collect();
    let total: f64 = asks.iter().sum();
    assert!(total > ground, "the asks overflow the tile, so it is shared");

    harvest(&mut w);

    let sown: Vec<f64> = ids.iter().map(|&id| sown_on(&w, id, s)).collect();
    assert!(w.tiles[s].field <= ground + 1e-9, "the tile's total never exceeds its ground");
    assert!((w.tiles[s].field - ground).abs() < 1e-9, "the overflowing asks fill it");
    assert!((w.tiles[s].field - sown.iter().sum::<f64>()).abs() < 1e-9, "the tile holds the clans' total");
    for (k, &a) in asks.iter().enumerate() {
        assert!((sown[k] * total - a * ground).abs() < 1e-9 * ground, "each clan's share follows its asks");
    }
    for &id in &ids {
        let others = w.tiles[s].field - sown_on(&w, id, s);
        assert!((fields::room(&w, id, s) - (ground - others)).abs() < 1e-9, "the room is the ground less the rest");
    }
    // Next turn each clan plans on its room, and the tile still stays within its ground.
    for &id in &ids {
        assert!(asked(&w, id, s) <= fields::room(&w, id, s) + 1e-9);
    }
    harvest(&mut w);
    assert!(w.tiles[s].field <= ground + 1e-9);
}

#[test]
fn a_clan_alone_sows_its_ground_as_before() {
    let (mut w, s) = staged();
    river(&mut w, s);
    let ground = fields::ground(&w, s);
    let id = clan(&mut w, s, &[s], 0.3 * ground / CLAN.work);
    w.clans.get_mut(&id).unwrap().fields = 1.0;
    let ask = asked(&w, id, s);
    assert!(ask > 0.0 && ask < ground, "a single sower's ask is under its ground");

    harvest(&mut w);

    assert!((sown_on(&w, id, s) - ask).abs() < 1e-12, "a lone sower sows what it asked");
    assert!((w.tiles[s].field - ask).abs() < 1e-12, "and the tile holds exactly that");
    assert!((fields::room(&w, id, s) - ground).abs() < 1e-12, "its room is its ground");
    let plots = fields::plots(&w, &w.clans[&id]);
    assert_eq!(plots, vec![(s, ask, ask * fields::yield_a_day(&w, s))]);
}

#[test]
fn free_leaves_a_home_another_clan_holds() {
    let (mut w, s) = staged();
    let ring: Vec<usize> = neighbours(&w, s).collect();
    let a = clan(&mut w, s, &[s, ring[0]], 6.0);
    let b = clan(&mut w, s, &[s, ring[1]], 6.0);
    assert_eq!(w.tiles[s].claim, Some(a));
    // B lives on A's tile and starves to nothing: it frees its own claim, not A's.
    w.clans.get_mut(&b).unwrap().people = 0.5;
    clans::eat(&mut w);
    assert!(!w.clans.contains_key(&b));
    assert_eq!(w.tiles[s].claim, Some(a));
    assert_eq!(w.tiles[ring[1]].claim, None);
}

#[test]
fn claim_land_never_takes_a_held_tile() {
    let (mut w, s) = staged();
    let ring: Vec<usize> = neighbours(&w, s).collect();
    let a = clan(&mut w, s, &[s, ring[0]], 6.0);
    homes::claim_land(&mut w, a + 100, &[s, ring[2]]);
    assert_eq!(w.tiles[s].claim, Some(a));
    assert_eq!(w.tiles[ring[2]].claim, Some(a + 100));
}

#[test]
fn a_clan_claims_the_shared_home_once_its_claimant_dies() {
    let (mut w, s) = staged();
    let ring: Vec<usize> = neighbours(&w, s).collect();
    let a = clan(&mut w, s, &[s, ring[0]], 6.0);
    let b = clan(&mut w, s, &[s, ring[1]], 6.0);
    w.clans.get_mut(&a).unwrap().people = 0.5;
    clans::eat(&mut w);
    assert!(!w.clans.contains_key(&a));
    assert_eq!(w.tiles[s].claim, None, "the home is unheld once its claimant is gone");
    clans::adjust(&mut w);
    assert_eq!(w.tiles[s].claim, Some(b), "B claims the home it lives on at its next move");
}

#[test]
fn homes_lists_every_clan_on_a_shared_home_and_reach_counts_them() {
    let (mut w, s) = staged();
    let ring: Vec<usize> = neighbours(&w, s).collect();
    let a = clan(&mut w, s, &[s, ring[0]], 6.0);
    let b = clan(&mut w, s, &[s, ring[1]], 4.0);
    let c = clan(&mut w, s, &[s, ring[2]], 5.0);
    let index = homes::homes(&w);
    assert_eq!(index.len(), 1);
    assert_eq!(index[&s], vec![a, b, c]);
    let (ids, people) = in_reach(&w, s);
    assert_eq!(ids, vec![a, b, c]);
    assert!((people - 15.0).abs() < 1e-9);
}

#[test]
fn plots_give_each_clan_its_own_days_on_a_shared_tile() {
    let (mut w, s) = staged();
    river(&mut w, s);
    let ground = fields::ground(&w, s);
    let ids: Vec<u32> = [0.4, 0.5].iter().map(|&f| clan(&mut w, s, &[s], f * ground / CLAN.work)).collect();
    for &id in &ids {
        w.clans.get_mut(&id).unwrap().fields = 1.0;
    }
    harvest(&mut w);
    for &id in &ids {
        let days = sown_on(&w, id, s);
        let plots = fields::plots(&w, &w.clans[&id]);
        assert_eq!(plots.len(), 1);
        assert!((plots[0].1 - days).abs() < 1e-12, "a clan's plot is its own days");
        assert!(plots[0].1 < w.tiles[s].field, "not the tile's total");
    }
}

#[test]
fn clans_on_one_home_read_one_festival_search_and_walk_the_land_path() {
    let (mut w, _) = staged();
    let home = index(RADIUS, Hex { q: -3, r: 0 }).unwrap();
    let fest = index(RADIUS, Hex { q: -2, r: 0 }).unwrap();
    let mid = raise(&mut w, fest);
    let a = clan(&mut w, home, &[home], 5.0);
    let b = clan(&mut w, home, &[home], 5.0);
    let mut s = Scratch::new(&w);
    let festivals = festival_costs(&w, &mut s, home);
    let want = path(&w, home, fest, |i| passable(&w, i)).unwrap();
    let f = festivals.iter().find(|f| f.market == mid).expect("the festival is in reach of the home");
    assert_eq!((f.cost, &f.path), (want.1, &want.0), "the search's route is the land path's");
    go(&mut w);
    for id in [a, b] {
        assert_eq!(w.clans[&id].market, Some(mid), "clan {id} goes to the festival");
        let r = &w.clans[&id].routes[0];
        assert_eq!((r.to, &r.path), (fest, &want.0), "clan {id} walks the route land::path gives");
        assert_eq!(r.cost, walk_cost(want.1, 1.0) * walkers(&w.clans[&id]));
    }
}

#[test]
fn by_market_lists_each_markets_clans_in_id_order() {
    let (mut w, _) = staged();
    let m1 = raise(&mut w, index(RADIUS, Hex { q: 2, r: 0 }).unwrap());
    let m2 = raise(&mut w, index(RADIUS, Hex { q: -2, r: 0 }).unwrap());
    let ids: Vec<u32> =
        (0..4).map(|k| clan(&mut w, index(RADIUS, Hex { q: 0, r: k - 2 }).unwrap(), &[], 5.0)).collect();
    for (id, mid) in [(ids[3], m1), (ids[0], m2), (ids[2], m1), (ids[1], m1)] {
        w.clans.get_mut(&id).unwrap().market = Some(mid);
    }
    let index_by = by_market(&w);
    assert_eq!(index_by.get(&m1), Some(&vec![ids[1], ids[2], ids[3]]));
    assert_eq!(index_by.get(&m2), Some(&vec![ids[0]]));
    let none = raise(&mut w, index(RADIUS, Hex { q: 0, r: 3 }).unwrap());
    assert!(!by_market(&w).contains_key(&none), "a market with no clans is absent");
}

/// A festival raised two steps east of the centre, where no civ's market stands.
fn festival(w: &mut World) -> (usize, u32) {
    let tile = index(RADIUS, Hex { q: 2, r: 0 }).unwrap();
    (tile, raise(w, tile))
}

#[test]
fn a_festival_with_three_homes_on_its_tile_reads_as_a_town() {
    let (mut w, _) = staged();
    let (tile, m) = festival(&mut w);
    clan(&mut w, tile, &[tile], 4.0);
    clan(&mut w, tile, &[tile], 4.0);
    assert!(towns(&w).is_empty(), "two clans side by side are neighbours, not a town");
    let c = clan(&mut w, tile, &[tile], 4.0);
    let found = towns(&w);
    assert_eq!(found.len(), 1);
    assert_eq!((found[0].tile, found[0].market), (tile, m));
    assert_eq!(found[0].clans.len(), 3);
    assert!(found[0].clans.contains(&c));
}

#[test]
fn a_town_forms_once_as_the_third_clan_arrives_and_its_note_comes_when_one_leaves() {
    let (mut w, _) = staged();
    let (tile, m) = festival(&mut w);
    clan(&mut w, tile, &[tile], 4.0);
    clan(&mut w, tile, &[tile], 4.0);
    form_and_fade(&mut w);
    assert!(!w.markets[&m].town);
    let third = clan(&mut w, tile, &[tile], 4.0);
    form_and_fade(&mut w);
    assert!(w.markets[&m].town);
    form_and_fade(&mut w);
    let formed: Vec<String> = w.events.iter().filter(|e| e.kind == EventKind::TownFormed).map(|e| e.text(&w)).collect();
    assert_eq!(formed.len(), 1, "a town forms once: {formed:?}");
    assert_eq!(formed[0], "A town forms at (2, 0): 3 clans live by the festival");

    w.clans.remove(&third);
    form_and_fade(&mut w);
    assert!(!w.markets[&m].town);
    assert!(w.notes[&m].iter().any(|n| n.text == "is no longer a town: 2 clans live here"));
}

#[test]
fn market_town_serialises_only_when_true() {
    let (mut w, _) = staged();
    let (_, m) = festival(&mut w);
    let json = serde_json::to_value(&w.markets[&m]).unwrap();
    assert!(json.get("town").is_none(), "no town, no key: the save and the digest see nothing");
    w.markets.get_mut(&m).unwrap().town = true;
    let json = serde_json::to_value(&w.markets[&m]).unwrap();
    assert_eq!(json["town"], true);
}

#[test]
fn the_judge_counts_town_clans_their_walk_and_the_farmers_outside_towns() {
    let (mut w, _) = staged();
    let (tile, _) = festival(&mut w);
    let far = index(RADIUS, Hex { q: -3, r: 0 }).unwrap();
    let a = clan(&mut w, tile, &[tile], 4.0);
    clan(&mut w, tile, &[tile], 4.0);
    clan(&mut w, tile, &[tile], 4.0);
    let farmer = clan(&mut w, far, &[far], 4.0);
    w.clans.get_mut(&a).unwrap().fields = FIELD.farmer;
    w.clans.get_mut(&farmer).unwrap().fields = FIELD.farmer;
    w.clans.get_mut(&farmer).unwrap().routes = vec![Route { to: far, path: vec![far], cost: 3.0 }];
    let mut watch = Watch::new(&w);
    for _ in 0..200 {
        watch.turn(&w);
    }
    let s = watch.scores(&w, &[]).towns;
    let t = &s.total;
    assert_eq!((t.towns_100, t.towns_200), (1, 1));
    assert_eq!((t.town_clans_200, t.town_farmers_200), (3, 1));
    assert_eq!(t.towns_fields_200, 0, "no clan sows: the town works no fields");
    assert_eq!(s.most_homes_200, 3);
    assert_eq!(s.first_town, None, "no event, no first town");
    // Clan-turns after +100: 100 turns, three town clans and one farmer outside.
    assert_eq!(t.town.clan_turns, 300);
    assert_eq!(t.farmers_outside.clan_turns, 100);
    assert_eq!(t.all.clan_turns, 400);
    assert_eq!(t.town.walk, 0.0, "clans at the market walk nothing");
    let walk = 100.0 * 3.0 / (4.0 * CLAN.work);
    assert!((t.farmers_outside.walk - walk).abs() < 1e-9, "the farmer outside walks its route each turn");
    assert_eq!(t.town.traded, 0.0, "no clearing has traded");
}
