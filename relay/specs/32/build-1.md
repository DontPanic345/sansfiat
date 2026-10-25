# Build 1: civs, phases and the pick in the engine

Piece #32, Founding. You build the engine side of founding, the store and the pick. Units
come in build-2 and the page in build-3; leave room for them as this brief says, and build
no unit code. Work in `engine/`. Read `engine/CLAUDE.md` first ("Adding a mechanic",
"Invariants"). `relay/specs/32/spec.md` has the whole plan if you need context.

## The design you're building (from `docs/design/founding.md`)

> The pre-game ends with three centres. Founding turns each into a civ, the player picks one
> to lead, and the main game starts.
>
> - Each centre becomes a civ, with a name and colour, owning its market. A capital city rises
>   on the market tile, and the market never fades.
> - The capital holds the land within 2 tiles as the civ's territory and works the tiles next
>   to it.
> - The player picks a civ to lead, or watches all three on the AI. The pick shows each civ's
>   clans, people, hearths, terrain and distance to the nearest rival.
> - Every clan within reach of a founding market joins the nearest one. A civ that gets none
>   takes the nearest unaligned clan. Other clans stay unaligned.
> - Clans that join keep living as they did; the clan rules run for every clan in the main
>   game, and the other markets keep forming and fading as in the pre-game.
> - A civ clan's daughters belong to its civ, so a civ grows as its clans do.
>
> The store: each civ has one store, holding every good. It starts with an endowment, 30
> grain, standing in for the boon year. Each turn, a fed civ clan hands half its leftover
> store to the civ. Food in the store spoils, as it does in a clan's store. The store has a
> cap. Past it, the excess is lost.

merge4's founding is the starting point (read, never edit): `merge4/src/clans.js`
`foundCivs`, `merge4/src/modules/civ/rules.js` `onStart` and `upkeep`, `merge4/src/main.js`
`candidateHtml`, `merge4/src/data.js` `CIVS`, `merge4/src/modules/civ/data.js`.

## What to build

### State and rules

- `data.rs`: `CIVS: [(&str, &str); 3] = [("Red", "#d64545"), ("Blue", "#3b6fd6"), ("Gold", "#d6a63b")]`
  and `CIV: CivRules { territory: 2, endowment: [0.0, 30.0, 0.0, 0.0, 0.0], levy: 0.5, store_cap: 50.0 }`,
  each field with a doc comment like the other rules.
- New module `core/src/civs.rs` with:

  ```rust
  pub struct Civ { pub id: u32, pub name: String, pub color: String, pub market: u32,
                   pub capital: usize, pub founded: u32, pub store: Basket, pub lost: f64 }
  ```

  (`lost`: goods cut by the cap last turn.)
- `world.rs`: `World` gains `phase: Phase`, `player: Option<u32>`, `civs: BTreeMap<u32, Civ>`,
  all serialised. `Phase` is `enum Phase { Pregame, Choose, Main }`, `#[serde(rename_all = "camelCase")]`.
  Build-2 adds `units: BTreeMap<u32, Unit>`; don't add it now.
- `Clan` gains `civ: Option<u32>` (`None` in `new_clan`). Both `divide` and `split` give the
  daughter her mother's `civ`. Change nothing else in the clan rules.

### Founding: `civs::found(w)`

Called from `pregame::check_end` once, right after it sets `w.end` and pushes
`PregameEnded`. It reads `w.end.centres` and draws no random numbers.

1. For each centre market, in `End.centres` order (the k-th takes `CIVS[k]`): a civ with id
   from `w.next_id`, `market` the centre, `capital` its tile, `founded` `w.turn`, store
   `CIV.endowment`. Push `CivFounded` (tile: the capital; `civ`, `market` set).
2. Joining. For each clan (id order), the civs whose market's `markets::reach(w, tile)` holds a
   tile of the clan's round; it joins the one with the least path cost to any of its round
   tiles, ties to the lower civ id. Then each civ with no clan, in id order, takes the
   unaligned clan whose home is fewest hex steps from its capital, ties to the lower clan id
   (none if there are no unaligned clans). Other clans stay unaligned (`civ: None`).
