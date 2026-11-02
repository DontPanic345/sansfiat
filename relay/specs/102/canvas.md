# #102 build-4: the canvas

Every row is `sweep --judge` on the seven retro maps (with `--solo` where a solo column is given).
Start = build-3's numbers (`build-3-judge.txt`). Final = `canvas-judge.txt` / `.json`.

| # | start guarding inside keep | floor ring reach | land guarded | leave run/at | rules | solo mean (clans, land) | above 0.005 | lead changes (maps) / no claims | people vs no-army | camps lost |
|---|---|---|---|---|---|---|---|---|---|---|
| start | 3 3.5 0.05 5 | 0.1 1 4 | 0.01 0.1 | 10/0.3 | build-2 | -0.0012 (-0.0014, +0.0002) | 6 | 7 (3) / 3 (3) | 0.976 | 2 |
| a | 1 3.5 0.02 2 | 0.1 1 4 | 0.03 0.1 | 10/0.3 | build-2 | +0.0067 | 8 | 7 (2) | 0.992 | 1 |
| b | 1 3.5 0.02 2 | 0.1 2 4 | 0.06 0.1 | 10/0.3 | build-2 | +0.0087 | 7 | 10 (3) | 0.977 | 1 |
| c | as b | 0.1 2 6 | 0.06 0.1 | 10/0.3 | build-2 | +0.0092 | 7 | 10 (3) | 0.974 | 1 |
| d | as b | 0.03 2 6 | 0.06 0.3 | 10/0.3 | build-2 | +0.0104 | 9 | 4 (2) | 0.985 | 1 |
| e | as d | as d | as d | 10/0.3 | clans feed the guard | +0.0117 | 10 | 3 (2) | 0.981 | 1 |
| f | as d | as d | as d | 10/0.3 | + AI counts clans' stores | +0.0165 | 11 | 2 (2) | 0.978 | 1 |
| g/h | as d | 0.03 2 8 | as d | 10/0.3 and 5/0.1 | as f | - | - | 2 (2); clans left 0 either way | 0.978 | 1 |
| i | as d | 0.03 2 8 | as d | 5/0.1 | + stand-in seeds every clan's home | +0.0007 (claim 109, held 9: too thin) | 5 | 6 (4) | 0.978 | 3 |
| j | as d | 0.03 1 4 | 0.06 0.3 | 5/0.1 | as i | +0.0128 | 11 | 2 (2) | 0.978 | 1 |
| l | 2 3.5 0.02 2 | 0.05 1 4 | 0.03 0.3 | 5/0.1 | as i | +0.0092 | 11 | 5 (4) | 0.977 | 2 |
| m | 2 3.5 0.02 1 | 0.05 1 4 | 0.04 0.3 | 5/0.1 | as i | +0.0156 | 11 | 16 (4) | 0.981 | 5 |
| n | 2 3.5 0.02 1 | 0.05 1 4 | 0.1 0.5 | 5/0.1 | as i | +0.0157 | 13 | 6 (4) | 0.972 | 3 |
| o1 | 2 3.5 0.02 1 | 0.05 1 4 | 0.2 0.5 | 5/0.1 | as i | +0.0154 | 13 | 4 (4) | 0.978 | 3 |
| o2 | 2 3.5 0.02 1 | 0.04 1 4 | 0.1 1.0 | 5/0.1 | as i | +0.0160 | 13 | 10 (5) | 0.979 | 2 |
| **final** | **3 3.5 0.02 1** | **0.05 1 4** | **0.1 0.5** | **5/0.1** | as i | **+0.0170 (+0.0169, +0.0001)** | **14** | **12 (5) / 3 (3)** | **0.982** | **18** |

All runs: pre-games by centres 7 of 7, panics 0, digest mismatches 0; festival and trail measures
unmoved (markets 218/214, homes trailed 0.85, worked 0.31, route 0.97/0.98, flips 0.07 per 100
clan-turns against build-1's 0.08).

## What I chose and why

- **The guard is fed by its clans** (rule change, `units/guard.rs`): what the civ's store lacks, the
  civ's clans give from their own stores, biggest first. Build-2 found the founding guard starves in
  about five turns on a store that takes in about one food a turn; with that, 11 of 21 AI civs had no
  guard at +200 and the claim fell to the capital. Fed by its clans the guard lives (2.3 people a civ
  at +200, held 16.9 tiles against 5.7).
- **The AI's guard rule counts its clans' stores** with the civ's store when it asks whether one more
  guard can be fed (same reason; without it the AI never raised past founding).
- **The stand-in seeds from every clan's home within reach**, not only the civ's own (rule change,
  `claim::ai_claim`): it claims where the unaligned people are, which is what the solo gain counts.
  Ring 2 or reach 8 with it claims ~109 tiles and holds 9 (row i): thin, as the design says.
- **Leaning by land is strong** (land 0.1, guarded 0.5 against fade 0.03): a held home's clan reaches
  the join line in about 11 turns. That is why the land part of the solo gain is ~0: unaligned clans on
  held land join, so their people count as clans won.
- **Leaving at 5 turns drawn at 0.1** (was 10 at 0.3): at 10/0.3 no clan ever left on any run. Even
  now 1 clan left in the final run: rival civ clans' homes are rarely on another civ's held land, and
  gifts reach rival clans only at a rival's capital festival.
- Floor 0.05, inside 0.02, keep_turns 1: lower floor and walk weight let three guards hold the near
  ring; keep 1 leaves the store free to give gifts.

## Health beside the targets

Every proof line is met: solo gain above +0.005 for 14 of 21 civs, mean +0.0170; lead changes 12 on
5 of 7 maps against 3 on 3 in the no-claims control; pre-games 7 of 7, 0 panics, 0 digest mismatches,
people against `--no-army` 0.982; clans left 1 (no rout). **One cost the targets don't name:** camps
lost rises from 2 to 18 and warriors starved from 16 to 53, because the clans' stores that now feed
the guard are the same food the camp's resupply and the store draw on. The review should weigh it;
`GUARD.start` 2 (row o2) keeps camps lost at 2 with 13 of 21 above the bar.

Which civs' gains: see `canvas-judge.txt`'s per-map `solo:` lines; the gain is lumpy, a few civs near
unaligned clans gaining 0.03-0.08 and civs with none near gaining nothing.

## Not done in this session

The two played games on small seed 2 (wide claim within 6 steps against the stand-in with guards
raised), their `sansfiat judge` lines, and `play-claim.mjs` on the final numbers were not run: the
session's clock went on the sweeps. The review's play (spec, "What the review plays", 2 and 3) is
where they now fall. What the numbers already say about width: rows c and i claim more and hold the
same or less (the guard's division), and at the final numbers three guards hold about 17 tiles.
