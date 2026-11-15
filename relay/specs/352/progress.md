# #352 Joining, made plain: progress

## Plan

1. A judge measure for the question: `motion.land_joins` and `motion.land_wait` (clans that came to
   a civ with their home on its land, and the turns they had waited there). **Done**; the control
   sweep `main-sweep.json` is main with only this measure added.
2. The rule (`leaning.rs`): a clan whose home is on a civ's land is that civ's (`land`), run by the
   `lean` step, after a claim order, after the `claim` step and quietly at founding. `LEANING.land`
   is gone. The levy is paid by the civ's clans only. **Done.**
3. The reason: `Event.by` (`land`, `gift`, `trade`), `Clan.gifts` (the part of a leaning gifts
   bought). **Done.**
4. Tests the rule moved, stage tests for each Decided line, `SAVE_VERSION` and pins.
5. The page and the text view: clans in their civ's colour, the clan panel's civ and levy, Standing's
   tips, the text view's lines.
6. Scores in `scores.md`, the ledger lines, `check.sh`, the PR.
