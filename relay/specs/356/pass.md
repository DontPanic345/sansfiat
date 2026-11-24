# #356 build-1: the control, and what Carry passes

## The control (`Config.exchange` off)

`Config.exchange` is a field of `Config` (default on; a save without it loads on). It is written only
when off, and `stats::digest` / `state_digest` leave `"exchange":false` out, so the off world digests as
the world before exchange did. (Without that, the one extra key in the config would move all 21.)

How it was run: `digests-off.sh` (this folder) is `relay/specs/340/digests.sh` with `--no-exchange` added to
each `digest` call (a flag `sansfiat digest` now takes). From `engine/`, after `cargo build --release`:

    ../relay/specs/356/digests-off.sh | diff - ../relay/specs/340/digests-main.txt     # nothing: all 21 equal

Run at the final commit: all 21 equal. Off: no unit trades, no good passes (no second route), no lot is written.

## Carry

A clan holding wares its own festival's last clearing didn't take (`own.goods` lacks the good) finds, in the
festivals its home reaches (`festival_costs`, the search `go` already made), the nearest other festival that
last traded one of them. It walks a second route there (`routes[1]`, `walk_cost` on the wares' weight at
`carriers`, paid from its days like any route) when the wares' worth at that festival's last prices
(`food_price`, in labour) covers `PASS.cover` of the walk, and it was fully fed last turn. At `trade`, after
every first clearing, each such clan clears at its second festival with the wares its first one left
(`exchange::carriers`, `markets::clear_at`); the festival's own clans, its civ's posting (not its gifts) and
the lots go as in any clearing.

### What running found

The wares a clan holds unsold are a few units of salt (it digs only to `SOURCE.hold` = 0.5 a person) and a
unit of salt clears at about a third of a food. At the walk's full cost (`cover` 1) the worth is a
hundredth of the cost: 1-3 walks a map, nothing moves. Three settings on the sweep (7 maps, one run each):

| setting | wares sold at a second festival, a map (salt / obsidian) | starving clan-turns (main 6,194) | starving share (main 0.0328) |
|---|---|---|---|
| `cover` 0.01, no fed gate | 2-274 / 0-9 (large 8: 274 salt) | 6,875 (floor 165: past it) | 0.0338 (floor 0.0014) |
| `cover` 0.002, fed gate | 0-21 / 0-11 | 7,099 | 0.0343 |
| **`cover` 0.01, fed gate** | **0-22 / 0-11** | **6,201** | **0.0327** |

A starving clan leaves it out. The kept setting is the last row. A few hundred units a map was reached only
without the gate, at a cost in health past the floor; with it the flow is tens of units a map.

### The kept setting, `sweep-build1.json` (this folder; `sansfiat sweep --judge --json` at the commit)

Carry runs only once civs play (`w.phase.playing()`): in the pre-game it moved the retro picks, and with them
`core/tests/cases.rs`'s scout errands on medium 8 (an owner's invariant, green on main). The three settings in the table above were
measured before that gate; the numbers below are the commit's.

| | main (`sweep-main.json`) | build-1 |
|---|---|---|
| obsidian `mean_steps_200` / `max_steps_200` | 5.35 / 18 | 5.35 / 18 |
| salt `mean_steps_200` / `max_steps_200` | 3.52 / 11 | 3.68 / 11 |
| festival-turns where salt trades | 9,346 | 10,347 |
| festival-turns where obsidian trades | 524 | 503 |

Wares carried a map (`carry_flow` in `core/tests/exchange.rs`, run `--ignored --nocapture`; clan-turns walking
a second route, clan-turns that sold there, salt and obsidian sold, festivals where a carrier sold):

    small 2:  walked  92, sold 78, salt  35, obsidian 8, 4 festivals
    small 3:  walked   8, sold  6, salt   0, obsidian 7, 3
    small 6:  walked   9, sold  8, salt   2, obsidian 1, 2
    small 9:  walked  30, sold  8, salt   0, obsidian 7, 1
    medium 8: walked  30, sold 26, salt  18, obsidian 6, 7
    medium 9: walked  18, sold 17, salt  47, obsidian 3, 5
    large 8:  walked 128, sold 39, salt 115, obsidian 0, 13

Tens to a hundred or so units a map: under the probe's "few hundred", which the gate cost (above). Obsidian's
steps from source don't move (it sells only at a capital with room, `probe.md`); salt's mean rises 0.16, past
the noise of its mean (floor 0.065), its furthest holds at 11.

Health (`judge --compare sweep-main.json sweep-build1.json`; floors from the one `--floor` run, of the first
row's code: the sweep's noise, which the gate doesn't change much):

- starving share 0.0328 -> 0.0319 (floor 0.0014), worst map's starving ratio 1.397 -> 1.057.
- starving clan-turns 6,194 -> 6,756 (floor 165, past it) over 277,891 -> 280,296 clan-turns (floor 1,271):
  2.23% -> 2.41% of clan-turns. The sweep's own controls, run with exchange on, move more (`control_all`
  5,509 -> 6,925), so a change this size is what any change in the second routes does to a trajectory. The
  review should weigh this line: it is the one health number that isn't within the floor.
- goods people over control 1.0127 -> 1.0103 (floor 0.026); goods starving 0.0265 -> 0.0281 (floor 0.0103).
- armies people over control 0.9991 -> 0.9833 (floor 0.032); settle people over control 0.9892 -> 0.9738.

Not measured here: unit trades (no unit carries wares until build-2's Fill) and the army-turns number.
