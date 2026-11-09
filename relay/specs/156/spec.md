# #156 Towns: spec

## What it delivers

Many clans can live on one tile. A home is no longer a claim. A clan's home tile may be held by another clan
or by nobody, and many clans may share it, while the tiles they gather stay exclusive. A tile's field ground is
shared among the clans that sow it. A farmer's home can step onto its festival's tile when living there feeds
it better, because its surplus no longer walks to market. A tile with several homes and a festival reads as a
town: the page draws its hearths together and the feed says when one forms. The engine carries the crowd: the
festival search runs once per home tile, clans are indexed by market once a turn, the AI's paths are cached,
and a large map past the pick runs under a bound.

1. **Homes and claims** (build-1). A clan's territory is its home tile and the tiles it claims
   (`Tile.claim == Some(id)`). Its home tile is in `territory` as now, but `Tile.claim` there may be another
   clan or nobody. A clan **gathers only on the tiles it claims**. Its **field job works every tile of its
   territory with ground, home included**, whoever claims the home. A home tile nobody claims is claimed by
   the first clan in the move step's order that lives there, as an unheld territory tile is today. Claims are
   written only on tiles that are unheld or already the clan's. `free` clears only the clan's own claims, as
   now. Daughters and clans moving on still settle unheld sites (`sites`).
2. **The home index** (build-1). `clans::homes(w) -> BTreeMap<usize, Vec<u32>>`: each home tile and the clans
   living on it, in id order. It replaces every place that finds a home through the claim:
   `markets::in_reach`, `stats::trails`, the view. It is derived, never saved.
3. **Shared field ground** (build-1). A tile's ground is shared among the clans sowing it, in proportion to
   the days each sows. A clan plans on the **room** a tile leaves it: its ground less the days the other clans
   sowed on it last turn. `sow` caps each tile at that room. The harvest step reaps every clan first, then
   scales each tile's field days and grain to its ground where the clans together sowed more, each clan by the
   same factor, and then applies the takes. Gathering territories are exclusive, so reaping all clans before
   any take reads the same stock as today's take-as-you-go. `Clan.sown` (`(tile, days)` this turn, sorted) holds
   each clan's own field days. `fields::plots` reads it, and `Tile.field` stays the tile's total.
4. **`Config.towns`** (build-1; `#[serde(default = "yes")]`, on in `Default`). Off is the control: a farmer's
   home never steps onto a tile another clan holds, so no two clans share a home. It acts only in the canvas's
   home rule.
5. **Scale** (build-2, digest-safe against build-1):
   - **The festival search once per home tile.** `markets::go` works out the costs from each home tile to the
     festivals in reach once a turn. The clans living on a tile share the result, and the chosen route's path
     comes from the same search when it is the path `land::path` gives. The search stops as soon as no
     festival left in reach can still be settled, bounded by the cheapest step a tile can cost (`probe.md`: a
     bound of `DAY` a step moved the digest).
   - **Clans indexed by market once a turn.** `trade`, `people`, `merge` and the view read one index of
     market → clans (id order), not a scan of every clan per market.
   - **The AI's paths cached per turn.** `ai::post`'s paths to rival capitals and the farthest clan are cached
     for the turn. The scouts' foraging search (`scout::round_plan`) is bounded by the labour it has:
     `scout-bound.diff`, which took scouts from 31.5 to 0.3 ms a turn on large 8 with the digest unchanged.
6. **The town** (build-3). `TOWN.homes` (3) in `data.rs`. A tile **reads as a town** when `TOWN.homes` or more
   clans live on it and a festival stands on it. `core/src/towns.rs`: `towns(w) -> Vec<Town { tile, market,
   clans }>`, and the markets step's check, which keeps `Market.town` and emits `TownFormed` when a festival's
   tile first reads as a town ("A town forms at (3, −2): 4 clans live by the festival"). When it stops, a note
   goes to the market. A town has no rule of its own: nothing else reads `Market.town`.
7. **The judge** (build-3): a `towns` group (`core/src/judge_towns.rs`) and the no-towns control as a seventh
   run a map (`cli/src/sweep_towns.rs`), with every bound in the table below printed as pass or miss.
