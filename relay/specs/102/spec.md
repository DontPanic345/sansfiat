# #102 A civ's land: spec

## What it delivers

A civ's land stops being a 2-step ring and becomes what it **claims** and can **hold**. The player
(or the AI's stand-in) paints a claim out from the capital; the civ raises a **guard**, people who
stay on its land and divide their guarding over the claim as a camp divides over its zone, so a wide
claim is thin. Claimed tiles the guard holds are the civ's **land**; the rest of the claim is paper,
which a rival unit walking in takes. Clans whose homes are on a civ's land lean to it every turn,
more under a thicker guard, pay its levy at its market and count toward its Standing. Civ clans can
now lean elsewhere too, and a civ clan that leans to a rival more than to its own civ for a run of
turns leaves to it.

Alongside, the land stops feeling wrong in the ways M2's play notes listed: a clan keeps its festival
until it's clearly not worth the walk and keeps a new one some turns; festivals sit nearer and are
more; trails wear where traffic is, under homes and shared routes rather than under every worked
tile; desert costs 2 to cross. The page draws each civ's land as a border, tints leaning clans,
loses the capital's circles and the market ring, and shows who walks to a selected festival.

**The question:** on the retro maps, does a civ that claims and guards gain Standing over one that
doesn't, and does the lead change hands after the pick? Answered by the judge (below), against two
controls: nobody claims, and only one civ claims.

## Shape: Easel and Canvas, with two easels

Taken as the piece suggests. The rules of the claim, the guard, leaning and leaving by land, and the
page's border can each be said exactly, and so can the festival and trail rules; their numbers
can't. The festival and trail numbers are judged against measures that don't involve claiming
(flips, trailed homes, the pre-game ending), so their easel tunes them itself to targets given here.
Whether claiming pays and the lead moves is judged only on the whole, so it is the canvas.

| Step | Part | Area | Why here |
|---|---|---|---|
| build-1 | easel | Festivals stick, spacing and reach, trails by traffic, desert, with their measures | Changes the pre-game and who goes to which market; the land rules are tuned on top of it, so it goes first |
| build-2 | easel | Core land: the claim, the guard, land, leaning and leaving by land, the levy, Standing on land, the AI's stand-in, the `claims` control | The mechanic, on starting numbers that run |
| build-3 | easel | The judge's land checks and controls, the text view and `play`, the page: border, tints, claim painting, the guard, the festival's walkers | Reads build-2's state |
| build-4 | canvas | The land's numbers until claiming pays and the lead moves, health held; a played game; the screenshots | Needs everything under it |

## The digests

Every step changes rules, so every step moves `stats::digest`. `SAVE_VERSION` goes from 2 to 3 in
build-1 and stays 3 after (no save is kept between builds of one piece); each step that moves the
digest re-pins `the_save_version_pins_the_rules` in `cli/tests/save.rs`. `relay/specs/101/trial/g.json`
is a version-2 save that `cli/tests/play.rs` reads for one scenario (a clan in reach of Red's market
at its last turn): build-1 rebuilds that test on a game it makes in the test, keeping what it checks,
and leaves #101's file as #101's record. The proof a step didn't break
the world is the judge's health group, not compare.sh.

## Settled: festivals, trails, desert (build-1)

1. **Desert costs 2 to cross** (`TERRAIN[DESERT].move_cost`). Measured: the pre-game still ends 7 of 7
   by centres (`probe.md`).
2. **A festival sticks both ways.** `Clan` gains the turns it has gone to its current festival
   (`market_turns`, 0 with none). `markets::choose`: a clan with a festival keeps it, whatever the
   worths, while `market_turns < MARKET.keep_turns`; after that it keeps it while its worth is above
   `−MARKET.stay_margin` and no other beats it by `MARKET.switch_margin`. A clan with none goes to the
   best worth above 0, as now. A festival that fades frees its clans as now. A civ's own clan still
   always goes to the capital's. So stopping needs the walk to be clearly not worth it, and a clan
   that just started doesn't flip back.
3. **Nearer and more festivals.** `MARKET.spacing` 6 and `MARKET.reach_walk` 14 to start (`probe.md`:
   both end 7 of 7); build-1 may move them within 5–7 and 12–16 to meet its targets.
4. **Trails by traffic.** `Tile.walked`, `walk_run` and `rest_run` give way to `Tile.traffic`, the
   people-walks across the tile, smoothed: each turn `traffic = TRAIL.keep × traffic + this turn's`.
   This turn's: every clan adds its people once to each tile on each walk it made inside its
   territory (`Walks`' paths, **home included**: every walk starts there), and its people to each
   tile of its festival route; a camp adds its people to each tile of its line on a turn it
   resupplies; a scout its people to each tile it walks. A tile with `traffic ≥ TRAIL.on` gets a trail;
   one with a trail loses it when `traffic < TRAIL.off` (`off < on`), except that a tile on a camp's line
   or a scout's route **in use** this turn keeps its trail (the scout's built trails stay while its route
   is walked, owner note 9). No trail on a mountain unless built, as now. Built trails (`trail_work`)
   are unchanged and then live by traffic like any other.
5. **Measures** (build-1 adds them to the judge, `sweep --judge`, as a `land` group at the pick and at
   +200, and records main's numbers before changing anything):
   - `homes_trailed`: share of clans of `CLAN.split_at / 2` people or more whose home has a trail;
   - `held_trailed`: share of held tiles that aren't a home and have a trail (main: 72–79%, M2 retro);
   - `route_trailed`: share of tiles on two or more clans' festival routes that have a trail;
   - `markets`: festivals at the pick and +200; `trading`: share of clans going to one;
   - `festival_changes` and `flips` per 100 clan-turns after the pick: a change is a clan's festival
     differing from last turn's (start, stop or switch); a flip is a change back to the festival (or
     none) it had within the last `MARKET.keep_turns` × 2 turns.
6. **Targets** (build-1's done): pre-game 7 of 7 by centres; `flips` at least 80% below main;
   `homes_trailed` ≥ 0.8; `held_trailed` ≤ 0.35; `route_trailed` ≥ 0.5; `markets` at the pick at least
   main's. Numbers it lands on go in `data.rs` with a note; misses are reported, not hidden.

## Settled: the land (build-2)

### The claim

7. **`Civ.claim: Vec<usize>`**, sorted, saved with the civ (`CivFlat`, `#[serde(default)]`): a connected
   set of land tiles (`TERRAIN[..].land`) holding the capital. Claims may overlap.
8. **`Order::Claim { tiles }`**, addressed to the civ (`{"type":"claim","tiles":[…]}`), wire name
   `claim`: replaces the claim. `check_order` refuses it unless the tiles are land, on the map,
   connected, and hold the capital (the capital may be left out of the list: it is added, as the
   camp's own tile is to its zone), and refuses it when `Config.claims` doesn't let this civ claim.
   It costs nothing and has no size limit: a civ can claim land it can't hold.
9. **At founding** every civ claims the AI's stand-in claim (item 22) when it may claim; otherwise its
   claim is its capital alone. The player repaints from there.

### The guard

10. **The guard is a unit**: `UnitState::Guard`, one per civ, at the capital (`home` is the capital and
    never moves), its actor's territory the claim. It is raised and disbanded as the camp is, by
    `Order::Raise { n, kind }`, where `kind` is `"camp"` (the default, so `{"type":"raise","n":2}` keeps
    its meaning) or `"guard"`; `Disband` names the guard's unit. A guard refuses `move`, `zone`,
    `switches`, `forage`, `pillage`, `resupply`, `route`. Its people come from the civ's clans one each,
    as warriors do, so raising it moves no Standing.
11. **It eats from the store**, `UNIT.food` a person a turn, at the capital: no pack, no line. When the
    store can't feed it, it goes hungry, and hungry `UNIT.hungry_turns` turns it loses one person a turn
    back to the civ's own clan, as a camp's warriors do. `civs::keep` adds `GUARD.keep_turns` turns of
    the guard's food to what the civ keeps out of posting and gifts. Holding costs food, which is the
    levy's; that's the cost of a wide claim.
12. **The guard divides** `people × GUARD.guarding` over the claim with `actors::divide`, each tile's
    share paying its walk from the capital (`Walks`, at `UNIT.inside`), as `camp::hold` does over a
    zone. A tile's **guard** is that share plus the guard share of any of the civ's camps holding it.
    `Civ.guard: Vec<f64>`, aligned with `claim`, saved, set in the `claim` step (item 14).
13. **Land.** A claimed tile whose guard is at least `CLAIM.floor` is **held**, and so is the capital
    always. A civ's **land** is its held tiles; the rest of its claim is **paper**. `civs::tile_civ(i)`
    becomes: the civ whose land holds `i`; held by two, the one with the thicker guard there, a tie to
    the lower id. `CIV.territory`, `civs::territory` and `civs::work_tiles` go, and the clans' territories
    no longer make land. Everything that read `tile_civ` reads land: line blocking (`line_by`, `holder`),
    forage against pillage (`rival_of`), the AI's post, the pick's cards, the judge's `land_changed`.
14. **The `claim` step**, after `units` in `TURN`: each civ's guard is divided (item 12); then each unit
    standing on a tile in another civ's claim where that civ's guard is under `CLAIM.floor`, and which
    isn't its capital, takes the tile out of that claim, and with it the claim's tiles no longer
    connected to the capital. `ClaimTaken` event (the civ that lost it, the unit's civ, the tile, the
    tiles lost), a note on both civs, and an alert for the player's civ. The guard is divided again
    after a take.

### Leaning and leaving

15. **Every clan leans**, a civ's own clans included: `Clan.leaning` is kept for all of them, and every
    leaning fades by `LEANING.fade` a turn (the `lean` step).
16. **Leaning by land.** In the `lean` step, a clan whose home is on a civ's land gains
    `LEANING.land + LEANING.guarded × g` to that civ, `g` its home tile's guard. A note when it starts.
17. **Gifts and trade.** A civ's gifts go to every clan at its market that isn't its own (unaligned and
    rivals' clans); trade leaning goes to every clan at its market, its own included. Both are
    otherwise as now.
18. **Joining** stays: an unaligned clan joins the civ it leans to most once that reaches
    `LEANING.join`. A clan that joins, at founding or by leaning, starts with its leaning to its civ at
    `LEANING.join` and its other leanings cleared.
19. **Leaving.** `Clan` gains `drawn: u32`, the turns in a row its strongest leaning to another civ has
    been more than its leaning to its own. At `LEANING.leave_run` it leaves to that civ: as joining
    (item 18), plus the `ClanLeft` event (its old civ, its new, the clan), notes on all three, an alert
    for the player's civ when it is either. **Its daughters go with it:** `Clan` gains `parent:
    Option<u32>`, set when a clan divides or sends a daughter; when a clan leaves, its living daughters
    still of its old civ leave with it, each its own `ClanLeft`. A civ's own clan never leaves (the
    capital stays its civ's).
20. **The levy by land.** A clan at a civ's market pays its levy if it is the civ's or its home is on
    the civ's land. A civ clan at another festival still pays nothing.
21. **Standing** of civ `c` = (people of `c`'s clans, wherever they live + people of `c`'s units +
    people of the unaligned clans whose homes are on `c`'s land) ÷ (people of every clan and unit).
    `stats::standing` changes; nothing else about it does.

### The AI's stand-in and the control

22. **The AI claims** (until R) the land tiles within `CLAIM.ai_ring` hex steps of the capital or of
    the home of any of its clans whose home is within `CLAIM.ai_reach` steps of the capital, kept to
    the part connected to the capital: a ring of its clans' homes. It sends `Order::Claim` in the `ai`
    step whenever that differs from its claim. **Its guard**: it raises one a turn while some claimed
    tile is under the floor and its store, after `keep`, holds `GUARD.keep_turns` turns of food for one
    more guard; it disbands one when its guard went hungry last turn. These are stand-ins: the canvas
    may change them; R replaces them.
23. **`Config.claims`**: `all` (the default; old configs load with it), `none` (the control: no civ
    claims beyond its capital or raises a guard; `Claim` and a guard's `Raise` are refused), or
    `only(k)` (the k-th civ in id order, 0-based, claims and guards as under `all`; the others as under
    `none`). Serde as `"all"`, `"none"`, `{"only":1}`.
24. **The numbers**, in `data.rs` with notes, starting values that run (the canvas tunes them):
    `CLAIM { floor: 0.1, ai_ring: 1, ai_reach: 4 }`, `GUARD { start: 3, guarding: 1.0, keep_turns: 5 }`,
    and `LEANING` gains `land: 0.01, guarded: 0.1, leave_run: 10`. At founding each civ that may claim
    raises `GUARD.start` guards beside its camp.

### Protected

25. **"Protected"** in this piece means: a held tile isn't taken by a rival unit standing on it (item
    14), and a guarded home leans more (item 16). Fighting for a held tile, and a clan's militia, are A's.

## Settled: the judge, the text view, the page (build-3)

26. **The judge's controls.** `sweep --judge` runs, per map, the army run twice (`claims: all`), the
    `--no-army` control as now, and a **no-claims control** (`claims: none`). With `--solo` it adds three
    runs, `only(0)`, `only(1)`, `only(2)`. The text and `--json` gain what's below; the time is printed.
27. **New measures**:
    - `standing.on_land_200`: mean over maps of the share of the world in unaligned clans on some civ's
      land at +200;
    - `standing.solo_gain` (with `--solo`): per civ, its Standing at +200 when it alone claims less its
      Standing in the no-claims control; reported per map, and in total the mean and the count of civs
      above +0.005 of 21: the question's first half;
    - `doubt` beside its no-claims control: lead changes and maps with a change in each;
    - `motion.clans_left` (a civ clan changing to another civ, from `ClanLeft`), `motion.clans_joined`,
      `motion.claim_taken` (tiles taken by rival units), and `land_changed` as now (it reads land);
    - `land.claimed`, `land.held` and `land.guard` per civ at +200 (mean tiles, mean guard people).
28. **The text view** (`cli/src/view.rs`) gains, for the led civ: claim tiles, held, paper, the guard's
    people and its thinnest held share; its clans and the unaligned clans on its land; and in the legal
    orders `{"type":"claim","tiles":[…]}` and `{"type":"raise","n":1,"kind":"guard"}`. `map` marks the led
    civ's land and paper. Still under 80 lines at the pick and +100 on small seed 2.
29. **The page.** `Sim.view()` gives each civ its `claim`, `guard` and `land` (held tiles); `territory`
    and `work` go. The map draws each civ's land as a border in its colour, solid round held land and
    dashed round paper; drops the capital's work circles and the market reach ring (`map-view.js`);
    tints each unaligned clan's circle by the civ it leans to most, stronger as the leaning nears
    `LEANING.join`; and when a festival is selected, draws the routes of the clans that go there and
    lists them in its panel. The civ's panel gains Claim / Unclaim painting (as the camp's zone paints,
    the capital kept and the claim kept connected), the claim's counts, and Raise guard / Disband
    guard. The tooltip names the tile's land's civ, or "paper of X". The feed and alerts carry
    `ClaimTaken` and `ClanLeft`. `www/play-claim.mjs` leads a civ, paints a claim out, raises a guard,
    ends ten turns and screenshots the border and the tints; it exits non-zero if the border or the
    claim's counts are missing. Like the other play scripts it isn't in `check.sh`; build-3, the canvas
    and the review run it.

## Left unsettled: the canvas (build-4)

The numbers of items 11, 12, 16, 19, 22 and 24, and the stand-in's shape, are found by running. The
canvas may change any easel rule that the runs show wrong, saying so as a decision. What proves it:

- **Claiming pays:** `solo_gain` above +0.005 for at least 14 of the 21 civs, and its mean positive.
- **The lead moves:** lead changes (margin `JUDGE.lead_margin`) on at least 4 of 7 maps in the claims
  run, and more than in the no-claims control. Main had 7 on 4 maps (`main-judge.txt`).
- **Health held:** pre-games by centres 7 of 7, panics 0, digest mismatches 0, people against the
  `--no-army` control at least 0.95, no civ left with no clans at +200, and the festival and trail
  targets of item 6 still met.
- **A wide claim is thin:** on one retro map, a led civ that claims everything within 6 steps with
  the starting guard ends with less land held at +100 than one that claims the stand-in ring. Played
  through `sansfiat play`, with `sansfiat judge` on each.
- **Motion is not a rout:** `clans_left` reported, and no civ loses half its clans by +200 on a map.

If claiming doesn't pay on the retro maps however it's tuned within the rules, the canvas says so with
the numbers, which answers the question no.

## Decisions (the owner can overrule)

- Design says land is what a civ claims (`war.md`); this piece counts only the **held** claim as land
  for Standing, leaning, the levy and lines, because "unguarded claim is paper" and otherwise the
  biggest free claim wins: a civ would gain Standing by painting the map.
- The guard is one unit per civ at the capital, eating from the store, with no pack or line; the camp's
  held tiles add to the guard on its own claim.
- Overlapping held claims go to the thicker guard, a tie to the lower id (as `supply.md`'s contested
  tile does); fighting over them is A's.
- A rival unit takes a tile only by ending its turn on it; the part of the claim cut off from the
  capital goes with it.
- Every clan leans now, civ clans included; gifts go to rivals' clans at a civ's market too, so a civ
  can win a rival's clans; a joining clan starts at the join threshold to its new civ.
- "Its daughters with it" is read as: its living daughters still of the old civ, one generation,
  tracked by a new `parent` field. A civ's own clan never leaves.
- Unaligned clans on a civ's land pay its levy at its market, as its own clans do.
- Festivals' and trails' numbers are tuned by their easel against measures that don't involve
  claiming, not by the canvas.
- `sweep --judge` gains the no-claims control always and the solo runs behind `--solo`, so later pieces
  don't pay four extra runs a map.

## What the review plays and measures

1. `./check.sh` passes; `cargo run --release -p sansfiat -- sweep --judge --solo`, and compare with
   `main-judge.txt` and build-4's `canvas-judge.txt`: the question's two halves and health.
2. Play small seed 2 through `sansfiat play` for 30 turns: paint a wider claim, raise a guard, watch the
   held count and a clan on the land lean; `sansfiat judge` the game. Does the view say what happened?
3. `node www/play-claim.mjs` and look at the screenshots: the border reads at a glance, no capital
   circles or market ring, leaning clans tinted, a selected festival's walkers shown.
4. The PR description answers the question with the solo gains and lead changes beside the controls
   and main, and gives the festival and trail measures beside main's.
5. Decided ledgers: `war.md` (claim, guard, land as the held claim, overlap, taking), `clans.md`
   (festivals stick, trails by traffic, every clan leans, leaving and daughters), `founding.md` (levy
   by land, gifts to rivals' clans, joining's start), `supply.md` (lines and forage read land),
   `map.md` (desert costs 2). The review also notes in an idea that `founding.md`'s "Clans joining or
   leaving after founding" out-of-scope line is now overtaken.

## Not in this piece

Fighting over a contested tile, militia, many armies (A); the AI's claiming beyond the stand-in (R);
goods; Standing in the page's top bar (P).
