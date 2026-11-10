# #157 build-4 (canvas): games in doubt that end

The AI civs now play with every lever: a planner (`core/src/units/plan.rs`) reads five needs in order (feed
the armies, hold the land, grow Standing, hurt the nearest rival, take a capital it outweighs) and gives the
orders of the first that has something affordable to do. Tune it until a led game is in doubt and some end
before the limit, on the retro maps, against the idle control and the stand-in. Read `spec.md` in full: its
table holds every bound, and its "Unsettled" says what you may change.

What the easels give you:

- build-1's judge (`core/src/judge_led.rs`, `cli/src/sweep_led.rs`): every watched run read as three led
  games, each with its rank changes, end, Standing and in-doubt, beside an idle control per civ.
  `sansfiat sweep --judge --json --floor` prints the led block per map and in total; `judge.md` holds the
  stand-in's numbers and the JSON paths. `sansfiat judge FILE` reads a played game the same way.
- build-2's frame and land (`plan.rs`): the need order, the patch claim, the guard, the gift and keep through
  `World::set_gift`/`set_keep`, `LEANING.leave_margin` at 1. `plan.md` holds what it did.
- build-3's armies (`plan.rs`, `core/src/units/reads.rs`): `attack`, `defence`, `outweighs`, `reaches`, the
  rival's line, the cut; raising, pillaging, cutting, meeting, marching, withdrawing, answering a cut, and idle
  armies disbanding. `armies.md` holds what it did.
- `probe.md`: a planner that only marched straight at the nearest capital ended 1 of 5 games early and took
  4 capitals, and starved 1055 warriors and lost 85 camps doing it. Fighting is not the problem; judgement is.

## The painting

Run the sweep and read the led block. Then, until every bound in `spec.md`'s table holds:

- **Doubt.** Led games in doubt at least 11 of 21 and on at least 5 of 7 maps; lead changes more than the
  stand-in's and on at least 5 maps. If ranks never move, find what holds them: no army ever outweighs a
  capital, armies go and starve, the land never changes hands. Say which.
- **Ends.** At least 3 of 21 led games end before the limit, on at least 2 maps, at most 14, and at most one
  before +50. A rush that ends games at +15 is not doubt.
- **Beats idle.** The mean over 21 above the floor and positive in at least 15: the AI's civ must do better
  than one left alone. Where it does worse, find the need that costs it (an army raised from its clans that
  starves, a gift that empties the store) and say so.
- **Losable.** Clans leave on at least 4 of 7 maps. `probe.md` found the margin alone moves nothing under the
  stand-in; once armies take land and pillage, move `LEANING.leave_margin`, `leave_run` or `leave_at` until
  they do. A clan must not leave for noise: say why each map's clans left (taken land, a rival's festival,
  pillage).
- **No army starves by mistake.** Find why armies starve and surrender. Starving because a rival cut a line
  the army could not hold is the game; starving because the planner marched past what its pack carries, stayed
  hungry where it could have withdrawn, or raised what the store couldn't feed is a mistake: fix the read or
  the number.
- **Decisions.** Every lever pulled on the maps; levers repeated more than the stand-in's. No planner order
  refused.
- **The player.** Play one game yourself: `sansfiat play --game g.json --seed 2 --size small`, leading the civ
  first at the pick, 100 turns past it, playing as well as you can (`relay/players/player.md` says how). Then
  `sansfiat judge g.json`. Does a rival come for you? Can it take your capital if your guard is thin? Write it
  in canvas.md. The play step plays three seeds after you.
- **Scale.** `sansfiat bench --size large 8 --after 50` at most 1.25 times the base's (build-1's commit), same
  session, two runs each.

You may change the needs' affordable tests, any `PLAN` number, the reads, and leaving's numbers. The need
order and "the planner gives only the player's orders" are the piece's Decided: keep them. No fight, supply,
economy or field number moves; if a bound can't be met without one, say which and why, and leave it.
Record each rule you change as a decision.

## Proof

`relay/specs/157/canvas.md`: every bound in `spec.md`'s table, pass or miss, with the stand-in's number,
build-3's, and yours, from `sansfiat sweep --judge --json --floor` on the seven maps (the floor's nulls
included). Per map: led games in doubt, ends (how and when), capitals fallen, rank changes, beats-idle, clans
left and why, warriors starved and why. What you tried, each with its sweep's numbers. The game you played,
its `judge` lines, and what it felt like. `engine/check.sh` green, `SAVE_VERSION` and pins moved if the digest
did. Each sweep takes about two minutes on the runner with the idle runs.
