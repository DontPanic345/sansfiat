# #341 A town where you look

**Question:** motion. Do more towns form on the retro maps, a capital among them on a small map,
and how does trade per person in a town compare with trade outside one? Three scores, read in
`sweep --judge` beside **main's sweep** as the control (`measure.md`, `tables.txt`):

| Score | How it is read | Main (8d1b4ed) |
|---|---|---|
| Towns at +200 | `towns.total.towns_200` summed over the seven maps, and the maps with one | 10 on 4 maps (small 9, medium 8, medium 9, large 8) |
| Capital towns at +200 | capitals whose tile reads as a town at +200 (build-1 adds the count), over the seven and on the four small maps | 5 of 21; 1 of 12 on small maps (small 9) |
| Trade per person, town against outside | food traded per person a turn after +100 by town clans, and by every clan outside a town (build-1 adds `outside`) | 0.0427 against 0.0803 |

Beside them, as health: starving (town clans, outside, the towns-off control), the armies
(`warriors_starved`, camps kept and lost), Standing and doubt, all from `judge --compare
main.json branch.json`. `towns.py` here prints the per-map table from any `--json` report.

The issue's "13 on 4 maps, none on small 3, small 9 or medium 8" is from before M5. Main now has
10 towns on 4 maps and none on small 2, 3 or 6, and small 9's capital is already a town at +200.
The control is main as it stands.

## What the spec found (`measure.md`)

- **A capital's market already counts** for the town rule: it is a market in `World.markets`,
  which `towns()` reads. Five of main's ten towns stand on capitals.
- **Farmers by a capital don't move in because walking less buys them nothing.** A farmer one
  tile from its festival frees 2 days by moving onto it, but its fields are capped by their ground
  and its gathering by the food on its tiles, so `could_reap` is the same to four places, and a tie
  loses to `TOWN.margin`. The design's reason to join a town ("the market is at the door") never
  reaches the home step.
- **The door step settles that** (`door.diff`, rule A): a step onto the clan's festival's tile is
  taken when it feeds the clan at least as well and its festival walk from there is less. The
  seven maps: towns 10 → 18, maps with one 4 → 6, and every test of towns, fields and clans
  passes. It adds one capital town (large 8's 2689) and none on a small map.
- **Capital towns on small maps don't hold.** On small 6 under A, three farmers live on civ 263's
  capital at +30, and one by +200. The civ's own clan was gone by +20: `civs::repair` passes the
  civ to its biggest clan wherever it lives and pins it there. Passing it to a clan on the capital
  first (C) changes nothing at +200 on its own. Small 3 has no field ground within two tiles of any
  capital and no farmer within four.
- **Town clans trade less per person than clans outside**, on main and under A (0.048 against
  0.101 under A). A clan on its festival's tile still brings at most `MARKET.carry` a person.
  Lifting that cap at the door (B) makes town clans trade five times as much, but towns fall from
  18 to 11. Not taken: it's a trade rule the owner hasn't agreed to, and it works against the
  piece's motion. It goes to the owner as an idea.

## What the piece delivers

1. **The door step** (build-1): rule A in `homes::step`, its reason a line in `TOWN.note`.
   `SAVE_VERSION` 27.
2. **The town rule's capital, proved** (build-1): a test that a capital with two more clans living
   on its tile reads as a town on the capital's market, and that the feed tells it.
3. **The judge's towns group** (build-1): a capital-town count at +100 and +200, an `outside` group
   (every clan not living in a town), and the sweep's towns lines print both beside the farmers
   outside.
4. **The page's town mark on a capital** (build-1): the capital's house covers the mark today (the
   house is at least 2.8 world units wide, the mark 1.1 wide at 0.7 above the tile's centre, and
   capitals draw after towns). The mark shows on a capital as on any town tile, and
   `www/play/towns.mjs` checks it with the capitals layer on.
5. **Capitals that become towns, and towns that hold** (build-2, the canvas): the home rules tuned
   on the retro maps so that clans farming near a capital come to live there and stay.

## Shape: Easel and Canvas

As the issue says. The easel (build-1) is settled to the line: rule A was measured here, and
the judge's counts and the page's mark are exact. What is left is where towns form and whether
they hold, and only the maps answer that: the canvas (build-2). The canvas may change rule A.

## Settled here, by running

- Main's sweep and A's, A + B's and A + C's (`tables.txt`).
- Rule A's numbers: 18 towns at +200 on 6 maps. build-1's sweep must give them exactly, since
  it builds the same rule.
- B (the whole basket at the door) is not taken, and C (repair to a capital resident) does
  nothing on its own at +200.

## Unsettled: build-2

Whether a capital becomes a town at +200 on more small maps than main's one (small 9), and holds.
The canvas aims for **a capital town at +200 on at least two of the four small maps**, with towns
over the seven at least main's 10 on at least 4 maps, health no worse than main's past the
`--floor` noise, and the trade-per-person rows reported. If no rule within the levers below gets
there, the canvas's answer is why, with the maps to show it. That is an answer to a question about
motion, not a failure.

Its levers are the home rules: `homes::step` (the door step, a town clan's leaving, the step
toward a festival from two tiles out), the civ's own clan (`civs::repair`, the pin), and
`TOWN`'s numbers. Not its levers: trade (`MARKET`, `markets::trade`), field ground, the town
rule's three homes, and any rule that pulls a clan to a capital because it is a capital. The design
says a town has no rule of its own.

## Decisions I made

- **Design says "its festival counts"** (the piece's goal), and it already does: main's capital
  markets are festivals for `towns()`, and five capitals are towns. The piece proves it with a
  test and changes no rule for it.
- **The door step:** a clan steps onto its festival's tile when that feeds it as well and walks it
  less, not only when it feeds it better by `TOWN.margin`. farming.md says a farming clan near a
  festival drifts to it because carrying costs it; the walk is that cost, and labour the clan can't
  spend isn't counted as food. It applies to any home step that lands on the festival's tile.
- **"Outside" is every clan not living in a town**, after +100, beside the judge's farmers outside
  towns, which stays.
- **A capital town is a capital whose tile reads as a town**, under the same three-home rule.
- **The whole basket at the door (B) is not taken**: an idea for the owner.
- **The control is main's sweep as it stands**, not the issue's numbers.
- The page marks a capital's town with the same mark as any town, moved only as far as it takes to
  show above the house.

## Ideas taken

None. The open ideas are about signposts, scouts, gifts, alerts and the claim. "The gift reaches no
one on the small maps" is near this piece: clans living on a capital trade at its market. The
review may report whether any town clan at a capital receives a gift, but the gift rule isn't
changed here.

## For the review

- `sansfiat sweep --judge --json` on the branch: `python3 -I relay/specs/341/towns.py main.json
  branch.json` (main's report: `git worktree add /tmp/main main`, or rerun it), and `judge --compare`
  for health. Give the three scores beside main in the PR, with `--floor` noise for towns at +200
  if build-2 reports it.
- Which capitals are towns at +200, map by map, from the sweep's towns lines.
- `node engine/www/play/towns.mjs`: the town mark shows on the capital with the capitals layer on.
- On the page or in `play`, look at one capital town on a small map at +200: the mark, the
  tooltip, the tile panel's clans, and the feed's line when it formed.
- `SAVE_VERSION` 27, and the pins in `cli/tests/save.rs` match.
