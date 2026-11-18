# #342 build-1 (clay): the herd on the actor

Read `relay/specs/342/spec.md` first: items 1–11 are this step, and its decisions are settled. You
shape the core the kilns and the glaze build on. Keep the start numbers: the glaze tunes them.

The spec step sketched this on main (`probe.diff`, `probe.md`): read it for where each piece hooks
in, not to copy. Its switches are environment variables and its `jobs` stayed in `fields.rs`; build
it properly. Its setting p4 is what you build: a herd started at `HERD.start` a person, the notch
reckoning the steady yield, the clan taking all of the increase.

## What you build

1. **The good.** `data/goods.rs`: `Use::Herd` and the row `animals` after `shellfish`
   (`weight 0.5, spoil 0.0, eaten true, uses Use::Herd`); `FOOD_N` 6; `ANIMALS` 5, `SALT` 6,
   `OBSIDIAN` 7. Every `[f64; N]` literal grows a 0 in the new slot (`TERRAIN`, `FEATURES.river`,
   `FOOD` gets a row with regen 0). Read every `match` on `Use` and every `GOOD[g].eaten` filter
   (`actors.rs`, `market.rs`, `markets.rs`, `civs.rs`, `units/`, `web/src/lib.rs`, `cli/src/`): a herd
   is food everywhere except where the spec says. #355's notes (`relay/specs/355/notes.md`) list
   the basket sums that count salt as food; `animals` is food, so those are right.
2. **`data/herds.rs`** with `HerdRules` and `HERD` (the spec's numbers and a `note`), re-exported in
   `data/mod.rs`. **`Config.herds`** in `data/game.rs`, `#[serde(default = "yes")]`, on in `Default`.
3. **`core/src/jobs.rs`**: move `fields::jobs` here (callers: `clans::reap_within`,
   `fields::home_toward`) and add the herding job: a `herds` share and the tiles with
   `herds::graze > 0`, with their own yearly walks at `HERD.walk × CLAN.inside` when the clan is a
   herder (else `CLAN.inside`). Return the herding days per herding tile beside the others.
   **Order of operations:** with herds 0 every float must come out bit for bit as today: keep the
   present `shares` products for gathering, fields and the source, and give herding its share only
   when it is above 0 (a job with share 0 gets nothing; `actors::split` handles an empty job).
4. **`core/src/herds.rs`**: `open(w)`, `graze(w, t)` (`HERD.graze[terrain] × fertility`, land only),
   `carries(w, tiles, days) -> f64` (Σ `min(graze, days × HERD.tend)`), `breed(h, k) -> f64` (the
   increase, negative above `k`, never past `k`; `h` to 0 at `k` 0 by the same rule), `herder(c)`,
   `radius(c)`, `start(w, id)` (a herd of `HERD.start × people`; the game on its own tiles pays
   what it can, in proportion to their stock, and the rest comes from beyond its land),
   `offtake(h, inc, k) -> f64` (what the clan takes of the increase: all of it, `inc.max(0)`; one
   function so the glaze can change it to what grows past `k ÷ 2`), `steady(k) -> f64`
   (`HERD.breed × k ÷ 4`), `notch(w, id)`, and `head(c) -> f64` (its herd: `store[ANIMALS]`, or the
   herd it would start when it has a share and none, for reckoning).
5. **Through the turn** (`clans.rs`, kept thin: it calls `herds::`):
   - `Clan.herds: f64`, `#[serde(default)]`. A daughter starts at her mother's; a clan that moves
     on keeps it (the herd is its store, so it goes too).
   - `reap`/`reap_within`: the herding job's `carries` and `breed(head, K)` give the herd's change,
     and `offtake` gives `Reaped.herd`, inside the same budget factor as gathering and grain.
     `Reaped` carries the change and `K` too.
   - `worth` (so `value`, `value_at` and the notch): a herding share counts `steady(K)` less what
     the harvest takes, on top of the food, as `worth` counts a source's dig whether or not the clan
     keeps it. Without this no civ herds (`probe.md`, p1–p3). `Reaped::food`
     adds it to `ANIMALS`. The budget leaves the herd out: `surplus_target × people` less the
     store's food without `ANIMALS`. Only once `herds::open`.
   - `harvest`: the herd breeds (a fall above `K` comes off the store); `basket` is the store
     without its animals plus the harvest (the increase taken) plus what was dug; the breeding stock
     stays in `store[ANIMALS]`. What the budget left of the increase is added to the herd.
   - `eat`: the clan eats its basket as now; short of `CLAN.stable_at` of need, it eats from its
     herd up to `HERD.lean` of it; uneaten animals in the basket go back to the herd. `fed_ratio`
     counts both.
   - `adjust` (the `move` step): `herds::notch` beside `fields::notch`, fields or herds, not both
     (spec item 8). The first turn a share rises from 0, `herds::start`.
   - Every site that passes `CLAN.territory_radius` for a clan's land (`dividing.rs` 73–74,
     `homes.rs` 116–134, `clans.rs` 617–635) passes `herds::radius(c)`.
6. **`EventKind::ClanHerding`**, emitted by the notch with the text in the spec. A note when a share
   falls to 0: "gave up its herd: grass fed it less than gathering".
7. **`SAVE_VERSION`** bumps; the pins in `cli/tests/save.rs` with it. Old saves: as #355 did when
   `N` grew (read how its baskets load).

## Tests

- `core/tests/herds.rs`, seconds each: a herd breeds toward `K` and no further; above `K` it falls
  to `K`; `K` is capped by the herding days and by the grass; a clan with fields never raises herds
  and one with herds never raises fields; a starting herd comes out of the game on the clan's tiles;
  a herder's territory reaches radius 3 and a gatherer's stays at 2; the breeding stock is never in
  the basket (so neither `trade` nor the levy takes it); a short clan eats at most `HERD.lean` of its
  herd; a daughter takes half the herd; with `herds` off nothing herds.
- A stage test in `core/stage/` (listed in `stage_test.rs`'s `LIST`), after `fields`'s pattern:
  a plains map with no river, one civ's clan, played past the pick: by a named turn its herding
  share is above 0, it holds animals and it ate animals. (Its territory past radius 2 is a unit
  test, not the stage's: on the retro maps herders don't yet widen, which is the glaze's.)

## Done

- The tests pass; `engine/check.sh` green.
- **Herds off equals main.** The probe met this in all 19,953 numbers, so it is exact. Build main in a worktree (`git worktree add /tmp/main main`), and on
  the branch set `herds: false` in `Config::default()` locally (don't commit it): `sweep --judge
  --json` (about 3.5 minutes) gives every map's scores equal to main's. A digest can't match (the
  save has a new slot), the scores must. A score that moves is a fault: find it (a float reordered,
  a sum over the new slot) and fix it.
- **Herds on, one sweep** (`sweep --judge --json`, herds on): write `relay/specs/342/clay.md` with,
  per map, the herders and animals at +200, and per civ its herding share and its Standing beside
  main's (`relay/specs/342/main.md`). Read them however you can (a temporary print, not committed: the judge's group is build-2's). This is for the glaze, not a bound: the start numbers need only
  run. Expect about p4 (`probe.md`): some 9 civs herd, about 200 herder clans and 350 animals at
  +200, starving near 1.15 times. Far below that, find why before you finish.
- `engine/CLAUDE.md`'s layout names `jobs.rs` and `herds.rs` in the `core/src/` row (stays under 8 KB).

Where the spec doesn't hold in the code, make the smallest change that works and record it as a
decision. Commit as `#342 build-1: the herd on the actor`.
