# #156 build-1: homes, claims and shared field ground — the comparison

Branch `relay/156` against its base (`dc26b06`, main's engine, identical to `relay/156`'s engine before this
step). Every number here comes from the binaries built from those two trees in this session.

## 1. The game is main's: `sweep --judge --json`

`sansfiat sweep --judge --json` on the branch and on the base. Compared by a script over the two reports: each map's
`standing`, `doubt`, `decisions`, `motion`, `land`, `economy`, `supply`, `fields` and `health` blocks, and each
map's pick turn. All seven maps: **every block equal, every pick equal, no panics on either side.**

| Map | Blocks equal (9 of 9) | Pick equal |
|---|---|---|
| small 2 | yes | yes |
| small 3 | yes | yes |
| small 6 | yes | yes |
| small 9 | yes | yes |
| medium 8 | yes | yes |
| medium 9 | yes | yes |
| large 8 | yes | yes |

The first branch run diverged on every map. The cause was `fields::room` reading `Tile.field` less the clan's own
days: a tile whose holder had let it go (move runs before harvest) kept its last turn's days, and the new holder
was given less ground than main gives it. `room` now counts only the other clans that still hold the tile in their
territory (`fields.rs`, `room`); the second run is the one above.

## 2. The digests move only because the save holds two new fields

The save's digest changes, as it must: `Clan.sown` and `Config.towns` are saved. To show nothing else moved, the
state digest (`digest --state`) was recomputed from a scratch copy of the branch in which the JSON's `towns` key
(in `config`) and each clan's `sown` are removed before hashing. The scratch copy is not committed.

| Run | Base | Branch, with `sown` and `towns` removed |
|---|---|---|
| `digest --seed 2 --size small --turns 279 --lead --state` | `a5526ae59236d6ff` | `a5526ae59236d6ff` |
| `digest --seed 3 --size small --turns 308 --state` | `ecac450c3e32eef2` | `ecac450c3e32eef2` |
| `digest --seed 8 --size large --turns 400 --state` | `ab8a5921b18e3ef1` | `ab8a5921b18e3ef1` |

All three equal. Build-1's own state digest (unstripped, `--state`) is `3990e3f328e95ff0` for the led game.

## 3. Pins moved

`SAVE_VERSION` 16 → 17. The branch's full digests, which moved because the save holds the new fields:

| Pin (`cli/tests/save.rs`) | Base | Branch |
|---|---|---|
| seed 1, turn 290 | `51330d17c09fea27` | `3a0454c3719f8ca7` |
| seed 2, turn 279 | `3dc731fb44207823` | `a6fe32f24e76ed60` |
| seed 3, turn 308 | `ad62a6f1d9f75274` | `f75b3464fa882a91` |
| seed 2, led, turn 279 | `60ba7faa9e2f09ce` | `b2990ed9baaef245` |
| led state digest | `a5526ae59236d6ff` | `3990e3f328e95ff0` |

The page check (`www/check.mjs`) pins the save version too: 16 → 17.

## 4. The bench

`sansfiat bench --size large 8 --after 50`, base and branch interleaved in one session, two runs each, nothing
else running:

| Run | ms a turn past the pick |
|---|---|
| base, run 1 | 64.80 |
| branch, run 1 | 76.26 |
| base, run 2 | 64.71 |
| branch, run 2 | 76.57 |

Branch is **1.18 times the base** (about 18% slower). The step adds no bound, and build-2 is where the scale work
goes (its brief sets 0.35 times). The likely cost is `fields::room`, which now scans every clan's territory for each
field tile it is asked about (the planning calls make many such asks), and `sites` building the home index. Both
are plain scans; build-2's market and home indices are the fix, and the bench here is the number it starts from.

## 5. Checks

- `cargo fmt --check`: clean. `cargo clippy --all-targets -- -D warnings`: clean.
- `cargo test --no-fail-fast`: every test binary passes after the pin updates (one pin, the led state digest,
  failed on the first run and is now `3990e3f328e95ff0`).
- `cargo test --test towns` (new, 9 tests): all pass.
- `./cross-build.sh` (native and wasm agree after every turn): `1 16 400`, `7 16 400`, `4 20 400`, `3 23 150` and
  the led `2 16 400`: all digests agree.
- `node www/check.mjs`: passes (after the version pin above).
- `node www/play/all.mjs`: every touch point passes.
