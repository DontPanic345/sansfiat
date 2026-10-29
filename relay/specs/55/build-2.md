# Build 2 (easel): leaning on the page

Piece #55, "Clans are the prize". Read `relay/specs/55/spec.md`, its section "The page
(build-2)", and build-1's summary. The engine already has the civ's `keep`, `gift`, `posted`,
`given`, `joined` and `gifted`, `Clan.leaning`, the `SetKeep`/`SetGift` actions and the
`ClanJoined` event. `engine/CLAUDE.md` has the commands and the page's layout.

## Deliver

- `web/src/lib.rs`: each civ row in `view()` gains `keep` (from `civs::keep`, the value in
  force), `keep_set` (the civ's `keep`, null for the default), `gift`, `posted`, `given`,
  `joined` and `gifted`. `rules()` gains `join` (`LEANING.join`) and `gift_max`
  (`CIV.store_cap`). Update the doc comments that list the fields. `act` already takes any
  `Action` as JSON: `{"type":"setKeep","food":5}`, `{"type":"setKeep","food":null}`,
  `{"type":"setGift","food":3}`. Check the serde names against `world.rs`.
- `www/select.js` `capitalPanel`, as the spec says:
  - Under the store table: "kept {keep} · posted {posted} · given {given} last turn", and
    "{joined} clans joined since founding".
  - "Leaning clans": up to 10 unaligned clans (`civ` null) with a leaning to this civ, strongest
    first. Each row shows the clan id, its people (`f2`), "starving" if it is, its leaning as a
    percentage of `rules.join`, and "{name} leads" if another civ's leaning is higher. A row
    click selects the clan's home tile, as other panel links select things (follow how
    `unitList` rows do it). Show "No clan leans towards {name} yet." when there's none.
  - For the player's civ only: the gift row ("Gift {gift} food a turn" with − and + buttons, in
    steps of 1, clamped to 0..`gift_max`), and the keep row ("Keep {keep}", − and +, in steps
    of 1 from the value in force, never below 0, and "Default" sending `food: null`, shown
    only when `keep_set` isn't null). They are panel actions like "Raise warrior".
- `www/select.js` tile panel: for a clan's tile, if the clan has leanings, "Leans to" with each
  civ's swatch and name and the percentage of `join`, strongest first.
- `www/feed.js` / `main.js`: `ClanJoined` shows in the feed (it already carries `civ`, so
  `mine` marks the player's). Add it to any kind list or filter the feed keeps.
- `www/check.mjs`: check the new civ fields (numbers, `keep_set` number or null), `leaning` on
  every clan (an object of civ id to number, empty for civ clans), the two new rules, and that
  `check('{"type":"setGift","food":3}')` is `""` for a leader and not `""` while watching.
- `www/play-lean.mjs` (model it on `play-camp.mjs`): lead the first civ, open its capital
  panel, press + on the gift three times, end 30 turns, and screenshot the civ panel and the
  panel of the first leaning clan into `relay/specs/55/screens/` (`civ-panel.png`,
  `clan-leaning.png`). Look at both screenshots and fix what reads badly.
- `engine/CLAUDE.md`: the layout table (civs: keep, posting, gifts; the leaning module, if
  build-1 made one; `play-lean.mjs`), the sweep's `--no-gifts` and `--lever G`, and the new
  actions in "Adding a mechanic" if it lists them.

## Done

- `./engine/check.sh` passes, and `node www/play-lean.mjs` runs and leaves the two screenshots.
- The panels show what the spec says, and a leaning clan's row is clickable.

Not yours: engine rules or numbers. If the view lacks something a panel needs, add it to
`view()`, not a rule.
