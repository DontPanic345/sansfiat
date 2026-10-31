# Bench, before build-1

`sansfiat bench 1 2 3` on main at `abfa2cd`'s code (the release build of this branch before any edit), three runs; best ms a turn of each seed (small, 200 turns).

| Seed | Best ms a turn | Runs |
|---|---|---|
| 1 | 1.558 | 1.656, 1.558, 1.558 |
| 2 | 3.471 | 3.471, 3.503, 3.516 |
| 3 | 1.710 | 1.710, 1.722, 1.812 |

The review holds the piece to within 10% of these (no more than 1.714, 3.818 and 1.881).

# Bench, after build-1

The same command, the same runner, after the `--state` flag, `events.rs`, the Basket helpers, `land::steps` and the renames. Three runs of `sansfiat bench 1 2 3` on the release build; best ms a turn.

| Seed | Before (best) | After (best) | After runs | Change |
|---|---|---|---|---|
| 1 | 1.558 | 1.551 | 1.561, 1.558, 1.551 | −0.5% |
| 2 | 3.471 | 3.486 | 3.517, 3.505, 3.486 | +0.4% |
| 3 | 1.710 | 1.712 | 1.719, 1.714, 1.712 | +0.1% |

Every seed is within 10% of its "before" number (the limits are 1.714, 3.818 and 1.881).

# After build-3

The same command on the release build after the actor behaviour (`Actor::{spoil, eat_by_share, eat_by_diet}`, `actors::stride`, the civ an actor). Three runs of `sansfiat bench 1 2 3`; best ms a turn.

| Seed | Before, build-1 (best) | After (best) | After runs | Change |
|---|---|---|---|---|
| 1 | 1.558 | 1.552 | 1.566, 1.558, 1.552 | −0.4% |
| 2 | 3.471 | 3.473 | 3.473, 3.476, 3.475 | +0.1% |
| 3 | 1.710 | 1.708 | 1.709, 1.708, 1.711 | −0.1% |

Every seed is within 10% of its "before" number (the limits are 1.714, 3.818 and 1.881).
