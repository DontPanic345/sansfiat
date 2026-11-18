# #338 spec-review probe: is anything in reach on the real maps?

Run on `relay/338` at c05367b (main's engine), all civs on the AI from the pick, every 10 turns to +110,
in a throwaway core test (not committed). For each standing civ:

- **a**: clans not the civ's whose market is its capital market (the gift reaches them).
- **b**: clans not the civ's whose home is on land a claim could join now: a walk from the civ's
  capital or settlements over land that is no rival's and no rival capital (`claim::check`'s rules;
  a claim has no size limit, `data.rs` `CLAIM` only shapes the first claim).

| map | civ | a, every sample | b at +0 | b at +50 | b at +110 |
|---|---|---|---|---|---|
| small 61288 | 200 (card 1) | 0 | 45 | 18 | 19 |
| small 61288 | 201 (card 2) | 0 | 83 | falls by +20 | — |
| small 61288 | 202 (card 3) | 0 | 44 | 42 | 42 |
| small 2 | 467 (card 1) | 0 | 87 | 59 | 21 |
| small 2 | 468 (card 2) | 0 | 74 | 53 | 20 |
| small 2 | 469 (card 3) | 0 | 15 | 4 | 3 |

(b) never drops to zero for any civ on any sample, and (a) never holds. So "nothing in reach", as
spec.md first set it, never holds on either map: a claim chained to a clan's home always brings a
clan (`leaning::land`). A line shown only then would never show in the played games. And the gift
reaches no clan on any sample: the clans at a capital market are all its own, so the gift line will
nearly always read "reaches no clan".

Cards are in civ id order (`civs::cards`), so the pick's second card on small 61288 is civ 201,
which falls to a rival by +20 when the AI plays it.
