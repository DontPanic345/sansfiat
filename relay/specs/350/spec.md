# #350 Fixes for M5

**Question:** does each `fix` finding of M5 now work as the line it names says, with its repro a test on a staged
world (or on small seed 2 for a text-view line, as `cli/tests/play.rs` does) that fails on main and passes here?
Nothing here is judged by the first four scores: the piece restores lines and adds no rule, number or lever. The
one score it can touch is Health (the digest), and only through finding 4. No play step.

**Shape: Henry Ford**, kept. Every finding is settled by the line it names, and the fixes split along one seam:
the engine and the text view (build-1), then the page (build-2). Neither needs the other.

The findings' games don't replay on main (their saves are versions 23 to 26; main reads 32), so every repro below
was re-made on main, or read from the code, and each test stages its own world.

## Findings and what settles each

### 1. The claim order says what it does and names its tiles (build-1)

Line: m5.md's #336 Decided, "The view says what a claim order does to the claim and names its tiles." Today the
legal line (`cli/src/view.rs` `legal`, the `Order::Claim` arm) reads `paints your claim (now 43 tiles)`. A claim
order *replaces* the claim with the tiles it lists (`orders.rs` `Order::Claim`: `claim::check` then `claim::set`),
so a 9-tile order drops every other tile and the clans whose homes are on them; nothing says so before it is sent.
`claim::change` says it afterwards, as a note ("last claim order" in the land block), which is not a preview.

Fix, text only:

- The legal line becomes the order with the current claim's tile ids filled in, then what it does:
  `{"type":"claim","tiles":[56,76,…,<tile>, ...]}  sets your land to exactly the tiles you list, with your capital and
  settlements: these are your 43 tiles; a tile you leave out is dropped, and a clan whose home is on it is no longer
  yours`. The ids are `c.claim` less the roots (`settle::roots`), which `check` adds back, so the list can be
  copied as it stands. The clause on clans is shown only when one of the civ's clans lives on its land (it always
  does on the real maps; the build may show it always).
- When the reach has a claim (`engine::outlook::reach(w, me).claim`, a `ClaimTo` with `tiles`), a second line gives
  that claim as a whole order ready to send, the current claim's ids plus the reach's tiles, and names what it adds:
  `{"type":"claim","tiles":[56,…,307]}  the claim in reach: adds 1 tile, 307 (-3, -5), and brings clan 429`. Its
  tiles are named as the view names a tile (`tile()`: id and hex). This is the open idea of #427.

Repro and test, `cli/tests/play.rs` (small seed 2 civ 1, the pattern of
`the_view_at_the_pick_lists_the_claim_and_its_new_legal_orders`): at the pick the claim line holds every id of the
land line and says a tile left out is dropped; the reach order is found in the view, sent as a line, accepted
(`ok`), and the next view's land is the old land plus exactly the reach's tiles, the named clan now the civ's (the
`clans:` block), and the civ's Standing not lower than before. On main the claim line holds no ids and there is no
reach order: the test fails. The pick's view stays under the test's 90 lines.

### 2. A signpost that draws nobody says why (build-1)

