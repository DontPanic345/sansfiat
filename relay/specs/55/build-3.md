# Build 3 (canvas): is joining a lever worth pulling?

Piece #55, "Clans are the prize". Read `relay/specs/55/spec.md` (above all "What's left
unsettled" and "What proves the painting"), builds 1 and 2's summaries, and their first sweep
in `relay/specs/55/sweep.md`. Design: `docs/design/founding.md`, `docs/design/clans.md`, and
merge4's "Clans are the prize" in `merge4/docs/DESIGN.md`, where it was capped at 10 food a
clan a turn, valued 5× by a starving clan, faded 3% a turn and joined at 1.

## What the easel gives you

- A civ posts its store beyond `civs::keep` at its own market and gives up to its `gift` from
  it to the unaligned clans there. Clans lean from trade and gifts, the leaning fades, and a
  clan joins at `LEANING.join`. `SetKeep`/`SetGift` are the player's lever, and the AI stays on
  the defaults.
- `sansfiat sweep --after N --maps retro [--no-gifts] [--lever G] [--no-army]`, with clans
  joined per civ, unaligned clans left, and the led civ against its rivals.
- The page: the civ panel's leaning clans with the gift and keep controls, a clan's leanings,
  `ClanJoined` in the feed, and `www/play-lean.mjs` for screenshots.

## Your job

Make joining a lever worth pulling, or show that it can't be with this design. Tune every
`LEANING.*`, and `CIV.store_cap`, `levy` and `endowment` if posting and gifting need a bigger
store. You may also change the default keep and the AI's gift. If numbers can't do it, change
a rule's form: how a gift is split, what trade leaning reads, whether a civ's gifts draw clans
to its festival in `markets::worths`, or an AI policy for its gift. You may change the easel
(engine, sweep and page) as you need. Record each change of form as a decision.

The proof is in the spec's "What proves the painting": clans join but not all of them, gifts
matter against `--no-gifts`, the lever pays at a cost a player can see, the camps hold, and the
speed holds. Play it too. Lead a civ through `play-lean.mjs` (or the page itself) and check
that a player can see a clan coming and what a bigger gift does. Retake the screenshots in
`relay/specs/55/screens/` once the numbers are final.

## Done

- `relay/specs/55/sweep.md` holds the final runs (default, `--no-gifts`, and `--lever` at two
  or more gifts, each the command and output), a table of every number before and after and
  why, and **the answer**: is joining a lever worth pulling, in a few sentences with the
  evidence, including where it fails.
- `./engine/check.sh` passes, and the tests still assert what holds, not your numbers.
- Your summary carries the answer and names the sweep file.
