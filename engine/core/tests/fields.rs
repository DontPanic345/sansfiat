//! The field job (`fields.rs`): ground, yield, the split's notch, the starving fallback, the farmer's
//! home, the harvest's separation of field grain, tame under fields, and fields off or before the pick.

use engine::actors::{split, Route};
use engine::clans::{adjust, harvest, reap};
use engine::data::*;
use engine::fields::{self, ground, home_toward, notch, plots, sow, yield_a_day};
use engine::hex::Hex;
use engine::land::{domestic, tend, wear};
use engine::map::index;
use engine::stage::Stage;
use engine::{Basket, Config, EventKind, World};

const EPS: f64 = 1e-9;
const RADIUS: i32 = 8;

fn at(q: i32, r: i32) -> usize {
    index(RADIUS, Hex { q, r }).unwrap()
}

/// The civ's capital, on the flat map.
const CAP: (i32, i32) = (4, -4);

/// A flat map of radius `RADIUS` with a civ founded at `CAP`, and nothing else.
fn founded() -> Stage {
    let mut s = Stage::flat(RADIUS);
    s.civs(&[at(CAP.0, CAP.1)], 6.0);
    s
}

/// Makes tile `i` a floodplain that has been held a hearth's worth of turns: fertility 1, wild grain 10,
/// tame at a hearth, and no grain in stock.
fn farm_tile(w: &mut World, i: usize) {
    let t = &mut w.tiles[i];
    t.flood = true;
    t.fertility = 1.0;
    t.wild_grain = 10.0;
    t.tame = TENDING.tame_turns;
    t.unworn[GRAIN] = 10.0 * domestic(TENDING.tame_turns);
    t.cap[GRAIN] = t.unworn[GRAIN];
    t.stock = [0.0; N];
}

/// Puts a clan of `people` on `tiles` (its home first), holding them, and returns its id.
fn hold(w: &mut World, tiles: &[usize], people: f64, fields: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let mut c = engine::clans::new_clan(id, tiles[0], people);
    let mut territory = tiles.to_vec();
    territory.sort_unstable();
    c.territory = territory;
    c.fields = fields;
    for &t in tiles {
        w.tiles[t].claim = Some(id);
    }
    w.clans.insert(id, c);
    id
}

/// A clan of ten people on a floodplain, with no stock anywhere: fields are its only food.
fn floodplain_clan() -> (Stage, u32, usize) {
    let mut s = founded();
    let f = at(-2, 0);
    let w = s.world_mut();
    farm_tile(w, f);
    let id = hold(w, &[f], 10.0, 0.0);
    (s, id, f)
}

/// The old `reap`, as it was before the field job: gathering alone, one job over the territory.
fn old_reap(w: &World, people: f64, store: &Basket, routes: &[Route], tiles: &[usize], yearly: &[f64]) -> Vec<Basket> {
    let work = split(people, people * CLAN.work, routes, &[1.0], &[yearly.to_vec()]).swap_remove(0);
    let mut takes: Vec<Basket> = Vec::new();
    let mut total = 0.0;
    for (&t, &labour) in tiles.iter().zip(&work) {
        let stock = &w.tiles[t].stock;
        let held: f64 = stock.iter().sum();
        if held <= 0.0 || labour <= 0.0 || w.tiles[t].fallow > 0 {
            takes.push([0.0; N]);
            continue;
        }
        let amount = (labour / CLAN.harvest_labour).min(CLAN.take_rate * held);
        total += amount;
        takes.push(stock.map(|s| amount * s / held));
    }
    let budget = (CLAN.surplus_target * people - store.iter().sum::<f64>()).max(0.0);
    if total > budget {
        let scale = budget / total;
        for t in &mut takes {
            *t = t.map(|x| x * scale);
        }
    }
    takes
}

#[test]
fn ground_is_days_from_floodplains_and_rivers() {
    let mut s = founded();
    let w = s.world_mut();
    let flood = at(-2, 0);
    w.tiles[flood].flood = true;
    assert!((ground(w, flood) - 20.0).abs() < EPS, "a floodplain takes 20 days");
    for (river, days) in [(1u32, 10.0 / 8.0), (4, 10.0 / 2.0), (8, 10.0)] {
        let i = at(-2, 2);
        w.tiles[i].flood = false;
        w.tiles[i].river = river;
        assert!((ground(w, i) - days).abs() < 1e-6, "river {river}: {}", ground(w, i));
    }
    let dry = at(-3, 1);
    w.tiles[dry].flood = false;
    w.tiles[dry].river = 0;
    w.tiles[dry].terrain = engine::data::PLAINS;
    assert_eq!(ground(w, dry), 0.0, "a dry plains tile takes none");
}

