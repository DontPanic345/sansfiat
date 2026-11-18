# #341 notes for the next builder

- A probe as an ignored test in `core/tests/` (`Stage::retro(size, seed)`, `ChooseCiv { civ: None }`,
  `w.run(1)` to +200) runs all seven maps in about 40 s after its build, against 3 minutes for a
  sweep. It reads exactly what the sweep's base run reads. The spec's probes printed homes on each
  capital, the farmers within four tiles and `homes::step`'s choices; delete them before committing.
- An env-guarded line (`std::env::var("X").is_ok() && …`) let the spec sweep a rule beside main
  from one build. Remove it before committing.
- `could_reap` with no route equals `could_reap` with one for most farmers by a festival: their
  labour isn't what binds.
