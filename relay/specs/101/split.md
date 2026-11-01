# #101 split by the owner

The owner merges #101 in two parts.

**Merged with #101:** Standing, the orders counter, the judge's scores and `sweep --judge` as a
report (no bands, not in CI), `sansfiat play`, the text view, `sansfiat judge FILE`, and the trial
game's save (`trial/g.json`), which a test reads.

**Held for the owner's review**, in its own PR: `engine/judge.toml` and the band check, `sweep
--judge` in `check.sh`, the `vision.md` Decided lines, the player and judge prompts
(`relay/players/`), and the trial's record (`play.md`, `bands.md`, the rest of `trial/`).

Why: a band on main's numbers holds whatever main does, including a change the owner made on
purpose. The camps' fall after #75 came with the founding rule the owner chose on #28, where the
owner said camps not starving doesn't matter to the game yet; a fail band on camps kept and
warriors starved made it the opposite precedent. So the army numbers are reported as `armies`,
not health, and nothing fails CI until the owner settles which numbers should.

The spec and build-3's brief still describe the bands as built; read them with this file.
