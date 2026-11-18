# #340 build-1 (ford): `data/` as a folder, and the CLAUDE.md files

Read `relay/specs/340/spec.md`. This step is a pure move: no number, no rule, no name changes.
Builds 2 and 3 edit `data/` after you.

## 1. `core/src/data.rs` becomes `core/src/data/`

`data.rs` is 1,137 lines. Make `core/src/data/mod.rs` hold `DAY`, the module doc and
`pub use` of every file below (`pub use self::goods::*;` …), so every `crate::data::X`,
`engine::data::X` and the `pub use data::{…}` in `lib.rs` build unchanged. One file a system,
each item moved whole with its doc and its `note`:

| File | What moves there |
|---|---|
| `goods.rs` | `Use`, `Good`, `GOOD`, `N`, `GOODS`, `FOODS`, the good indices, `SOURCE`, `KEEPING`, `ARMS` |
| `map.rs` | `Terrain`, `TERRAIN` and its indices, `SIZES`, `SMALL_TILES`, `size_radius`, `MAP`, `FEATURES`, `RIVER`, `scaled`, `Food`, `FOOD`, `FERTILITY` |
| `land.rs` | `TRAIL`, `DEPLETION`, `TENDING` |
| `clans.rs` | `CARRY`, `MARKET`, `PREGAME`, `CLAN`, `FIELD`, `TOWN` |
| `civs.rs` | `CIVS`, `CIV`, `LEANING`, `CLAIM`, `SETTLE`, `PEOPLE_LEFT_MIN` |
| `units.rs` | `SCOUT`, `PLAN`, `UNIT`, `Fighter`, `FIGHTERS`, `FIGHT` |
| `game.rs` | `Claims`, `Config` and its `Default`, `END`, `NOTES`, `JUDGE` |

Keep the items in each file in the order they stand in `data.rs`. Each file starts with a one-line
`//!` saying what it holds. If a file passes 400 lines, split it along the table's line and say so
in your summary. A `use super::DAY;` (or `use super::*`) where a file needs another's item.

Done: `git diff --stat` shows `data.rs` deleted and the folder added with about the same line
count; `cargo build` and `cargo test -p engine` pass; `relay/specs/340/digests.sh` (from `engine/`, after
`cargo build --release`) is byte for byte `digests-main.txt`.

## 2. CLAUDE.md, the root

"Every number lives in `engine/core/src/data.rs`" becomes `engine/core/src/data/`, a file per
system. Nothing else changes there.

## 3. `engine/CLAUDE.md`: the layout a line a folder

It is 8,108 of 8,192 bytes, and the layout has a line per file or two. Rewrite the Layout table
to one row a folder: `core/src/` (the world and its systems: name the main files in a phrase
each, in the order a turn reads them), `core/src/data/`, `core/src/units/`, `core/stage/`,
`core/tests/`, `cli/src/`, `web/src/`, `www/`, `www/map/`, `www/screens/`, `www/play/`. A row
names what a folder holds and the few files a newcomer needs first; it is not a file list.
Keep every other section as it is, except: "Every rule's number and `Config`" points at
`data/`. Aim for under 6.5 KB so later pieces have room; `check.sh` holds 8 KB.

Run `engine/check.sh` once at the end. Commit as `#340 build-1: data as a folder`.
