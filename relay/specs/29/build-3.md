# Build 3 (canvas): can a civ keep an army, and how far does a line reach

Piece #29. Read `relay/specs/29/spec.md` and build-1 and build-2's summaries. The easel gives
you the camp, its pack, line, forage and pillage, the AI's post, the page, and
`sansfiat sweep --maps retro --after 200` with `--no-army` as the control; build-2's first run
is in `relay/specs/29/sweep.md`.

Paint:

- Tune the numbers in the spec's "Unsettled" table (`CIV`, `UNIT`, the AI's post), and change
  the easel where it doesn't hold — a rule's form, the line's colours, the AI — recording each
  change as a decision.
- Proof: on all seven maps, every civ has a camp 200 turns after the pick, posted a real line
  from its capital, not on it; its clans' people end within a margin you name of the control's
  (the army costs, visibly, but doesn't collapse them). Report how far a camp of `UNIT.start`
  holds steady, in steps over open ground and over hills, and play a camp past its reach on
  the page until it goes hungry and back. It should feel like a handful of days' walk.
- Write the final sweep (army and control), the reach and screenshots to
  `relay/specs/29/sweep.md` and `screens/`. Keep `./engine/check.sh` passing; tests on real
  maps assert what holds, not counts.
