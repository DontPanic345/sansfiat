use crate::report::{line, report};
use crate::{size, USAGE};
use engine::data::PREGAME;
use engine::{stats, Config, World};
use std::fs;

pub(crate) fn run(args: &[String]) -> Result<(), String> {
    let mut config = Config::default();
    let (mut turns, mut save, mut every, mut why) = (PREGAME.max_turns, None, 0u32, None);
    // An explicit `--turns` runs on past the pre-game's end; by default the run stops there.
    let mut explicit = false;
    let mut out = Output::default();
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        if out.flag(flag) {
            continue;
        }
        let value = it.next().ok_or(format!("{flag} needs a value\n{USAGE}"))?;
        let bad = |_| format!("{flag}: not a number: {value}");
        match flag.as_str() {
            "--seed" => config.seed = value.parse().map_err(bad)?,
            "--turns" => {
                turns = value.parse().map_err(bad)?;
                explicit = true;
            }
            "--radius" => config.radius = value.parse().map_err(bad)?,
            "--size" => config.radius = size(value)?,
            "--every" => every = value.parse().map_err(bad)?,
            "--save" => save = Some(value.clone()),
            "--why" => why = Some(value.parse().map_err(bad)?),
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    let mut w = World::new(config);
    let mut shown = 0;
    while w.turn < turns && (explicit || w.end.is_none()) {
        w.run(1);
        if out.events_text() {
            print_events(&w, &mut shown);
        }
        if every > 0 && w.turn.is_multiple_of(every) && out.text() {
            println!("{}", line(&stats::summary(&w)));
        }
    }
    if let Some(path) = save {
        fs::write(&path, serde_json::to_string(&w.save()).unwrap()).map_err(|e| format!("{path}: {e}"))?;
    }
    report(&w, &out);
    if let (Some(id), true) = (why, out.text()) {
        print_notes(&w, id);
    }
    Ok(())
}

/// What `run` and `replay` print: the report, or the world as JSON, or the events as JSON; the
/// events in text between the summary lines unless `--no-events`.
#[derive(Default)]
pub(crate) struct Output {
    pub(crate) json: bool,
    pub(crate) events_json: bool,
    pub(crate) no_events: bool,
}

impl Output {
    /// Takes the flag if it is one of ours.
    pub(crate) fn flag(&mut self, flag: &str) -> bool {
        match flag {
            "--json" => self.json = true,
            "--events-json" => self.events_json = true,
            "--no-events" => self.no_events = true,
            _ => return false,
        }
        true
    }

    /// Whether the plain-text lines (summaries, the report) are printed.
    fn text(&self) -> bool {
        !self.json && !self.events_json
    }

    pub(crate) fn events_text(&self) -> bool {
        self.text() && !self.no_events
    }
}

/// Prints the events from index `*shown` on as `turn  text`, an alert marked `!`.
pub(crate) fn print_events(w: &World, shown: &mut usize) {
    for e in &w.events[*shown..] {
        let mark = if e.alert { "!" } else { " " };
        println!("{:>6} {mark} {}", e.turn, e.text(w));
    }
    *shown = w.events.len();
}

/// Prints the reasons of subject `id`, newest first, as `turn  text`.
pub(crate) fn print_notes(w: &World, id: u32) {
    match w.notes.get(&id).filter(|list| !list.is_empty()) {
        Some(list) => {
            for n in list.iter().rev() {
                println!("{:>6}  {}", n.turn, n.text);
            }
        }
        None => println!("no reasons for {id}"),
    }
}
