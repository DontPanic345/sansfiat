# Build 4 (kiln): the page shows territories and routes; the prototypes are archived

Piece #54, "Actors, and clans that live off their land". Builds 1–3 moved the clans onto the
actor model: the round is gone; each clan holds a connected territory (`Tile.claim`, sent to
the page as `claims`), walks one route to its festival, and the land wears. Read their
summaries first: they say what `web/src/lib.rs`'s `view()` and the clans' JSON hold now, and
what build-1 changed in `www/` to keep the page running. You redraw the page for the new
model, and archive two prototypes. Read `engine/CLAUDE.md` first.

**Keep the clay's shape.** Where it doesn't give the page what it needs, make the smallest fix
in `web/src/lib.rs` that works and record it as a decision. Change no rule.

## The design

From `docs/design/clans.md`: a clan holds a connected patch of tiles around its home; it walks
one route, to one festival, each turn; trails wear where routes are walked; a tile harvested
past what it regrows wears down.

## What to build

### The page (`engine/www/`, `engine/web/src/lib.rs`)

- **Territory**: each clan's tiles, from `claims`, as the faint fill in the clan's colour
  with a firmer line where it meets other ground (as `map-view.js` draws territory now). Drop
  anything still drawing rounds or "round + neighbours".
- **Routes**: each clan's festival route drawn from home to the festival along its path, a
  thin line in the clan's colour (or one colour for all, if that reads better), under the
  festival rings and over the territory. `view()` sends each route's path if it doesn't yet.
  The "rounds" toggle becomes "routes".
- **Wear**: the tooltip gives a tile's worn foods as shares ("game worn 30%") and its unworn
  cap where worn. `view()` sends wear only for tiles with some (a list, as `tame` is).
- **Tooltip**: "clan 12's territory", not round/territory; the festival a hovered home walks
  to.
- **Selection panel** (`select.js`): a selected clan shows its people, territory size, its
  festival and route cost (labour a year), and fed.
- **Legend**: "round" becomes "festival route"; add "worn land" only if you draw it.
- **`check.mjs`**: covers `claims` matching the clans, every territory connected and holding
  its home, every route's path ending on its festival's tile, the new tooltip fields; and the
  `clanMerged` feed kind.
- **Screenshots** with `node www/play.mjs` (see `engine/CLAUDE.md`): a small seed mid
  pre-game (say turn 150) with territories and routes, and the same at its end. Look at them,
  fix what reads badly, and save the two you keep under `relay/specs/54/screens/`.

### Archive `hex-economy/` and `hex4x/`

Both are tagged already (`prototype/hex-economy`, `prototype/hex4x`). `merge4/`, `scout/`,
`supply-lines/` and `paint-army/` stay.

- Delete each folder's contents, but leave two files in each, because `.github/workflows/ci.yml`
  runs `npm test` in both and the runner can't push workflow files: a `package.json` whose
  `test` script passes without any test files (e.g. `"test": "echo archived"`), and a
  `README.md` of one line: archived, the tag to check out, and that the owner may delete the
  folder once `ci.yml`'s `js` matrix drops it.
- `tools/site.sh` publishes only folders with an `index.html`, so neither is published after.
- `README.md` (the repo's): drop their rows; say the prototypes not listed are under
  `prototype/<name>` tags. Leave `merge4/` and the other prototypes' own text alone (they're
  frozen, even where they name hex-economy).

## Done

- `engine/check.sh` passes; `npm test` passes in `hex-economy/` and `hex4x/`.
- Two screenshots in `relay/specs/54/screens/`, named in your summary, with a line on what
  each shows.
