# #152 build-4 (easel): the page shows the end

Read `spec.md` items 20–25 and decisions 3–4, and `docs/design/page.md` (Screens, the end;
its Decided ledger's last line is the one this step retires). The code: `web/src/lib.rs`
(`view`, `phase`, `pick`), `core/src/civs.rs` (`cards`, `Card`), `core/src/over.rs` and
`World.standing` (build-2), `www/screens/end.js`, `www/screens/play.js` (`standing()`, the
turn loop, Play), `www/model.js` (its `history`), `www/bar.js` (the phase chip, the turn),
`www/screens/pick.js`, `www/panel.js` (how a button greys with `check`'s reason),
`www/main.js` (`game.stage`), `www/save.js`, `www/check-stage.mjs`, `www/play/end.mjs`,
`www/play/pick.mjs`, `www/play.mjs`.

## What to build

1. **`Sim.view()`** gains `over` (`null`, or `{winner, why: "conquest" | "limit" | "fell", by,
   turn}`, `by` null but for fell, `turn` the world's turn it ended on), `limit` (`{turns, at}`:
   `over::limit` and `over::ends_at`; `null` before the pick) and `standing`
   (`[[turn, {civ: share}]]` from `World.standing`, the turn being the world's). `phase()` reads
   `"over"` in `Over`. Update the doc comment of `view` and `phase`.
2. **`civs::cards`** carry `limit: u32` (`over::limit`, the limit in force, turns after the pick); the
   card's last line reads "The game ends after 200 turns, at turn 437" (`at` = the turn now
   plus the limit, since the pick is now).
3. **`model.js`** drops its own Standing history and reads `v.standing`; the end screen and
   anything else that read `model.history` read the engine's. A loaded game (Continue or a
   file) has its whole line.
4. **The top bar** shows the turn as "turn 312 of 437" from `v.limit` in the main game, and in
   `over` the phase chip reads "Game over" (its note "Blue won"); its tooltips say what each is.
   `pre = v.phase !== 'main'` and the like must treat `over` as the main game for everything
   but orders.
5. **The end screen** (`end.js`), `show({view, player})` (or the args you need) builds:
   - the header and the line under it, as `spec.md` item 23 words them, for won, lost and
     watched, and before the end ("Turn 312 of 437" / "Blue leads", the leader by the judge's
     rule is not needed: the highest Standing now);
   - the chart as now, from `view.standing`, a fallen civ's line included;
   - Play on and New game as now.
   Remove "The game goes on" and the comment about R filling `over`.
6. **When the game ends** (`play.js`, where a turn's view is taken): the first view in `over`
   stops Play (as the pick does), and shows the end screen. It shows once per game: Play on
   returns to the map, End turn and Play keep working (the world runs on), and Game → Standing
   so far opens the screen again. A game loaded already over opens on the map, not the end
   screen. Orders' buttons grey with the engine's "the game is over" (the panel's `check`):
   check that it does, and fix it if a button doesn't go through `check`.
7. **`check-stage.mjs`**: a staged game with `limit: 2`, two `EndTurn`s through `Sim.act`:
   `view.phase` is `"over"`, `view.over.why` is `"limit"`, `view.standing.length` is 3, and
   `Sim.check` of an order gives "the game is over".
8. **`www/play/end.mjs`** rewritten: on staged games (`game.stage(spec)`, `StageSpec`'s
   `civs`, `lead`, `armies`, `limit`), every step through the page's own buttons and keys:
   - **Conquest**: two civs, led by the first, its army on the rival's capital with the
     rival's capital clan small (find what falls in a turn or two; build-2's `over.rs` test
     has one). End turn until the end screen shows: the header "You won", the line "last
     capital", a polyline per civ. Screenshot `end-won.png`.
   - **Fell**: three civs, a rival's army on the led capital and the led civ's clan there
     small. Header "You lost", the line names the taker (the step checks Fell before Conquest,
     so the led civ's fall is told as its fall). Screenshot `end-lost.png`.
   - **Limit**: three civs, `limit: 3`, End turn three times: the line says "turn limit".
     Screenshot `end-limit.png`. Then **Play on**: the map is back, End turn advances the turn,
     the civ panel's Raise… is greyed with "the game is over", and Game → Standing so far
     shows the screen again; **New game** reaches start.
   - **Watched**: the limit stage watched: a stage with no `lead` stays at the pick, so press
     the pick's Watch, then End turn three times: the header names the winner, not you.
   `pick.mjs` checks each card's "The game ends after" line and that its turn is the pick's
   turn plus `view.limit.turns` once picked.
9. **`docs/design/page.md`** is the owner's: don't edit it. The review retires the ledger's
   "Until a game can end" line.

## Done

`engine/check.sh` green (it runs `check.mjs` and every play script); the three screenshots in
`relay/specs/152/end/` (small: `page.shot(file, 0.5)`); `engine/CLAUDE.md`'s `www/play/` row
says what `end.mjs` covers.
