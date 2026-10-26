# M1 retro: decisions, ideas and the game (#31, #32)

One angle of the M1 retro (#30): every decision the spec, builds and review recorded on #31 (The
map and the page, PR #39) and #32 (Founding, PR #48), every idea the sessions raised, and what
they mean for `docs/design/` and the next milestone. #23 is left out. Everything here is a
proposal: the owner owns `docs/design/` and `relay/prompts/`.

Sources: the `<!-- relay -->` block on each step issue (#38, #40–#46, #47, #49–#53), the crew's
PR comments on #39 and #48, `relay/specs/31/` and `relay/specs/32/`, and the crew logs of runs
40211144705 and 40232162428. Where a finding says "measured", the numbers come from a scratch
probe outside the repo: the engine on main, watched (all three civs on the AI) from the pick for
200 turns, on small seeds 2, 3, 6, 9, medium 8, 9 and large 8.

## The headline

**The civ's store can't feed its one warrior.** In the measured games, 14 of 21 civs lost their
starting warrior to hunger within 200 turns of founding, the first of them 22 turns after the
pick (medium 8, Red: pick at 245, starved at 267). The levy brings in about 1 to 3 food a turn,
and the store loses 10% a turn to spoilage, so a store of about 10 holds steady against one
warrior. A second warrior drains it. Build-2 saw this (idea "AI civs' warriors starve when the
store dries") and three sessions asked for the capital's work tiles to be worked (#47 spec,
#53 review, #51's idea). Every unit M1 still has to build (scout, army, supply) eats from this
store, so it matters before they start.

## Process

1. **Decisions a player sees were made by the spec and builds, not the proposal.** These ended up
   on screen or in the rules later pieces build on, and none is in `founding-and-scout.md` or
   `docs/design/`:
   - The store's numbers: cap 50 (merge4's `baseCap`), levy 0.5, spoil 10% a turn, and the order
     spoil, levy, feed, cap (#47 decision 8). The top bar shows "30 / 50 store".
   - A warrior's numbers: labour 3.5, food 1, hungry 3 turns, a clan needs 2+ people to give one,
     and making costs no food (#47 decision 9, `data.rs` `UNIT`).
   - What a civ's land is: the capital's 2-step territory, then any tile on a civ clan's round
     (spec.md #32 decision 6). The tooltip says "Red, clan 40's round". #27 (Captains,
     territory and armies) will have to work on this definition.
   - Warriors don't hold land, block, fight or limit a stack (spec.md #32 decision 14). #27
     inherits this.
   - Clicking a tile cycles through what's on it (#49 decision 2, #52 decision 3). That's a UI
     rule the scout will inherit, because it starts on the capital too.
   - How the capitals, units and routes look. #32's build-4 never ran (`crew.md` §1, `briefs.md`
     §1), so the review chose houses, shields and dashed routes on its own (#53 decision 3), with
     no brief.
   - #31: mountains are `land: false` (#38 decision 2), which the scout's bridge has to undo.
     Also river crossing 2.0, marsh 3, floodplain fertility 2.2, and floodplain coast turning to
     plains and losing its sea foods (spec.md #31 decisions 4, 7, 8).

   **Proposed edit to `relay/prompts/session.md`:** split `decisions` into the ones a player
   or a later piece would notice and the rest, with a flag (`"design": true`), and have the crew
   list the flagged ones in the PR description under "For docs/design/". The owner then has one
   list to fold in, instead of picking them out of six comments.

2. **The spec settled design lines that weren't open.** `docs/design/README.md` says a session
   settles "a question they leave open". Three of #32's decisions changed explicit lines:
   - "the civ's clans' stores are empty at founding": not applied (#47 decision 5, read as
     narrative).
   - "works the tiles next to it": defined and drawn, but nothing works them (#47 decision 7).
   - "nearest civ clan": built as any civ's clan, then fixed by the review (`briefs.md` §7).

   Each reading is defensible, but they were recorded as ordinary decisions. **Proposed edit to
   `relay/prompts/spec.md`:** "If a line of the design can't or shouldn't be built as written,
   say so as a decision that starts 'Design says …', and the crew puts it at the top of the PR
   description."

3. **Ideas repeat across steps and go nowhere.** Adding `node www/check.mjs` to CI was raised 5
   times (#45, #46, #47, #53, plus #31's PR body). Working the capital's tiles came up 3 times,
   pre-game tuning 3 times (#40, #44, #46), starving feed noise twice (#44, #45) and a headless
   harness twice (#45, #53). None became an issue. **Proposed edit to `relay.mjs`:** gather each
   piece's ideas, deduplicated by title, into one "Ideas" section of the PR description when the
   review finishes. The planner and the retro can then read one list per piece.

4. **No step measured the main game.** #31 had a sweep for its question. #32 asked none, and its
   review played one warrior's life through the page (`review.md` §4) but never ran a civ for
   long. That's why the store's failure went unmeasured even though build-2 saw it. **Proposed
   edit to `relay/prompts/review.md`:** a piece that adds a loop running every turn (an income, a
   cost, a growth) runs it for N turns on a few seeds and reports the numbers. **Proposed engine
   chore:** `sansfiat sweep --main N` reports each civ's store, units, clans and people N turns
   after the pick. The scratch probe used here is about 40 lines.

5. **Builds that changed a brief's numbers recorded it well, and that should stay.** Builds
   changed `delta_river` from 12 to 10, river sources, the top-up's region rule and
   `centres_seen` (#42, #44). In each case they put the evidence in the summary and corrected
   `spec.md`. Nothing to change.

## Game

### Did any decision contradict `docs/design/` or the vision?

6. **Units and the levy move food for free.** A warrior 20 tiles from the capital eats from the
   store as if it stood beside it, and every civ clan's levy reaches the store wherever the clan
   lives. Vision, *No abstractions* 3: "Every unit, building and action costs real goods and real
   people's time". Both rules are in `founding.md`, so the builds followed the design. But this
   is the abstraction #29 (supply lines) exists to replace. **Proposed:** the supply interview
   (#29) covers carrying the levy in and food out to units. `founding.md` gains a line: "The
   store feeds units and takes the levy without carrying, as a stand-in until supply lines."

7. **The levy may be weakening the clans that pay it** (measured; suggestive, not proven). After
   founding, civ clans' people fell on 3 of 5 maps (small 2: 236 → 191 in 200 turns; medium 8:
   178 → 140; small 9: 189 → 169). Unaligned clans grew on 4 of the 5 (small 2: 513 → 597). The
   levy takes half of a fed clan's leftover store each turn, and clans.md's store is that clan's
   buffer against a short turn. Civ clans' average store fell from about 0.3 to about 0.1 after
   founding, while unaligned clans stayed near 0.2. Blue on small 2 went from 17 clans to 7. Civ
   clans may also sit on more crowded land, so this needs a controlled run (levy 0) before
   anyone acts on it. It's still worth the owner watching for (see *What to play*).

8. **A civ can only shrink against the world.** Clans don't join after founding (`founding.md`,
   out of scope), so a civ holds about a quarter to a third of the world's people at the pick
   (small 2: 236 of 749) and only its own daughters after that. The *Influence* win in the
   vision needs clans to join. That's not a contradiction now, but it's a scope line to lift
   before winning is designed.

9. **Smaller mismatches with the design's wording:**
   - `map.md`, deserts: "kept to a share of the land as in merge4". They come out about 12%
     against the 20% quota, because only plains can become desert (#41 idea).
   - `map.md`, rain shadow: "on the side away from the sea". The wind is random per map, so a
     desert can sit on the sea side of a ridge (#40 idea).
   - `founding.md`: "They don't have to be able to reach each other". Natural centres can be in
     different regions (medium seed 3 has no land route to a rival, #52), but topped-up centres
     are always in one region (build-4's rule, spec.md #31 decision 13). That's harmless, but
     worth stating in `map.md`.
   - `scouting.md` says "Each civ starts with one scout", and `founding.md` says each starts
     with one warrior. Whether a civ starts with both has to be settled before #28.

### Proposed edits to `docs/design/`

10. **`founding.md`, The store:** state the numbers as starting values to tune: cap 50, levy
    half, spoil as a clan's store (10%), order spoil → levy → feed units → cap. Replace "the
    civ's clans' stores are empty at founding" with the reason the endowment exists ("they'd be
    empty after a boon year spent; the endowment stands in"). Add what the cap stands for, per
    vision 4 (the capital's granary, say). Evidence: #47 decisions 5 and 8.

11. **`founding.md`, Founding:** either drop "and works the tiles next to it", or say the capital
    works them with its own people from a later piece. As written it promises an income that
    doesn't exist, and the measured stores show the gap. Evidence: #47 decision 7, the headline.

12. **`founding.md`, Units:** "Disbanding sends its person back to the nearest clan of its civ".
    Add "making one costs no food". Add the warrior's numbers (labour one person's work, food
    one person's need, three hungry turns). Add "a warrior holds no land, blocks nothing and any
    number share a tile, until the army piece." Evidence: #53 decision 1, spec.md #32
    decisions 9–14.

13. **`founding.md`, a civ's land:** "A civ's land is its capital's territory and the rounds of
    its clans; the tooltip names it." Or, if the owner wants territory to mean only what's held
    by force, say so before the #27 interview. Evidence: spec.md #32 decision 6.

14. **`founding.md`, The pick:** "distance to the nearest rival" is hex steps, plus a yes/no for
    whether any rival can be reached over land. Say whether that's what's meant or whether it
    should be the walking cost (#50 idea "Rival distance by path"). The cards show big
    differences (small 2: Red 18 clans / 98 people, Gold 12 / 59; medium 9 after the top-up:
    Red 87 people, Gold 52). If uneven starts are intended, `founding.md` can say so.

15. **`map.md`, Rivers and Also:** add the numbers the builds settled: crossing +2, marsh 3,
    floodplain fertility 2.2 (above the 2.0 cap), and floodplain coast becomes plains. Or say
    instead that a floodplain coast keeps its sea foods (#42 idea). Mountains are impassable and
    clans can't stand on them. Evidence: spec.md #31 decisions 4, 7, 8.

16. **`map.md`, On the page:** "Clans starving the same turn are one line" (#45 decision 2).
    Consider making the event itself "starving for N turns" in `clans.md`. Late in a run 15–50
    clans flip to starving every turn, hovering at the 0.8 threshold (#44, #45 ideas), so as an
    event it says little about why.

17. **`clans.md`, The pre-game ends:** small maps end by the turn limit one run in four (30 of
    40 on their own; #44, evidence.md). The cause is markets, not the map: only one market
    reaches 4 traders at 10-step spacing. Tier (b) of the top-up never fired in 90 runs. Either
    accept the top-up as the normal small-map ending, or tune `centre_clans`, reach or spacing
    (see candidate B).

### Candidate pieces for the next milestone

Against what's still open in M1: #27 (paint-army) and #29 (supply-lines) wait on interviews;
#28 (scout) is specced in the proposal but paused.

- **A. The store's income, before or inside #29.** Give the civ's store a real income and real
  carrying: the capital working its tiles, the levy carried in, food carried out to units.
  This is a design question first. The supply-lines interview is the natural place for it,
  because supply lines are about moving food. If #29's interview isn't soon, a small piece where
  the capital works its tiles with its own people would stop warriors starving. The ask:
  measured, 14 of 21 civs can't keep one warrior.
- **B. The pre-game's economy (a question piece).** Why do small maps get one busy market and
  several 1–2-trader ones? Sweep `centre_clans`, market reach and spacing. Evidence: #44 idea,
  evidence.md, 10 of 40 small seeds topped up. This is low risk, and the sweep already exists.
- **C. The main-game top bar and feed.** Once founding exists, the page is still the pre-game's:
  two "People" stats side by side (#52 idea), market reach drawn as 3 hex steps when the engine
  reaches by cost (#45 idea), the legend over the map (#46 idea), rivers faint at fit zoom
  (#46 idea), and starving lines every turn. This is the vision's "feedback when things happen:
  what changed, why, and where". It could fold into the next piece that touches the page.
- **D. Scout (#28), unpaused once A is settled.** Fold in: whether the civ starts with a scout,
  a warrior or both (§9); mountains as `land: false` (§1); and the click cycle on the capital.
  Without A, the review will watch scouts starve the way warriors do.
- **Chores, not pieces:** `sweep --main N` (§4); a save version so old logs fail with a reason
  (#49, #53 ideas); a headless play harness committed under `engine/www/` (`crew.md` §7);
  `check.mjs` in CI (owner, `crew.md` §3).

**Suggested order:** the #29 interview (with A's questions in it), B in parallel since it touches
only the pre-game, then #28 with D's fold-ins, then #27. C goes with whichever comes first.

### What to play on main now

The page is at https://dontpanic345.github.io/sansfiat/engine/. Add `?size=small&seed=2` (or
another size and seed) to the URL to start on a known map.

1. **Small seed 2, lead a civ.** Click *Run to end*: it stops at the pick, turn 277. Read the
   three cards. Do they tell you enough to choose, and does *Show* find each capital? Lead Red.
   Make a warrior, right-click a tile across a river and read the cost preview, then end turns
   until it arrives. Disband it. Look for: can you find your capital, units and route at a
   glance? Are the two "People" numbers confusing?
2. **Small seed 2, watch all three to about turn 420.** Look at Blue and Gold: their warriors go
   hungry and disband (about turns 350–360 and 405), and Blue's clans fall from 17 to about 7.
   Look for: does the feed tell you *why*? Is a civ that can't feed one person the game you want
   at this stage?
3. **Medium seed 8, lead and make several warriors.** The store empties fast (the #32 review
   drained it to 0 with 15). Even one warrior starves 22 turns after the pick when watched (Red,
   turn 267). Look for: is the
   store worth showing as "n / 50" when it never gets near 50?
4. **Small seed 3.** It ends by the turn limit at about 500, with two topped-up centres. Compare
   its cards with seed 2's. Look for: do topped-up civs feel worse to lead?
5. **Large seed 8, the map.** At fit zoom, can you find the rivers and the closed country? Turn
   on *View → hidden costs* and find a sealed valley's bridgeable mountain. Play at *fast* past
   turn 200, when the engine slows to 24–37 ms a turn (#45). Look for: is the legend in the way,
   and does a busy late map still read?

Your play notes on 2 and 3 decide whether A comes before everything else.
