# #356 build-4 (glaze): armies fed at others' markets, tuned, and the scores

Read `spec.md` here, `pass.md`, `kiln.md` and the earlier builds' summaries. You tune the
kiln's numbers on the retro maps and may change the kiln's rules where a number alone can't
make them work; record each change as a decision.

## The numbers

- `UNIT.salt` (salt a person a pack takes on Fill, start 1.0)
- `KEEPING.packs` (salt a civ store holds for packs beyond keeping, start 10)
- `PLAN.trade_at` (the pack's turns under which a fed-short army seeks a market, start 2.0)
- any number build-1 left on the kept passing (`pass.md` names it)

`probe.md` measured what you start from: 0–0.63 salt a turn reaches a capital (mean about 0.18,
half the civs under 0.1), and a unit of salt clears at a median 0.34 food. So a median civ's
reserve gives about 7 food over 200 turns. If the reserve fills on too few civs for any army to
carry salt abroad, the salt reaching a store is the lever: give the levy its own share for salt
(`KEEPING.levy`, today `CIV.levy`'s 0.2, with `exchange` on only) and record it as a decision.
Don't make salt from nothing or let a store buy it beyond what its clans bring.

## What proves it

`cargo run -p sansfiat --release -- sweep --judge --json` (about 3 minutes a run) beside the
`exchange`-off control in the same report and main's `sweep-main.json`:

- **Army-turns fed at a market not the civ's** at +200: above 0 on most of the seven maps is
  the aim; the control is 0. Give the food they bought over the armies' eating beside it:
  a count that rests on crumbs says so. If no setting gets armies there, say what stops them (no salt in
  stores, no market in reach, the planner's march) in `scores.md`; don't force it with a rule
  the design doesn't hold.
- **Largest share of trades by count and by worth** at +200, and which good: reported beside
  the control.
- **Health** within `judge --compare`'s floor of the control: people over control, starving
  clan-turns. Obsidian's mean steps from source should stay at or above build-1's.

Allow yourself at most five sweeps. Keep the best that holds health; don't chase noise
(`judge --compare` with `--all` gives the floor).

## Done

`relay/specs/356/scores.md`: a table of each try, the final row beside the control and main for
the two numbers, the trade group's shares by good, obsidian and salt's steps, and every score
`judge --compare sweep-main.json sweep-final.json` flags past its floor. `sweep-final.json`
here. `engine/check.sh` whole once if you changed the engine; pins bumped if the digest moved.
