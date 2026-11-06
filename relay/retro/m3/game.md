# M3 retro: the game

One angle of the M3 retro: each piece's question measured on main (ec1e47b), the owner's play notes
(`owner.md` §1) traced to the code, the `## Decided` lines M3 added, and what a person meets
playing main today. Everything here is a proposal; the owner owns `docs/design/`.

Sources: PR 107, 118, 124, 126, 134, 142 descriptions; `relay/specs/101`–`104`; PR 124's
`relay/specs/101/play.md` (on `claude/wonderful-lamport-aduj1e`); `owner.md`. "Measured" means a
release build of main: `sweep --judge` (44.5 s), `sweep --judge --solo`, `sweep --after 200 --maps
retro`, `scout_errand` (plain and `FED`), a 30-turn `play` of small seed 2 as Red, and two scratch
probes on a copy of the engine (a scout tracer over ten small seeds, and an economy counter over
the seven retro maps). The copy also ran `sweep --judge` with a trail on flat land. Scratch files
are outside the repo; nothing in it changed but this file.

## Verdict

**M3 built every system it set out to except R, and with R missing the game still has no
contest.** Claiming pays a little (+0.012 Standing a civ, down from +0.017 when L merged), the
lead changes hands after the pick (10 times on 6 of 7 maps), and fighting works in scripts. But
with every civ on the AI nobody fights (0 people fallen, 0 tiles taken, 0 surrenders on all seven
maps), no AI army grows past its founding three, no clan leaves a civ, and no game ends. The
economy is the deeper limit: 14 of 21 civs never hold 15 food, a three-warrior army's packs, in
200 turns; the guard starves in a led game within ten turns; and 13 of 14 scouts still starve on
their first errand, 11 of 14 even with the store kept full. The owner's four play notes are all real: two are page bugs (the scout's
buttons, Disband all), one is a search that is right about what it skips but slow and silent
about it, and one (a trail on flat land) measurably feeds the game. R needs an economy that can
feed an army before its planner is worth tuning, so the next game work is R's end rules, then
settling with supply as a flow, then R's planner.

## 1. Each piece's question, measured on main

| Piece | Question | The PR said | Main now | Answer |
|---|---|---|---|---|
| #100 H | Same game, new shape? | Digests and the retro sweep byte-identical (PR 107) | Digest mismatches 0 in `sweep --judge` | **Yes.** |
| #101 G | Can a session play a led civ and say what it couldn't tell? | Sonnet led Red 108 turns for about $0.20 and found the dead gift (PR 118, `play.md`) | `sweep --judge` on main reproduces `relay/specs/104/glaze-judge.txt` total for total; `play` and `judge FILE` work (§4) | **Yes.** The judge is a report; the held half is PR 124. |
| #102 L | Does a civ that claims gain Standing over one that doesn't? | +0.0170 a civ, 14 of 21 above +0.005 (PR 126) | **+0.0120, 10 of 21** above +0.005; 7 of 21 lose by claiming; on small 3 and small 6 all three civs lose | **Yes on average, smaller since A**, and all of it is clans won: unaligned people on held land add +0.0000. |
| #102 L | Does the lead change hands after the pick? | 12 on 5 maps; no-claims control 3 on 3 | **10 on 6 maps**; no-claims control 5 on 3 | **Yes**, with every civ on the AI. Whether a *led* game is in doubt is R's question. |
| #103 P | Every touch point on the page at three sizes, no rule moved? | 26 of 26, 15 of 15 levers with a button (PR 134) | Not driven here | Taken from the PR. Two buttons are wrong (§2, notes 1 and 2). |
| #104 A | Does the Red and Blue story come out, and does fighting move Standing? | Skirmish yes (+2.9), surrender yes, stack stops 9–10 of 13 steps out (PR 142) | AI maps: people fallen 0.00, tiles taken 0, armies surrendered 0; camps kept 13 of 21, warriors starved 26, camps lost 6 | **In scripts only.** The AI never fights, so fighting moves no Standing on the maps the game is played on. |
| #105 R | Is a led game ever in doubt; do games end? | Unbuilt | No end state; `end.js` says "The game goes on" | **Unanswered.** |

