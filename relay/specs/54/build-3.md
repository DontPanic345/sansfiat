# Build 3 (kiln): festivals, and people moving between clans

Piece #54, "Actors, and clans that live off their land". Build-1 (the clay) built the actor
model and moved clans onto it; a clan's festival is its market, reached by its one route, and
the `go` step already picks the festival worth the most. Build-2 made the land wear and trails
follow the walking. Read their summaries first: build-1's names the actor's types as they are.
You change how festivals form and fade, what clans carry to them, and add people moving
between clans there and small clans merging. Work in `engine/`; read `engine/CLAUDE.md` first.
`relay/specs/54/spec.md` has the whole plan.

**Keep the clay's shape.** Where it doesn't hold for what you need, make the smallest fix that
works and record it as a decision. **Don't tune**: build-5 tunes every number. Pick starting
numbers that run and say what they are.

## The design (from `docs/design/clans.md`)

> A clan's market is a **festival**. Clans have little to trade there, since food is too
> heavy to carry far: they trade people, and later ideas and small goods such as obsidian.
>
> - **One route.** Each clan walks one route, to one festival, each turn. It changes festival
>   when another is worth more.
> - **Forming.** A festival forms where clans in reach have none worth going to, kept a
>   distance from other festivals. A clan can hold one on its own.
> - **Clearing.** The festival clears with the market algorithm and swaps the goods.
> - **Fading.** A festival nobody comes to fades away.
>
> - Clans at one festival trade people. Starving clans shed people to growing ones, so a
>   growing clan there grows faster, and a starving one shrinks faster, than it would alone.
> - A clan too small to stand alone merges into another at its festival, or its people
>   spread across the clans there.

The pre-game still ends on centres as now (`pregame.rs`): a festival lasted `centre_age`,
with `centre_clans` traders, a hearth in its reach, `centre_spacing` from the others.

## What to build (`core/src/markets.rs`, `core/src/clans.rs`, `core/src/world.rs`)

### Forming

In the `markets` step, after fading: each clan with no festival this turn (`market` is
`None`) that has been settled `MARKET.form_after` turns (`since`) raises a festival on its
home, if no festival is within `MARKET.spacing` hex steps of it. Clans go biggest first (then
lower id), each checked against the festivals raised before it this turn. `MARKET.min_clans`
goes. `MarketFormed` per festival, as now. A clan that raised one goes to it from next turn by
the `go` step (its route cost is 0).

### Fading by attendance

`Market.activity` smooths the number of clans that came (`activity = fade × activity + (1 −
fade) × traders`), and a festival fades when it falls under `MARKET.min_attendance` (start
0.5, replacing `min_volume`). A festival one clan keeps coming to stays. Centres and civs'
markets never fade, as now. A new festival starts at an activity that lasts a few turns
without anyone (as `raise` does now).

### Carrying little

A clan brings at most `MARKET.carry × pop` food (start 0.2) to clear: what it brings is its
basket scaled down evenly to that; what it didn't bring stays in its basket, and the cleared
goods come back on top. `clear` still conserves every good exactly, and so does the whole
trade.

### People

A new step `people` in `TURN` right after `eat` (before `civs`), so it reads this turn's
`fed_ratio`, `starving` and harvest. At each festival in id order, of the clans that came this
turn (in id order):

- each starving clan offers `MARKET.shed × pop × (1 − fed_ratio)` people (start `shed` 1.0);
- each growing clan (this turn's surplus, `max(0, nutrition(harvest, pop) − pop)` as `eat`
  reads it, above 0) has room for that surplus in people, one food a person;
- the offers go to the growing clans in proportion to their room; if the offers are more than
  the room, all are scaled down to it. People carry no food; people are conserved exactly.
- The festival records the people moved this turn (`Market.moved`), and the world a running
  total for the sweep.

People may move between clans of different civs: civs are out of scope (spec decision 12).

### Small clans

After people move, in id order: a clan under `CLAN.min_pop` (start 2.0) at a festival where
another clan came merges into the clan there whose home is fewest hex steps from its own
(ties to the lower id): its people and store join that clan, its tiles' claims are cleared,
and it's removed. A new `EventKind::ClanMerged` (`clan` the small one, `other` the one it
joined; text "Clan 12 merges into clan 7"). A small clan alone keeps on and dies under one
person, as now. If a merged clan's market or civ matters elsewhere (civs' clan counts, units
taking a person), check it still holds.

### Reported

- `stats::Summary` gains `moved` (people moved at festivals last turn) and the running total;
  `sansfiat sweep`'s series gains a people-moved column (moved in each `--every` span).
- `www/feed.js` (and `check.mjs` if it lists event kinds) knows `clanMerged`.

## Tests (`core/tests/markets.rs`, `core/tests/festivals.rs` or similar)

On hand-built worlds (`common::flat()` / `big()`, `put`, `open`):

- A lone clan settled `form_after` turns raises a festival on its home and goes to it; a
  second clan within `spacing` of it doesn't raise one; biggest first within one turn.
- A festival one clan keeps coming to never fades; one nobody comes to fades; centres don't.
- The carry cap: a clan brings at most `carry × pop`; goods are conserved over the trade.
- People: a starving and a growing clan at one festival: people move from the first to the
  second, the total is conserved, the move is capped by the room; a starving clan at no
  festival sheds nobody; two growing clans share the offer by room.
- A clan under `min_pop` merges into the nearest at its festival (ties to the lower id);
  `ClanMerged`; its tiles are free; alone, it doesn't merge.
- A save with merges replays to the same world.

## Done

- `engine/check.sh` passes.
- `./target/release/sansfiat sweep --size all` (after `cargo build --release -p sansfiat`)
  runs; its output in `relay/specs/54/festival-sweep.md`, and one sentence in your summary on
  how many festivals form, how many people move, and whether the pre-game still ends.
