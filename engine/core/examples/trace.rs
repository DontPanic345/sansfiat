use engine::{units, Action, Config, Phase, World};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let radius: i32 = a[1].parse().unwrap();
    let seed: u32 = a[2].parse().unwrap();
    let every: u32 = a.get(3).map_or(10, |s| s.parse().unwrap());
    let mut w = World::new(Config { seed, radius, ..Config::default() });
    while w.phase == Phase::Pregame {
        w.run(1);
    }
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    let pick = w.turn;
    let mut seen = 0;
    for n in 0..=300 {
        if n % every == 0 {
            let st = engine::stats::standing(&w);
            for c in w.civs.values() {
                let caps: Vec<String> = w.civs.values().map(|o| format!("{}", engine::land::steps(&w, c.capital, o.capital))).collect();
                let army: Vec<String> = w.units.values().filter(|u| u.civ == c.id && u.camp().is_some()).map(|u| format!("army {:.1} at {} dest {:?} hungry {} pack {:.1}", u.people, w.at(u.home), u.dest().map(|d| w.at(d)), u.hungry, u.store.iter().sum::<f64>())).collect();
                let clans = w.clans.values().filter(|k| k.civ == Some(c.id)).count();
                let people: f64 = w.clans.values().filter(|k| k.civ == Some(c.id)).map(|k| k.people).sum();
                let athome: f64 = w.clans.values().filter(|k| k.civ == Some(c.id) && k.home == c.capital).map(|k| k.people).sum();
                let notes: Vec<String> = w.units.values().filter(|u| u.civ == c.id && u.camp().is_some()).flat_map(|u| w.notes.get(&u.id).into_iter().flatten().rev().take(5).map(|n| format!("        [{}] {}", n.turn - pick, n.text))).collect();
                println!("+{n} {} st {:.3} store {:.1} levied {:.1} gift {:.1} guard {:.1} claim {} held {} clans {clans} people {people:.0} at capital {athome:.0} fallen {:?} caps {} | {}", c.name, st[&c.id], c.store.iter().sum::<f64>(), c.levied, c.gift, engine::claim::guard_people(&w, c.id), c.claim.len(), engine::claim::held(&w, c.id).len(), c.fallen, caps.join(","), army.join("; "));
                if std::env::var("NOTES").is_ok() { for l in notes { println!("{l}"); } }
            }
        }
        w.run(1);
        for e in &w.events[seen..] {
            use engine::events::EventKind::*;
            if matches!(e.kind, Skirmish | Retreated | TileTaken | ClaimTaken | CapitalFell | ArmySurrendered | ClanLeft | GameOver | LineCut | CampPillaged | WarriorsStarved) {
                println!("   +{} {:?} civ {:?} other {:?} at {}", w.turn - pick, e.kind, e.civ, e.other, w.at(e.tile));
            }
        }
        seen = w.events.len();
    }
    let _ = units::camp(&w, 0);
}
