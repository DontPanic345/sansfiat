# #104 Armies: spec

## What it delivers

A civ raises as many armies as its store can fill packs for, each of **warriors** and
**archers**, and they fight for ground the way `war.md` tells it. Fighting is a rule, not an
order: it happens wherever people of two civs stand on one tile, every turn, by one rule in one
module (`units/fight.rs`). Guards on overlapping claims skirmish with small losses to both. A
big stack takes thinly guarded tiles one a turn and stalls at a capital, where the guard draws
back and the clans turn out as militia. A defender that stays put entrenches. A side losing
badly retreats. An army cut off from home eats its pack, goes hungry and surrenders to the
civ that holds the ground. A capital falls when a rival army stands on it with no militia
left, and its civ is out. The scout becomes a unit like the others: it has the switches,
forages round it, refills its pack, keeps its route's trail and can be raised again. On the
page, armies are stacks with their size, the panels raise by kind, and every fight is in the
feed, the alerts and "Why".

The AI doesn't fight (that is R's). It keeps its one army as now. On the AI-only maps nothing
overlaps (measured below), so fighting happens where the player, or a story script, makes it.

**The question:** played as scripts on the retro maps, does the Red and Blue story come out,
and does fighting move Standing?

It is asked in these scores (`docs/process.md`):

- **Standing.** In each of the three story scripts, the led civ's and the rival's Standing at
  the script's end, beside the same script played with fighting off (`--no-fight`, the
  control). Fighting moves Standing if the two differ by more than `JUDGE.lead_margin` (0.005)
  for some civ in at least two of the three stories.
- **Motion.** Per story and per control: people lost to fighting (both sides), tiles that
  changed hands by fighting, and armies surrendered. The control has none of these, by
  construction.
- **The story**, measured per script (item 22). The skirmish's losses are small and two-sided,
  and few tiles change hands. The stack takes thinly held tiles and stops at the guarded
  capital. The cut-off army surrenders to the rival.
- **Health.** `sweep --judge` beside main's (`relay/specs/104/main-judge.txt`, committed with
  this spec). Nothing panics, people are conserved (item 9), the AI's Standing, armies and
  guards at +200 move only by what the scout and foraging changes explain, and the no-fight
  control matches the fight run on maps where nothing fought.
- **Decisions.** The new levers: raise archers, raise a scout, raise into an army, disband by
  kind, and the scout's switches. Each has a button on the page and an order in `play`.

The control is the same script, map and civ with `Config.fight` off. For the AI-only health
check it is main's judge.

## Measured before cutting

- `sweep --judge` on main takes 26 s. Its report is `main-judge.txt`.
- **No overlap on AI maps.** Small 2 at +200, small 9 at +200 and medium 9 at +200 have no
  tile in two civs' claims. The AI's stand-in claim never reaches a rival's. So AI-only runs
  fight only where a camp ends on a rival's held tile, which the AI's posts never do. The
  stories carry the question, and the judge's sweep is a health check.
- **Stores are small, and the camp at home eats them.** Every civ's store is 0.0 at the pick
  on all seven maps: the boon year's surplus goes into the camp's and the scout's packs. The
  levy brings about 3–4 food a turn. A camp at the capital resupplies from the store each turn
  and the guard eats from it, and together they eat more than the levy brings: played with no
  gift, small 2's Red sits at 3.4 by +60 and medium 9's Gold at 12–14, and the led civ's guard
  starves within ten turns. With the camp away or disbanded the guard lives and the store
  fills: medium 9 Gold banks 46 by +35, nine warriors' packs (`spec-review.md`). So a civ keeps
  about four people of units at home, or banks for a stack of eight or nine raised at once. A
  "big stack" is that, beside a guard of 1–3 spread over 25–50 tiles. The likeliest story maps
  are medium 9's Gold (10 steps from Blue, the closest pair on the retro maps) and small 2's Red
  (13 steps from Blue). Build-3 chooses, and says why.