8. **The page and the text view** (build-3). Clans sharing a home are drawn as a cluster of small hearths on
   the tile rather than one circle on another. A town tile carries a town mark, and its tooltip reads
   "Town: 5 clans at the festival". Selecting a tile with homes lists the clans living there. The feed shows
   `TownFormed`. The text view lists towns. `www/play/towns.mjs` checks them on a staged town.
9. **The home rule** (build-4, the canvas). A farmer's home may step one tile a turn toward its festival's tile,
   onto any passable land, whoever holds it, when living there feeds it better. Then the canvas tunes whatever
   makes the town's clans feed.
10. **The design edits** (build-3): `clans.md` Clans, and `architecture.md`'s Tiles shared and Scale rows
    (the text is in build-3's brief).

## Shape: Easel and Canvas

The piece's `Shape:` line suggests Easel and Canvas, and I take it. The spec step sketched the canvas's rule
on main (`probe.md`, `probe.diff`, about 40 lines) and ran it on the retro maps. Towns form on 5 of 7 maps by
+200, people and land move half as much again, and aligned Standing rises 0.318 → 0.341. But starving runs at
3.97 times the no-fields control against main's 1.22, settled farmers hop home 7.8 times per 100 clan-turns
against 0.03, and the large turn costs 37% more. So the rule is easy and its consequences are the work:
the hard part is the town that feeds, judged only on the maps. That is a canvas. Under it go the settled
pieces, built so the painter has a fair ground: homes that don't claim and gather nothing they don't hold
(the sketch gathered shared home tiles twice), ground shared by area, a search that scales, and a judge that
prints every bound.

- **build-1** (easel): homes and claims, the home index, shared field ground, `Config.towns`, tests on a stage.
  With no shared home it behaves exactly as main.
- **build-2** (easel): scale. The festival search per home tile, the market index, the AI's path cache and the
  scouts' bound, each proved digest-equal to build-1, and the bench.
- **build-3** (easel): the town (`TOWN.homes`, `towns.rs`, `TownFormed`), the judge's `towns` group and the
  no-towns control, the page, the text view, `towns.mjs`, and the design edits. Proved on a staged town, since
  no home is shared until build-4.
- **build-4** (canvas, twinned): the home rule, then the tuning until towns form, feed, trade more and walk
  less on the retro maps against the no-towns control, with the large turn under its bound.

## The question and how it is measured

**The question:** motion and health. On the retro maps, towns form where farmers and a festival meet. A town's
clans trade more and walk less. A large map runs under a bound past the pick.

**The control.** The no-towns control is the same map with `Config.towns` off. It is today's game: no home is
shared. The pick is untouched, since farmers exist only after it, so the control differs from the run only
from the pick. A move counts when it is past `sweep --judge --floor`'s floor.

A **town** is a tile with `TOWN.homes` (3) or more homes and a festival on it. A **town clan** lives in a town.
A **farmer** is `fields::farmer`. The **festival walk** is a clan's route cost over its days (`routes` cost ÷
`people × CLAN.work`). **Brought** is the food a clan carries to its festival a turn (`basket` capped at
`MARKET.carry × people`, as `trade` caps it), per person.