Other totals on main (`sweep --judge`): top Standing at +200 0.096, aligned 0.200; spread +200
0.062; AI orders 21.9 per 100 turns, of which claim 16.0, move 3.1, raise 1.05; clans joined 130,
**clans left 0**; claimed tiles taken 0. Festivals: 218 at the pick, flips 0.09 per 100
clan-turns (M2: 4.35).

### Does the economy stop war before the rules do?

PR 142 says no store fills more than about three packs. Counted on main with every civ on the AI
for 200 turns after the pick (scratch probe; a warrior's pack is `UNIT.pack` 5 food; the store cap
is 60):

| | Civs |
|---|---|
| Store averages under 8 food | 16 of 21 |
| Store never reaches 15 food (a three-warrior army's packs) | 14 of 21 |
| Store averages 24–43 food (5–8 more warriors' packs) | 4 of 21: small 9 civ 487, medium 9 civ 877, large 8 civs 2708 and 2710 |
| Levy a turn | 1.0–19.3 food; median about 7 |
| Largest army ever | 3 people, one army, on every civ |
| Guard averages under 0.2 people | 7 of 21 |

So on three civs in four, yes: the store never holds one army's packs, the levy goes to the guard,
gifts and the founding army, and there is nothing to raise with. On the other quarter the store
sits near the cap and the AI raises nothing (raise 1.05 per 100 turns, all replacements). There
the AI stops war, not the economy, and only R can show whether those civs fight.

## 2. The owner's play notes

| Note | Cause | Kind | Proposal | Cost |
|---|---|---|---|---|
| 1. Scout: Route from…, Route to…, Move to… | The scout's "Move to…" arms `'to'` and sends `route` from the armed start or the scout's tile (`www/screens/play.js:482`): it is Route to under the wrong name. `Order::Move` checks for a camp only (`orders.rs:119`), and a scout with no route walks home (`scout.rs:555–563`). | Page bug, engine gap | Rename the button Route to…. Add a scout `Move`: `ScoutState.dest`; `check_move` takes a scout; with a `dest` and no route, `scout_turn` walks to it and stays, foraging, instead of walking home. Route from… keeps arming the start. | Small |
| 2. Disband all disbands one at a time | `disband()` in `www/panels.js:261–270` offers per kind, then "all", which means *every kind in proportion*, by count. The stepper opens at option 0 with count 1 (`panel.js:131`), and switching to "all" keeps count 1, so Go sends `{"type":"disband","n":1}`. The page never sends the engine's real "all" (`n` absent, `orders.rs:37–45`). Its cap is `floor(people)`, so a fought army of 2.6 keeps 0.6. Reproduced with `play` on the skirmish story's save: guard 1.8, `n:1` leaves 0.8; no `n` leaves 0. | Page bug | "All" sends no `n` and needs no count: a button, not a stepper option. The guard and the scout have the same stepper, so the guard isn't different in code. It only shows no "all" word, which is likely why it reads right. | Trivial |
| 3. The scout checked one mountain, one possible, ignored two | See below. It skips a mountain only when the cheapest route through it, even as a pass, can't beat the known route. That is `scouting.md`'s rule, and every skipped mountain in ten traced errands met it. But the search orders by nearness, not promise, so it is slow on long ranges, and the panel never says why a mountain is left. | View gap; search slow | Order a range's mountains by their best case (cost through it plus the walk there). The panel's "Unrevealed" row lists the ones it won't check: "can't beat the known route (best 21 against 16)". | Small |
| 4. A trail should always make a difference | Flat land has no hidden cost (`map.rs:1146–1159`), so `step_cost` keeps plains at 1.0 under a trail (`land.rs:42–48`); `scouting.md` says "Flat land gains nothing from a trail". | Design | A plains and coast trail at 0.75. Measured below: more trade and alignment, more held land, longer lines, less churn. Rerun the pre-game sweep with it. | Small |
| 5. Component and integration tests | `core/tests/scout.rs` paints real maps into a case; `tests/common` has `flat()`; `story` scripts a led civ. Nothing places units, gives scripted orders and checks an outcome in one builder. | Gap | A `Scenario` in `tests/common`: a seed or painted map, units placed, orders by turn, assertions on the world. The scout tracer used here (about 150 lines) is most of one. | Medium |
| The first errand | `scout_errand` on main: 13 of 14 lost; 11 of 14 with the store full. The line and foraging can't feed two people seven steps out. | Rule gap | The supply ring (M4); until then a one-person scout or a pack sized to the errand, and an idle scout that waits on rich ground. | Small |

**Note 3, traced.** A scratch probe ran small seeds 1–10. For each it found the target 4–10 steps
from the led capital whose possible route crosses the biggest unrevealed range (5–46 mountains),
routed the scout there with its store and pack kept full, and printed each turn's reveals and each
range mountain's best-case cost.

- **What it skips, it skips rightly.** On small 1 the scout revealed four mountains (one a pass) and
  settled at 16. The three it never visited cost at best 17, 21 and 21 through them. On small 3, 4, 5,
  6 and 8 every unrevealed mountain likewise cost at least the known route. If the owner's two were
  impassable, a pass on them still couldn't have made a cheaper route, and nothing on the page
  says so.
- **The search is slow on long ranges.** Ordering is mountains first, then the nearest by walk
  (`scout.rs:250–262`), and a mountain takes a whole turn begun beside it. On small 9 it walked 11
  turns round the map (t18–t28) to reach a range's far end, while two passes whose best case was 46
  and 48 against a known 80 sat unvisited. Small 7, 9 and 10 had no route after 30 fed turns; the
  rest found one by t4–t19.
- **The design is sound and the view misled.** `scouting.md` says reveal "tiles on the possible
  route that could close the gap", and the code does. The debug view shows true costs and the panel
  shows a count, so a correct skip looks like neglect.

**The first errand.** `relay/specs/104/scout.md` found 13 of 14 scouts lost. On main it is the
same table, row for row: **13 of 14 lost**, only small 6 to its nearest rival living. With the
civ's store set full every turn (`FED=1`), **11 of 14 are still lost**. So the empty store at the
pick is not the main cause. The scout is two people eating 2 food a turn. Its pack holds 5 turns.
Seven steps out its line carries about 1 a turn, and foraging round it strips its own and the
neighbouring tiles within a few turns. The scout tracer (§2, note 3) kept every scout alive only
by refilling its pack each turn. What would feed it is what the owner's supply ring (`owner.md` §3)
describes: inside the ring the line carries more than the scout eats, and outside it the pack and
foraging must last the errand. Until then: a one-person scout (half the food for half the labour),
or a pack that holds the errand (about 30 turns of food for 30 turns out), and an idle scout that
waits on rich ground rather than at the stripped capital.

**Note 4, measured.** The engine copy gave plains and coast a hidden trail cost from an environment
variable and ran `sweep --judge` once at 0.75 and once at 0.5. Trails then wear by traffic as now.
The scout builds none on flat land: `TRAIL.work` has no plains entry. Cheaper walks change the
pre-game, so each map is a different game; one run each.

| | Main | Plains trail 0.75 | 0.5 |
|---|---|---|---|
| Pre-games by centres | 7 of 7 | 7 of 7 | 7 of 7 |
| Festivals at the pick; trading share | 218; 0.90 | 211; 0.93 | 208; 0.97 |
| Festival changes, flips per 100 clan-turns | 0.87, 0.09 | 0.56, 0.10 | 0.27, 0.02 |
| Aligned share at +200; top Standing | 0.200; 0.096 | 0.232; 0.112 | 0.292; 0.120 |
| Lead changes | 10 on 6 maps | 12 on 5 | 16 on 6 |
| Held tiles a civ at +200 | 16.3 | 21.3 | 28.5 |
| AI orders per 100 turns | 21.9 | 17.2 | 8.8 |
| Motion: land tiles changing hands | 2425 | 2099 | 1169 |
| Camps kept; warriors starved | 13; 26 | 12; 22 | 11; 30 |
| People fallen to fighting | 0 | 0 | 4.67 |

A flat trail ties clans to their festival (trade up, changes down) and pulls more of them into
civs. The guard's walks shrink, so the same guard holds more land. A line's carry is `labour ×
carry / cost`, so a line on trailed plains carries about 1.3× at 0.75 and 2× at 0.5. At 0.5 the AI
fights for the first time, but motion halves and AI orders fall by more than half. 0.75 gives most
of the gain without stilling the map; take it as the starting value.

## 3. The ledger

Every `## Decided` line M3 added (#100–#104), by file. P = promote into the prose, W = reword
(as said), S = strike. `process.md` keeps values out of design files, so a line carrying a number
is W: the number moves to `data.rs` and the line keeps the relation.

### actors.md

| Line | | Why |
|---|---|---|
| Every actor's store spoils the same way; the civ is an actor (#100) | P | True and the shape M4's farmer builds on |
| A clan eats to its diet share, then the excess at a lower value; a camp or scout in proportion to its pack (#100) | P | True; say "army" |
| Every walk pays its steps from the labour left, one at a time (#100) | W | Add "a step part-paid carries to the next turn": the owner's day's walk (§2) starts here |
| Every order names its civ and unit, through one door (#100) | S | Engineering, already in `process.md` and `architecture.md` |
| The player's orders are logged; the AI's are worked out again (#100) | S | Engineering; `architecture.md` |
| A civ may raise warriors; the order names the civ (#100) | S | Overtaken by #104's raise by kind and place |
| A camp is sent only to land, never onto a built pass (#100) | P | A rule a player meets; say "army" |
| An army's and a guard's people stand on their tiles as they guard them (#104) | P | Makes a thin line thin in a fight |

### clans.md

| Line | | Why |
|---|---|---|
| A clan keeps a new festival some turns, then until clearly not worth it (#102) | P | Flips 4.35 → 0.09 per 100 clan-turns |
| Festivals sit closer, so there are more (#102) | P | 142 → 218 at the pick |
| Trails follow traffic; a line and a route keep theirs while used (#102) | W | Add "a trail always cuts the walk, flat land included" if note 4 is taken |
| Every clan leans; by land, more under a thicker guard (#102) | P | It is how L pays: all of the solo gain is clans |
| A civ clan drawn to a rival for five turns leaves, with its daughters (#102) | W | "Five" to `data.rs`. Clans left: 0 on main, so Standing is not losable this way yet |

### founding.md

| Line | | Why |
|---|---|---|
| Unaligned clans on a civ's land pay its levy (#102) | P | True |
| Gifts reach every clan at the festival not the civ's own, rivals' too (#102) | P | True; makes gifts contested |
| A joining or leaving clan starts at the join line; other leanings cleared (#102) | P | Keeps leaving from bouncing |
| A civ keeps a turn of its guard's food, out of gifts (#102) | P | True, though the guard still starves (§4) |
| Any number of armies of warriors and archers; raise by kind and place; disband by kind or all in proportion (#104) | W | Add "Disband all takes every person" (note 2) |
| A civ may raise a scout any time, with a pack from the store (#104) | P | True; the store is the limit |

### map.md

| Line | | Why |
|---|---|---|
| Desert costs 2 to cross (#102) | W | "Desert is slow to cross"; the 2 is `data.rs`'s |
| The bar: pre-game counts, then Standing (#103) | W | Move to `page.md` |
| Festival routes off by default, drawn on selection (#103) | W | Move to `page.md` |

### page.md

| Line | | Why |
|---|---|---|
| Start screen: New game, Continue, Load; a bare link opens it (#103) | P | |
| The pre-game plays on the map, Skip to the pick (#103) | P | |
| Standing chips; "you"; the leader ▲; click selects (#103) | P | |
| The panel's rows per kind are settled (#103) | P | |
| A greyed button says why; Raise… and Disband… open a count (#103) | W | "Disband all" is a button, not a count (note 2) |
| Escape steps back one thing at a time (#103) | P | |
| Alerts stay until clicked or selected (#103) | P | |
| The feed opens only at 1400 px or wider (#103) | W | "On wide windows"; the width is the page's |
| Two columns at laptop and wider; a bottom sheet on a phone (#103) | P | |
| Territory tints off by default (#103) | P | |
| The atlas: three variants at 128×148, units at 64×64 (#103) | W | Keep "`atlas.json` is the artist's brief"; the sizes live in it |
| Until a game can end, "The game goes on" (#103) | S | When R lands |

The scout's buttons need a line too: Move to…, Route from…, Route to… (note 1).

### scouting.md

| Line | | Why |
|---|---|---|
| A scout forages its tile and the next, paying the walk (#104) | P | Merge with M2's "the camp's switches" line |
| A scout keeps its route until cleared; its trails count as walked (#104) | P | |

Also reword the prose "Flat land gains nothing from a trail" if note 4 is taken, and add the order
of search (best case first) and that the panel says why a mountain is left (note 3).

### supply.md

| Line | | Why |
|---|---|---|
| Lines and forage read land as the held part of the claim (#102) | P | |
| Foraging takes only what it takes; the tile rests in proportion (#104) | P | Warriors starved 53 → 26 |
| A scout has the army's switches and one-shot orders (#104) | P | Duplicates scouting.md's M2 line: keep one |
| A cut-off army can't be reinforced (#104) | P | |

### war.md

| Line | | Why |
|---|---|---|
| Land is the held part of the claim; the rest is paper; the capital always held (#102) | P | |
| One guard at the capital, no orders, fed by the store then its clans (#102) | W | "No orders but raise and disband" |
| Overlap to the thicker guard, a tie to the older civ; no claim over a rival capital (#102) | P | |
| A rival unit ending its turn on paper takes it (#102) | W | "A rival army": #104 says a scout takes nothing |
| Standing counts unaligned clans on a civ's land (#102) | P | It adds +0.0000 on main: a clan on held land joins first |
| The AI's stand-in claim (#102) | S | When R lands; until then mark it a stand-in |
| Two civs on a tile fight; each loses a fifth of the other's strength (#104) | W | "A share"; the fifth is `FIGHT.rate` |
| Only armies, guards and militia fight; a scout neither fights nor takes (#104) | P | |
| Warriors 1/1; archers 0.8 attacking, 1.4 defending (#104) | W | "Archers defend better than they attack"; values in `data.rs` |
| Archers need held forest and a turn of bow-making (#104) | P | |
| Entrench a tenth a turn up to half again; a guard always entrenched; hungry at half (#104) | W | Relations only |
| A march stops on the first tile with a rival's people (#104) | P | |
| Retreat under half; guards and militia never retreat (#104) | W | "Badly outmatched" for the half |
| Clans defend home at three tenths; the guard drawn back to a besieged capital (#104) | W | "A fraction of a warrior" |
| Besieged: no harvest, no levy (#104) | P | |
| Surrender after three hungry, cut-off turns (#104) | W | "A run of"; three is `data.rs`'s |
| A capital falls with no guard, army or militia; its store fills the takers' packs (#104) | P | R ends the game on it |

### vision.md (PR 124, not on main)

| Line | | Why |
|---|---|---|
| Standing counts a civ's clans and its units' people | P | Raising neither wins nor loses Standing |
| The lead changes only past half a point | W | "A margin"; the value is `JUDGE.lead_margin` |
| An army mustn't starve its own side | S | The owner (`owner.md` §5) |

## 4. What a person meets playing main

**Played:** `play --seed 2 --size small --civ 1`, Red, 30 turns from the pick (237). Red starts
third at 5.6% with store 0, gift 2, a three-warrior army with a full pack, a scout and a guard of
3 on a 24-tile claim. Raise warriors isn't offered: the store can't fill a pack.

- +0: route the scout to Blue's capital (known 15, possible 14) and raise a guard (4). By +5 the
  guard is 2 and hungry four turns; the scout sits hungry at (-8, -4) with no tile left to search.
  "why 477": "its civ's store could not feed its guard".
- +5: gift 0 so the levy feeds the guard. By +10 the guard is 1, 16 of 24 tiles are paper, the two
  unaligned clans on Red's land are off it (one joined), and Gold jumps 6.3% → 12.1%.
- +20: the two clans leaning to Red (0.26, 0.11) trade at another festival, so a gift can't reach
  them; the view says so. Shrink the claim to 8 tiles and post the army toward them.
- +30: Red 4.8%, third; the AI control's Red 2.8%. The guard has starved to 0; 2 of 8 tiles held.
  `judge`: 5 orders, 27 of 30 turns with none, no fight, nothing taken.

PR 134's two 30-turn page games (seeds 2 and 4) used Gift, a guard, a claim, Move to…, a zone and
an alert, and found the scout's hunger unexplained and Move to… unwarned past the line. The #101
trial (PR 124's `play.md`, before L) found one real lever, the gift, and 50 idle turns.

**What the player decides:** the claim's size against one guard; gift and keep; where the army
stands; a scout's route. Each is set once or twice: the store is empty, so nothing else can be
afforded.

**What happens to them:** the guard starves and the claim turns to paper; clans drift to whoever
feeds them; a rival climbs. Nothing attacks, nothing is lost to a fight, and the game never ends.

**What is dead:** raising (no store), archers (a store and forest), fighting (no AI ever comes),
the scout (starves or sits), the end screen ("The game goes on"), leaving (0 clans left).

### What the game needs next, in order

1. **R's end, without its planner.** The turn limit, `Phase::Over`, last capital or most Standing,
   the end screen. It is small, it gives the scores a finish line, and every later judge reads it.
2. **Settling, with supply as a flow (M4).** The store is the binding limit: 14 of 21 civs never
   hold one army's packs, the guard starves in ten turns, and a scout seven steps out starves even
   on a full store. The farmer
   makes a store worth taking (`farming.md`). The owner's day's walk and kilograms per labour-day
   (`owner.md` §2–3) give the line a capacity and a ring. Plains trails (note 4) belong here.
3. **R's planner, tuned on that economy.** Tuned now, it would learn that raising never pays,
   because on three civs in four it doesn't. On a fed economy the same planner has armies to move,
   and its question (is a led game in doubt?) can be answered.
4. **The scout's buttons, Disband all, the scout's errand fed, the search's order and its reasons.**
   Small page and rule fixes that can ride with any piece.

So: R's end before farming; farming before R's planner is worth tuning.

## For other angles

- **For players:** `sweep --judge` has no economy group. Store mean and max, turns at a pack's
  worth, levy a turn, the largest army, and the guard would have shown the "three packs" limit on
  every PR, not only in A's stories. The probe here is about 40 lines.
- **For architecture:** a scenario builder (note 5) and a scout `dest` (note 1) touch
  `tests/common` and `ScoutState`. The search's `through` runs two Dijkstras per range mountain per
  step of the loop: fine at 46 mountains, worth watching on large maps.
- **For briefs:** L's solo gain fell from +0.0170 to +0.0120 after A merged, and no step re-ran
  L's question. A review that re-runs the earlier pieces' scores would have caught it.
- **For prompts:** PR 142's "the economy stops the stack" was measured on Red only. The AI maps
  show the AI, not the economy, stopping war on a quarter of the civs.

## Ideas

| # | Idea | Source | Value | Cost | Rec |
|---|---|---|---|---|---|
| 1 | R's end rules, turn limit and end screen, ahead of its planner | §1, §4 | high | small | take |
| 2 | Supply as a flow: kg per labour-day, a supply ring (M4) | `owner.md` §2–3, §1 table | high | large | take (M4) |
| 3 | Disband all sends no `n`: a button | note 2 | high | trivial | take |
| 4 | Scout buttons: Move to… (new `dest`), Route from…, Route to… | note 1 | high | small | take |
| 5 | Feed the scout on its errand: one person, or a pack sized to it; an idle scout waits on rich ground | `scout_errand` (11 of 14 lost even fed), `relay/specs/104/scout.md` | high | small | take |
| 6 | A trail on flat land at 0.75 | note 4, probe | high | small | try (with the pre-game sweep) |
| 7 | An economy group in `sweep --judge` | §1 | high | small | take |
| 8 | Order the search by best case; the panel says why a mountain is left | note 3, probe | medium | small | take |
| 9 | A `Scenario` test builder: map, units, scripted orders, assertions | note 5 | medium | medium | take |
| 10 | Rerun earlier pieces' questions at review (L's solo fell 0.017 → 0.012) | §1 | medium | small | take |
| 11 | R's planner, tuned after settling | §4 | high | medium | later (after M4) |
| 12 | Clans leaving: 0 on main; tune the run or the margin so Standing is losable | §1, `clans.md` | medium | small | try |
| 13 | Numbers out of the war.md ledger into `data.rs` relations | §3 | low | small | take (at promotion) |
| 14 | The guard eats before gifts and the army's refill when the store is short | §4 play | medium | small | try |
| 15 | Move to… warns past the line's reach; the scout's hunger has a reason | PR 134 | medium | small | take |
| 16 | Strike "An army mustn't starve its own side" | `owner.md` §5 | low | trivial | take |