- **A march on a rival capital starves on the way.** Medium 9 Gold's camp of 3 sent at Blue's
  capital, at the pick or at +20, has its line cut by length three turns out and is gone in
  eight, two steps short of Blue's claim. The path bends round rough country, a line is cut
  past about cost 28, and the pack is five turns. The stack story stages: it marches to the last
  tile whose line isn't cut, refills, and then pushes.
- **Hunger beats surrender at 5.** A camp loses a warrior a turn from its third hungry turn, so
  the starting camp of 3 is gone on its fifth. Surrender is checked at `UNIT.hungry_turns`,
  before hunger's take (item 15).
- **The AI re-paints its claim every turn** it differs from the stand-in. A tile a stack takes
  would be re-claimed under it the next turn and taken again, counted each time. The stand-in
  skips tiles a rival army holds (item 8). Tiles the stack walked over come back behind it,
  which is how the rival closes round it.
- **The scout starves on its first errand.** Small 2, Red, routed from its capital to Blue's
  at the pick: it has its route by the next turn, goes hungry on the second, and is gone by +30.
  It forages only on a route and never on its own land, supplies only the turn's shortfall, and
  dies with nobody to raise another.
- Small 2's Red's guard also starves by +30 with its store at 0. That is the land piece's
  economy, not this piece's, but it means a story leading Red has to feed its guard.

## Shape: Clay, Kiln, Glaze

I took the shape the piece suggests. The hard centre is one rule: who is on a tile, how strong
they are, what each side loses, who holds the tile after, and who retreats. Every other part
hangs off it: militia adds a side, surrender and a fallen capital end a side, the page draws
the outcome, the stories provoke it. Once the rule's shape and its invariants exist, those parts
are settled. Its numbers only work together, though: the loss rate against entrenchment against
militia against the retreat line against the hunger that stops the stack. And only the stories
on real maps say whether they do. So the shape is Clay, Kiln, Glaze.

| Step | Part | Area | Why here |
|---|---|---|---|
| build-1 | clay | Stacks and kinds; many armies; raise and disband by place and kind; `units/fight.rs`: presence, strength, losses, entrenchment, retreat and holding, with `hold`'s contest folded in; a march stops on a contested tile; `Config.fight`; the fight's counters and notes; tests and invariants | Everything after reads a stack and the fight's outcome |
| build-2 | kiln | The war rules on the clay: archers and bow-making, militia, the capital's guard drawn back, siege, surrender, the fallen capital; their events, notes and alerts; the CLI's view and legal orders | Needs the fight rule's sides and counters |
| build-3 | kiln | The scout as a unit, and foraging that takes only what it takes; the judge's fight counts and the no-fight control; `sansfiat story` and the three scripts, run with their controls | Needs surrender and the fallen capital for the scripts; the scout is independent of the clay |
| build-4 | kiln | The page: stacks with their size, Raise and Disband by kind, the scout's switches, the fight in the feed and the alerts, "Why"; `www/play/army.mjs` | Needs every event kind and order |
| build-5 | glaze | Tune losses, entrenchment, retreat, surrender, militia and the kinds' strengths together until the three stories play out on real maps; health on the judge | Needs everything under it |

I cut the kiln in three by area: the rules (core, units and claims), the proofs (the scout,
the judge and the scripts, mostly CLI), and the page.

## Settled: stacks, kinds and armies (build-1)

1. **Kinds.** A `Fighter` enum, `warriors` and `archers` (camelCase on the wire), indexes a
   `FIGHTERS` table in `data.rs`. Each row has `attack`, `defence` and a `note`. Starting
   numbers: warriors attack 1.0 and defence 1.0; archers attack 0.8 and defence 1.4. Archers
   differ in attack and defence, not in range: fighting is always over one tile (`war.md`), so
   range would need a fight across tiles, which the design doesn't have. Horsemen wait for
   herds.

2. **The stack.** `CampState` gains `stack: [f64; FIGHTERS]`, people by kind. The invariant:
   `stack.iter().sum() == actor.people` within `EPS`, for every army, after every step and
   order. Anything that takes people from an army takes them from every kind in proportion:
   hunger, losses, disbanding with no kind named, surrender. The JSON stays one flat object
   (`UnitFlat`) and gains `stack`. The page and the view read it.

