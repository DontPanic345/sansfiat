# #152 build-5: the limit per size

`END.limit` stays at the starting numbers: **small 200, medium 250, large 300** (turns after the
pick). Measured on this branch (all three easels built); the reason is in the `note` beside `END.limit`
in `data.rs`.

## In time to matter

Each retro map, every civ on the AI, `Config.limit` 600, read from `World.standing` (a throwaway
example, `core/examples/settle.rs`, not committed): the leader by the judge's rule (the lead passes
only to a civ more than `JUDGE.lead_margin` ahead), at each turn past the pick.

| Map | Last lead change | Leader at the limit (200 / 250 / 300) | Leader at +600 | Matches | Leaders at +150 / 200 / 250 / 300 / 350 / 600 |
|---|---|---|---|---|---|
| small 2 | +0 | 474 | 474 | yes | 474 all |
| small 3 | +0 | 269 | 269 | yes | 269 all |
| small 6 | +17 | 353 | 353 | yes | 353 all |
| small 9 | +348 | 486 | 487 | **no** | 487 / 486 / 486 / 487 / 487 / 487 |
| medium 8 | +0 | 932 | 932 | yes | 932 all |
| medium 9 | +73 | 877 | 877 | yes | 877 all |
| large 8 | +5 | 2710 | 2710 | yes | 2710 all |

The leader at the limit is the leader at +600 on **six of seven** maps. Small 9 is the late map: its top two
(Standing .149 and .133 at +200, .127 and .136 at +300) swap fourteen times and settle at +348; it
matches at +150 and from +300, not at +200 or +250, which is a coin flip, not a limit to chase. The
`review-probe.md` read medium 8 as a map whose third civ overtakes between +300 and +400; on this
branch medium 8 is settled from the pick, so the late maps are one, not two (the probe ran on main with
the start applied as a throwaway edit; this run is the built branch). The AI's lead is settled by +73 on six maps, so no limit in reach binds the AI's
world from below except small 9, and none could make that one settle before +348.

## Room for conquest

The `conquest` story (`stories.md`), each civ led, fight on:

| Map | First army on a rival capital | First capital fell | Farthest army (steps, at turn) | Last march |
|---|---|---|---|---|
| small 2 | +13, +21 | +76 | 23 steps, +77 | waves every 10 to 13 turns to the limit |
| small 3 | +20 | +51 | 24 steps, +54 | the same |
| small 6 | +12 | +14 | 23 steps, +28 | the same |
| small 9 | never (one tile short, at 13 and 14 steps) | never | 14 steps, +66 | the same |
| medium 8 | never | never | 25 steps, +74 | the same |
| medium 9 | never | never | 16 steps, +87 | the same |
| large 8 | never | never | 35 steps, +24 to +37 (capitals 37 to 39 steps) | the same |

The latest turn any story's army first got where it got: a fall at **+76** on small, the farthest at
**+87** on medium, and +37 on large. The limits are 2.6, 2.9 and 8 times those. The story's waves
don't stop: each is raised as the store refills and marches again every 10 to 13 turns, so "the latest turn
the army is still marching" is the limit itself; what the limit bounds is how many waves a game holds, and
`stories.md` shows that more waves don't move the end: the second capital was never taken in any of them. A limit far past this drags a game
the AI settled by +73.

Why not shorter: +150 would give small 9's match too (487 at +150), but it is a coin flip there (the
leader is 486 at +200 and +250) and it leaves small a limit under twice its latest fall. Why not longer: +300
on small would buy the same coin flip with 100 more turns of every small game.

## Wall time

`sansfiat sweep --judge` at these limits (every run played to its end and at least to +200):
**34.3 s**, against main's 33 s (build-3 measured 33.8 s for the same binary before the limits were
read; the run is 7 maps by 5 runs, in parallel threads). 600 turns past the pick on all seven maps
took 19 s in the probe. Wall time does not bind the limit.

## The sweep at these limits

```
total end: games ended before the limit 0 of 7 (no fight 0 of 7), turns to the end 228.6 (228.6), capitals fallen 0 (0)
total armies: camps kept 21 of 21, warriors starved 0, camps lost 0, people vs control 0.990
total health: pre-games by centres 7 of 7, panics 0, digest mismatches 0
```

Per map: limit +200 on the four small maps, +250 on both medium and +300 on large 8, each ending at the
limit with no capital fallen, with and without fight. Winners at the limit: small 2 Gold, small 3 Gold,
small 6 Red, small 9 Blue, medium 8 Blue, medium 9 Red, large 8 Gold. (The AI plays no army, so no
AI-only game ends early; the judge reads the end on every run.)
