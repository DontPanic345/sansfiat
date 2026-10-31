mod bench;
mod clear;
mod digest;
mod judge;
mod map;
mod play;
mod replay;
mod report;
mod retro;
mod run;
mod sweep;
mod view;

use std::{env, process::exit};

use engine::data::{size_radius, SIZES};
use engine::GOODS;

use bench::bench;
use clear::clear_file;
use digest::digest;
use judge::judge_game;
use map::map;
use play::play;
use replay::replay;
use run::run;
use sweep::sweep;

pub(crate) const USAGE: &str = "\
sansfiat run [--seed N] [--turns N] [--radius N | --size small|medium|large] [--every N] [--save file.json] [--json | --events-json] [--no-events] [--why ID]   (--why prints the reasons of a clan, market, civ or unit, newest first)
sansfiat map [--seed N] [--radius N | --size small|medium|large]   (the map as text, with counts)
sansfiat replay <file.json> [--json | --events-json] [--no-events]
sansfiat play --game FILE [--seed N --size NAME [--civ K]]   (a game a line at a time: the file is a save, resumed by each call; without --civ a new game prints the civs' cards; lines on stdin, the view after each accepted one; FILE.lines.jsonl records every line)
sansfiat judge FILE [--json]   (a played game judged turn by turn, beside the same map with every civ on the AI: Standing, doubt, motion and the player's decisions)
sansfiat sweep [seed ...] [--turns N] [--radius N | --size small|medium|large|all] [--every N]   (seeds default 1..=10, size small; a series line every N turns, default 50)
sansfiat sweep --judge [--bands FILE] [--json]   (the retro maps, every civ on the AI: the army run twice and the --no-army control, scored by the judge over 200 turns past the pick; reads judge.toml's bands, exits 1 on a fail)
sansfiat sweep --after N [--maps retro | seed ... [--size NAME]] [--no-army] [--no-gifts] [--no-zone] [--lever G]   (the camps and the clans: run to the pick, all on the AI, N turns on; per civ its camp, the clans that joined and the totals; --no-army, --no-gifts and --no-zone (AI camps paint no zone) are the controls; --lever G leads the lowest civ with gift G, its camp posted where the AI would; --maps retro is the seven maps)
sansfiat bench [seed ...] [--turns N] [--radius N | --size NAME]
sansfiat digest [--seed N] [--radius N | --size NAME] [--turns N] [--lead] [--state]   (prints turn and digest, from turn 0; --lead plays the scripted game: picks the first civ and gives orders; --state hashes the world without its log)
sansfiat clear <file.json | ->   {\"baskets\": [{\"grain\": 4}, {\"fish\": 2}], \"prices\"?: {\"grain\": 1}}
sansfiat goods";

/// The radius of the map size called `name`.
pub(crate) fn size(name: &str) -> Result<i32, String> {
    size_radius(name).ok_or_else(|| {
        format!("unknown size {name}; sizes: {}", SIZES.iter().map(|s| s.0).collect::<Vec<_>>().join(" "))
    })
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("run") => run(&args[1..]),
        Some("replay") => match args.get(1) {
            Some(path) => replay(path, &args[2..]),
            None => Err(USAGE.to_string()),
        },
        Some("sweep") => sweep(&args[1..]),
        Some("bench") => bench(&args[1..]),
        Some("digest") => digest(&args[1..]),
        Some("map") => map(&args[1..]),
        Some("play") => play(&args[1..]),
        Some("judge") => judge_game(&args[1..]),
        Some("clear") => clear_file(args.get(1).map(String::as_str).unwrap_or("-")),
        Some("goods") => {
            println!("{}", GOODS.join(" "));
            Ok(())
        }
        _ => Err(USAGE.to_string()),
    };
    if let Err(e) = result {
        eprintln!("{e}");
        exit(2);
    }
}
