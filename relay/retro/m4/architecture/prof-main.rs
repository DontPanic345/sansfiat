use engine::{Action, Config, Phase, World, TURN};
use std::time::Instant;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let seed: u32 = a[1].parse().unwrap(); let radius: i32 = a[2].parse().unwrap(); let after: u32 = a[3].parse().unwrap();
    let config = Config { seed, radius, army: true, gifts: true, zone: true, ..Config::default() };
    let mut w = World::new(config);
    while w.phase == Phase::Pregame { w.run(1); }
    w.act(Action::ChooseCiv { civ: None }).unwrap();
    let mut t = vec![0f64; TURN.len()];
    for _ in 0..after {
        let mut c = w.clone();
        for (i, s) in TURN.iter().enumerate() { let st = Instant::now(); (s.run)(&mut c); t[i] += st.elapsed().as_secs_f64(); }
        w.run(1);
    }
    let tot: f64 = t.iter().sum();
    for (i, s) in TURN.iter().enumerate() { if t[i]*1e3/after as f64 > 0.05 { println!("{:10} {:7.2}", s.name, t[i]*1e3/after as f64); } }
    println!("{:10} {:7.2}", "total", tot*1e3/after as f64);
}