3. Leave clans' stores as they are (the spec's decision 5).
4. `w.phase = Phase::Choose`. (Build-2 adds each civ's starting warrior here.)

### The pick and the phases

- `Action::ChooseCiv { civ: Option<u32> }` (`{"type":"chooseCiv","civ":12}`, or `null` to
  watch). Valid only in `Choose`, and `civ` must be a civ. It sets `w.player` and
  `w.phase = Main`.
- `EndTurn` is refused in `Choose` with "choose a civ to lead, or watch".
- `World::run(turns)`: when the phase is `Choose`, it first acts `ChooseCiv { civ: None }`
  (through `act`, so it's logged), then carries on. Document it on `run`. The CLI's `run` and
  `sweep` stop at the end as now.
- `civs::cards(w) -> Vec<Card>` (serialisable) for the pick, one per civ in id order: `civ`,
  `name`, `color`, `capital` (tile), `q`, `r`, `clans` (count of its clans), `people` (their
  pop summed), `hearths` (hearths on its land), `terrain` (the three commonest terrain names
  on its land with their share, `[(name, share)]`, ties by terrain index), `rival_steps` (hex
  steps from its capital to the nearest other capital), `rival_by_land` (whether any other
  capital can be reached over passable land: one `cost_map` with an infinite bound).

### The tile's civ and the civ's land

- `civs::tile_civ(w, i) -> Option<u32>`: the civ whose capital is within `CIV.territory` hex
  steps of tile `i`; else the civ of the clan whose round holds it (`tiles[i].claim`); else
  `None`. A civ's land is every tile whose civ it is (`civs::land(w, civ)`, sorted).
- `civs::work_tiles(w, civ)`: the capital's tile and its on-map neighbours. Nothing harvests
  them in this piece (the spec's decision 7); the page draws them.

### The store: the `civs` step

A new `Step { name: "civs", run: civs::upkeep }` right after `eat` in `TURN`. For each civ in
id order:

1. Spoil: every good × `(1 − CLAN.spoilage)`.
2. Levy: each clan of this civ with `fed` hands `CIV.levy` × its `store` to the civ (taken
   from the clan's store, good by good).
3. (Build-2 feeds units here.)
4. Cap: if the store holds more than `CIV.store_cap` in all, scale every good down so it holds
   exactly the cap; `lost` is what was cut (else 0).

With no civs the step does nothing, so the pre-game is unchanged.

### Markets

`markets::fade` keeps every civ's market as well as `End.centres`.

### Events

- `Event` gains `civ: Option<u32>` and `unit: Option<u32>` (`unit` is for build-2; it stays
  `None` here). Keep `World::event`'s signature for the existing callers (they set the new
  fields to `None`) and add a helper for civ events. Fix the struct literal in `land::hold`.
- `EventKind::CivFounded`, worded in `Event::text` as `"Red is founded at (q, r), on market 271"`.
  Event text stays the one wording, used by the CLI and the page.

### CLI

- `run`'s report, when founding happened: one line per civ: name, capital `(q, r)`, market,
  clans, people, store total, and hearths on its land. Its `--json` output carries `civs`
  through the world JSON as now.
- `digest`: runs on through the pick (via `run`), so its digests now cover founding and
  watching. Keep the output format.

### Docs

Update `engine/CLAUDE.md`: the layout table (`civs.rs`, the new world fields and steps, the
action), and "Adding a mechanic" if anything in it changes.

## Tests (`core/tests/civs.rs`, plus edits where named)

Use `common::big()` (radius 12) with three markets at least 10 apart (as
`topped_up_markets_survive_the_turns_after_the_end` in `core/tests/pregame.rs` does), clans
placed with `put`, and either `w.end` set by hand followed by `civs::found(&mut w)`, or a real
end at the limit (`w.turn = PREGAME.max_turns - 1; w.run(1)`).

- Three civs, named Red, Blue, Gold in `End.centres` order, distinct colours, capital on each
  market's tile, store exactly 30 grain, `founded` the end turn; one `CivFounded` per civ;
  phase `Choose`, `player` `None`.
- A clan with round tiles in reach of two founding markets joins the cheaper; on a tie, the
  lower civ id. A clan out of every reach stays unaligned, unless a civ with no clan takes it:
  the nearest by home hex steps, ties to the lower clan id.
- `EndTurn` is refused in `Choose` (the world unchanged); `ChooseCiv` is refused in the
  pre-game and in `Main`, and for an id that isn't a civ; `ChooseCiv` with a civ sets
  `player` and `Main`; with `null`, `Main` and no player. `World::run` past the end watches
  (the log holds `ChooseCiv { civ: None }`).
- Daughters: a civ clan that divides, and one that splits, give their daughter the civ; an
  unaligned clan's daughter is unaligned.
- Store: spoils by `CLAN.spoilage`; a fed civ clan hands over half its store (its own halves);
  an unfed civ clan and an unaligned clan hand nothing; past the cap the store is cut to 50 and
  `lost` says by how much.
- A civ's market with zero activity doesn't fade over several turns.
- `tile_civ`: within 2 of a capital is that civ's, even on another civ's clan's round; beyond,
  a civ clan's round tile is its civ's; an unaligned clan's tile has none. `cards`: clans,
  people, hearths and `rival_steps` on a constructed world; `rival_by_land` false when sea
  splits two capitals.
- Real map: small seed 2 (ends at turn 276) run to the end: three civs on the three centres,
  every clan's `civ` is a civ or `None`, and a civ has no clan only if no clan was left
  unaligned. Then
  `ChooseCiv { civ: Some(first) }` and 100 more turns: stores never above the cap after the
  `civs` step, every civ clan's civ exists, the civs' markets still exist.
- Replay: the existing `replaying_a_save_rebuilds_the_world` style, through the pick: a save
  with `ChooseCiv` replays to an equal world with equal events.

## Constraints

- **The pre-game must not change.** No new random draws and no change to the arithmetic of
  existing steps. Existing tests must pass as they are, except edits for struct literals
  (`Event`, `Clan`) and tests that relied on running past the end (they now watch).
- Float maths: only `+ - * /`, `sqrt`, `floor`, `round`, `min`, `max`, `abs`; anything else
  through `libm`. No `HashMap`; sort before relying on an order.
- `cargo test` at `opt-level = 3` is about 90 s now; keep real-map founding tests to one or
  two small seeds that end early (small 2 ends at 276, 9 at 299).

## Done when

`cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`./build-web.sh`, `./cross-build.sh` and `node www/check.mjs` pass. `cross-build.mjs` runs
`endTurn` for every turn, and small seed 1 reaches its end (turn 382) within its 400 turns, so
it would hit the refusal. Add `Sim.phase()` to `web/src/lib.rs` (`"pregame" | "choose" |
"main"`) and have `cross-build.mjs`, before each `endTurn`, act
`{"type":"chooseCiv","civ":null}` when the phase is `"choose"`: the same as native `digest`,
whose `World::run(1)` watches at the pick. `check.mjs` stops at the end and needs nothing yet. In your result, name any existing test you changed and why, and
give `cargo test`'s time.
