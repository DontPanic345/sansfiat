# #151 build-1 (ford): `map::parse`, `Stage`, `StageSpec`

You build the stage every later step uses: a world placed on a known map, given orders and
checked with the real engine. Read `spec.md` items 1–9 first (they are settled; follow them),
then `core/tests/common/mod.rs` (the hand-built worlds this replaces, in build-2),
`core/src/script.rs` (a public module like the one you add), `core/src/map.rs`'s `glyph`,
`ascii` and `ascii_marked` (lines 1163–1210), and `engine/CLAUDE.md`.

## Deliver

1. **`core/src/maptext.rs`** (new; `map.rs` is 1200 lines, so nothing goes into it but one
   re-export line, `pub use crate::maptext::{grid, parse, Parsed};`).
   - `grid(text) -> Result<(i32 /*radius*/, Vec<char> /*by tile index*/), String>`: the shared
     grid parser for the map and the layers. Rows are `ascii`'s: row `r` from `−radius` to
     `radius`, each indented, one char a tile with single spaces between. Radius is
     `(rows − 1) / 2`. Blank lines and anything after the last full row (the `map` command's
     legend and count lines) are ignored; a bad row count, a row of the wrong width or a
     missing glyph is an `Err` naming the row and column. Use `map::index(radius, Hex)` to
     place each char; do not invent a second indexing.
   - `parse(text) -> Result<Parsed, String>` with `Parsed { radius, tiles: Vec<Tile> }`: the
     glyph table of `spec.md` item 2 on top of `grid`. Tiles are the bare ones of item 3.
2. **`core/src/stage.rs`** (new, `pub mod stage;` in `lib.rs`): `Stage`, `StageSpec`, `RETRO`
   and the retro cache as `spec.md` items 3–9 say. The API, as the tests below use it:
   `Stage::parse(text)`, `Stage::flat(radius)` (sea on the ring at distance `radius`, plains
   inside), `Stage::retro(size, seed)`, `from_spec(&StageSpec)`, `marks`, `mark`, `hidden`,
   `food`, `clan`, `civs`, `lead`, `army`, `order`, `act`, `turn`, `until`, the reads of item 7,
   `world()`, `world_mut()` (for the odd painting a test needs; say in its doc that it
   bypasses the log), `into_world()`, `ascii()`. Every method has a doc comment. A staged
   world is not a save: do not add a `save()`; the stage's doc says so.
   - Write `Stage::parse`'s world exactly as `flat_radius` in `core/tests/common/mod.rs` writes
     its own (same Config, same `rng` and `next_id`, same cleared fields). Read the field list
     there and copy it; build-2 swaps the fixtures onto your code and every existing test must
     still pass untouched, so any difference you leave will show up there.
   - `civs` and `lead` and `army` do what `common::three`, `led` and `put_camp` do. `army`
     also sets the camp's people, `stack` (all warriors) and pack to match the people given:
     read `units::Unit`, `CampState` and `units::pack_cap` for the fields.
   - `StageSpec` derives `Serialize, Deserialize` (camelCase); `serde_json` is already a
     dependency of core.
3. **`RETRO`** (`[(&str, &[u32]); 3]`, the same seven maps) moves from `cli/src/retro.rs` to
   `stage.rs`; `cli/src/retro.rs` reads it from there (`retro_maps` is unchanged).

## Tests (`core/tests/stage.rs`)

- Round trip: `ascii(parse(t)) == t` for a hand-written text with every glyph; and on all
  seven retro maps at the pick (through `Stage::retro`), `ascii(parse(ascii(w.tiles)).tiles)
  == ascii(w.tiles)`.
- A bad text is refused with the row and column named: an unknown glyph, a short row, an even
  row count.
- The `map` command's output parses as printed: build it with `map::ascii` and append the
  legend line the CLI prints.
- A parsed `~ . .` style 3-ring map (or `Stage::flat(4)`) has the base's `rng` and `next_id`,
  no clans and no markets.
- `marks`, `hidden` and `food` set what they say on the tiles they name; `mark('A')` is the
  tile that `at`-style lookups give.
- `civs` + `lead` + `army` place two civs, lead the first and put its camp on a tile; an
  `order` that is legal moves the camp and an illegal one returns its `Err` text; `turn(3)`
  ends three turns; `until` returns `Some(k)` and `None`.
- The retro cache: two calls for the same map return worlds with the same digest, and the
  pre-game runs once (a counter behind a `#[cfg(test)]` accessor, or timing: the second call
  is under a tenth of the first's).
- `StageSpec` round trips through `serde_json`, and `from_spec` of a spec builds the same
  world (digest) as the builders that say the same.

## Done

`cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass; no digest
moved (`cli/tests/save.rs` pins untouched). `engine/CLAUDE.md` gets one row for `stage.rs` and
`maptext.rs` (it must stay under 8 KB). Write what the API ended up as, in two lines, in
your summary: build-2 to build-4 depend on it.
