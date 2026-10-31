use crate::report::report;
use crate::run::{print_events, Output};
use crate::USAGE;
use engine::{Save, World};
use std::fs;

pub(crate) fn replay(path: &str, args: &[String]) -> Result<(), String> {
    let mut out = Output::default();
    for a in args {
        if !out.flag(a) {
            return Err(format!("unknown flag {a}\n{USAGE}"));
        }
    }
    let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let save: Save = serde_json::from_str(&text).map_err(|e| format!("{path}: {e}"))?;
    let w = World::replay(&save)?;
    if out.events_text() {
        print_events(&w, &mut 0);
    }
    report(&w, &out);
    Ok(())
}
