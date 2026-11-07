# #153 build-3: the feeding

A scout takes only what a tile regrows (`land::spare`, `forage::graze_at`), leaves its errand for rich
ground when its pack holds under `SCOUT.low` turns of eating, forages round it for the pack's room until
it holds `SCOUT.fed`, and goes back. An idle scout waits on rich ground. Rich ground's worth is
`scout::ground_worth` (`round_plan`, shared with `forage_round`). Starting numbers: wander 4, low 2.0,
fed 4.0, rich 1.5.

## scout_errand at the starting numbers: 6 of 28 lost

`cargo test --release -p engine --test scout_errand -- --ignored --nocapture --test-threads=1`.
The survival test is `#[ignore = "build-5 tunes the feeding"]`; the refilled control is a second ignored report.

|---|---|---|---|---|---|---|---|
| small 2 | route to the rival | 43 | 2 | 8 | 1 | 0 | no |
| small 2 | route to Lead's target | 3 | 1 | 33 | 2 | 0 | yes |
| small 2 | move to Lead's target | 3 | 1 | 34 | 0 | 0 | yes |
| small 2 | idle | 0 | 0 | 50 | 0 | 0 | yes |
| small 3 | route to the rival | 0 | 0 | 47 | 1 | 0 | yes |
| small 3 | route to Lead's target | 0 | 0 | 47 | 1 | 0 | yes |
| small 3 | move to Lead's target | 0 | 0 | 48 | 0 | 0 | yes |
| small 3 | idle | 43 | 2 | 10 | 0 | 0 | no |
| small 6 | route to the rival | 0 | 0 | 33 | 1 | 0 | yes |
| small 6 | route to Lead's target | 45 | 2 | 6 | 2 | 1 | no |
| small 6 | move to Lead's target | 4 | 1 | 48 | 0 | 0 | yes |
| small 6 | idle | 0 | 0 | 50 | 0 | 0 | yes |
| small 9 | route to the rival | 46 | 2 | 4 | 1 | 0 | no |
| small 9 | route to Lead's target | 0 | 0 | 28 | 8 | 0 | yes |
| small 9 | move to Lead's target | 4 | 0 | 30 | 0 | 0 | yes |
| small 9 | idle | 0 | 0 | 50 | 0 | 0 | yes |
| medium 8 | route to the rival | 0 | 0 | 46 | 2 | 0 | yes |
| medium 8 | route to Lead's target | 0 | 0 | 46 | 2 | 0 | yes |
| medium 8 | move to Lead's target | 0 | 0 | 48 | 0 | 0 | yes |
| medium 8 | idle | 0 | 0 | 50 | 0 | 0 | yes |
| medium 9 | route to the rival | 0 | 0 | 33 | 3 | 0 | yes |
| medium 9 | route to Lead's target | 0 | 0 | 33 | 3 | 0 | yes |
| medium 9 | move to Lead's target | 13 | 2 | 35 | 0 | 0 | no |
| medium 9 | idle | 0 | 0 | 50 | 0 | 0 | yes |
| large 8 | route to the rival | 46 | 2 | 5 | 0 | 0 | no |
| large 8 | route to Lead's target | 0 | 0 | 35 | 3 | 1 | yes |
| large 8 | move to Lead's target | 0 | 0 | 48 | 0 | 0 | yes |
| large 8 | idle | 0 | 0 | 50 | 0 | 0 | yes |
scouts lost: 6 of 28: ["small 2 route to the rival", "small 3 idle", "small 6 route to Lead's target", "small 9 route to the rival", "medium 9 move to Lead's target", "large 8 route to the rival"]; tiles revealed 30, trails built 2

The six lost: small 2 route to the rival, small 3 idle, small 6 route to Lead's target, small 9 route to
the rival, medium 9 move to Lead's target, large 8 route to the rival. The losses are a scout that bounces
between grounds and spends its labour walking (small 3 idle: two grounds a step apart, re-picked each
turn its tile is worth under its eating; small 2 to the rival: ground 4 steps off, three turns' walk for
a pack it eats in two). The routed scouts that live spend most turns wandering and reveal 30 tiles in all.

## The refilled control (14 routed errands, pack refilled each turn)

|---|---|---|---|---|---|---|---|
| small 2 | route to the rival | 0 | 0 | 0 | 10 | 4 | yes |
| small 2 | route to Lead's target | 0 | 0 | 0 | 2 | 0 | yes |
| small 3 | route to the rival | 0 | 0 | 0 | 1 | 0 | yes |
| small 3 | route to Lead's target | 0 | 0 | 0 | 10 | 2 | yes |
| small 6 | route to the rival | 0 | 0 | 0 | 8 | 5 | yes |
| small 6 | route to Lead's target | 0 | 0 | 0 | 2 | 2 | yes |
| small 9 | route to the rival | 0 | 0 | 0 | 15 | 0 | yes |
| small 9 | route to Lead's target | 0 | 0 | 0 | 12 | 4 | yes |
| medium 8 | route to the rival | 0 | 0 | 0 | 12 | 2 | yes |
| medium 8 | route to Lead's target | 0 | 0 | 0 | 11 | 3 | yes |
| medium 9 | route to the rival | 0 | 0 | 0 | 23 | 0 | yes |
| medium 9 | route to Lead's target | 0 | 0 | 0 | 5 | 2 | yes |
| large 8 | route to the rival | 0 | 0 | 0 | 22 | 0 | yes |
| large 8 | route to Lead's target | 0 | 0 | 0 | 3 | 1 | yes |
scouts lost: 0 of 14: []; tiles revealed 136, trails built 25

A living scout at the starting numbers does 30 of the control's 136 tiles revealed (22%), and builds 2 of
its 25 trails: under a quarter, so build-5 has this to raise (an `ask` if it stays there).

## sweep --judge, before and after (`judge --compare`, no floor in either report)

Before is the branch at build-2's head, after is this build. The AI's idle scouts now wander, and the
scores that moved:

| group | metric | before | after |
|---|---|---|---|
| standing | aligned_200 | 0.2364 | 0.2630 |
| standing | top_200 | 0.1076 | 0.1177 |
| doubt | lead_changes | 3 | 6 |
| decisions | ai_orders_per_100 | 9.04 | 10.68 |
| motion | land_changed | 1612 | 2183 |
| motion | clans_joined | 90 | 137 |
| land | held_200 | 21.76 | 24.29 |
| land | trading_200 | 0.9250 | 0.9321 |
| economy | store_mean | 16.06 | 23.12 |
| economy | turns_at_pack | 238.6 | 380.9 |
| armies | people_vs_control | 0.9969 | 0.9835 |

The pre-games are unchanged (trading_pick, markets_pick, pregame_by_centres 7 of 7); the end is the
same (turns_mean 228.6). This is more than a small move: scouts that live (or wander) change who the AI
reaches and what its stores hold, so the standing and motion groups shift by a tenth. Build-5 counts the
AI's scouts lost by +200.