Line: clans.md (#353): a daughter or a clan moving on "may go to its civ's signpost … by chance weighted by the walk
and the tile's worth". The rule works: in the repro the tile's worth fell from 106% to 0% because other clans
settled beside it and one of them now gathers it (a tile another clan holds feeds a newcomer nothing:
`clans::reckoned`, used by `site_value` through `settle::worth`), and `settle::draw` passes over a signpost whose
worth is under `SETTLE.fed` (90%). What is broken is that the view says "would feed a daughter 0%" and nothing more,
`why <civ>` shows only the post, and the player cannot tell the signpost will never draw anyone. The finding's
"lists no good site after it" is the rule too: `signpost_sites` keeps only free tiles that feed above `SETTLE.fed`
within `SETTLE.plan_range`, and none was left; the legal line already says "none free nearby".

Fix, a reading in core used by both front ends (no rule, no digest change):

- `settle::idle(w, tile) -> Option<String>` (or a name the build likes): `None` while `settle::worth(w, tile) >
  SETTLE.fed`; else why it draws nobody: when `w.tiles[tile].claim` is a clan, "clan N gathers this tile, so it
  feeds a newcomer nothing"; else "its ground would feed a daughter only N%"; both end "a clan goes only where it
  is fed at least 90%: remove it and post on a free site".
- The text view's signpost line (`view.rs` `settle_lines`) adds it after the worth; the wasm view's `signposts`
  entries (`web/src/lib.rs`, near line 330) gain `idle` (the text or null); the page's signposted tile panel
  (`www/panels-land.js`, the Worth row) shows it as a reason line or in the row's tip. The page part is build-1's
  too: it is three lines beside the view field, and build-2 does not touch `panels-land.js`.

Test: `core/tests/settle.rs`, a staged world with a signpost on a free tile whose worth is above `SETTLE.fed`
(`idle` is `None`), then a clan's claim set on that tile (`w.tiles[t].claim = Some(k)`): `idle` names clan k, and
`settle::draw` from a neighbouring clan of the civ returns `None`. It fails on main by not compiling (the reading is
new); also assert the text view line through `cli/tests/play.rs` only if a small-2 signpost can be made idle in a
few lines (it needn't be).

### 3. The army's line says which store it stands at (build-1)

Line: supply.md Decided (#354), "A unit's line runs to the store of its civ, capital or settlement …". The finding's
"at the capital" text was on the version-23 build: #365 changed it to "at its store, no limit", and on main army 448
would read `at its store, no limit, draws on its settlement at (6, -2)` (the spec's probe built `da69498^` and
replayed the log to read it). The literal words are gone. What remains wrong on main is that `view.rs` `unit_line`
(line ~357, `carry.is_infinite()`) still doesn't say where the army is. It can also be false: `units::line` can end
at a market (`exchange::market_end`). An army on a market tile whose stall holds food then reads "at its store" while
`draws_on` names another store.

Fix, view text (`cli/src/view.rs`, about 10 lines). Take the line's end, `tiles.last()` or the unit's home when the
path is empty. When `settle::store(w, u.civ, end)` is some, it reads `at {settle::store_name(..)}, no limit` ("at
your capital", "at its settlement at (6, -2)", whatever `store_name` says). Otherwise it reads `at the market at
{tile}, no limit`. `draws_on` adds the store line's days when the line isn't empty (`draws on its settlement at
(6, -2), 6.0 days`).

Test: a `#[cfg(test)]` module in `view.rs` (the pattern of `cli/src/game_judge.rs:366`), or `cli/tests` if the
binary's functions can't be reached. Stage the world of `core/tests/stores.rs` (`stage()`, a capital and a
settlement) and stand an army on the settlement's tile. `unit_line` names the settlement and does not say "at its
store". This fails on main.

### 4. Why an army draws on the store it does (build-1)

Line: the same Decided line, "… that feeds it for the least labour, and draws on that store alone". In the repro the
rule is working as it is coded. Army 490 on 505 has a stocked capital (49.2 food) one step away and a stocked
settlement (22.8) three steps away. The step into the capital tile (marsh, a river, a trail with no trail cost)
costs 4.0. The three trail steps to the settlement cost 1.0 each, 3.0 in all. `line_among` doubles the path out
(`actors::walk_cost`), giving 8 days against 6, so the settlement wins. The broken half is that `why 490` says only
"its line brings food to the camp" (`units/camp.rs` ~356), once, when supplying starts. It names no store and gives
no reason the nearer one lost, so the player sees a nonsense line.

Fix, a note (notes are `#[serde(skip)]` and write nothing a digest reads, so no `SAVE_VERSION` change):

- `units/line.rs` gains a reading, for example `pub fn store_choice(w, unit) -> Vec<(usize, Option<f64>, bool)>`.
  For each of `stores(w, unit)` it gives the line's days (`walk_cost` of `line_among` with that one end; `None`
  when no path), and whether the store holds a turn of the unit's food. It reuses `line_among`, so it can't drift
  from the choice.
- In `camp.rs`, when the camp is supplying and either supplying starts or the store it draws on (`store_of`)
  differs from the last note's, the note names the store and why: `its line brings food from its settlement at (6,
  -2), 6.0 days: the least labour of its stores (your capital 8.0 days)`. Each other store is shown with its days,
  as `holds under a turn of its food`, or as `cut off`. The build may keep the store it last named in a
  `#[serde(skip)]` field, or compare against the last note's text. It must not write a note every turn.

Test, `core/tests/stores.rs`, beside `a_line_ends_at_the_nearest_store_that_holds_a_turn_of_food`. Twin 1: the army
next to the capital, the capital's tile made dearer to enter than the walk to the settlement (marsh, set in
`stage.world_mut()`), pack half full, one turn. `store_of` is the settlement, which is already true on main, and
the army's note names the settlement and the capital with both lines' days, which fails on main. Twin 2: a plain
capital whose store is under a turn of the army's food, so the note says the capital "holds under a turn".

The pricing itself (twice the path out, a step's cost charged on the tile entered, so the return walk is not priced)
is how every line, ring and scout route is priced. It stays. Changing it would move the digest and is an idea for
the owner, below.

### 5. Salt prints as 0.0, never -0.0 (build-1)

Line: goods.md Decided, "A store's panel lists every good." The finding's `salt -0.0` was on #355's branch before
its review: main's store line (`view.rs`, `stock`) prints `v.max(0.0)` since #375, and Rust's `max` gives +0.0 for
-0.0. A small-2 game on main, led as civ 1 and viewed every 10 turns for 110 turns, prints no `-0.` anywhere
(spec run). The page's `f2` gives "0" for -0. So the finding is met on main and has no failing repro.

Fix: none needed in the formatting. The build adds a guard so it stays met: a test on a staged world whose civ
store holds `-0.0` and `-1e-12` salt, rendering through the same function the view uses. If the view's store line
can't be reached from a test without the binary, move the formatting of a store's goods into a small `pub(crate)
fn stock(store) -> String` in `cli/src/view_goods.rs` and give it a unit test there (`#[cfg(test)]`, as
`game_judge.rs` has). This test passes on main; the summary says so, as the one exception to "fails on main".

### 6, 7, 8. The page: an armed click stays armed until Escape (build-2)

All three page findings have one cause, which the spec's probe found in the #338 live log and then reproduced in
headless Chrome (a probe on a `/tmp` copy of `www`, failing on main and passing with the change below). **Move to…,
Claim/Paint and Unclaim/Unpaint, and the P and U keys, are toggles**: a second press turns the mode off.

- **6, Move to… then a click.** `www/screens/orders.js` `arm` (line 47): `routing.arm = routing.arm === kind ? null :
  kind`. In the log (`relay/specs/338/play/small-2.live.jsonl`, around t313-317) the player pressed Move to… (on),
  hovered, pressed it again (off, the hint back to "Right-click a tile…"), then clicked (-4,-2): with nothing armed,
  the click went to `onPick` and selected clan 307. Line: page.md, a unit's row holds Move to…, and "Escape clears a
  selection or an armed click".
- **8, the first claim drag after a selection only selects.** `www/screens/play.js` lines 657-658: P sets painting to
  null when it is already `paint` or `claim` (U the same for unpaint). Painting survives End turn (`refresh` keeps
  it; only `select` of another thing, line 471, and Escape clear it). In the log (t289-291, the claims at 14,-1 and
  12,-2, and at least 12 more) Claim was still on from the last turn, the player pressed P, which turned it off, and
  the one-tile drag (no movement, `map/input.js` 57-61) became a pick that selected the clan. Line: page.md, "P and
  U paint".
- **7, no one-touch way back to the civ panel.** No claim moves the panel: a paint keeps `civ:` selected. The clan
  panel after a claim was finding 8's pick. With 8 fixed the panel stays on the civ through every claim. The one
  touch back from anywhere already exists: your Standing chip in the top bar selects your civ (`bar.js` 30-33,
  `selectCiv`; `play/civ.mjs` 47-50 proves it), but nothing says so: the player cycled the capital instead.

Fix (about 6 lines of code):

- `arm(kind, h)` arms `kind` and never disarms (`routing.arm = kind`); its doc comment says so. Escape (`play.js`
  ~633), a click off the map (`orders.js` ~71) and a done order still disarm.
- P sets `paint` and U sets `unpaint` (`setPainting('paint')`, `setPainting('unpaint')`), never null. The panel's
  Paint/Claim and Unpaint/Unclaim buttons do the same (`panelDraw`'s `paint: (mode) => setPainting(mode)`, line
  531), so a button and its key agree. Escape stops painting, as the hint already says.
- The chips' tooltip (`bar.js` ~151) says a click selects that civ ("Click to select your civ" on yours; the
  rival's chip, if it selects too, says so). `play/live.mjs`'s header lists the chips as buttons by their civ name,
  so a script session sees the touch (it is the driver the play steps read the page through).

Tests (each fails on main, passes here):

- `www/play/camp.mjs` lines 132-138 pin the toggle ("a second press of Move to… disarms it"). Change it: Move to…
  pressed twice leaves `body.dataset.armed === 'move'`, then a click on a free neighbour sets
  `game.view().units[camp].dest` to that tile. Escape still disarms (the existing check above it stays).
- `www/play/civ.mjs`, after its claim (~line 106; the radius-8 plains stage, the civ selected by its chip, free
  tiles from `nb`): with Claim on, end the turn, press P, click a second free neighbour: the claim grows by that tile
  and the selection is still `civ:<id>`. Then click a clan on the map, then the civ's chip: the selection is
  `civ:<id>` again (one touch), and the chip's title names the touch.
- `www/play/buttons.mjs` passes as it stands (its Claim/Unclaim cases switch mode, which the change keeps); if a
  case pressed a button twice to turn it off, it changes to Escape.

## Took

- **A signpost whose tile is taken should say so or go** (#353 review): the "say so" half, as item 2. Making the
  signpost go is a rule (the owner's), and stays open.
- **No one-touch way back to the civ's panel** (#338 review): the same as finding 7.
- **The claim order names a tile by coordinates but gives no tile id** (#427 play): item 1's reach order names
  every tile by id, ready to send.

## Decisions I made

- The claim order is shown as the whole order it is: the legal line carries the current claim's tile ids, says
  the order sets the land to exactly the listed tiles, and says what leaving a tile out does. The order itself is
  unchanged (a repaint stays a repaint): an order that adds tiles to the claim would be a new order, not a fix.
- The reach's claim is printed as a ready order (the claim's ids plus the reach's tiles). It names tiles by id, as
  the order takes them (the #427 idea).
- A signpost that can't draw anyone stays (removing it would be a rule the owner hasn't chosen); the view and the
  page say why it draws nobody, naming the clan that gathers its tile.
- The salt finding is met on main since #375; its test is a guard that passes on main.
- Finding 3's literal words are gone on main (#365). The fix is what the finding asks for: the view names the store
  or market the army stands at.
- Design says a line runs to the store that feeds a unit "for the least labour". The code prices a line as twice its
  path out, so a store reached by trail can beat a nearer one across marsh (finding 4). That is the rule as built for
  every line, and it stays. The fix is the reason in `why`, and the pricing goes to the owner as an idea.
- Move to…, the paint and claim buttons, and P and U arm and never disarm. Escape (or a click off the map, or a new
  selection) disarms. page.md's keys line says Escape clears an armed click, and a toggle made a second press turn
  the mode off unseen. `camp.mjs`'s pinned "a second press disarms" changes with it.
- Finding 7 needs no new button. With finding 8 fixed, a claim leaves the civ selected, and the Standing chip already
  selects your civ in one touch. The chip's tooltip says so, and `live.mjs` lists the chips. Putting the civ first in
  the capital's cycle would break "your unit first", which `civ.mjs` relies on.

## Unsettled

Nothing. Every build has an exact done: each named test fails on main (stash the fix, or for a new function,
the test with its call reverted) and passes here, `engine/check.sh` passes, and the page scripts pass.

## Measuring

Nothing that moves the world changes: every engine change is view text, a derived reading, or a note
(`#[serde(skip)]`). So no score moves and there is no sweep. The proof of Health is that the digests stay main's:
`cli/tests/save.rs`'s pins pass unchanged and `SAVE_VERSION` stays 32 (`engine/check.sh` runs both). If a build
finds it must move a digest, it stops and says so: that would be outside this piece.

The real-map check is the text view on small seed 2, the map the findings came from. Item 1's test sends the reach's
claim order there. The spec ran it by hand on main, with the land line's 43 ids plus tile 307: accepted, land 43 to
44, clan 429 joined, Standing 21.5% to 22.3%. Build-1 writes a short log of a small-2 session (led as civ 1, the
view at the pick and after `end 30`, the claim line, the reach order sent, a signpost posted and its line, an army
raised and its line and `why`) to `relay/specs/350/view.md`, so the review reads the words a player meets.

## For the review

- Run `engine/check.sh`, and check that `SAVE_VERSION` and the save pins are main's.
- For each test named above, revert its fix on the branch (or check out main's file) and see it fail, as the builds
  record, for at least items 1, 4 and 6/8.
- Read `relay/specs/350/view.md`. Then play small seed 2 as civ 1 with `sansfiat play` for a few lines: send the
  reach's claim order from the view as printed. It must be accepted and grow the land by exactly the tiles it names.
- On the page (`node www/play.mjs` or `play/live.mjs`, small 2): select the army, press Move to… twice and click a
  free tile. The army gets that destination. Claim with P, end a turn, press P and click: it claims and the panel
  stays on the civ. Click a clan, then your chip: the civ panel is back.
- Write the answer per finding into the PR, with what was already met on main (items 3's words and 5).