#[test]
fn a_floodplain_on_a_river_takes_both() {
    let mut s = founded();
    let w = s.world_mut();
    let i = at(-2, 0);
    w.tiles[i].flood = true;
    w.tiles[i].river = 8;
    assert!((ground(w, i) - 30.0).abs() < 1e-6);
}

#[test]
fn yield_is_zero_on_wild_grain_and_one_and_a_half_grain_at_a_hearth() {
    let mut s = founded();
    let w = s.world_mut();
    let i = at(-2, 0);
    farm_tile(w, i);
    w.tiles[i].tame = 0;
    assert_eq!(yield_a_day(w, i), 0.0, "wild grain yields nothing in a field");
    w.tiles[i].tame = TENDING.tame_turns;
    w.tiles[i].fertility = 1.0;
    assert!((yield_a_day(w, i) - 1.5 * FIELD.grain).abs() < EPS);
}

#[test]
fn sow_takes_at_most_each_tiles_ground_and_passes_the_rest_on() {
    let mut s = founded();
    let w = s.world_mut();
    let (a, b) = (at(-2, 0), at(-2, 1));
    w.tiles[a].flood = true;
    w.tiles[b].river = 1;
    // Work over the ground is dropped where no other tile can take it.
    let days = sow(w, 0, &[a, b], &[30.0, 2.0]);
    assert!((days[0] - 20.0).abs() < EPS && (days[1] - 1.25).abs() < 1e-6, "{days:?}");
    // Work under the ground stays where it is, and spare goes to the tile below its ground.
    let days = sow(w, 0, &[a, b], &[5.0, 2.0]);
    assert!((days[0] - 5.75).abs() < 1e-6 && (days[1] - 1.25).abs() < 1e-6, "{days:?}");
}

#[test]
fn the_split_rises_a_notch_a_turn_from_zero_while_fields_feed_better() {
    let (mut s, id, _) = floodplain_clan();
    let w = s.world_mut();
    for turn in 1..=4 {
        let before = w.clans[&id].fields;
        notch(w, id);
        let after = w.clans[&id].fields;
        assert!((after - before - FIELD.notch).abs() < EPS, "turn {turn}: {before} to {after}");
    }
    assert!((w.clans[&id].fields - 4.0 * FIELD.notch).abs() < EPS);
    let farming = w.events.iter().filter(|e| e.kind == EventKind::ClanFarming).count();
    assert_eq!(farming, 1, "ClanFarming fires once, when the share rises from 0");
}

#[test]
fn a_starving_clan_with_fields_drops_a_notch_a_turn() {
    let (mut s, id, _) = floodplain_clan();
    let w = s.world_mut();
    w.clans.get_mut(&id).unwrap().fields = 0.3;
    w.clans.get_mut(&id).unwrap().starving = true;
    notch(w, id);
    assert!((w.clans[&id].fields - 0.2).abs() < EPS);
    notch(w, id);
    assert!((w.clans[&id].fields - 0.1).abs() < EPS);
    notch(w, id);
    assert!(w.clans[&id].fields.abs() < EPS);
}

#[test]
fn a_clan_with_no_field_ground_keeps_no_share() {
    let mut s = founded();
    let w = s.world_mut();
    let dry = at(-2, 0);
    w.tiles[dry].flood = false;
    w.tiles[dry].river = 0;
    let id = hold(w, &[dry], 10.0, 0.0);
    for _ in 0..3 {
        notch(w, id);
    }
    assert_eq!(w.clans[&id].fields, 0.0);
}

#[test]
fn field_grain_leaves_no_stock_and_no_wear_and_days_stay_under_ground() {
    let mut s = founded();
    let w = s.world_mut();
    let (f, p) = (at(-2, 0), at(-2, 1));
    farm_tile(w, f);
    w.tiles[p].stock[GRAIN] = 4.0;
    w.tiles[p].cap[GRAIN] = 10.0;
    w.tiles[p].unworn[GRAIN] = 10.0;
    let id = hold(w, &[f, p], 10.0, 0.5);
    let c = w.clans[&id].clone();
    let (_, yearly) = engine::clans::walks(w, c.home, &c.territory);
    let reaped =
        reap(w, c.id, c.people, &c.store, &c.routes, &c.territory, &yearly, c.fields, &vec![true; c.territory.len()]);
    let k = c.territory.iter().position(|&t| t == p).unwrap();
    let kf = c.territory.iter().position(|&t| t == f).unwrap();
    let grain_before = w.tiles[f].stock[GRAIN];

    harvest(w);

    assert_eq!(w.tiles[f].stock[GRAIN], grain_before, "a field draws on no stock");
    assert_eq!(w.tiles[p].take, reaped.takes[k], "a tile's take is the gathering alone");
    assert_eq!(w.tiles[f].take, [0.0; N], "field grain is not in a tile's take");
    assert!(w.tiles[f].field > 0.0 && w.tiles[f].field <= ground(w, f) + EPS);
    assert!((w.tiles[f].field - reaped.days[kf]).abs() < EPS);
    let food = w.clans[&id].harvest;
    assert!(food[GRAIN] > reaped.takes.iter().map(|t| t[GRAIN]).sum::<f64>(), "the field's grain is in the harvest");
    wear(w);
    assert_eq!(w.tiles[f].wear[GRAIN], 0.0, "field grain wears nothing");
    let c = w.clans[&id].clone();
    assert_eq!(plots(w, &c).first().map(|x| x.0), Some(f));
}