| Score | Measure | Control | Sketch (`probe.md`) |
|---|---|---|---|
| Motion | **Towns form**: a town at +200 on at least 5 of 7 maps, and at least 10 over the seven. | No-towns control: none | 5 of 7; 18 |
| Motion | **Where farmers and a festival meet**: at +200, at least three quarters of town clans are farmers, and every town's clans work fields. | | not measured |
| Motion | **They walk less**: over the clan-turns after +100, town clans' festival walk is under half the walk of farmers outside towns. | Farmers outside towns, same runs | 0 against 0.16–0.23 |
| Motion | **They trade more**: over the same clan-turns, town clans bring at least 1.1 times the food per person to their festival that farmers outside towns do. Brought per person over all clans is no lower than the control's, less the floor. | Farmers outside towns; no-towns control | not measured |
| Motion | People, land and clans change hands no less than in the control, less the floor. | No-towns control | 1420, 2353, 285 against 953, 1150, 177 |
| Standing | Aligned Standing at +200 no lower than the control's, less the floor. Reported per civ. Not a goal. | No-towns control | 0.341 against 0.318 |
| Health | **Towns feed**: starving clan-turns over the seven maps at most 1.25 times the control's share, and on no map over 1.6 times, read on the ratio less its floor. Town clans starve at a rate no higher than farmers outside towns. | No-towns control | 3.2 times main's share (3.97 against 1.22, each against no-fields) |
| Health | **Settled farmers stay**: settled farmers' home moves per 100 clan-turns under half the gatherers' rate, as #155 holds it. A farmer's move into a town counts. | Gatherers, same runs | 7.8 against 0.21 |
| Health | The store: the maps where the civ that farms most holds a pack half its turns, and the civs never at a pack, each no worse than the control's by more than 1. | No-towns control | 3 against 5; 4 against 1 |
| Health | **Scale**: `sansfiat bench --size large 8 --after 50` at most **0.45 times main's** ms a turn, measured on the same machine in the same session. Build-2 alone, with no towns, reaches 0.35 times. | Main | main 54.8 ms here; sketch 75.2 |
| Health | Pre-games by centres 7 of 7. Panics 0. Digest mismatches 0. With towns off, every map's `standing`, `motion`, `economy`, `land` and `fields` blocks equal main's. | Main | 7, 0, 0 |
| Doubt | Lead changes and the spread at +200, against the control. Reported, not bound. | No-towns control | swings 3 → 31 |
| | `engine/check.sh` green at every step. `www/play/towns.mjs` passes. | | |

## What I settled

- **A home is not a claim** (the piece's Decided), and the shape it takes: home in `territory` (so walks,
  connectedness and the field job read it as today), claim exclusive, gathering only on one's own claim, fields
  on the home whoever holds it. Decisions 1–3.
- **How the ground is shared**: in proportion to the days sown, planned on last turn's room. Decision 4.
- **When a tile reads as a town** (the designer settles): 3 or more homes and a festival on the tile. Two
  clans side by side is a neighbour, not a town. Three is the least that reads as "many hearths" on the page,
  and the sketch's towns at 3 came out on 5 of 7 maps.
- **The scale bound** (the designer settles): `bench --size large 8 --after 50` at most 0.45 times main's in
  the same session. A ratio, because this runner is 2.4 times slower than the one `architecture.md`'s 23 ms
  came from. 0.45 because the probe found 0.35 within reach with no towns (scouts' bound and a crude search
  exit: 19.4 of 54.8 ms), and towns add clans: the sketch added 15% more clans and 37% more turn. Main is 55 ms
  on this runner, and 0.45 of it is about 25 ms, about 10 ms on `architecture.md`'s machine, which is its
  target. The review writes the measured ms into the Scale row.
- **Whether a staged change keeps every digest** (the designer settles): no, not through build-1. Build-1
  adds saved state (`Clan.sown`, `Config.towns`), so the digest moves and `SAVE_VERSION` bumps. Its proof that
  nothing behaves differently is that every map's scores equal main's. Keeping the digest would need
  rule-read state left out of the save, against the engine's invariants. Build-2 and build-3 keep build-1's
  digest exactly. Build-3's `Market.town` is skipped when false. Build-4 moves it again and bumps.
- **The page's look**: a shared home's clans as small circles in a ring round the tile's centre, each in its
  own colour and starving ring, their size by people but capped smaller than a lone clan's. A town gets a
  placeholder town icon (`icon.town` on the atlas) at the tile's top. On by default with the clans layer.
- **Where the code goes**: `towns.rs` (the town reading and its event), `judge_towns.rs`, `sweep_towns.rs`.
  `judge.rs` (641 lines) splits its score structs into `judge_scores.rs`, and `sweep_judge.rs` (599) its
  totals into `sweep_total.rs`, before either grows.

## Unsettled, and which step holds it

All in build-4, the canvas:

