//! Settling: a clan settles after `SETTLING.settle_after` turns in a row as a farmer, goes back after
//! `SETTLING.leave_after` turns in a row under `SETTLING.gathers`, and stays as it was between; the clan
//! a civ is founded on is settled at founding; the feed tells each change, and the judge counts them.
//! Staged on bare plains with a civ at the centre.

use engine::clans::new_clan;
use engine::data::SETTLING;
use engine::events::EventKind;
use engine::hex::Hex;
use engine::judge::Watch;
use engine::map::index;
use engine::settling::{self, living};
use engine::stage::Stage;
use engine::World;

const RADIUS: i32 = 4;

/// Bare plains with a civ founded at the centre on a clan of 5 people.
fn staged() -> (World, u32, usize) {
    let mut stage = Stage::flat(RADIUS);
    let centre = index(RADIUS, Hex { q: 0, r: 0 }).unwrap();
    let civ = stage.civs(&[centre], 5.0)[0];
    (stage.into_world(), civ, centre)
}

/// A clan of no civ at (2, -1) with field share `fields`.
fn clan(w: &mut World, fields: f64) -> u32 {
    let id = w.next_id;
    w.next_id += 1;
    let home = index(RADIUS, Hex { q: 2, r: -1 }).unwrap();
    let mut c = new_clan(id, home, 6.0);
    c.territory = vec![home];
    c.fields = fields;
    w.clans.insert(id, c);
    id
}

/// Runs the settling step `n` times and returns the clan's state after.
fn steps(w: &mut World, id: u32, n: u32) -> bool {
    for _ in 0..n {
        settling::step(w);
    }
    w.clans[&id].settled
}

fn texts(w: &World, kind: EventKind) -> Vec<String> {
    texts_of(w, kind, None)
}

/// The feed's lines of `kind`, of clan `id` only when given.
fn texts_of(w: &World, kind: EventKind, id: Option<u32>) -> Vec<String> {
    w.events.iter().filter(|e| e.kind == kind && id.is_none_or(|i| e.clan == Some(i))).map(|e| e.text(w)).collect()
}

#[test]
fn the_clan_a_civ_is_founded_on_is_settled_at_founding() {
    let (w, civ, centre) = staged();
    let own = w.civs[&civ].clan;
    let c = &w.clans[&own];
    assert_eq!((c.home, c.fields), (centre, 0.0), "the civ's own clan, on the capital, has no fields");
    assert!(c.settled, "settled at founding, whatever its ground");
    assert!(texts(&w, EventKind::ClanSettled).is_empty(), "the founding says it; the feed has no settle line");
}

#[test]
fn a_farmer_settles_after_its_turns_in_a_row() {
    let (mut w, _, _) = staged();
    let id = clan(&mut w, SETTLING.farms);
    assert!(!steps(&mut w, id, SETTLING.settle_after - 1), "not yet: one turn short");
    assert!(steps(&mut w, id, 1), "settled on the turn its run reaches the turns");
    assert_eq!(
        texts(&w, EventKind::ClanSettled),
        vec![format!(
            "Clan {id} settles at (2, -1): a fifth or more of its days on fields for {} turns",
            SETTLING.settle_after
        )]
    );
}

#[test]
fn a_turn_under_the_line_starts_the_run_again() {
    let (mut w, _, _) = staged();
    let id = clan(&mut w, SETTLING.farms);
    steps(&mut w, id, SETTLING.settle_after - 1);
    // Between the two lines: a gatherer's run toward settling ends.
    w.clans.get_mut(&id).unwrap().fields = (SETTLING.farms + SETTLING.gathers) / 2.0;
    assert!(!steps(&mut w, id, 1));
    w.clans.get_mut(&id).unwrap().fields = SETTLING.farms;
    assert!(!steps(&mut w, id, SETTLING.settle_after - 1), "the run starts again from nothing");
    assert!(steps(&mut w, id, 1));
}