#[test]
fn a_farmers_home_steps_toward_its_best_field_one_tile_a_turn() {
    let mut s = founded();
    let w = s.world_mut();
    let (home, mid, field) = (at(-3, 0), at(-2, 0), at(-1, 0));
    farm_tile(w, field);
    let id = hold(w, &[home, mid, field], 10.0, 0.5);
    let before = w.clans[&id].clone();
    assert_eq!(home_toward(w, &before, &before.territory), Some(field), "its best field tile");
    let gap = w.tiles[home].hex().distance(w.tiles[field].hex());
    adjust(w);
    let c = w.clans[&id].clone();
    assert_eq!(c.home, mid, "one tile toward the field");
    assert_eq!(w.tiles[c.home].hex().distance(w.tiles[field].hex()), gap - 1);
}

#[test]
fn the_civs_own_clan_stays_on_the_capital() {
    let mut s = founded();
    let w = s.world_mut();
    let cap = at(CAP.0, CAP.1);
    let north = at(CAP.0, CAP.1 - 1);
    farm_tile(w, north);
    let civ = *w.civs.keys().next().unwrap();
    let id = w.civs[&civ].clan;
    w.clans.get_mut(&id).unwrap().territory = vec![cap.min(north), cap.max(north)];
    w.clans.get_mut(&id).unwrap().fields = 0.5;
    w.tiles[north].claim = Some(id);
    adjust(w);
    assert_eq!(w.clans[&id].home, cap);
}

#[test]
fn fields_off_or_before_the_pick_keep_the_share_at_zero_and_reap_as_before() {
    // Before the pick: no civs, so the field job is shut even with a share.
    let mut s = Stage::flat(RADIUS);
    let f = at(-2, 0);
    farm_tile(s.world_mut(), f);
    let id = hold(s.world_mut(), &[f], 10.0, 0.0);
    let w = s.world_mut();
    notch(w, id);
    assert_eq!(w.clans[&id].fields, 0.0);
    let (_, yearly) = engine::clans::walks(w, f, &[f]);
    let store = [1.0, 2.0, 0.0, 0.0, 0.0];
    for share in [0.0, 0.5] {
        assert_eq!(
            reap(w, 0, 10.0, &store, &[], &[f], &yearly, share, &[true]).takes,
            old_reap(w, 10.0, &store, &[], &[f], &yearly)
        );
    }

    // Config off, with civs: the same.
    let mut s = founded();
    let w = s.world_mut();
    w.config.fields = false;
    let (f, p) = (at(-2, 0), at(-2, 1));
    farm_tile(w, f);
    w.tiles[p].stock[GRAIN] = 6.0;
    w.tiles[p].cap[GRAIN] = 10.0;
    w.tiles[p].unworn[GRAIN] = 10.0;
    let id = hold(w, &[f, p], 10.0, 0.0);
    for _ in 0..3 {
        notch(w, id);
    }
    assert_eq!(w.clans[&id].fields, 0.0, "off is the control: no clan farms");
    let c = w.clans[&id].clone();
    let (_, yearly) = engine::clans::walks(w, c.home, &c.territory);
    let tiles = c.territory.clone();
    for (pop, store) in [(10.0, [0.0; N]), (2.0, [0.0; N]), (40.0, [9.0, 0.0, 0.0, 0.0, 0.0])] {
        for share in [0.0, 0.5] {
            let got = reap(w, c.id, pop, &store, &c.routes, &tiles, &yearly, share, &vec![true; tiles.len()]).takes;
            assert_eq!(got, old_reap(w, pop, &store, &c.routes, &tiles, &yearly), "pop {pop} share {share}");
        }
    }
}

#[test]
fn on_real_maps_with_fields_off_reap_is_the_old_reap() {
    for seed in 1..=2 {
        let mut w = World::new(Config {
            seed,
            radius: 16,
            army: true,
            gifts: true,
            zone: true,
            fields: false,
            ..Config::default()
        });
        w.run(60);
        for c in w.clans.values() {
            let (_, yearly) = engine::clans::walks(&w, c.home, &c.territory);
            for (pop, store) in [(c.people, c.store), (2.0, [0.0; N])] {
                let got =
                    reap(&w, c.id, pop, &store, &c.routes, &c.territory, &yearly, 0.5, &vec![true; c.territory.len()])
                        .takes;
                assert_eq!(got, old_reap(&w, pop, &store, &c.routes, &c.territory, &yearly));
            }
        }
    }
}

