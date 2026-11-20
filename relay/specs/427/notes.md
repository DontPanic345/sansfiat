# #427 notes for the next builder

- Settling is a reading: `Clan.settled` and `Clan.settle_run` change only in `settling::step` (the `settling` step,
  before `markets`) and `settling::found` (founding, from `civs::found`). A rule that starts reading it moves the game,
  and the sweep then stops matching main's.
- Both fields are skipped in serde while false/0, so a pre-pick world's digest is main's; post-pick digests moved
  (SAVE_VERSION 27). Saves under `relay/specs/3xx/play/` are version 26 and no longer load.
- The tuning probe: an ignored test in `core/tests/` running `Stage::retro` with `ChooseCiv { civ: None }` and 200
  turns over the seven maps (20 s after its build) dumped every clan's `fields` a turn; `sweep.md` replayed the state
  machine on that dump for each pair of turns. Settling being a reading makes an offline replay exact for the turns.
- `civs::repair` passes a civ to another clan when its own dies; the new own clan is not settled by it (only the
  founding clan is). On a bare staged map (`www/play/fields.mjs`) the founding clan starves out within 4 turns.
- The text view's `play` test caps the pick view under 90 lines: the living split rides on the `clans:` line for that.
- The page's settled mark is a square in the clan's circle: a house would read as the settlement mark. A capital's own
  clan sits under the capital's house and its centre ring, so its square rarely shows.