#[test]
fn a_settled_clan_stays_between_the_lines_and_goes_back_under_a_tenth() {
    let (mut w, _, _) = staged();
    let id = clan(&mut w, SETTLING.farms);
    assert!(steps(&mut w, id, SETTLING.settle_after));
    w.clans.get_mut(&id).unwrap().fields = SETTLING.gathers;
    assert!(steps(&mut w, id, 10 * SETTLING.leave_after), "at a tenth it stays settled however long");
    w.clans.get_mut(&id).unwrap().fields = 0.0;
    assert!(steps(&mut w, id, SETTLING.leave_after - 1), "not yet: one turn short");
    assert!(!steps(&mut w, id, 1), "back to gathering");
    assert_eq!(
        texts_of(&w, EventKind::ClanGathers, Some(id)),
        vec![format!(
            "Clan {id} goes back to gathering at (2, -1): under a tenth of its days on fields for {} turns",
            SETTLING.leave_after
        )]
    );
}

#[test]
fn a_settled_clan_that_moves_its_home_stays_settled() {
    let (mut w, _, _) = staged();
    let id = clan(&mut w, SETTLING.farms);
    assert!(steps(&mut w, id, SETTLING.settle_after));
    let to = index(RADIUS, Hex { q: 1, r: -1 }).unwrap();
    let c = w.clans.get_mut(&id).unwrap();
    c.home = to;
    c.territory = vec![to];
    assert!(steps(&mut w, id, 3 * SETTLING.leave_after), "a home that moves toward its fields keeps its state");
    assert!(texts_of(&w, EventKind::ClanGathers, Some(id)).is_empty());
}

#[test]
fn the_founding_clan_goes_back_as_any_clan_does() {
    let (mut w, civ, _) = staged();
    let own = w.civs[&civ].clan;
    assert!(steps(&mut w, own, SETTLING.leave_after - 1));
    assert!(!steps(&mut w, own, 1), "with no fields, it goes back after its turns");
}

#[test]
fn the_living_split_reads_settled_clans_as_farming() {
    let (mut w, civ, _) = staged();
    let id = clan(&mut w, 0.0);
    w.clans.get_mut(&id).unwrap().civ = Some(civ);
    let l = living(&w, civ);
    let army: f64 = w.units.values().filter(|u| u.civ == civ).map(|u| u.people).sum();
    let own = w.clans[&w.civs[&civ].clan].people;
    assert_eq!((l.farming, l.gathering, l.settled, l.clans), (own, 6.0, 1, 2), "the own clan farms, the other gathers");
    assert_eq!(l.army, army, "the army part is its units' people");
}

#[test]
fn the_judge_counts_settled_clans_by_civ_and_unaligned_and_their_changes() {
    let (mut w, civ, _) = staged();
    let own = w.civs[&civ].clan;
    let id = clan(&mut w, SETTLING.farms);
    let mut watch = Watch::new(&w);
    for t in 1..=200 {
        // The civ's own clan keeps a farmer's share, so it stays settled; the clan of no civ settles,
        // and goes back once its share drops at +150.
        w.clans.get_mut(&own).unwrap().fields = SETTLING.farms;
        if t == 150 {
            w.clans.get_mut(&id).unwrap().fields = 0.0;
        }
        settling::step(&mut w);
        watch.turn(&w);
    }
    let s = watch.scores(&w, &[]).settling;
    let mine = &s.civs[&civ];
    let people = w.clans[&own].people;
    assert_eq!((mine.settled_100, mine.settled_people_100, mine.settled_200, mine.clans_200), (1, people, 1, 1));
    let u = &s.unaligned;
    assert_eq!((u.settled_100, u.settled_people_100, u.settled_200, u.clans_200), (1, 6.0, 0, 1));
    assert_eq!((u.settles, u.goes_back, mine.settles, mine.goes_back), (1, 1, 0, 0));
    assert_eq!((s.clans_changed, s.clans_changed_twice), (1, 1));
    assert_eq!(s.total.clan_turns, 400);
}

#[test]
fn a_daughter_starts_as_her_mother() {
    let (mut w, _, _) = staged();
    let id = clan(&mut w, SETTLING.farms);
    steps(&mut w, id, SETTLING.settle_after);
    let c = w.clans.get_mut(&id).unwrap();
    c.people = 400.0;
    let home = c.home;
    let ring: Vec<usize> = engine::land::neighbours(&w, home).collect();
    let c = w.clans.get_mut(&id).unwrap();
    c.territory = std::iter::once(home).chain(ring).collect();
    engine::dividing::divide_land(&mut w, id);
    let daughter = w.clans.values().find(|c| c.parent == Some(id)).map(|c| c.id).expect("a daughter");
    assert!(w.clans[&daughter].settled, "she starts settled, as her mother");
}
