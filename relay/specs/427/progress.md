# #427 Settling, read — progress

Plan (whole step):
1. Taken from relay/341: the judge's capital-town counts and `outside` group (`judge_towns.rs`, `sweep_towns.rs`), the
   town mark above a capital (`towns.js` after `capitals` in `LAYERS`, `play/towns.mjs`), the scout cases to Lead's target
   only (`tests/cases.rs`), and the two capital-town tests in `tests/towns.rs`. Not the door step (a home rule: out of scope).
2. The settled state: `Clan.settled`, `Clan.settle_run`, `settling.rs` (the `settling` step, founding's `found`,
   `living`), `SETTLING` in `data/clans.rs`, events `ClanSettled`, `ClanGathers`. SAVE_VERSION 27.
3. The page: a settled clan is drawn as a house (`layers/clans.js`); the civ panel's Living row is by way of life plus a
   Settled row; the clan panel's "Lives by" row; feed kinds.
4. The text view: a `living:` line and each clan's way of life.
5. The judge: `judge_settling.rs` (settled clans/people at +100/+200 by civ and unaligned, settles and go-backs, clans
   changed), `sweep_settling.rs` (per map and total, beside towns and capital towns at +200).
6. Tune the turns on the retro maps (probe), sweep branch vs main, play, steps.json → review.

Done: 1–6. Turns tuned at 10 and 10 (`sweep.md`); the branch sweep matches main's in every score (`compare-main.txt`);
`sansfiat judge` prints a settling line for the led civ. Ledger lines in farming.md and founding.md. Handed on to the
review (the crew adds the play step).
