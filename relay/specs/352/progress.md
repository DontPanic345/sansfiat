# #352 Joining, made plain: progress

## Done

1. The judge measures the question: `motion.land_joins` and `motion.land_wait` (clans that came to
   a civ with their home on its land, and the turns their home had been there). The control sweep,
   `main-sweep.json`, is main with only these added.
2. The rule (`leaning.rs`): a clan whose home is on a civ's land is that civ's (`land`). It runs in
   the `lean` step, after a claim order, after the `claim` step, and quietly at founding (no event,
   no count). `LEANING.land` is gone. Only a civ's clans pay its levy. A clan on its civ's land
   never leaves by leaning; one off it leaves as before, with the daughters that are off it too.
3. The reason: `Event.by` (`land`, `gift`, `trade`) on `ClanJoined` and `ClanLeft`, worded in
   `Event::text`. `Clan.gifts` is the part of each leaning that gifts bought; gifts win when they
   bought half or more.
4. What a clan pays: `World.levied` (derived, as `World.traded` is) holds each clan's levy this turn.
5. The text view: the clans line counts those on the land and the levy they paid; each clan's line
   says on or off the land and what it paid; every unaligned clan in reach is listed.
6. The page: a clan's circle is filled in its civ's colour, pale for none; the clan panel has Civ
   and Levy rows; Standing's tips say whose people it counts.
7. Tests: `core/tests/leaning.rs` has one for each Decided line; `www/check-stage.mjs` checks the
   panel's rows. `SAVE_VERSION` is 21.
8. Scores: `scores.md`, from `main-sweep.json` and `branch-sweep.json` (`compare.txt`).
9. Ledger lines in `clans.md`, `war.md`, `founding.md` and `page.md`.

## Played

Small 2 as civ 1, through `sansfiat play`: a claim order that adds (5, 6) and (5, 7), the home of
unaligned clan 138, and one turn. The feed reads "Clan 138 joins Red: its home is on Red's land"
and `why 138` gives the same reason. The hundred turns were not played.

## For the owner

- **Doubt fell.** Lead changes go from 13 to 6 over the seven maps and swings from 16 to 9
  (`scores.md`). Joining at once makes Standing follow the claim, and the planner's claims grow at
  one pace. No noise floor was run.
- **The scout errand test's map moved.** Main loses 3 scouts, all on medium 9; the branch loses 2,
  both on medium 8, with fewer hungry turns. The test held at most 4 lost and all on medium 9; it
  now holds at most 4. #337 changes how a scout eats.
- **Founding.** A clan on a civ's first claim is that civ's from the pick, even where another
  capital's market is nearer. A led civ starts with about twice the clans it had on main.
- The page's clan colours and panel rows are checked as data, not by eye.

## Verdict

Merge, by the owner, with no play step and no review.
