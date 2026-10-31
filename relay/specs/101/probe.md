# #101 spec: what main does today

Taken by the spec on main at `a8c1d49`, release build, a 4-core runner, with a throwaway program
outside the repo (not committed) that runs each retro map to the pick with every civ on the AI, then
200 turns, and reads Standing as spec item 1 defines it (clans' and units' people of the civ over the
world's clans' and units' people).

| Map | Pick | By limit | Lead changes (no margin) | Clans changing civ | Standing +0 | +100 | +200 | Spread +0 / +100 / +200 |
|---|---|---|---|---|---|---|---|---|
| small 2 | 283 | no | 1 | 33 | .106 .075 .051 | .088 .176 .062 | .088 .212 .060 | .055 / .114 / .152 |
| small 3 | 219 | no | 3 | 0 | .065 .065 .076 | .058 .064 .058 | .058 .064 .058 | .011 / .006 / .006 |
| small 6 | 267 | no | 6 | 15 | .076 .059 .044 | .088 .068 .042 | .132 .094 .041 | .033 / .047 / .091 |
| small 9 | 216 | no | 0 | 4 | .118 .065 .060 | .126 .061 .059 | .125 .061 .059 | .057 / .067 / .066 |
| medium 8 | 296 | no | 7 | 3 | .037 .031 .031 | .034 .025 .038 | .034 .025 .038 | .006 / .014 / .014 |
| medium 9 | 380 | no | 8 | 0 | .028 .028 .037 | .027 .026 .031 | .019 .026 .031 | .010 / .005 / .012 |
| large 8 | 374 | no | 0 | 27 | .027 .014 .015 | .034 .010 .018 | .038 .010 .018 | .013 / .024 / .028 |

What it settles:

- **Standing moves on main**, so the judge has something to read before L: on small 2 one civ goes
  from 7.5% to 21% in 200 turns, by clans joining.
- **Lead changes need a margin.** medium 8 and 9 count 7 and 8 changes where the civs are within
  half a point of each other: two near-equal shares changing places, not a lead won. Spec item 4
  counts a change only when the new leader is ahead of the old by `JUDGE.lead_margin`; the starting
  value is 0.005 and the canvas tunes it.
- **Most people are in no civ**: the three civs together hold 6–35% of the world. The judge reports
  that share (`standing.aligned`).
- **Time.** The program took 17.4 s wall clock for the seven maps; `sweep --after 200 --maps retro`
  takes 17.6 s, and with `--no-army` 16.2 s. The judge runs the army sweep twice and the control once:
  about a minute in CI. `run --seed 2 --size small --turns 383` (the pick and 100 turns) takes 1.7 s,
  and large seed 8 to +100 takes 11.5 s, so `play` can rebuild its world by replay on every call.
- `bash relay/specs/100/baseline/compare.sh` prints six `ok` lines on main today (27 s), so #100's
  baseline is still main's, and it is this piece's digest proof.
