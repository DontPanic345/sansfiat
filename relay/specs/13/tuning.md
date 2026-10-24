# Tuning pass (build-4)

Release build, radius 16, seeds as listed, `sansfiat sweep`. Only numbers in `data.rs` changed; no rule, and nothing
that looks at fertility. "Ratio" is the centres' mean `fertility_near / fertility_land` (land within 2 steps of the
market vs all land); `-` means no centres at the end.

## Before (starting values)

`sweep 1..10`: **0 of 10** ended by centres, every seed at the turn limit (499), cover 1.00 throughout. 4–5 markets
at the end, 48–137 clans.

What stopped it (seed 1 at turn 200, a debug dump of every market's clans): clans harvest what they need and eat
it, so stores are 0.00 and nobody has a surplus, so the gather bonus never applied. Gains from trade were 0–4 food
against walks of 2×cost = 2–6 labour, so 1–3 clans traded at a market. A market gets activity 4.0 on forming, loses
20% a turn and goes under `min_volume` 2 in about 4 turns, unless 4+ clans keep trading. Markets formed in bursts
(5 steps apart, so five or six at once), each with a trader or two. None reached age 30 with 4 traders.

## Round 1: the desire to gather and stores

- `MARKET.gather_bonus` 2 → 6: the bonus is what a clan will walk for a festival; 6 covers a round trip from the
  edge of reach (3 × 2), so a clan with a surplus goes from anywhere in reach.
- `CLAN.spoilage` 0.3 → 0.1: three tenths of a store spoiling a turn leaves nothing to bring, so nothing can be a
  surplus.

`sweep 1..10`: **0 of 10**. Markets 4–6, hearths up a little on some seeds. No change in traders: stores were
still 0.00 because clans spend all their labour on what they need.

## Round 2: more labour than need

- `CLAN.work` 2.5 → 3.5: a clan that has labour to spare harvests to its `surplus_target` (1.5 × need) and keeps a
  surplus, which switches on the bonus. It also leaves labour for the walk.

`sweep 1..5`: **0 of 5** at the limit, but markets 4–9 and a few seeds had a single centre (ratios 1.27, 1.56).

## Round 3: markets that last

- `MARKET.min_volume` 2 → 1 and `MARKET.fade` 0.8 → 0.9: a clan or two trading keeps a market above the floor, so
  it can reach centre age (30) while traders build up. Activity lasts 2–3 times as long.

`sweep 1..5`: **2 of 5** by centres (seed 1 at turn 227, seed 3 at 355). Seeds 2, 4, 5 end at the limit.

## Round 4: fewer, busier markets

- `MARKET.spacing` 5 → 8: with 5, five or six markets formed in a burst and shared the clans (1–2 traders each).
  Wider spacing leaves fewer markets, each with more clans in reach. Tried on seeds 6–10 at spacing 5 first:
  **0 of 5**. At 8: 3 of 10 over all ten seeds (before: 2 of 10 at spacing 5).

## After

```
seed   1: ended 363 by centres  clans 112  people  524.4  markets  9  hearths  47  cover 1.00  fertility ratio 1.19
seed   2: ended 499 by limit    clans 100  people  454.3  markets  7  hearths 251 cover 1.00  fertility ratio -
seed   3: ended 373 by centres  clans 120  people  530.6  markets  8  hearths  45  cover 1.00  fertility ratio 1.06
seed   4: ended 499 by limit    clans  98  people  494.6  markets  9  hearths 165 cover 1.00  fertility ratio 1.40
seed   5: ended 499 by limit    clans 124  people  576.6  markets 10  hearths  90  cover 1.00  fertility ratio 1.18
seed   6: ended 499 by limit    clans  71  people  319.3  markets  6  hearths  30  cover 1.00  fertility ratio 1.05
seed   7: ended 348 by centres  clans 156  people  598.7  markets 11  hearths 166  cover 1.00  fertility ratio 0.97
seed   8: ended 499 by limit    clans 104  people  477.3  markets  6  hearths  44  cover 1.00  fertility ratio 1.19
seed   9: ended 499 by limit    clans 144  people  548.6  markets 10  hearths  61  cover 1.00  fertility ratio -
seed  10: ended 499 by limit    clans  61  people  344.3  markets  7  hearths 109  cover 1.00  fertility ratio -
3 of 10 ended by centres, median end turn 499, mean fertility ratio 1.15
```

(Some seeds at the limit have centres at turn 499, which `fertility ratio` is taken over; `-` is none.) Time: 2.4–7.9 ms
a turn.

The limit of four rounds is used; the target (6 of 10) is not met: **3 of 10**.

## What is stopping it

- **Cover is never the problem**: 1.00 by turn 150–250 on every seed. People spread over all the land they can reach.
- **Hearths are not the problem** on most seeds (30–250 by the end); centres lack them only early. Seed 2 has 251
  hearths and still no centre.
- **Traders per market is.** A centre needs 4 clans trading there last turn, for 30 turns running, and the market
  must survive the whole time. In the sparse world a market keeps 1–2 traders (at the end of the run, seed 2: every
  market has 0–2). Expected gains from trade are 0–4 food against round trips of 2–6 labour; clans in reach at path
  cost 2–3 rarely find it worth the walk unless they hold a surplus, and clans eat what they harvest. Markets that
  do get 4 traders get them in short bursts, and a market with age 30 is rare (most are under 10 turns old; they
  fade as soon as a few clans stay away).
- **Many markets, few lasting:** every turn at most one forms, so there are always many young markets splitting the
  same clans. Spacing helps, but a market forming at every home held 20 turns with 4 clans in reach is cheap, and
  most of them go.
- The three that did end have ratios 1.19, 1.06 and 0.97 (seed 7): centres are near land only a little richer than
  average; with 3 centres per seed the sample is too thin to say they are on fertile land (see the review).
- Native and wasm runs of the same seed diverge (seed 1: native ends turn 363, the browser at 314), so a save made on
  one doesn't replay identically on the other. Likely `exp`/`powf` differences compounding; not investigated.

What would move it further with numbers alone, untried (rounds used up): `MARKET.min_clans` 4 → 3, `PREGAME.centre_clans`
4 → 3, `MARKET.reach` 3 → 4 (more clans in reach), `CLAN.surplus_target` above 1.5. Those loosen the test for a
centre rather than make a market busier; a rule that makes clans want to trade more (gains from trade that grow
with how different two clans' foods are) would be the real fix, and that is a rule change.
