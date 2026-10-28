# Build 2 (kiln): the land wears, trails follow the walking, grain keeps taming

Piece #54, "Actors, and clans that live off their land". Build-1 (the clay) built the actor
model and moved clans onto it: territories, divided labour, walks priced by
`actors::walk_cost`, and a record of which tiles were walked each turn. Read its summary
first: it names the actor's types and functions as they are. You add three rules to the land.
Work in `engine/`; read `engine/CLAUDE.md` first. `relay/specs/54/spec.md` has the whole plan.

**Keep the clay's shape.** Where it doesn't hold for what you need, make the smallest fix that
works and record it as a decision. **Don't tune**: build-5 tunes every number. Pick starting
numbers that run and say what they are.

## The design (from `docs/design/clans.md`)

> - Food that's taken regrows each turn towards its cap. Game breeds: a hunted-out tile
>   recovers slowly.
> - **Depletion.** A tile harvested past what it regrows wears down: its cap falls. It
>   recovers once it rests.
>
> **Trails.** A route walked year after year wears a trail. Trails belong to the tile, not to
> whoever wore them, so everyone uses them. A trail nobody walks wears away again.
>
> **Taming.** Grain on a tile a clan holds tames a little each turn, and keeps taming through
> the main game; its grain grows with it. Grain on land nobody holds slowly goes wild again.

## What to build

### Depletion

- Each tile gets a wear per food, `0.0..=DEPLETION.max`, and keeps its unworn caps: the
  generated cap for each food, and for grain the tamed cap (`land::hold` computes it from
  `wild_grain` and `tame`). The tile's `cap` is always unworn × (1 − wear), so everything that
  reads `cap` (regrow, the harvest, the page) sees the worn cap. Stock above a fallen cap is cut
  to it.
- Each turn, per tile and food, compare what clans took this turn with that food's
  sustainable yield at its current cap: `regen × cap` for foods that don't breed, and for game
  (`breeds`) its most, `(regen / 4 + restock) × cap`. Take above the yield raises the wear by
  `DEPLETION.rate × (take − yield) / cap`; take at or under it (or none) lowers it by
  `DEPLETION.recover`. Clamp to `0..=max`. You'll need the turn's take per tile and food:
  record it in the harvest step (or wherever the clay leaves room). Run it as its own step in
  `TURN` after `eat` (call it `wear`), or inside `hold`; say which.
- A tile is **depleted** when some food's wear is at least `DEPLETION.depleted_at`.
- `DepletionRules` in `data.rs` with doc comments: `rate`, `recover`, `max`, `depleted_at`.
  Start around `rate 0.5, recover 0.02, max 0.6, depleted_at 0.1`.

### Trails from walking

- Each tile counts the turns in a row it was walked (on any path the clay records: inside a
  territory or along a route) and the turns in a row it wasn't. Walked `TRAIL.after` turns in
  a row, it gets a trail; unwalked `TRAIL.fade` turns, a trail wears away. Holding a tile no
  longer makes a trail; `held`/`unheld` stay for taming and forming.
- `land::hold`'s doc comment and `TRAIL`'s say so.

### Open-ended taming

- `tame` counts on with no ceiling while held (after `TENDING.after` held turns, as now), and
  falls a step a turn unheld, as now.
- A hearth is still `tame ≥ tame_turns`; `HearthTamed` fires once as `tame` reaches it.
- The grain multiplier on wild grain: `1 + (domestic − 1) × tame / tame_turns` up to
  `tame_turns`, then on by `TENDING.beyond` per further `tame_turns` (start `beyond` 0.25).
  This is the unworn grain cap; wear applies on top.

### Reported

- `stats::Summary` gains `depleted` (tiles), and `sansfiat sweep`'s series (build-1's
  `--every` lines) gains a depleted column.
- `core/tests/common/mod.rs`'s `flat_radius` resets every tile field by name: reset the new
  ones.

## Tests (`core/tests/land.rs`, or a new `core/tests/wear.rs`)

On hand-built worlds (`common::flat()`, `put`):

- A food taken past its yield turn after turn wears its cap down, never past `max`; the cap is
  unworn × (1 − wear); stock is cut to a fallen cap.
- Resting (no take, or take under yield) lowers wear back to 0 and the cap back to unworn.
- Game's yield uses its breeding most; per food: wearing a tile's game leaves its fruit cap.
- A tile walked `TRAIL.after` turns in a row gets a trail; a tile held but never walked (home,
  say) doesn't; an unwalked trail fades after `TRAIL.fade`.
- Taming past `tame_turns` keeps raising the grain cap by `beyond` per `tame_turns`; unheld,
  it falls back; `HearthTamed` once.
- A depleted tile counts in `summary().depleted`.

## Done

- `engine/check.sh` passes.
- `./target/release/sansfiat sweep --size all` (after `cargo build --release -p sansfiat`)
  runs; its output in `relay/specs/54/land-sweep.md`, and one sentence in your summary on
  how many tiles deplete and whether the pre-game still ends.
