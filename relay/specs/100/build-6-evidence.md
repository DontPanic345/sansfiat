# Build 6: the page's orders and the play scripts, evidence

Run in this session on relay/100 after the page edits.

## Checks

- `engine/check.sh`: exit 0. The extended `www/check.mjs` ends with its `ok: medium seed 3 …` line.
- `bash relay/specs/100/baseline/plays.sh /tmp/plays`: five `ok` lines (camp exit 1, scout 0, hold 0,
  lean exit 1, read 0), each matching main's stdout and exit code.
  - `plays/read.txt` was changed in this build, on purpose: its refusal line names the save version, and build-4
    made `SAVE_VERSION` 2 (spec item 12). The play is unchanged and still checks only that #error names the
    version. The baseline README says so.
- `bash relay/specs/100/baseline/compare.sh`: six `ok` lines (digests and the retro sweep unchanged).
- `node relay/specs/100/build-6-panel.mjs` (exit 0; the output is `panel-output.txt`): presses the panel buttons the
  plays don't press, in headless Chrome, and checks each one's effect on the world:
  - Raise warrior (the civ's panel): the camp takes a person, 3 -> 4. It is greyed at the pick (the store can't fill
    the packs), so the script ends one turn first.
  - Disband 1: one warrior goes, 4 -> 3. Raise 1 (greyed until the store fills the packs; the script ends turns
    until it is enabled): one comes back.
  - Auto-forage and Auto-supply: each switch flips and flips back.
  - Resupply and Forage: accepted, no error.
  - Paint, then Undeploy: a neighbour of the capital joins the zone (territory 1 -> 2), and Undeploy returns the
    camp to its own tile.
  - Move, then Stop: a move to a rival tile sets the camp's destination; Stop clears it.
  - Move to a rival tile (one step from a rival capital, so the tile stays rival-held as the turns pass), end turns
    until the camp arrives, then Pillage: enabled and accepted, no error. The pillage itself resolves at the turn's end;
    the script checks the order was accepted, not the event.
  - Route to here, from the capital to a tile three steps out (the first candidate the engine takes as land), then
    Clear route: the route is set and cleared.
  - No page errors.
  - Screenshots: `panel/panel-camp.png`, `panel/panel-pillage.png`, `panel/panel-scout.png`. They show the state at
    each check; they were not reviewed one by one.

## What the play screenshots show

One clause each, from `/tmp/plays`:

- `camp/camp-ordered.png`: turn 283, Camp 719 at the capital, 3 warriors with a full pack; Raise 1 greyed with "the store can't fill their packs".
- `camp/camp-line.png`: turn 291, the same camp fed from the store, pack 12 of 15, with its line home drawn to the capital.
- `camp/camp-hungry.png`: turn 296, the camp at (-14, 1) hungry for a turn with its line cut off, and the red line drawn back across the map.
- `hold/hold-painted.png`: turn 325, Red's camp 392 with its zone of 4 of 8 tiles painted, no line home, cut off, and the events for the deployment.
- `hold/hold-held.png`: turn 326, tile (1, 7) listed as held by Red's camp, and Blue's camp line cut by Red's army.
- `hold/hold-cut.png`: turn 328, Blue's camp 394 at (1, 7) hungry, with its line cut by Red's army at (2, 7).
- `hold/hold-rival.png`: turn 291, Blue's camp 394 at (1, 7) holding a zone of 2 tiles with a 2-step line home.
- `scout/route-both.png`: turn 283, Scout 720's route set from the capital to (-13, 1), known 7 steps cost 11, possible 7 steps cost 7.
- `scout/trail-built.png`: turn 285, the route found at cost 9, a trail built at (-9, -3), and the Start here, Route to here and Clear route buttons.
- `read/alerts.png`: turn 294, Camp 719 marching to (-14, 1) eight steps away, with the alert rows highlighted in the feed.
- `read/camp-why.png`: turn 293, Camp 719's Why list (its line brings food to the camp; raised with 3 warriors) and the Raise 1 and Disband buttons under it.
- `read/clan-why.png`: turn 292, the tile (-2, -13) panel for clan 1, with its reasons (took in clan 403; began starving).
- `read/country-on.png`: turn 293, the whole map with the hidden country on, Camp 719 selected.

## Bench

Not run here (the review's).
