# #56 Armies hold ground: sweep and tuning

`cargo run -p sansfiat -- sweep --after 200 --maps retro [--no-zone]`: the seven retro maps (21 AI
civs), all on the AI, 200 turns after the pick. AI camps post 3 steps out (`UNIT.post`) with
3 warriors, so the AI's zone is `min(UNIT.zone, people − 1) = 2` tiles of its line, 3 with the camp's
own. **held** counts tile-turns held beyond the camp's own tile (at most 2 × 200 = 400 a civ).
**cut by army** stays 0 because AIs never paint to cut. It's there to measure what guarding costs;
`www/play-hold.mjs` shows what a cut does.

## Totals

| run | `UNIT.guarding` | `UNIT.inside` | kept first camp | camps' hungry turns | warriors starved | tile-turns held | cut by army | clans joined | clans' people |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `--no-zone` (control) | – | – | 21/21 | 0 | 0 | 0 | 0 | 183 | 1394.8 |
| zone | 1.0 | 0.25 | 21/21 | 0 | 0 | 4143 | 0 | 173 | 1431.6 |
| zone | 1.5 | 0.25 | 20/21 | 4 | 1 | 6722 | 0 | 218 | 1393.6 |
| **zone (chosen)** | **1.0** | **0.2** | **21/21** | **0** | **0** | **7732** | **0** | 173 | 1431.6 |

Per civ, at the chosen numbers: 18 of 21 held 353–397 of their 400 (both line tiles nearly every
turn). The few lower ones (199–279) are camps whose second line tile is forest or hills, whose walk
eats its share. Most packs end at 7.5 of 15 against the control's 12. Guarding takes a third of a
posted camp's labour, so the line fills the pack more slowly, but the camp still eats every turn.

- At **guarding 1.0, inside 0.25** a 3-warrior camp's share of 1.0 a tile pays the 0.5 walk to the
  first line tile but not the 1.0 walk to the second, so it held one tile (~199 a civ).
- At **guarding 1.5** both tiles are held, but a camp 3 steps out needs 6 of its 10.5 labour on the
  line. Setting 4.5 aside left nothing to fill the pack. Packs drained to 0 in 10 of 21 civs, and
  one camp starved a warrior. That's more starving than the control, which the brief rules out.
- **Chosen: guarding 1.0 a warrior, inside 0.2, floor `UNIT.guard` 0.1, AI zone `UNIT.zone` 2, cap
  `people` tiles with the camp's own.** A 3-warrior camp holds both plains line tiles (share 1.0,
  walks 0.4 and 0.8), and the zoned run starves no more than the control: 0 hungry turns either way.
  Clans' people and joins are within the noise between runs.

## The scripted cut (`node www/play-hold.mjs`, small seed 2)

Red, led, raises 10 warriors at its capital (turn ~280). It marches to the last tile of its own land
on the way to Blue's camp at its post (120), refills its pack there, then marches onto Blue's tile
with auto-forage off. It paints Blue's tile's walkable neighbours (4 here; the rest are water) as its
zone. Standing on Blue's tile, Red's 10 warriors × 1.0 over 5 tiles beat Blue's 3 × 1.0 over 3:
Red takes Blue's line tile and Blue's own tile. Blue's line has to leave through a neighbour, and
every neighbour is Red's, so the feed says "Blue's camp's line is cut by Red's army". Blue eats its
pack and forages its tile, and three turns later "Blue's camp at (-10, -3) goes hungry". Red's camp
is cut off too (its line runs through Blue's land) and starves a warrior a turn. Holding someone
else's ground costs you. Screens in `screens/`: `hold-rival.png` (Blue's camp and zone before),
`hold-painted.png`, `hold-held.png`, `hold-cut.png` (Blue's camp selected, its line drawn to the
tile where Red's ground stops it, marked with a cross; the feed).

## Review, after merging main (#75 scouts)

The merge changed the maps' costs, so some AI posts' lines no longer feed a camp at all. The
review made the AI zone only when its line (read without held ground) still carries a turn's
food on the labour guarding leaves, and undeploy otherwise. `sweep --after 200 --maps retro`:

| | kept first camp | hungry turns | warriors starved | tile-turns held | cut by army |
|---|---|---|---|---|---|
| zone | 15 of 21 | 39 | 19 | 3397 | 0 |
| `--no-zone` | 15 of 21 | 39 | 19 | 0 | 0 |

`play-hold.mjs` now runs on small seed 7 (seed 2's big camp starves on the long march to the
rival's post after the merge) and stages at the capital.
