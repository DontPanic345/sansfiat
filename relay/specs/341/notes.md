# #341 notes for the next builder

- A probe as an ignored test in `core/tests/` (`Stage::retro(size, seed)`, `ChooseCiv { civ: None }`,
  `w.run(1)` to +200) runs all seven maps in about 40 s after its build, against 3 minutes for a
  sweep. It reads exactly what the sweep's base run reads. The spec's probes printed homes on each
  capital, the farmers within four tiles and `homes::step`'s choices; delete them before committing.
- An env-guarded line (`std::env::var("X").is_ok() && …`) let the spec sweep a rule beside main
  from one build. Remove it before committing.
- `could_reap` with no route equals `could_reap` with one for most farmers by a festival: their
  labour isn't what binds.

## Build-1 (the easel)

- The page's mark check passed on main's layer with `differing > 0`: the capital's house left 14 pixels
  of the mark in a 48 px square. The check is `>= 100` now (the mark on a capital shows 405 px).
- `LAYERS` order: `towns` draws after `capitals` in `LAYERS`. A later layer reorder must keep it so.
- `could_reap` is `pub(crate)`, so a probe outside the crate reads the door step's values only by
  an `eprintln!` in `homes::step`'s `weigh` closure (removed before commit).
- The judge's bound "town clans trade more" reads 0.0476 in a town against 0.1006 outside: the line's
  name says the opposite of its numbers under rule A. The number is what it is; the name is the table's.
- Save files under `relay/specs/338/play/` and `relay/specs/355/play/` are version 26 and no longer
  replay at SAVE_VERSION 27.

## Build-2 (the canvas)

- `core/tests/cases.rs` and `core/tests/units.rs` read one real map's history at a fixed turn (the scout's settling
  on medium 8 to the rival within 200 turns; civ 469's army on small 2 at +100, fed by its line). Any home rule that
  moves a map's history can turn them red without a fault in the rule: `drift.diff` at drift 5 and over reds
  `units.rs`, at 4 it reds `cases.rs` (medium 8). A probe with `Stage::retro` and `towns()` over the seven maps
  (35 s) plus `cargo test --release -p engine --test cases --test units` (15 s) is the quick loop; the whole
  `cargo test --release -p engine` takes seven minutes.
- The pregame's pick world (`Stage::retro`) depends on the home rules too: a rule that moves farmers moves where
  the pre-game ends and so the civs' founding.
