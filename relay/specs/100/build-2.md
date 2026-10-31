# Build 2 (ford): the units module

Piece #100. Read `relay/specs/100/spec.md` (items 1, 4, 5, 6 and 7's `units::walk` aside). This is a
move and a reshape of state; **no rule changes**, and the world's JSON does not change by a byte.

## The work

1. **Split `core/src/units.rs` into `core/src/units/{mod,line,camp,scout,ai}.rs`** as spec item 4 lists
   them (`git mv units.rs units/mod.rs` first so history follows). Visibility: what other modules call
   stays `pub` and is re-exported from `units` (`units::camp`, `units::line`, `units::route`,
   `units::cut_off`, `units::cut_at`, `units::path_between`, `units::turns`, `units::pack_cap`,
   `units::zone_cap`, `units::route_plan`, `units::raise_at_founding`, `units::raise_scout_at_founding`,
   `units::check_*`, `units::order_*`, `units::leader`, `units::ai`, `units::step`, `units::post`,
   `units::disband`, `units::raise`, `Counters`, `Unit`, `UnitKind`, `Plan` …); the rest is
   `pub(super)`/private. `strip`, `supply`, `room`, `rival_of` and `name_of` serve both the camp and the
   scout, so they sit in `mod.rs`, not `camp.rs`. Keep every function body as it is; this step moves code.
2. **`Unit` and `UnitState`** (item 5) in `units/mod.rs`. The shape:

   ```rust
   pub struct Unit { pub id, pub civ, pub actor: Actor, pub banked: f64, pub hungry: u32, pub state: UnitState }
   pub enum UnitState { Camp(CampState), Scout(ScoutState) }
   ```

   with `Deref<Target = Actor>`/`DerefMut` kept, `Unit::kind()`, `camp()`, `camp_mut()`, `scout()`,
   `scout_mut()`, `held()`, `tile()`. Camp code takes `&CampState` from `u.camp()`; `join` and
   `raise_scout_at_founding` build the two states. Do not leave dead fields on the wrong kind.
3. **The JSON stays flat** (item 6, and "The digests" item 1): give `Unit` a private flat mirror
   struct holding the old 27 fields in the old order (spec item 6 has the list, read from main's JSON)
   and `#[serde(from = "UnitFlat", into = "UnitFlat")]` (`Clone` is needed for `into`). `kind` is
   written from the state; reading a unit picks the state by `kind` and drops the other kind's fields.
   **A scout writes the camp's fields as `raise_scout_at_founding` sets them, not as `Default`:**
   `auto_forage: true`, `auto_supply: true`, `dest: null`, the other flags `false`, `held: []`,
   `guard: []`, `cut_by: null`. A camp writes `route: null`, `heard: false`, `blocked: false`. `#[serde(flatten)] actor` works inside the
   mirror as it does today.
4. Fix the callers: `civs::holder` (`u.held()`), `civs`, `leaning`, `stats`, `web/src/lib.rs`
   (`u.route` for a scout → `u.scout()`; the view's JSON of a unit is built from `serde_json::to_value(u)`
   as now), `cli/src/main.rs` (`u.dest` etc. through `camp()`), and the tests that touch fields
   (`core/tests/units.rs`, `scout.rs`, `alerts.rs`, `notes.rs`, `civs.rs`: `put_camp` and friends).

## Tests

`core/tests/units.rs`: a camp's and a scout's `serde_json::to_value`/`to_string` keys, in order, equal the
27-key list of spec item 6 (copy the list into the test).

## Done

`./check.sh` green. The four watching runs' `digest` and the led run's `digest --state` are identical to
`relay/specs/100/baseline/` on every turn (`diff` empty — run all five and say so); the retro sweep
`diff` against `baseline/retro-sweep.txt` is empty. No file in `core/src/units/` is over about 600 lines.
`engine/CLAUDE.md`'s layout row for `units.rs` is replaced by one row per file.

## What build-3 and later rely on

`Unit.state`, `Unit::camp()/scout()`, and the five files. The orders' `check_*`/`order_*` are still the
functions they were, in `camp.rs`/`scout.rs`/`mod.rs` by subject; `units::walk` and `units/scout.rs`'s
`march` are still there for build-3.

The comparison with the baselines is `bash relay/specs/100/baseline/compare.sh`: six `ok` lines.
