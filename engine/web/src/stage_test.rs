//! The stage door: the page's stage tests. The list, a test built as a `Sim` with its run attached,
//! and the turns the page plays. The runner and the verdict are core's (`engine::stage_test`).

use engine::stage_test::{catalogue, Run, StageTest};
use wasm_bindgen::prelude::*;

use crate::Sim;

#[wasm_bindgen]
impl Sim {
    /// The stage tests, as JSON: each one's `name`, `claim`, `turns`, `kept` (its kept stores as
    /// words), `holds` (its holds as words) and `json`, the file's own text, which `stage_test`
    /// builds the test from.
    pub fn tests() -> String {
        catalogue().expect("the listed tests build").to_string()
    }

    /// A test built from its JSON (one of `tests()`'s `json`s, or a changed copy): its world staged,
    /// with its run attached. `Err` says why not.
    pub fn stage_test(json: &str) -> Result<Sim, String> {
        let test: StageTest = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let (world, run) = Run::new(&test)?;
        Ok(Sim { world, lead: engine::script::Lead::default(), staged: true, run: Some(run) })
    }

    /// Plays one turn of the test (none once it is done) and returns `test_state()`. `Err` when this
    /// is no test, or an order is refused: the engine's reason.
    pub fn test_step(&mut self) -> Result<String, String> {
        let run = self.run.as_mut().ok_or("this game is not a stage test")?;
        if !run.done() {
            run.step(&mut self.world)?;
        }
        self.test_state()
    }

    /// The test's turn, turns, whether it is done and passed, and each hold's state, as JSON:
    /// `{turn, turns, done, passed, holds: [{words, state, turn?, why?}]}`. `Err` when this is no test.
    pub fn test_state(&self) -> Result<String, String> {
        let run = self.run.as_ref().ok_or("this game is not a stage test")?;
        Ok(run.state_json(&self.world).to_string())
    }
}
