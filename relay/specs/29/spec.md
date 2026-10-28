# #29 A civ's army lives in its camp

## What it delivers

Warriors stop being units that walk about one by one. Each civ has one **camp**: an actor on
#54's model (`core/src/actors.rs`) whose people are warriors belonging to the civ. The player
moves the camp. It carries a **pack** (its stock) and eats from it; at the end of the turn, on
the labour its move left, it **forages** the tile it stands on (or **pillages** a rival's) and
**resupplies** along a **line** to the capital, where the civ's **store** is. Civ clans pay a
**levy** into the store when they walk to the capital's market. So an army costs the clans the
people (raising), the food (the levy) and the carrying (the camp's labour on its line).

The question the piece answers: on the retro's seven maps (small 2, 3, 6, 9; medium 8, 9;
large 8), all three civs on the AI, each raising its camp at the pick, does every civ still
have a camp 200 turns later? `sansfiat sweep` answers it, with a control run with no army.

## Shape: Easel and Canvas (as the piece suggests)

The camp's machinery is settled by the design and the piece: it is groundwork. What isn't
settled is whether the numbers let a civ keep an army and how far a line feels like it reaches.
That is judged by the sweep and by playing, so it sits on top, in one canvas step.

| Step | Part | What | Model |
|---|---|---|---|
| build-1 | easel | Engine: founding from the biggest clan, the levy by route, the camp actor, pack, line, forage/pillage, switches, raise/disband/move, events, the AI's orders; warriors removed; tests | sonnet medium (off the ladder) |
| build-2 | easel | The page (line, panel, buttons, switches, feed), the sweep with its control, `supply-lines/` archived, docs | haiku medium |
| build-3 | canvas | Tune the levy, pack, line, forage and the AI's post so every civ keeps its camp, and the line's reach feels right; `relay/specs/29/sweep.md` | opus low |
| review | | Against the design; plays a camp off its line and back through the page | opus low (coin) |

## Settled

1. **Founding.** Clans join civs as now (in the market's reach). Then at each centre, among
   the clans that joined it, the one with the most people (ties to the lower id) whose home is
   within `MARKET.reach` path cost of the festival becomes the civ: `Civ.clan`; with none
   there, its biggest joined clan (or the one it took). Its home at founding is the capital;
   the centre's festival moves to that tile and never fades. (Bounding the home keeps the
   capitals near the centres, which are `PREGAME.centre_spacing` apart, so `tile_civ`'s
   capitals don't crowd.) The civ's own clan keeps its home on the capital: it does not
   recentre or move on (it still adjusts its territory around it), and when it divides the
   half holding the capital stays the civ's clan. If that clan is ever gone (merged, died),
   `Civ.clan` passes to the civ's biggest clan; the capital stays.
2. **The levy** replaces "every fed civ clan, each turn". A civ clan whose festival route goes
   to the capital's market that turn pays `CIV.levy` of its surplus into the store, in the
   `trade` step at that market: its harvest beyond its need (`clans::surplus`), taken in
   proportion from its harvest, before `eat`. So growth, which `eat` reckons on the harvest
   beyond need, sees less: the levy slows growth as the design says. The civ's own clan always
   walks there (a route of cost 0 from the capital). Other civ clans pay nothing.
3. **The store** starts at `CIV.endowment` (the boon year), spoils at the clan rate and is
   capped, as now. The first camp's pack is taken from it at founding.
4. **The camp** is a `Unit` with `kind: Camp`, built on `Actor`: `people` are its warriors,
   `home` its tile, `store` its pack, `routes` its line (one `Route` to the capital, or none).
   One per civ. It holds no territory beyond its tile, and blocks no one.
5. **Raising** takes one person from each of `n` civ clans (the biggest first, each with at
   least `UNIT.min_clan`), wherever they live, and adds them to the camp; with no camp it makes
   one at the capital. Each raised warrior's pack share comes from the store. Refused when no
   clan can give or the store can't fill the pack. Founding raises `UNIT.start` warriors.
6. **Disbanding** `n` warriors (or all) sends them to the civ's own clan (`Civ.clan`), not the
   nearest. Disbanding all removes the camp; the civ may raise again. Their share of the pack
   goes to that clan's store.
7. **The pack** holds `UNIT.pack` turns of food per warrior. The camp eats `UNIT.food` per
   warrior each turn from it, and it spoils at `CLAN.spoilage` as every store does
   (`actors.md`). Short, it is hungry; `UNIT.hungry_turns` in a row and it loses
   warriors (one a turn, disbanded to the civ's clan as starved) until fed or gone.
8. **The turn**, in the `units` step after `civs`: the camp's labour is
   `people × UNIT.labour`. The ordered move spends it first, as `march` does now but with
   every warrior walking: a step costs `step_cost × people`, so a camp moves at one person's
   pace whatever its size, and a turn spent wholly on the march leaves nothing. On what's
   left: auto-forage takes from the tile; then auto-supply puts labour on the line for what
   eating needs this turn and foraging didn't cover, then for refilling the pack, only as
   much as the store holds to carry. Then it eats. One-shot buttons
   (resupply, forage, pillage) are orders for this turn's end, run on the leftover labour even
   with the switch off.
9. **The line** is `land::path` from the camp to the capital over walkable tiles not on a
   rival civ's land (`civs::tile_civ`). Its cost is `walk_cost(path cost, 1.0)`, as every route.
   Food carried = labour put on it × `UNIT.carry` / cost, taken from the store (as much as it
   holds). At the capital the cost is 0: the camp fills from the store with no labour. The
   camp's own tile never blocks its line (`land::path` doesn't test its start), so a camp
   just over a border can still draw; any other rival tile does. No path, or a line that
   can't carry the camp's food for a turn with all its labour (`people × UNIT.labour`, not
   what a march left), is **cut off**.
10. **Foraging and pillaging.** Foraging (own civ's or unclaimed land) turns labour into the
    tile's food stock at `UNIT.forage` per labour; pillaging (a rival civ's land) at
    `UNIT.pillage`, which is more. Either way the tile is stripped: `Tile.fallow` turns during
    which it yields nothing to anyone and regrows nothing. Clans harvesting it get nothing, so
    the camp foraging near home takes from its own clans.
11. **Switches.** `auto_forage` and `auto_supply` are on by default, per camp, set by an action.
12. **Events**: camp raised, warriors raised, disbanded, resupplied, cut off, went hungry,
    starved, foraged, pillaged. Resupplied and foraged are not logged every turn: only when
    they start after a turn without (so the feed isn't flooded).
13. **Actions** replace `MakeUnit` / `MoveUnit` / `DisbandUnit`: `Raise { n }`, `Disband { n }`
    (`None` = all), `MoveCamp { to }`, `SetSwitches { forage, supply }`, `Resupply`, `Forage`,
    `Pillage`. Each validated in `validate` with a reason, logged, replayed.
14. **The AI** (the civs the player doesn't lead, and all three when watching) keeps its camp:
    raises it if it has none and can, and posts it at a tile chosen by a rule build-1 writes
    and the canvas tunes, with a starting rule of "a few steps from the capital towards the
    nearest rival reachable by land, on own land", or, with no rival reachable (the design
    lets civs be cut off from each other), the same steps towards its farthest clan. It does
    nothing else. It runs in the engine (a step before
    `units`), so the page and the CLI watch the same game; it draws no random numbers.
15. **The sweep** gains a main-game mode: run to the pick, watch all on the AI, run
    `--after 200` turns, report per civ: camp alive, warriors, pack, hungry turns, warriors
    starved, **camps lost** (starved out to nothing, so the AI raised another), line cost and
    carry, turns cut off, foraged/pillaged tiles, its clans' people; and the same with
    `--no-army` as the control. The control is an engine switch (`Config.army`, default on,
    `serde(default)` so old saves load): no camp at founding, the AI raises none, and **no
    levy**, so the control's clans show what the whole army costs them. Default map set for
    this mode is the seven maps.
16. **Warriors as single units go**, and `lead_orders` and `cross-build.mjs` change to the new
    actions (lead the lowest civ; raise; move the camp out and back; disband some at n == 60).

## Unsettled, and who holds it

| What | Proof | Step |
|---|---|---|
| `CIV.levy`, `CIV.endowment`, `CIV.store_cap`, `UNIT.start`, `pack`, `carry`, `forage`, `pillage`, `fallow`, `hungry_turns` | the sweep: every civ on the seven maps keeps the camp raised at the pick for 200 turns: no camp lost (the AI re-raising one doesn't count as keeping it), and at least `UNIT.start` warriors at the end; its clans' people stay within a margin of the control (the army costs them, but they don't collapse) | build-3 |
| The AI's post | the same sweep, with the camp a real line away (not sitting on the capital) | build-3 |
| How far a line reaches | `sweep.md` reports the longest line a camp of `UNIT.start` holds steady on, in steps; by play it should reach a handful of steps on open ground (a turn is a year, so not "days"), much less over hills, and a camp past it should go hungry in a few turns. At the starting numbers it can't: a warrior's 3.5 labour over a line of `2 × steps` carries `1.75 × carry / steps` food, so at `carry` 1.0 a camp is cut off two plains steps out | build-3 |
| The line's colour scale on the page | by eye on screenshots | build-3 may change build-2's |

## Decisions the piece left open

1. The civ's own clan is pinned to the capital (settled 1): otherwise the capital, store and
   market would drift off with it, and the design says the market never fades.
2. "Design says the levy is a share of its surplus": the surplus is the store after eating,
   as the current levy takes it.
3. Starvation loses warriors one at a time rather than the whole camp, since a camp is many
   people and the goal is that civs keep their army.
4. Rival armies' ground blocking lines is F's; this piece blocks only rival civ land.
5. Raising draws from any civ clan, not just those near the capital: the warriors walk to the
   camp, and "drawn from the civ's clans" says nothing of distance.
6. The pack and the store hold every food; eating takes in proportion to what's held, as now.
7. "Design says the levy slows the clans' growth": the current levy takes from the store after
   eating, which growth never sees (and a clan with less store just harvests more). So the
   levy takes a share of the turn's harvest beyond need, at the capital's market, before
   growth is reckoned (settled 2). (spec-review)
8. A camp moves at one person's pace: every warrior walks each step, so the move costs
   `step_cost × people`. Otherwise a bigger camp would march further. (spec-review)
9. Auto-supply refills the pack as well as covering the turn's shortfall: otherwise a camp's
   pack, once spent on a march, never fills again off the capital. (spec-review)
10. A camp is cut off by its line's reach with all its labour, not what this turn's march
    left, so a marching camp isn't reported cut off every turn. (spec-review)
11. The pack spoils as every actor's store does (`actors.md`). (spec-review)
12. The control has no levy as well as no camp, so the margin against it is the army's whole
    cost to the clans: people, food and carrying. (spec-review)
13. Keeping an army means the camp raised at the pick is never starved out: an AI that re-raises
    a lost camp would make "every civ has a camp" true however badly it's fed. (spec-review)
14. With no rival reachable by land, the AI posts its camp towards its farthest clan, so every
    camp is a real line from its capital. (spec-review)
15. The civ's clan is chosen among clans living near the festival, not any that walk to it
    (some walk 10 steps): otherwise two capitals could land side by side. (spec-review)
