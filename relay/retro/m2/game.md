# M2 retro: decisions, ideas and the game (#54, #29, #55, #28, #56, #57)

One angle of the M2 retro: every decision M2's sessions recorded, every idea they raised, the
merged game measured on main (70454be), and the design edits that follow. Everything here is a
proposal; the owner owns `docs/design/`.

Sources: the `<!-- relay -->` blocks on #58, #60–#67, #69–#74, #76–#81, #83–#88, #90–#93,
#95–#98; the crew's PR comments on #59, #68, #82, #75, #89, #94; `relay/specs/<n>/spec.md`
(Settled, Decisions, "Design says" lines); the owner's reply on #28. "Measured" means a release
build of main in the scratchpad, run on the M1 retro's seven maps (small 2, 3, 6, 9; medium 8, 9;
large 8), all civs on the AI: `sansfiat sweep` (pre-game), `sweep --after 200 --maps retro` with
each control and `--lever 6/15`, and a scratch probe (`run --turns pick+N --json`, checkpoints at
+0, +50, +100, +200, +300).

## Verdict

**M2 built every system it set out to, and each passes its own question, but the main game is a
frozen world with nothing to decide.** The pre-game is sound: 7 of 7 maps end by three centres on
their own (turn 215–379), people level off (+1–4% over the last 100 turns). After the pick the
world barely moves: total people change under 3% in 300 turns on every map, and on medium 8 no
civ's numbers move at all from +100 to +300. The AI never cuts a line (0 turns cut by an army in
every sweep), AI scouts never set a route (0 trails built), and joining stops within about 50
turns. The player's levers (gift, keep, camp post, zone, scout route) are each set once; nothing
threatens the player, and there is no way to win or lose. Two pieces regressed after they
merged: camps kept fell from 21 of 21 (#29) to 15 of 21 once scouts and the owner's founding
change landed (#75), and joins under the default gift fell from 183 (#55) to 84.

## 1. Measured: each piece's question on main

| Piece | Question | Measured on main | Answer |
|---|---|---|---|
| #54 | Does the pre-game level off and form three centres? | 7/7 by centres, 0 top-ups; ends small 215/218/266/282, medium 295/379, large 373; people +1 to +4% over the last 100 turns; cover 0.99–1.00; large 29.6 ms a turn | **Yes.** Large is still slow, and levelling is partly the end rule's doing (#66). |
| #29 | Can every civ keep its army 200 turns? | 15 of 21 kept the first camp; 39 hungry turns, 19 warriors starved; 2 camps lost (medium 8 Blue, medium 9 Red); 4 camps sit at 1 warrior for 300 turns (small 3 Gold, small 6 Red and Gold, medium 9 Blue) | **No, since #75.** Every civ that fails is small (4–6 clans, 14–28 people). The owner accepted this on #28 ("a camp is only a cost until the features that use it are built"). |
| #55 | Do clans join civs; are gifts a lever? | Defaults: 84 joins (median per civ 0, max 21); gifts off: 9; no army: 0 (no levy, so no store to give); `--lever 6`: led civ 14.7 vs rivals 4.5; `--lever 15`: 14.9 vs 3.3. Joins come early: the first join is 8–18 turns after the pick, and nearly all are in by +100. small 3 and medium 9 get none. | **Yes, gifts are a lever, but a set-once one** that saturates near gift 6 and does nothing where no unaligned clan walks to the capital. |
| #28 | Do the scouts' trails get used? | AI scouts are given no orders (spec #28 Settled 6), so 0 trails are built in any watched game. 7 of 21 AI scouts starve within 5–8 turns of the pick while sitting at their capital (store empty); one more at +69. | **Never, in an AI game.** In a led game the review (#80) found 5 of 9 cross-country routes starved before they finished. Built trails fade 30 turns after use stops. |
| #56 | Does held ground cut lines? | AI camps hold 1–3 tiles (3397 tile-turns); cut by an army: 0 in every sweep. `--no-zone` gives the same hunger (39/19). Only `play-hold.mjs` shows a cut, on small seed 7. | **Only when the player does it.** The AI never cuts and never answers a cut (#91 idea). |
| #57 | Does the page explain the game? | Not measured here (page). The owner's play notes below are the test. | See the owner's notes. |

**Other numbers from the probe:**

- **Civs are tiny against the world.** At +200, civs hold 1–4% of the world's people on medium
  and large, 4–20% on small. small 3's civs have 4–6 clans each of 100.
- **Stores either sit full or empty.** Ten civs sit at 47–53 food (the cap less spoiling) for
  200 turns, with nothing to spend it on beyond a gift of 2. Seven sit at 0–9.
- **Trails are worked land, not routes.** 72–79% of held tiles carry a trail on every map, and
  only 0–3% of trails are off clan land. 14–36% of clan homes have one, and only where another
  clan's walk crosses it (see owner note 5).
- **Nothing breaks.** No crash or NaN on any map. What starves: scouts, and the camps of small
  civs. What never happens: an army cut, an AI scout route, a lost camp re-raised and kept, a
  clan leaving after the first 100 turns.

Unchecked: the page itself (I did not drive it), and whether the 15/21 drop is all scouts or
partly the founding change (#75 build-1 reported 15/21 before the scout existed: the founding
change alone did it, and scouts held it there).

## 2. What does the player decide each turn?

Leading a civ after the pick, the levers are: gift and keep (set once; the AI default of gift 2
already joins the clans that will join); raise or disband warriors (a cost with no use: nothing
attacks, and the AI never cuts); move the camp (right-click only) and paint a zone (useful only
to cut a rival, which needs a big camp and starves your own camp in rival land, #91/#92 ideas);
a scout route (a two-click order that usually starves the scout, and whose trail fades).

Nothing happens to the player that needs an answer. Rivals never come near, the world's people
don't change, and joins finish early. A turn's decision is "end turn". The one interesting
lever, gifts against rivals for clans, is real (14.7 vs 4.5) but it's a dial set once. It would
become a game if gifts were per clan (#84 idea), if rivals competed for the same clans at the
same time, and if joining counted towards a goal (the vision's *Influence* win).

What's missing for a person to enjoy it at this stage, in order:

1. **A goal.** Influence (share of the world's people in your civ) is measurable today, with the
   sweep's join counts. Without one there's no reason to pull any lever.
2. **Pressure.** An AI that uses its camp: post on the contested festivals, pillage, cut a line.
   Today the army only costs (8% of clans' people at #29; measured: civ people 1033 with the
   army vs 992 without, so now within noise).
3. **A world that keeps moving after the pick.** Good and bad years (out of scope in clans.md)
   or farming (`actors.md`, Later). Today the pre-game's levelling is also the main game's
   stillness.
4. **Units that survive their first errand.** Scouts starve, small civs' camps sit at one
   warrior, and nothing replaces either.

## 3. Decisions recorded across M2, by design file

T = take into `docs/design/`, R = reject (change the code back or differently), A = ask the
owner. Numbers stay in `data.rs` unless the line says otherwise.

### actors.md

| Decision | Source | |
|---|---|---|
| One `walk_cost` = 2 × path × weight; inside walks weighted `CLAN.inside` (0.25), routes 1; units' zone walks 0.2 | #58 d2, #91 d2 | T |
| "Does better with a tile" = territory value (nutrition per person up to target); short of target adds/swaps, at target drops | #58 d3, #61 d1 | T |
| Work within a tile is spread over its foods by stock | #58 d4 | T |
| A hard territory bound (`CLAN.territory_radius`) keeps per-actor work flat | #58 d6 | T (clans.md Scale) |
| A camp marches at one person's pace (step × people) | #69 d2 | T |
| A scout walks on its whole labour (not one person's pace) | #77 d1 | A: the two units disagree; pick one rule |
| Guarding is `UNIT.guarding` labour a warrior, divided over held tiles, spent only when deployed | #91 d1 | T |

### clans.md

| Decision | Source | |
|---|---|---|
| Growth reads this turn's harvest beyond need, not the store | #58 d5 | T |
| A festival is in reach when its route costs under `reach_walk` (20) | #58 d7, #65 | T, but see owner note 4 |
| Desire = base + company (per clan last turn, capped) + a surplus bonus; switch only for a margin | #58 d7 | A: owner note 3 shows clans flip in and out; add a margin for stopping too |
| A clan carries at most 0.2 food a person to a festival | #58 d8 | T |
| A festival fades by attendance, not volume; a lone clan can hold one | #58 d9, #63 | T |
| People move: starving offer shed × people × (1 − fed), growing take by surplus, conserved; never the last person | #58 d10, #63 d2–3 | T |
| A small clan merges into the nearest home at its festival (the "spread" option dropped) | #58 d11 | T: drop the "or its people spread" clause |
| People move and clans merge across civs, so a civ can lose clans and people | #58 d12, #63 d5 | **A**: this is clans leaving a civ, which founding.md calls out of scope; #57 now alerts on it (`PeopleLeft`, 92 events on small 2) |
| Depletion per food, own step after eat | #58 d13, #62 d1–3 | T |
| Taming has no ceiling, slower past the hearth (beyond 0.1) | #62 d6, #65 d3 | T |
| Trails from walking only; holding makes none | #62 d5 | T, revised by owner note 5 |
| The pre-game ends by centres only once people changed under 5% over 100 turns | #65 d1 | T: clans.md says "spread"; say "levelled off" |
| A centre is a clan, the biggest first (owner's own change) | #28 owner reply | already in |
| Retuned spread: split_at 8, hungry_split 5, scout_range 14, surplus_target 2.0 | #65 d2 | numbers only, no edit |

### founding.md

| Decision | Source | |
|---|---|---|
| The civ's own clan stays on the capital; if it's gone the civ's biggest clan takes over | #67 d1, #69 d7 | T |
| Raising takes from any civ clan, biggest first, partial raises allowed; founding goes one a clan in turn | #67 d3, #70 d1 | T |
| Design says the levy is a share of surplus: built as 0.2 of the basket a clan brings to the capital's market, since surplus is 0 at every capital | #69 d1, #72 d1 | T: "a share of what it brings", with the reason |
| The levy stops when the store is at its cap; cap 60 | #72 d2, #86 d1 | T |
| `CIV.pull`: civ clans favour their capital's market by 3 labour, the control too | #72 d3 | A: the owner called it tuning; the stickiness idea may replace it |
| Each civ's scout is made at founding from two clans, its pack from what the camp leaves | #74 d3, #76 d4, #77 d6 | T |
| No raise or disband for scouts; a starved scout is gone and its people lost | #74 d3, #77 d5 | R: design says "made and disbanded like any unit"; 7 of 21 AI scouts die by +50 |
| The AI posts its camp 3 steps toward the nearest rival (else its farthest clan), on own land | #67 d4, #69 d7, #70 d7 | T (a new "AI civs" section) |
| AI civs give their scout no orders | #74 | A: so AI scouts never build a trail |
| Joining: only unaligned clans lean; from gifts (log of food a person, ×5 if starving) and trade at the civ's market on turns it posted; fades 3% a turn; joins at 1.0; daughters copy, hosts keep | #81 d1–8 | T, and lift the out-of-scope line (#81 d9, #87 idea) |
| The civ posts its store beyond keep at its own market; the default keep is the camp's full pack; every civ starts on gift 2 | #81 d3, d5, d6 | T |
| A gift goes into the clan's own share and is eaten that turn | #84 d1 | T |
| A civ's land is the capital's 2-step ring plus its clans' territories (`civs.rs:216–233`) | M1 code, unrecorded in M2 | **A**: owner notes 6 and 11 |

### supply.md

| Decision | Source | |
|---|---|---|
| Auto-supply refills the pack as well as the turn's need | #69 d3 | T |
| Cut off is judged on all the camp's labour; its own tile never blocks | #69 d4 | T |
| The pack spoils | #69 d7 | T |
| Starvation loses one warrior a turn | #67 d2 | T |
| A turn spent marching leaves nothing for the line, so a hungry camp loses warriors walking home | #70 d2, #73 d1 | A (#72/#73 ideas) |
| Auto-forage takes only the shortfall but strips the tile and lays it fallow 5 turns | #70 d3–4 | **R**: strip only what's taken (owner note 9, #73 idea) |
| Reach: a line feeds while cost ≤ labour × carry / food (10.5): 5 plains steps, 2 hills | #72 d4 | T as the felt reach |
| The zone is the camp's territory, home in it, at most `people` tiles; spreads a ring a turn; moving clears it | #88 d1, #90 d2, d5 | T |
| Contested tile to the larger guard share; a tie stays; capitals can't be painted; a line's ends never block | #90 d1, d4 | T |
| Design says a held tile is the civ's land: held land counts only in the line test | #90 d3 | T, reword supply.md |
| Your own held tile lets your line through rival land | #88 d3 | T |
| The AI zones only over its own line, and only when its line still feeds it after guarding | #88 d5, #92 d1 | T |
| `LineCut` is distinct from `CampCutOff` | #91 d4 | T (page) |

### scouting.md

| Decision | Source | |
|---|---|---|
| Worn and built trails both go to the hidden cost; a marsh trail only bridges its river | #74 d1, #76 d1 | T |
| Outward order: a mountain candidate widens to its range, nearest to the scout first | #76 d2 | T |
| With no reveal in reach, build the reachable trail on the possible path | #76 d3 | T |
| The scout feeds first, forages only on a route, never pillages | #76 d4 | **R**: the owner wants auto-forage and pillage as other units (note 9) |
| An idle scout walks home and sits | #77 d2 | T |
| Route found is heard early and again when built; costs shown as summed step cost | #77 d4, #74 d5 | T |
| Partial trail work never decays | #74 | T |

### map.md and the page

| Decision | Source | |
|---|---|---|
| Design says hidden country is drawn at a glance: it's on a View toggle, off by default | #93 d7 | T |
| Reasons: kept 8, shown 5, written when a rule acts; repeats dedupe | #93 d1–3, #96 d1–3 | T |
| Alerts: led civ only, edge-triggered; camp or scout hunger, cut lines, a clan joining a rival, `PeopleLeft` | #93 d4–6 | T |
| The save version pins the rules (`SAVE_VERSION`, pinned digests) | #93 d8, #95 d1 | T |
| The player's camp line always drawn faintly; selecting a clan brightens it | #93 d9 | T |
| Each market drawn with a 3-hex-step ring | M1 page, unchanged in M2 | R (owner note 2) |

## 4. Owner's play notes

The owner played small seed 1. Each note below has its cause in the code and what I'd propose.
These are code readings: the only one I ran is note 3.

1. **Clan 10 routes to clan 2's festival through the desert, because desert walks cheaper than
   forest.** *Working as designed; design gap.* Desert is `move_cost 1.0` with no food
   (`core/src/data.rs:42`), the same as plains and as merge4; forest is 2.0 (`data.rs:39`).
   Route cost is movement only (`markets.rs:80`, `actors.rs:50`). Crediting food found on the way
   would mean costing every route by its foraging, which works against "every walk is costed the
   same way" (`actors.md`). **Propose:** desert costs 2 to cross (water is what makes desert hard), not
   a food credit. Small, and it needs one pre-game sweep.
2. **A market has a radius in the View toggles; only path cost matters.** *Bug (stale page).*
   `engine/www/map-view.js:144–152` draws a 3-hex-step ring round every market, commented
   "approximated by hex steps … as the page always has", and `map-view.js:1083–1087` outlines it.
   The engine's reach is path cost: a route under `MARKET.reach_walk` 20, a 10-step path
   (`markets.rs:65–80`). M1's retro flagged the same ring (#45 idea). **Propose:** drop the ring.
   Selecting a festival shows the clans that walk there and their routes, which #57 already
   brightens for a clan.
3. **Clan 10 drops its festival and picks it up again.** *Design gap.* Measured: `run --seed 1
   --size small --turns 40 --why 10` shows it at turns 35–39: "went to the festival … worth 4.0",
   "stopped going: none is worth the walk", "went … worth 2.4", alternating while it starves.
   Its route costs 14 (7 steps, three of them desert). Worth is gain + desire + surplus bonus 8
   − cost (`markets.rs:65–85`). The surplus bonus switches on and off with the store
   (`markets.rs:74`), and a starving clan's gain swings. `choose` has a margin for switching
   festivals but none for stopping (`markets.rs:95`: `cur.1 > 0.0`). **Propose:** stickiness in
   both directions: a clan keeps its festival until its worth falls below −margin, and a clan
   that switches keeps the new one some turns (the owner's own #28 idea). Small.
4. **Festivals are few and far apart.** *Working as tuned; design question.* `MARKET.spacing` 8 hex steps
   (`data.rs:280`), `form_after` 20, `reach_walk` 20 (a 10-step path). Build-5 of #54 raised
   reach 12 → 20 and desire 1 → 3 because nobody walked. Measured: 11–12 festivals for 100–160
   clans on small, 20–24 on medium, 43–45 on large. **Propose:** spacing 5–6 and reach_walk 12–14,
   then rerun the pre-game sweep: centres need enough clans at one festival (`centre_clans`),
   so this trades against the pre-game ending. A question piece, or part of the next tuning.
5. **No trail under a clan's home; every worked tile gets one.** *Design gap.* `mark_walked`
   (`core/src/actors.rs:297–308`) marks a territory tile only if it has a predecessor on a walk
   (`before[k].is_some()`), so home never counts. Any tile walked 10 turns running gets a trail
   (`land.rs:262–271`, `TRAIL.after` 10), whatever the traffic. Measured: 72–79% of held tiles
   carry a trail on all seven maps, and 0–3% of trails are off clan land. 14–36% of homes have
   one, only where someone else's route crosses. #54 build-5 raised it as an idea. **Propose:**
   trails by traffic: each walk adds its people to every tile it passes, home included. A tile
   wears a trail above a threshold of people-walks and loses it below. Then trails are densest at
   home and on shared routes and thin out at the edges, as the owner expects. Medium: the
   pre-game sweep has to hold.
6. **What decides if a clan joins a civ? Clans join outside its ring.** *Working as designed;
   design gap.* Joining is leaning ≥ `LEANING.join` 1.0 (`leaning.rs:58–82`, `data.rs:473–482`).
   Leaning comes from gifts and trade at the civ's market (`markets.rs:160–166`), and any
   unaligned clan whose festival is the capital's can get it, up to a 10-step path away. The
   ring is `CIV.territory` = 2 hex steps (`civs.rs:216–233`, `data.rs:436`), an M1 stand-in no
   design file defines now. **Propose:** the owner defines a civ's land in `founding.md` (see
   §5). The tile panel shows a clan's leanings; also tint leaning clans on the map (#84 idea).
7. **No way to move the camp; camp, unit and warrior are muddled.** *Design gap (discoverability).*
   Moving is a right-click with your own camp selected (`www/main.js:497–502`, from
   `map-view.js:429`). The camp panel says "Right-click a tile to move it there" only when the
   camp is idle (`www/select.js:164`), and the scout starts on the same tile, so a click on the
   capital may select the scout or the civ instead. The scout uses buttons that arm a map click
   ("Route to here"). **Propose:** a "Move to…" button that arms a click, as the scout's does. In
   `founding.md`, one vocabulary: a *unit* is the camp or the scout; the *army* is the camp; its
   people are *warriors*. The page says "camp" and never "unit".
8. **A unit panel at the bottom, with the same buttons for any unit.** *Design gap.* The selection
   panel is `#detail` on the right (`www/index.html:78–80`), built per kind in `select.js`
   (`campPanel` :127, `scoutPanel` :196); the leaning list falls below its fold (#86 idea).
   **Propose:** a page design file (`docs/design/page.md`) holding the panel layout, the shared
   button row (move, forage, pillage, resupply, switches, disband) and the keys. The camp and
   scout then share one panel shape.
9. **The scout got into the sealed valley, starved, went home; nobody followed, and the route
   faded.** *Design gap, partly a bug.* The scout forages only the tile it stands on, only on a
   route, never on rival land (`units.rs:1215–1218`). `strip` zeroes the tile's whole stock and
   lays it fallow 5 turns whatever it took (`units.rs:486–501`, `UNIT.fallow` `data.rs:528`). A
   scout waiting beside a mountain gets one meal and then nothing; 3 hungry turns lose a person
   (`data.rs:521`, `units.rs:1247–1253`). It never refills beyond the turn's need, and a line
   from inside a sealed valley can't carry. When the route is found or the scout starves out,
   the route clears and it walks home (`units.rs:1205–1211`). Nobody walks the trail, and it
   wears away in 30 turns (`land.rs:271`, `TRAIL.fade`). #80's review measured it: 5 of 9
   cross-country routes starved, and it set merge false. **Propose:** the scout gets the camp's switches
   (auto-forage and pillage, and foraging the tiles round it, not only its own). `strip` takes
   only what it needs and lays fallow in proportion. The scout refills its pack while foraging. A route the civ uses keeps
   its trail: a camp's line walked counts (#78 idea), or the scout walks the route home. And a
   lost scout can be raised again.
10. **Spacebar should end the turn.** *Gap.* The key handler (`www/main.js:773–791`) has no case
    for space. **Propose:** add it. Trivial.
11. **Gold circles on the tiles next to the capital, and a thick ring at radius 2.** *Bug (dead UI)
    and design gap.* The circles are the capital's "work tiles" (`civs.rs:249–253`, sent as `work`
    by `web/src/lib.rs:122`, drawn at `map-view.js:172–176` and `743–747`). That comes from M1's
    "the capital works the tiles next to it". `founding.md` no longer says it, and nothing works
    them. The thick ring is the civ's land outline: `CIV.territory` 2 steps
    (`map-view.js:157–171`). **Propose:** remove the work circles. Keep the outline only once
    `founding.md` says what a civ's land is (note 6).

**Also: the design files cite deleted folders.** `map.md:6, 28, 37` and `clans.md:9` cite merge4;
`scouting.md:8, 60` cites `scout/`; `supply.md:11` cites merge4's numbers. Those folders are gone,
and the code is at the `prototype/merge4` and `prototype/scout` tags (#93, #97, #98 ideas).
`scouting.md` also still says clans wear trails "walking a round", which #54 removed.

## 5. Proposed edits to `docs/design/`

1. **`founding.md`, Out of scope and a new "Joining" section.** Lift "clans joining after
   founding", "posting to markets" and "AI beyond keeping units". Add the leaning rules in prose
   (gifts and trade at your market, fading, a threshold, daughters keep it), the keep, and that
   every civ starts on the same gift. Evidence: #81 d1–9, #87 idea; built and measured
   (`--lever 6`: 14.7 vs 4.5).
2. **`founding.md`, the levy:** "a share of what a civ clan brings to the capital's market, until
   the store is full". Evidence: #72 d1–2: surplus was 0 at every capital.
3. **`founding.md`, a civ's land:** define it, or say the 2-step ring is a stand-in. Evidence: owner
   notes 6 and 11; `civs.rs:216–233` decides line blocking, forage vs pillage and the AI's post
   with it.
4. **`founding.md`, units:** one vocabulary (unit = camp or scout; warriors are the camp's
   people). Scouts are raised and disbanded like the camp. Move by a button as well as a
   right-click. Evidence: owner note 7; 7 of 21 AI scouts dead by +50.
5. **`founding.md`, a new "AI civs" section:** the post (3 steps toward the nearest rival, else the
   farthest clan), zoning only over its own line, the default gift. Add whether AI civs scout and
   whether they ever cut. Evidence: #70 d7, #88 d5, #92 d1; cut by army 0 in all sweeps.
6. **`supply.md`, Past the line:** foraging takes what it needs and the tile rests in
   proportion. Add "a march turn draws nothing on the line" if the owner keeps it. Evidence:
   #70 d3, #73 idea, owner note 9.
7. **`supply.md`, the zone:** what's built: the zone is the camp's territory, up to one tile a
   warrior, spreading a ring a turn; the thicker guard holds a contested tile; held ground
   blocks rivals' lines and opens your own. Evidence: #90 d1–5, #91 d1.
8. **`scouting.md`:** the scout has the camp's switches and forages round it. A trail on a route
   the civ uses (a camp's line) counts as walked. Drop "walking a round". Point at
   `prototype/scout`. Evidence: owner note 9, #78 idea, #80 review.
9. **`clans.md`, Festivals:** "A clan keeps its festival unless another is worth clearly more, or
   it's clearly not worth the walk", for stickiness both ways. Make merging one rule (the
   nearest). Say whether people moving at festivals may take people out of a civ. Evidence: owner
   note 3, #58 d11–12, #63 d5.
10. **`clans.md`, Trails:** "A trail wears where many people walk often: under homes and along
    shared routes, thinning at the edges." Evidence: owner note 5; 72–79% of held tiles trailed.
11. **`clans.md`, The pre-game ends:** "once people have levelled off", not "spread". Evidence: #65 d1.
12. **`map.md`:** desert is slow to cross (note 1). Hidden country sits behind a View toggle,
    off by default (#93 d7). Point at `prototype/merge4`.
13. **New `docs/design/page.md`:** the panel (bottom, one shape for every unit, owner note 8), keys
    (space ends the turn), reasons and alerts (#93), and no market radius (owner note 2).
14. **`vision.md`, Winning:** consider choosing *Influence* first, since joining exists and the
    sweep can already count it. It would give the levers a purpose. Evidence: §2.

## For other angles

- **For crew:** a page that didn't load reached main (#92: missing brace in `select.js`). CI
  never runs a play script. Also, the working tree at the time of this retro had uncommitted
  deletions of the archived stubs and `ci.yml` edits (someone's work in progress, not mine).
- **For players:** `sweep --after` reports nothing about scouts or trails. Add scouts alive and
  trails built, and a "people change after the pick" column. The probe here is about 60 lines
  and could become that.
- **For prompts:** builds measured their own piece and never re-ran an earlier piece's question. Both
  regressions (camps 21 → 15, joins 183 → 84) went unreported until this retro. A review
  could re-run the earlier pieces' sweeps.

## Ideas

Deduplicated across every step's and comment's ideas, the owner's notes and mine. Value / cost /
recommendation.

| # | Idea | Source | Value | Cost | Rec |
|---|---|---|---|---|---|
| 1 | A goal: an *Influence* win (share of the world's people) | mine, §2 | high | medium | try (M3) |
| 2 | An AI that uses its army: contest festivals, pillage, cut lines, answer a cut | #91 idea, mine | high | medium | take (M3) |
| 3 | A main game that moves: good and bad years, or farming | mine; clans.md Out of scope | high | large | later, early M3 design |
| 4 | Scout fixes: the camp's switches, forage round it, `strip` takes only what it needs, refill the pack | #73, #80 (review), #79 pack, owner 9 | high | small | take |
| 5 | Raise and disband scouts (replace a lost one) | #77, #78, #79 | high | small | take |
| 6 | Festival stickiness both ways (stopping margin, and staying after a switch) | owner (#28 reply), owner 3 | high | small | take |
| 7 | A "Move to…" button for the camp; one unit vocabulary | owner 7 | high | small | take |
| 8 | Trails by traffic (people-walks, home included) | #65, owner 5 | high | medium | take |
| 9 | Define a civ's land in design | owner 6, 11; M1 retro | high | small (design) | take (ask owner) |
| 10 | Syntax-check `www/*.js` or run a play script in CI | #92 | high | small | take (crew) |
| 11 | The AI raises only what it can feed (small civs) | #72 | medium | small | take |
| 12 | Give to a chosen clan, a per-turn decision | #84 | medium | small | try |
| 13 | Leanings on the map (tint by the civ, strength as alpha) | #84 | medium | small | take |
| 14 | Make the lever's cost bite (keep follows what the camp eats; gifts can draw on the pack) | #87 | medium | small | try |
| 15 | A camp's line keeps its trails alive | #78, owner 9 | medium | small | take |
| 16 | The AI's post flickers with clan claims: post by path cost within its land | #91 | medium | small | take |
| 17 | Supply on the march (a line still draws on a march turn) | #72, #73 | medium | small | try |
| 18 | Scouts at the capital feed from their clan, not the store | #79 | medium | small | try (or covered by 4) |
| 19 | AI scouting toward rivals or markets | #77 | medium | medium | later |
| 20 | Cutting deep in rival land: a corridor home, cheaper forage on held tiles | #91, #92 | medium | medium | later |
| 21 | More, nearer festivals (spacing 5–6, reach 12–14), swept against the pre-game | owner 4 | medium | small | try |
| 22 | Desert costs 2 to cross | owner 1 | medium | small | try |
| 23 | Drop the market's hex ring; show who walks to a festival | owner 2, M1 #45 | medium | small | take |
| 24 | Remove the capital's work-tile circles | owner 11 | low | small | take |
| 25 | A bottom unit panel with one button row; `page.md` | owner 8, #86 fold | medium | medium | try |
| 26 | Space ends the turn | owner 10 | low | small | take |
| 27 | Mine-only feed filter and a help panel | #93 | medium | small | try |
| 28 | Skip `clans::adjust` re-division for fed, unchanged clans (large at 29.6 ms a turn) | #66 | medium | medium | try |
| 29 | Measure levelling off apart from the end rule (people change after the pick) | #66, mine | medium | small | take (sweep column) |
| 30 | Sweep reports scouts alive and trails built; re-run earlier pieces' sweeps at review | mine | medium | small | take |
| 31 | Festival reasons: name who traded or moved; one line a turn | #96, #98 | low | small | take |
| 32 | "Let go" reason shows the change (three decimals or the tiles) | #96 | low | small | take |
| 33 | Point design files at `prototype/*` tags; fix "walking a round" | #93, #97, #98, coordinator | low | small | take |
| 34 | Drop archived stubs from `ci.yml` and delete them | #64, #66, #93, #94 | low | small | take (in progress) |
| 35 | Drop `Tile.unheld` | #62, #66 | low | small | take (chore) |
| 36 | Fed as yes/no in the clan panel; wear in the tile panel | #64 | low | small | later, with 25 |
| 37 | `--keep K` in the sweep | #86 | low | small | later |
| 38 | Leaning clans all join in one turn: watch for it | #85 | low | small | later |
| 39 | Scout outward order walks back toward home | #80 | low | small | later |
| 40 | Demo scout route through untrailed country; play-read waits for a camp alert | #80, #97 | low | small | later |
| 41 | Rename markets to festivals in the engine | #64 | low | medium | later |
| 42 | Small-map centres barely on fertile land | #65 | low | small | later |
| — | Ideas since resolved: pre-game only by top-up (#61), medium 10 two centres (#61), few people move (#63), levy brings nothing (#70), gift over 5 unpaid (#85), capital across the sea (#70), marching camps cut for a turn (#91), scouts' 42 people (#79) | | | | drop |

## Owner's play notes (dictation)

_Left for the owner's play-through dictation. The notes received so far are in §4._
