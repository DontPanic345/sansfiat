# #57 Reading the world: spec

## What it delivers

The page explains the game. Each clan, festival, camp, scout and civ keeps a short log of
**reasons**: a line each time a rule acts on it, which the selection panel shows, newest first.
Events that concern the player are **alerts**, picked out in the feed and kept in a box at its
top. The map says who holds a tile, draws each clan's festival route and the player's camp's line,
and puts hidden country (valleys, sealed valleys, necks) on a toggle. A save carries a
**version**, and a save from another version fails with a reason. `merge4/` is archived.

No rule changes: every world digest is the same before and after this piece.

## Shape: Henry Ford

Taken as the piece suggests. Everything below is settled; what's left (the wording of each
reason, how the alerts box looks) matters little and is judged in passing. The piece splits
along one seam: the engine records and exposes (reasons, alerts, the save version, the CLI), and
the page shows (panels, map, toggle, alerts box) and archives `merge4/`.

- **build-1 (ford): the engine.** Reasons, alerts, `PeopleLeft`, the save version, `Sim.notes`,
  the CLI, tests.
- **build-2 (ford): the page.** Reasons in the panel, the alerts box, who holds a tile, routes and
  the line on the map, the hidden-country toggle, the load error, a play script, archiving
  `merge4/`.

Most of what the goal names is already on the page: clan territories and festival routes are
drawn (`map-view.js` `drawOverlay`), the selected camp's line is drawn (`setLine`), the tooltip
names the clan and civ on a tile. build-2 fills the gaps listed below, not a redraw.

## Settled (decisions the owner can overrule)

### Reasons

1. **Where they live.** `World.notes: BTreeMap<u32, Vec<Note>>`, keyed by the subject's id (clans,
   markets, civs and units all draw ids from `next_id`, so one key space holds them), with
   `Note { turn: u32, text: String }`. It is `#[serde(skip)]`, like `events`: derived from the
   turns, rebuilt by a replay, so saves, `state()` and `stats::digest` don't change.
2. **The rule for keeping them**, from merge4's `note`: a subject keeps its last `NOTES.kept = 8`,
   oldest dropped; a note whose text equals the subject's latest only moves that note's turn on.
   Notes of subjects that no longer exist are dropped at the end of each turn (in `end_turn`,
   after the steps). The panel shows the latest 5 (`NOTES.shown`).
3. **When a rule writes one**: when it changes something about the subject, or declines to when
   the player would expect it to (a starving clan finds no better site). Not a per-turn status
   line: "fed 92%" every turn is what the panel's numbers are for. Text is short, lower case
   after the turn, and gives the why with its numbers, as merge4's do ("left (3, −2) for (5, −1)
   after starving 4 turns: worth 1.8 there against 1.1").
4. **What records reasons, at the least** (a build may add more):
   - *Clan*: `adjust` (tiles added or dropped, and why; home moved), `move_on` (moved, or
     starving with no better site in range), `go` (changed festival, with the worths; stopped
     going), `eat` (began starving, with its fed share and its worst-missing food), `people`
     (people shed or taken in at the festival), `merge` (merged into, or took in, a clan),
     `divide` and `split` (on mother and daughter), `levy` (paid its civ), `lean` (joined a civ),
     a gift received (`leaning::give`).
   - *Festival* (market): formed (in reach of how many clans), cleared (traders and goods
     swapped, when anything traded), people moved there, became a centre. Faded is the event:
     the market is gone, so its notes are too.
   - *Camp*: raised and warriors raised or disbanded, marched (where to, how far it got), deployed
     and held, resupplied (food and line cost), cut off (by distance or land) and cut by an army
     (whose), foraged or pillaged (food, where), hungry, starved. The AI's choices: posted (where
     and why), painted its zone.
   - *Scout*: route set, found or impossible, trail built, pass found, blocked, hungry.
   - *Civ*: founded, the levy collected, store over its cap (food lost), gift given and to how
     many clans, posted at its market, a clan joined, the player's keep or gift changed.
5. **Scouts get reasons too.** The piece names clans, festivals, camps and civs; the scout is the
   civ's other unit and records through the same call, so leaving it out would be the odd case.
6. **Cost.** A note is formatted only when a rule acts, so work per clan stays flat in the number of
   clans. The engine's `bench` must not slow by more than 10% (build-1 measures it).

### Alerts

7. **An alert is an event, flagged in the engine**: `Event.alert: bool` (`#[serde(default)]`), set
   in `end_turn` after the steps, so the CLI and the page agree and tests can hold it. Only in
   `Phase::Main` with a led civ (`w.player`), and only for:
   - *hunger in the camp*: `CampHungry` and `WarriorsStarved` of the player's camp; also
     `ScoutHungry` and `ScoutStarved` of its scout (decision: the scout is the player's too);
   - *a line cut off*: `CampCutOff` and `LineCut` of the player's camp;
   - *a clan joining a rival*: `ClanJoined` with a civ other than the player's;
   - *people leaving its clans*: `PeopleLeft` for the player's civ (below).
8. **Alerts fire when a trouble starts**, not every turn it lasts: an event is an alert only when
   no event of the same kind about the same subject (the unit; the civ for `PeopleLeft`; the clan
   for `ClanJoined`) was an alert candidate on the turn before. `CampCutOff` is not raised every
   turn a camp stays cut off: it fires when the line is newly cut, or when a rival's army was the
   cause and the line is cut by distance instead (`units::camp_turn`). A camp cut by distance for
   many turns reports it once, so the edge rule and the feed agree. (Corrected in build-1: this
   said "every turn".)
9. **`PeopleLeft` is a new event, not a new rule.** It reports what `markets::people` already does:
   the people a civ's clans shed at festivals to clans not of that civ (unaligned or a rival's),
   plus the people of a civ's clan that `merge` folds into a clan not of the civ. Summed per civ
   over the `people` step, one event per civ per turn when the sum is at least 0.1 people, at the
   civ's capital, with the amount in a new `Event.amount: Option<f64>` (skipped when none).
   Text: "{n:.1} people leave {civ}'s clans for others". Moves between two of a civ's own clans
   don't count. The sum follows the rule's own split: clan *i*'s offer × moved/offered × room
   share of clan *j*.

