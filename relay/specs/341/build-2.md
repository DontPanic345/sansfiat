# #341 build-2 (canvas): capitals that become towns, and towns that hold

Read `relay/specs/341/spec.md`, `measure.md` and build-1's summary and evidence
(`build-1-evidence.md`). This is the piece's unsettled work. Searching for what works is the job,
and you may change the easel (build-1's door step) where the maps call for it.

## What the easel gives you

- **The door step** in `homes::step`: a clan steps onto its festival's tile when that feeds it as
  well as staying and its festival walk from there is less. On the seven maps it takes towns at
  +200 from main's 10 on 4 maps to 18 on 6, and adds one capital town (large 8's 2689) and none on a
  small map.
- **The judge's counts**: capital towns at +100 and +200 per civ (`TownCiv.capital_towns_*`),
  `TownScores.capitals_200`, and an `outside` group (every clan not living in a town) beside
  `town` and `farmers_outside`. The sweep's towns lines print them. `relay/specs/341/towns.py`
  prints the per-map table from any `--json` report.
- **The mark** shows on a capital's town on the page.

## The control

Main's report with its `--floor` noise is `relay/specs/341/main-floor.json.gz` (main at 8d1b4ed:
`gunzip -k` it and pass it to `judge --compare` and `towns.py`). Don't rerun main unless main has
moved (`git log -1 origin/main`). `door-vs-main.txt` is the easel's rule beside it. The easel is
already a little past main's floor on aligned standing (0.522 → 0.504, floor 0.008) and on the goods
run's starving (0.0265 → 0.0294, floor 0.0019; small 6 and small 9 carry it). Its war falls by
half, but that is all small 2 (10 tiles taken → 0).

## What you are after

A capital that is a town at +200 on **at least two of the four small maps** (main and the easel:
one, small 9's), with:

- towns at +200 over the seven maps at least main's 10, on at least 4 maps;
- health no worse than main's past main's floor: starving (town clans, outside, all, and the goods
  run's), the armies (`warriors_starved`, camps kept and lost) and digest mismatches 0. Where the
  easel's goods-run starving or aligned standing stays past the floor, say map by map why;
- trade per person in a town against outside, reported (not a target).

If no rule within your levers gets there, say why, map by map, with the evidence: which capitals
have no ground or no farmer near (small 3 has neither), and which have them but lose their clans.
That answers the question.

## What the spec saw (`measure.md`)

- Small 6, civ 263 (capital 378, no field ground, farmers one tile out): under the door step three
  farmers live on the capital at +30, and one by +200. Clan 316 leaves by +50; 304 stops farming at
  +60 and leaves by +140, by the shared-home rule (a non-farmer on a shared home weighs the step to
  the middle of its land, `homes::step`).
- The civ's own clan was gone by +20 (small 6's civ 263, small 3's civ 198). `civs::repair` passes
  the civ to its biggest clan wherever that clan lives, and `clans::adjust` pins that clan where it
  is (`is_civ_clan`), so the capital loses its fixed resident. Passing it to a clan on the capital
  first changes nothing at +200 on its own (A + C in `tables.txt`), but may with a rule that keeps
  clans there.
- Small 2's capital 219 has field ground (30 days) and 22 clans at its festival, six of them farmers
  1 to 3 tiles out, and one home on it at +200. Farmers two or more tiles out lose their fields when
  they step toward the festival (the territory is trimmed to the radius around the new home).
- Freed days buy nothing in `could_reap` for a clan whose fields are capped by their ground and
  whose gathering is capped by its tiles' food: that's why the door step needs its tie.

## Your levers

The home rules: `homes::step` (the door step, a town clan's leaving its festival's tile, the step
toward a festival from further out), the civ's own clan (`civs::repair`, the pin in
`clans::adjust`), and `TOWN`'s numbers in `core/src/data/clans.rs`.

Not your levers: trade (`MARKET`, `markets::trade`: lifting the carry cap at the door is the
owner's idea now), field ground (`FIELD`), the town rule's three homes (decided), and any rule that
draws a clan to a capital because it is a capital. A town has no rule of its own (farming.md); what
holds a clan on a capital must hold it on any festival's tile.

The civ's own clan is the exception the design already makes ("the civ's own clan stays on the
capital", farming.md Decided, #155). A repaired civ clan pinned off the capital is a case #155
doesn't cover, so passing the civ to a clan on the capital first, or bringing the new civ clan home
to the capital, is within your levers (spec.md, Its levers). Find out first how the civ's clan goes
by +20 on small 6 and small 3: merged at its festival (`markets::merge`, under `CLAN.min_pop`), dead
(`clans::eat`'s `dead`), or something else. Whether a rule fixes the cause or the repair depends on
what you find.

Each rule you keep gets its reason in a `note` beside its number, a test in `core/tests/towns.rs`
(or `town_homes.rs` if build-1 split it) on a built map, and a line in `homes::step`'s doc. Bump
`SAVE_VERSION` and the pins once, at the end, if the digest moved since build-1.

## Evidence (`relay/specs/341/canvas.md`)

Each rule you try: what it is, the `towns.py` table, capital towns map by map, and the health
totals beside main (`judge --compare`). For the rule you keep: `judge --compare` against `main-floor.json.gz`, and the three
scores of spec.md beside main's and the easel's. Keep it short: a row a try.

Run `engine/check.sh` once at the end. Commit as `#341 build-2: capitals that become towns`.