#[test]
fn a_field_tile_tames_one_plus_fields_steps_a_turn_while_held_past_the_wait() {
    let mut s = founded();
    let w = s.world_mut();
    let (f, p) = (at(-2, 0), at(-2, 1));
    for &i in &[f, p] {
        w.tiles[i].wild_grain = 10.0;
        w.tiles[i].tame = 10;
        w.tiles[i].claim = Some(9999);
        w.tiles[i].claim_run = TENDING.after;
    }
    w.tiles[f].field = 5.0;
    tend(w);
    assert_eq!(w.tiles[f].tame, 10 + 1 + FIELD.tame);
    assert_eq!(w.tiles[p].tame, 10 + 1);
}

#[test]
fn a_field_tile_does_not_tame_before_the_wait() {
    let mut s = founded();
    let w = s.world_mut();
    let f = at(-2, 0);
    w.tiles[f].wild_grain = 10.0;
    w.tiles[f].tame = 10;
    w.tiles[f].claim = Some(9999);
    w.tiles[f].claim_run = TENDING.after - 2;
    w.tiles[f].field = 5.0;
    tend(w);
    assert_eq!(w.tiles[f].tame, 10, "held one turn short of the wait: no taming yet");
}

#[test]
fn ground_is_a_pure_function_of_the_map() {
    // No rng: asking twice gives the same answer, and the world's stream is untouched.
    let mut s = founded();
    let w = s.world_mut();
    let before = w.rng;
    let _ = (ground(w, at(-2, 0)), fields::open(w));
    assert_eq!(w.rng, before);
}

/// The judge's fields group over 15 turns the test plays by hand: a farmer from turn 1 on (in the
/// civ), and an unaligned gatherer. Turn `t` is scored after the world is set for it.
#[test]
fn the_judge_counts_settled_farmers_and_gatherers_apart() {
    use engine::events::Event;
    use engine::judge::Watch;
    use engine::judge_fields::FIELD_SETTLED;
    let mut s = founded();
    let w = s.world_mut();
    let civ = *w.civs.keys().next().unwrap();
    let (f, f2, g, g2) = (at(-2, 0), at(-2, 1), at(2, 2), at(3, 2));
    farm_tile(w, f);
    let farmer = hold(w, &[f, f2], 10.0, 0.0);
    w.clans.get_mut(&farmer).unwrap().civ = Some(civ);
    let gatherer = hold(w, &[g], 10.0, 0.0);
    let pack = engine::units::pack_cap(UNIT.first as f64);
    let mut watch = Watch::new(w);
    for t in 1..=15u32 {
        w.turn += 1;
        let c = w.clans.get_mut(&farmer).unwrap();
        c.fields = 0.5;
        c.starving = t == 12 || t == 14;
        if t == 13 {
            c.home = f2;
        }
        let c = w.clans.get_mut(&gatherer).unwrap();
        c.starving = (3..=5).contains(&t);
        if t == 6 {
            c.home = g2;
            w.emit(Event::new(EventKind::ClanMoved, g2).clan(gatherer));
        }
        let store = if t % 3 == 0 { pack } else { 0.0 };
        w.civs.get_mut(&civ).unwrap().store = [0.0; N];
        w.civs.get_mut(&civ).unwrap().store[GRAIN] = store;
        watch.turn(w);
    }
    let scores = watch.scores(w, &[civ]);
    let mine = &scores.fields.civs[&civ];
    let total = &scores.fields.total;
    assert_eq!(FIELD_SETTLED, 10);
    assert_eq!(mine.settled_turns, 5, "settled from its eleventh farming turn: turns 11 to 15");
    assert_eq!(mine.settled_home_moves, 1, "its move at turn 13, once");
    assert_eq!(mine.settled_moved_on, 0);
    assert_eq!(mine.settled_starving, 2, "turns 12 and 14");
    assert_eq!(total.settled_turns, 5);
    assert_eq!(total.gatherer_starving, 3, "the gatherer's turns 3 to 5, apart from the farmer's");
    assert_eq!(total.gatherer_moved_on, 1);
    assert_eq!(total.gatherer_home_moves, 1);
    assert_eq!(total.starving, 5);
    assert_eq!(mine.store_turns_at_pack, 5);
    assert_eq!(mine.turns, 15);
    let per_civ: u32 = scores.fields.civs.values().map(|c| c.store_turns_at_pack).sum();
    assert_eq!(per_civ, scores.economy.turns_at_pack, "the per-civ split sums to the economy's");
    assert_eq!(scores.fields.civs_never_at_pack, 0);
}