### The save version

10. `Save` gains `version: u32` (`#[serde(default)]`, so a save without one parses as 0), first in
    the JSON. `world::SAVE_VERSION = 1`; `World::save` writes it. `World::replay` refuses any
    other version before replaying: "this save is from an older build (no version); this build
    reads version 1" for 0, "this save is version N; this build reads version 1" otherwise. The
    CLI's `replay` and `Sim.load` pass that through, and the page shows it in `#error` as it
    already shows load errors.
11. **The version guards the rules.** A test, `the_save_version_pins_the_rules`, plays seeds 1–3 on the
    small map to the pick and 20 turns on, watching, and seed 2 led by the scripted game of
    `digest --lead` (so the player's orders, camp and scout paths are pinned too), and holds
    those four `stats::digest`s to constants beside `SAVE_VERSION`. Any change that moves the digest must bump `SAVE_VERSION` and the
    constant together; `engine/CLAUDE.md`'s invariants say so. A save made before a rules change
    would replay into a different world, maybe silently; the version turns that into a reason.

### The page

12. **The selection panel** shows each subject's reasons under its numbers, newest first, as "turn
    · text", headed "Why": every clan shown in a tile panel, the market there, a camp, a scout,
    a civ's capital panel (the civ's notes). `Sim.notes(id)` gives them as JSON.
13. **Who holds a tile.** The tooltip and the tile panel already name the clan and the civ's land;
    they also name a camp's warriors holding the tile ("Held by Red's camp"), and a clan's tile
    says whether it is home, territory, or on the clan's festival route.
14. **Routes.** All clans' festival routes stay drawn as now. Selecting a clan (a tile panel with a
    clan) draws its territory outline and its route brighter and the others dimmer.
15. **The camp and its line.** The player's camp's line home is drawn at all times, thin and
    faint; selecting the camp draws it as now (coloured by what it carries, cut ground marked).
    Rival camps' lines show only when selected, as now.
16. **Hidden country on a toggle.** Design (`map.md`) says hidden country is drawn so you can tell it
    apart at a glance; the piece puts it on a toggle. The closed-country outlines (zones 1–3:
    valley, sealed valley, neck) are drawn only with View → "Hidden country" (key H), **off by
    default**, and the tooltip names the zone only when it is on; the legend lists them only when
    on. Passes stay drawn always: they are terrain the scout and clans use. "Hidden costs (debug)"
    stays its own toggle.
17. **The alerts box** sits at the top of the feed panel: the last 5 alerts, newest first, each
    clickable as a feed item is. Alerts in the feed list get a red left border and a "!" before
    the text. While the feed is hidden, the Feed button shows the count of alerts since it was
    last open.

### The CLI and the archive

18. `run` and `replay` print alerts with a `!` after the turn (`"  412 ! Red's camp …"`); the JSON
    outputs carry `alert`. `run … --why ID` prints that subject's notes at the end.
19. **Archive `merge4/`** as `hex-economy/` is: the tag `prototype/merge4` exists. Delete the folder's
    contents but a one-line `README.md` naming the tag and a `package.json` whose `test` is
    `echo archived` (so `ci.yml`'s `js` matrix stays green untouched), and drop its row from the
    repo's `README.md`. The design files that cite `merge4/…` paths are the owner's: an idea, not
    an edit.

## Left unsettled

Nothing that matters to the goal. The wording of each reason (build-1) and the look of the alerts
box and the dimming (build-2) are judged in passing by the review from build-2's screenshots.

## Tests the piece must have

- Notes: a run to the pick and 50 turns on, watching, gives notes to clans, markets, civs, camps
  and scouts; a divided clan's note names its daughter and the daughter's names its mother; no
  subject holds more than `NOTES.kept`; a gone subject has none; a replay rebuilds the same notes.
- `the_save_version_pins_the_rules` (item 11): build-1 takes the four digests from the code as it
  finds it, before any change, so the test also proves the piece changed no rule.
- Alerts: with a led civ, its camp moved far out on its land goes hungry and the first `CampHungry`
  is an alert and the next turn's isn't; a rival's `ClanJoined` is an alert and the player's own
  isn't; nothing is an alert when watching; `PeopleLeft` sums match the people the step moved off
  the civ's clans.
- The save version: a save round-trips; one without `version`, or with 2, fails naming the version.
- `check.mjs`: `Sim.notes` returns `[{turn, text}]`, events carry `alert`, and a versionless save
  fails to load with a reason.
- `www/play-read.mjs` (build-2) plays it through the page.

## Not in this piece

New rules, a help panel or guide, a "mine only" feed filter, rivals' leanings on the map.