3. **Names.** In code and on the wire an army stays `UnitKind::Camp` (`"camp"`), since
   `founding.md` calls the army a camp of warriors. That spares every caller and every play
   script a rename. The page and the text view call it an army, as "Red's army 475".

4. **Many armies.** A civ may have any number. `Order::Raise` becomes
   `{ n, kind: UnitKind, fighter: Fighter, into: Option<u32> }`, with `fighter` defaulting to
   warriors and `into` to none:
   - `kind: camp, into: None` makes a new army at the capital with `n` of `fighter`, their
     packs filled from the store (refused if the store can't fill them, as now).
   - `kind: camp, into: Some(id)` joins army `id`, which must be the civ's and have a line that
     isn't cut (`cut_off` false). The recruits walk the line, so a surrounded army can't be
     reinforced. It is refused with "its line is cut: raise at the capital".
   - `kind: guard` is as now. `kind: scout` is build-3's (item 19). Until then it is refused as now, and so are archers until build-2 (item 17).

   `Order::Disband` becomes `{ unit, n: Option<u32>, fighter: Option<Fighter> }`: `n` of that
   kind, or of every kind in proportion with none, or the whole army with `n` none. The
   helper `units::camp(w, civ)` stays, as the civ's first army by id, for the AI, `Lead`, the
   view and the judge. The AI keeps its one army. `Lead` raises into its army (`into`), so
   `digest --lead` keeps its shape.

5. **Entrenchment.** `CampState` gains `still: u32`, the turns since the army last took a step
   (0 on the turn it moves; reset by a `Move` it acts on). Defending, its strength is
   multiplied by `1 + min(still × FIGHT.entrench, FIGHT.entrench_cap)`. Starting numbers: 0.1
   a turn, capped at 0.5. The guard counts as entrenched at the cap, since it never moves.

## Settled: the fight (build-1, `units/fight.rs`)

6. **Presence.** A civ's people on tile `t` this turn:
   - An **army's** people are split over the tiles it holds in proportion to its guard shares
     there (`CampState.guard`, after `hold`'s spread). An undeployed or marching army has all
     its people on its own tile. A deployed army's are spread thin, as its warriors are.
   - A **guard's** people are split over its civ's held claim in proportion to the guard's own
     shares (`claim::divide`'s first part, before the camps' shares are added). On the capital
     that is a share like any other tile's until build-2 draws it back (item 13).
   - **Militia** is build-2's (item 12). The clay leaves a place for it: a side is a list of
     parts, each with people, strength per person and where its losses go.
   - **Scouts don't fight.** A scout passes through anyone's land and is never a side. Only
     armies, guards and militia fight. Only an army takes paper (`claim::step` today lets any
     unit). This is a decision: a two-person party fighting a border would only lose people.

7. **Contested.** A tile is contested when two or more civs each have presence over
   `FIGHT.floor` people there (starting at 0.01). Since a held claim tile always has guard
   presence, "a claimed tile a rival enters" is the same case. Paper has no presence: an army
   on it takes it unopposed, as now.

8. **The rule, each turn, on each contested tile**, in tile order:
   - A side is one civ's parts there. Its **strength** is the sum over parts of people ×
     (attack, or defence for the side that held the tile at the start of the turn) ×
     entrenchment (defenders only) × `FIGHT.hungry` (starting at 0.5) for an army that went
     hungry last turn. The holder at the start of the turn is the army holding it, else
     `claim::tile_civ`. With three sides, each side's "other" is the sum of the rest.
   - **Losses** follow Lanchester's linear law: a side loses `FIGHT.rate × (the others'
     strength)` people, capped at its presence there (starting rate 0.1). It is split over its
     parts in proportion to their people there. An army's loss comes off its stack in
     proportion. A guard's comes off the guard unit. So thin guards on a border cost each other
     a little a turn, and a stack of ten facing a guard share of 0.1 loses almost nothing while
     the share is wiped out.
   - **Retreat.** After the losses, a side whose strength there is under `FIGHT.retreat` × the
     others' (starting at 0.5) retreats. Only armies retreat. An army whose own tile it is
     moves at once to the nearest tile its civ holds that isn't contested, by path cost. It ends
     its march and keeps its zone only if the zone is still connected, else it undeploys. With
     no such tile it stays and fights on. An army holding the tile as part of its zone stops
     holding it. **Guards and militia don't retreat**: a guard is spread over its land, not a body
     that falls back, and two guards' shares on an overlap differ by their walks from two
     capitals, so a guard's retreat would end every skirmish on its first turn
     (`spec-review.md`). A guard loses a tile only when all its people there fall: then the tile
     leaves its claim, as `claim::take` does today, with the part of the claim no longer joined
     to the capital. A stack does that in a turn; a rival guard never does.
   - **Holding.** The stronger side after the losses holds the tile; a tie stays with the holder.
     This replaces `hold`'s `beaten` test (larger share, then held last turn, then lower civ),
     which the fight folds in. `hold` keeps the spread (candidates, `divide`, the guard floor),
     and the fight decides between civs. Between two armies of one civ, `hold`'s rule stands.
     Guard against guard needs nothing new: `claim::tile_civ` already gives the land to the
     thicker guard, so an overlapped border skirmishes and neither side's claim shrinks unless
     one retreats.
   - **Tiles changing hands.** A tile changes hands when its holder at the start of the turn no
     longer holds it at the end: it retreated, or none of its people are left there. Each change
     is counted on the civ that won it (`Counters.taken`).
   - **The AI's stand-in claim** (`claim::ai_claim`) skips tiles a rival army holds or stands
     on. Without it the AI re-paints a taken tile under the stack the next turn, the guard's
     share returns, and the stack fights and takes its own tile every turn. Once the army moves
     on, the tile comes back to the claim.
   - **With `Config.fight` off** (the control): no losses and no retreats. Holding is the
     strength comparison alone, and everything else is as main.

9. **Counters and conservation.** `Counters` gains `fallen: f64` (the civ's people lost in
   fights: army, guard and, from build-2, militia), `taken: u32` (tiles it won by fighting)
   and `surrendered: u32` (its armies that surrendered, build-2). The world's people after a turn
   equal its people before, less the people fallen, less hunger's and the clans' own changes.
   A test holds the fight step to this on a built map.

10. **The march** stops on the first tile it enters where a rival has presence: arriving
    contests it (`war.md`). Paper and unclaimed land don't stop it. So a stack through a
    guarded claim takes one tile a turn, and "the dispersed guards barely slow it" means one
    step a turn instead of two or three.

11. **The step's order** in `units::step`: fallow countdown, every army marches, `hold`
    spreads, **`fight::step`** resolves every contested tile, then each unit's turn (forage,
    supply, eat, hunger, and from build-2 surrender), then the `claim` step as now. Each fight
    writes one note on each army in it and on each civ whose guard or militia fought. An
    example: "fought Blue's guard at (−4, −10): strength 6.0 against 0.4; lost 0.04 warriors,
    killed 0.10; holds it". The first turn a tile is contested emits `Skirmish` (civ, other,
    unit). A retreat emits `Retreated` and a change of hands `TileTaken` (civ that lost, other,
    unit). These notes are the "Why" of a fight.

## Settled: the war rules (build-2)

12. **Militia.** A clan of a civ defends its home tile when a rival army stands on it. Its
    people are a part of its civ's side there, at `FIGHT.militia` (starting at 0.3) × a
    warrior's defence, and its losses come off the clan's people. Unaligned clans raise no
    militia: they are nobody's. Militia never retreats.

13. **The capital.** While a rival army stands on a civ's capital, the guard is drawn back.
    Its whole people defend the capital, entrenched at the cap, and its shares over the rest of
    the claim are nothing until the capital is clear, so the claim goes paper behind it. The
    civ clans whose home is the capital are its militia. Any of the civ's armies there fight
    too. `Civ` gains `pressed: bool` (a rival army on its capital this turn), which
    `claim::divide` reads.

14. **Siege.** A capital is besieged while every walkable neighbour of it is held or stood on by
    a rival army. Besieged, its own clans (home on the capital) harvest nothing and no levy
    reaches the store. The store keeps spoiling. `Civ` gains `besieged: bool`. This is
    `war.md`'s "an army round a capital stops its clans working, and its store spoils down".
    Walls and sieges of cities are M9's.

15. **Surrender.** An army hungry `FIGHT.surrender` turns in a row (starting at 3, equal to
    `UNIT.hungry_turns`, and checked before hunger's take, since from that turn hunger takes a
    warrior a turn and a camp of three would be gone before a later threshold) whose line is
    cut by a rival surrenders whole to that rival. A line is cut by a rival when the line over
    walkable ground alone would carry a turn's food and the line that respects rivals' land and
    held ground doesn't; the rival is the first whose land or held ground lies on the first
    line (a new `line::cut_by_rival(w, unit) -> Option<u32>`; `line::cutter` falls back to the
    lowest rival and won't do). Its people and pack join that civ's nearest clan by steps (ties
    to the lower id), and it is counted in its civ's `surrendered`. An army whose own tile it
    took is never on a rival's land, so the rival is read off the line, not `tile_civ` of its
    tile. A hungry army whose line is open, or cut by its length alone, starves as now. `ArmySurrendered` (civ, other, unit, amount
    people) is an alert for both civs, with a note on both. With `Config.fight` off there is no
    surrender.

16. **A fallen capital.** After the fight, a capital falls when a rival army stands on it and
    its civ has no part left there: no guard, no army and under one person of militia (a clan
    under one person is gone, as `clans.rs` has it). Its civ is out:
    - `Civ.fallen: Option<(u32, u32)>` holds the turn and the civ that took it.
    - Its claim is cleared and its guard disbands.
    - Its clans go unaligned (`civ: None`, leanings cleared).
    - Its armies and scouts disband, their people to the nearest clan by steps.
    - Its store fills the packs of the taking armies on the tile, by id, and the rest is lost
      (the designer's call: a sacked store feeds the army that sacked it).
    - Its capital's market is an ordinary festival from then on.
    - `CapitalFell` (civ, other) is an alert for every civ.

    A fallen civ has no land (`tile_civ` skips it, its capital included), no Standing, no AI
    turn and no orders. The player's are refused with "your civ has fallen". The game goes on:
    its end is R's. Everything that reads `capital` for a rule skips a fallen civ: `tile_civ`,
    `claim::check`, `zone_ok`, the line, `ai`, `levy`, `leaning::lean` and `stats::standing`.

17. **Archers** are raised only by a civ that holds a forest tile (its land). Their bows take a
    turn of their labour: an army that archers join marches nothing on the turn they are
    raised (`CampState.bows`, cleared at the turn's end). Nothing else is spent, and there is
    no wood good. Refused with "you hold no forest to make bows on".

18. **Events and alerts.** `Skirmish`, `TileTaken`, `Retreated` (build-1), `ArmySurrendered`,
    `CapitalFell`, `ArchersRaised` and `ScoutRaised` (build-2, build-3) join `EventKind` with
    their wording in `events.rs`. Alerts for the led civ: a skirmish begun on its land or by its
    army, a tile it lost, its army retreating, any surrender it is party to, and any fallen
    capital.

## Settled: the scout, the judge and the stories (build-3)

19. **The scout as a unit.**
    - `ScoutState` gains `auto_forage` and `auto_supply`, written to the flat fields it already
      carries. `Switches`, `Forage`, `Pillage` and `Resupply` take a scout.
    - It forages (or pillages, on a rival's land) its own tile and the tiles within
      `SCOUT.reach` (starting at 1), richest first, paying each walk there and back from its
      labour. That holds with or without a route.
    - It takes only what it needs, and auto-supply refills its pack to full, as the camp's does.
    - Its route stays set once its trails are built: it keeps the route until `ClearRoute`, and
      while it is set the known path's tiles count as walked (`used`), so its trail holds.
    - `Raise { kind: scout, n }` makes a scout of `n` people at the capital, its pack filled
      from the store. A civ may have any number. `Disband` takes a scout.
    - **Foraging takes only what it takes**, for every unit (`strip`). The tile's stocks drop
      by what was taken, in proportion, and it rests `ceil(UNIT.fallow × taken / held)` turns
      (`supply.md`, Decided, M2 retro).

20. **The judge.** `Watch` and `Scores` gain a `fight` group: people fallen, tiles taken by
    fighting and armies surrendered, per civ and in total. `sweep --judge` runs one more
    control per map, `fight: false`, and prints the group beside it. `judge FILE` prints the
    group for the played game. `play` gains `--no-fight` for a new game.

21. **The stories.** `sansfiat story NAME [--seed N --size NAME --civ K] [--no-fight]
    [--turns N] --save FILE` plays a scripted story for the led civ through `World::act`, finding
    its units, tiles and rivals at run time, and writes a save that `judge FILE` scores. The
    scripts live in `cli/src/story.rs`, one function each. AI civs play as ever.
    Every story sets gift 0 and keeps its guard by the AI's own rule (`ai::land`'s raise and
    disband, made callable), and doesn't keep its camp at the capital beside the guard: the two
    together outeat the levy (Measured).
    - **skirmish**: the led civ claims a corridor toward its nearest rival until it overlaps the
      rival's held claim, and holds it, its camp posted on its own land off the capital.
      Default 80 turns.
    - **stack**: it banks the store (the starting camp sent out to forage on its own land, or
      disbanded), raises every warrior the store fills at once, into one army, marches it to the
      last tile of the path whose line isn't cut, refills, and then pushes at the nearest rival's
      capital, auto-forage and auto-supply on. Default 80 turns.
    - **surrender**: it marches an army into the rival's held land, past its guard, and halts it
      two or more steps inside. The rival re-claims the tiles behind it (item 8), which cuts its
      line. Default 60 turns.

    Each story is run with `--no-fight` too, and judged. The table goes in
    `relay/specs/104/stories.md`.

22. **The story's measures**, per script, fight against control. These are what the glaze tunes
    to and the review reads:
    - **skirmish**: turns contested; people fallen per contested tile-turn on each side (small:
      the glaze settles the band, starting from "under 0.05"); losses on both sides above zero;
      tiles taken by fighting over the run (few: at most a third of the contested tiles).
    - **stack**: claimed tiles taken on the way; whether it reaches the capital; turns at the
      capital; whether the capital falls (it should not against a guarded capital); the stack's
      people lost to fighting and to hunger.
    - **surrender**: turns from cut off to surrender; the people who went to the rival's clan;
      both civs' Standing against the control.
    - all: the led civ's and the rival's Standing at the end, and their difference from the
      control.

## Settled: the page (build-4)

23. The page draws each army as a stack with its people as a number, archers marked, several
    armies on a tile cycling as a stack does now. The camp panel ("Red's army 475") gains the
    stack by kind, its strength in attack and defence, and entrenchment. Its buttons gain
    Raise… (a stepper with warriors or archers, `into` this army, greyed with the check's
    reason) and Disband… with a kind. The civ panel's Raise… offers warriors and archers (a new
    army), guard and scout. The scout panel gains the switches, Forage, Resupply and Disband….
    The feed and sounds gain the new kinds, alerts as item 18 says. Selecting an army or a civ
    after a fight shows the fight's note among its reasons: that is "Why".
    `www/play/army.mjs` covers it, and `play/all.mjs` runs it.

## Left unsettled: the glaze (build-5)

The numbers, which only work together: `FIGHT.rate`, `entrench` and `entrench_cap`, `retreat`,
`surrender`, `militia`, `hungry`, the kinds' attack and defence, and `SCOUT.reach`. The glaze
plays the three stories on real maps with their controls, reads item 22's measures, and tunes
until:
- the skirmish costs both sides a little and decides nothing;
- the stack takes thinly held tiles and is stopped at a guarded capital;
- the cut-off army surrenders to the rival;
- a capital with no guard can fall to a stack, shown on one map;
- `sweep --judge` stays in health beside main.

It may change the kiln: a rule's form, a story's script, a threshold. Each change is a decision
with the run that showed it. If the economy can't feed a stack worth the name on any retro map,
it says so with the numbers, since that is part of the answer. It doesn't change the economy's
numbers (`UNIT.pack`, the levy, the store) to make it so. The notes go in `glaze.md`.

## Decisions (the owner can overrule)

- Fighting is Lanchester's linear law: a side loses a share of the other side's strength in
  people a turn, capped at its people there. The curve is linear. Entrenchment multiplies a
  defender's strength.
- Archers differ in attack and defence (weaker attacking, stronger defending), not in range.
- An army's people stand on its held tiles in proportion to its guard shares there, and a
  guard's over its held claim the same way. So a thin line is thin in a fight too.
- Scouts neither fight nor take paper. Only armies take paper.
- A march ends on the first tile it enters where a rival has people.
- Raising into an army needs its line uncut. A surrounded army can't be reinforced.
- Archers need held forest, and the army they join loses its march that turn to bow-making.
- Militia comes only from a civ's clans, at a fraction of a warrior's defence, and never
  retreats. The capital's guard draws back whole while a rival stands on it, leaving the rest
  of the claim paper.
- A capital is besieged when rival armies hold all of its walkable neighbours. Then its own
  clans harvest nothing and no levy reaches its store.
- A surrendered army joins the nearest clan of the rival that cuts its line, with its pack, after
  `UNIT.hungry_turns` hungry turns and before hunger takes a warrior. With its line open, or cut
  by length alone, it starves as now.
- Only armies retreat. A guard loses a tile only when all its people there fall.
- The AI's stand-in claim skips tiles a rival army holds.
- A fallen capital's store fills the taking army's packs, and the rest is lost. A fallen civ
  stays in `w.civs` with `fallen` set, and has no land, Standing or turn.
- An army stays `camp` in code and on the wire, and "army" on the page.
- The scout forages its tile and its neighbours (reach 1), keeps its route until cleared, and
  may be raised again in any number.
- The stories are Rust scripts behind `sansfiat story`, not lines files, because unit and clan
  ids shift between a fight run and its control.

## What the review plays and measures

1. `./check.sh` passes, with `play/all.mjs` and `army.mjs` in it.
2. `sweep --judge` beside `main-judge.txt`: the fight group, the no-fight control, and health.
   Say what moved and why: the scout and foraging changes, or fighting.
3. The three stories with their controls, from `stories.md` and the glaze's `glaze.md`. Re-run
   one (`sansfiat story stack` on the glaze's map) and `judge` it. Does the Red and Blue story
   come out, and does fighting move Standing (the answer, with the numbers)?
4. Play a fight on the page through `play.mjs`. Raise archers and warriors into two armies,
   march one onto a rival's guarded tile, and read the feed, the alert and "Why".
5. Invariants: the stack sums to people, people are conserved through a fight, and a replay
   matches (`cross-build.sh`, `the_save_version_pins_the_rules`, with `SAVE_VERSION` bumped).
6. The PR description gives the stories' table, fight against control, the judge's totals
   beside main's, the answer, and the screenshots of a fight on the page.
7. The Decided ledgers:
   - `war.md`: the fight rule, presence, the march stopping, militia, the capital's guard,
     siege, surrender, the fallen capital, archers and kinds, scouts not fighting;
   - `supply.md`: foraging takes only what it takes, the scout's switches, raising into an
     army needing its line;
   - `founding.md`: many armies, raise and disband by kind and place, a scout raised again;
   - `scouting.md`: the scout's foraging reach, its route kept until cleared;
   - `actors.md`: an army's people over its held tiles.

## Not in this piece

The AI fighting, or choosing to (R); horsemen; walls, forts and city sieges (M9); knacks (M5);
goods, a wood good; the economy's numbers; the game's end (R).