- **The home rule's shape and margin.** The sketch steps a farmer's home toward its festival when the move
  (territory with the new tile, trimmed, and route repriced) beats staying by 0.01 food a person. It runs
  after the field step, and the two pull against each other (7.8 home moves per 100 settled clan-turns). The
  canvas settles how the festival pull and the field pull make one step a turn. Proof: the "settled farmers
  stay" bound.
- **Why town clans starve, and the rule that stops it.** The sketch's starving (3.2 times main's) comes partly
  from gathering shared home tiles twice, which build-1 removes. The canvas measures what's left first, then
  finds who starves (town clans, their neighbours squeezed for land, daughters) and changes the rule that causes
  it, within the piece's Decided. Proof: the "towns feed" bound.
- **Which numbers, if any, move**: `TOWN`'s margin and any new number, `FIELD.farmer` if the town pull needs a
  line, and the territory radius only for a farmer living in a town, if that is what feeds it. `TOWN.homes`
  stays 3: it is the reading, not a rule.
- **The scale bound with towns.** Towns add clans and the home rule adds a search per farmer. The canvas keeps
  the large turn under 0.45 of main's, reusing build-2's per-tile search for the home rule's route pricing.

## What the review runs

- `engine/check.sh`.
- `sansfiat sweep --judge --json --floor` on the branch and on main (a worktree), then
  `sansfiat judge --compare main.json branch.json`. The `towns` block's bounds go in the PR, each pass or miss,
  with the old and new numbers of every score in the table.
- With towns off (`Config::default()`'s `towns` false, locally): `sweep --judge --json` equals main's in every
  map's `standing`, `motion`, `economy`, `land` and `fields` blocks.
- `cargo test --release -p engine --test towns -- --nocapture`.
- `sansfiat bench --size large 8 --after 50` on the branch and on main, two runs each, in the same session.
- `node www/play/towns.mjs`, with a look at its screenshot: the cluster of hearths and the town mark.
- A played game (`sansfiat play --game g.json`, large seed 8, the first civ): 100 turns past the pick. Read
  the feed's "A town forms", the tile's clans, and whether the capital becomes a town.

No new button or order: nobody orders a clan into a town.

## Decisions I made that the piece left open

1. **A clan's home stays in its territory, but the claim on it may be another's.** Walks, the field job,
   `connected` and `best_drop`'s "never drop home" read the territory as today, so the change sits in who
   claims and who gathers, not in every caller of `territory`.
2. **A clan gathers only on tiles it claims.** "Clans' gathering territories stay exclusive": the claimant
   gathers a shared home tile, and the clans living there gather their own tiles round it.
3. **A clan's fields may be on its home tile whoever claims it, and on its own claimed tiles, nowhere else.**
   Homes at the market and fields along the water: the home tile's ground is the town's shared ground.
4. **Field ground is shared in proportion to the days each clan sows**, each clan planning on the room last
   turn left it. Equal shares would hand a small clan ground it can't work. First come first served would make
   the harvest order a rule.
5. **Daughters and clans moving on still settle unheld tiles.** A town grows by farmers stepping home toward
   their festival, the way they move anywhere ("because it feeds them better"). A daughter founding in a town
   would be a second door the design doesn't name.
6. **`Config.towns` is the control**: off, a farmer's home never steps onto a tile another clan holds.
7. **The town is read each turn and remembered only to tell the feed.** `Market.town` is the one bit of state,
   and no rule reads it.
8. **Design says the AI's rival paths are cached per turn.** They cost 0.12 ms a turn on large 8. The AI's
   scouts cost 31.5 of 54.8 ms, in searches over the whole map (`probe.md`). The cache covers both: `post`'s
   paths cached per turn, and the scouts' search bounded by the labour it has, which gives the same plan. The
   scale bound can't be met without it.
9. **The scale bound is a ratio to main in the same session**, 0.45, with the ms written into
   `architecture.md`'s Scale row. There is no `judge.toml`, so the row is the bound's place.
10. **The shape is Easel and Canvas**, as the piece says: the sketch shows the rule is small and its
    consequences are what only the maps judge.
