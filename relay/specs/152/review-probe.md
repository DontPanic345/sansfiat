# #152 spec-review probe: when does an AI game's lead settle?

Run in spec-review on `main` (25e5bb6) with the piece's start applied (`UNIT.start` and
`GUARD.start` 0 in `data.rs`, so the AI raises no army), a throwaway test, not committed. Each
retro map, every civ on the AI, 600 turns past the pick. The leader follows the judge's rule
(`JUDGE.lead_margin` 0.005); "spread last moved" is the last 10-turn sample where the spread
(highest − lowest Standing) moved by more than the margin.

| Map | Pick | Lead changes | Last change | Spread last moved | Leader +200 / +250 / +300 / +600 | Standing at +200 | at +600 |
|---|---|---|---|---|---|---|---|
| small 2 | 237 | 2 | +5 | +380 | 474 / 474 / 474 / 474 | .052/.095/.153 | .049/.117/.143 |
| small 3 | 245 | 0 | — | +250 | 269 all | .109/.039/.152 | .115/.038/.149 |
| small 6 | 318 | 5 | +60 | +270 | 355 all | .092/.095/.124 | .109/.092/.121 |
| small 9 | 240 | 14 | +346 | +600 | 487 / 487 / 486 / 487 | .094/.126/.132 | .058/.123/.166 |
| medium 8 | 273 | 6 | +346 | +370 | 931 / 931 / 932 / 932 | .053/.047/.045 | .054/.086/.045 |
| medium 9 | 431 | 0 | — | +400 | 877 all | .083/.050/.033 | .073/.050/.033 |
| large 8 | 369 | 2 | +5 | 0 | 2710 all | .024/.026/.038 | .027/.028/.037 |

All seven maps, 600 turns each, run in 19 s on parallel threads (release), so wall time does not
bound the limit.

## What it says

- On five maps the AI's lead is settled by +60, so with no army on the AI a watched game is
  decided long before any limit in reach; a limit there is about the player's room, not the
  AI's.
- Two maps never settle by the judge's rule inside +350: small 9's top two are neck and neck
  (.137/.134 at +300) and swap the lead fourteen times; medium 8's third civ overtakes between
  +300 and +400 (.053 to .087). No limit makes "the leader at the limit is the leader the game
  settles on" true on every map, so build-5's constraint is restated as a count over the seven
  maps, with the two late maps reported.
- The starting limits (200/250/300) put the leader at the limit equal to the leader at +600 on
  six of seven maps (medium 8 at 250 is 931, at +600 932).
