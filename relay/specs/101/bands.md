# The judge's bands

`engine/judge.toml` holds a band for every number in `sweep --judge`'s total except the counts it
is built from (`health.camps_of`, the AI orders `by_kind`, `doubt.spread_100`, which `spread_200`
covers). Main's values are `build-1-judge.json`, and this branch reproduces them exactly
(`bands-branch.txt`: every band ok). "Half gift" is `data.rs` `gift_default` 2.0 → 1.0, run on a copy
of the engine (`bands-gift-half.txt`).

The rule: **health fails**, since a fall there is a bug. **The four scores warn**, since later pieces
mean to move them, at about a fifth to a quarter of main's value either side, so half the default gift
trips them. One score also fails: `motion.clans_changed` has a fail floor. Clans changing hands is
what gifts and armies are for, so a third of it gone means a piece undid that work. A piece that
means to cut it moves the band and says so.

| Metric | Main | Half gift | Band | Why there |
|---|---|---|---|---|
| health.panics | 0 | 0 | fail > 0 | a map that panics can't be played |
| health.digest_mismatches | 0 | 0 | fail > 0 | the game is deterministic |
| health.pregame_by_centres | 7 | 7 | fail < 7 | every retro pre-game ends by the centres |
| health.people_vs_control | 0.984 | 0.984 | fail < 0.90, warn < 0.95 | armies cost 2% of the people; a tenth is a war nobody meant |
| health.camps_kept | 15 / 21 | 15 | fail < 10, warn < 13 | the AI keeping its camps alive |
| health.warriors_starved | 19 | 21 | fail > 40, warn > 28 | twice main is camps no longer fed |
| health.camps_lost | 3 | 5 | fail > 8, warn > 5 | most maps losing a camp is a broken army |
| standing.top_200 | 0.091 | 0.068 | warn < 0.075 or > 0.15 | the leader's share is what gifts and armies build |
| standing.aligned_200 | 0.184 | 0.157 | warn < 0.16 or > 0.30 | the civs' share of the world |
| doubt.lead_changes | 7 | 9 | warn < 3 or > 15 | none is a game decided at the pick; many is flicker |
| doubt.maps_with_lead_change | 4 | 4 | warn < 2 | doubt on one map only is no doubt |
| doubt.spread_200 | 0.053 | 0.029 | warn < 0.035 or > 0.10 | the gap first to last: too small and nothing separates the civs |
| doubt.swings | 16 | 15 | warn < 8 or > 32 | Standing that never swings is on rails |
| decisions.ai_orders_per_100 | 2.29 | 1.90 | warn < 1.5 or > 5.0 | the AI's decisions per civ per 100 turns |
| decisions.levers_repeated | 0.81 | 0.905 | warn > 0.9 | an AI that only repeats itself isn't deciding |
| motion.clans_changed | 82 | **49** | **fail < 55**, warn < 66 or > 160 | the gift's work; a third gone is undone work |
| motion.people_changed | 398 | 274 | warn < 300 or > 800 | the people in the clans that move |
| motion.land_changed | 2522 | 1526 | warn < 2000 or > 5000 | tiles whose civ changes |

## The perturbation

`gift_default` 2.0 → 1.0 (the default gift halved, as #75 halved the joins), on a copy of the engine,
so `data.rs` on the branch never changed. `sweep --judge` exits 1:

    WARN decisions.levers_repeated 0.9047619047619049 (above 0.9)
    WARN doubt.spread_200 0.029426420863348353 (below 0.035)
    FAIL motion.clans_changed 49 (below 55)
    WARN motion.land_changed 1526 (below 2000)
    WARN motion.people_changed 273.8422478835444 (below 300)
    WARN standing.aligned_200 0.156761669931856 (below 0.16)
    WARN standing.top_200 0.06797398320112438 (below 0.075)

Halving the gift leaves health untouched (people, camps, digests), so only the scores could catch it,
and they do: one fail and six warns. Without the `clans_changed` floor this regression would pass CI
with warnings only.

## `JUDGE.lead_margin` and `JUDGE.sample`

Each variant was run on a copy of the engine, totals over the seven maps:

| lead_margin, sample | lead changes (per map) | maps with one | swings (per map) |
|---|---|---|---|
| none (`probe.md`), – | 25 | – | – |
| 0.0025, 5 | 7 (1,1,4,0,1,0,0) | 4 | 41 (11,0,13,8,0,9,0) |
| **0.005, 10** (kept) | 7 (1,1,4,0,1,0,0) | 4 | 16 (3,0,9,2,0,2,0) |
| 0.01, 20 | 5 (1,0,4,0,0,0,0) | 2 | 3 (3,0,0,0,0,0,0) |

Both stay as build-1 set them. **lead_margin 0.005**: the near-tie flicker (25 raw) is gone by
0.0025 already, and 0.005 gives the same seven, so it is not on an edge. At 0.01 the real changes on
small 3 and medium 8 vanish too. **sample 10**: at 5 the swings count noise (41, with 13 on one map);
at 20 four of the seven maps show none, and a band on 3 can't tell a fall from chance. At 10 the
swings come to 16, spread over five maps.
