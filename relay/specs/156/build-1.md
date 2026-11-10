# #156 build-1 (easel): homes that don't claim, and shared field ground

Many clans will live on one tile. This step makes the engine able to carry that: a home is no longer a
claim, a clan gathers only what it claims, and a tile's field ground is shared among the clans that sow it.
Nothing in this step puts two homes on one tile. Build-4's home rule does that. So with every map as it is,
the game plays exactly as main. Read `spec.md` items 1–4 and decisions 1–6. `probe.md` says what the spec step
ran, and `probe.diff` is a rough sketch of build-4's rule, with the flaws this step removes.

The code: `core/src/clans.rs` (`seed`, `free`, `sites`, `move_on`, `adjust`, `divide_land`, `split`, `reap`,
`harvest`, `Reaped`), `core/src/fields.rs` (`sow`, `jobs`, `plots`, `home_toward`), `core/src/actors.rs`
(`edge`, `best_drop`, `change`, `step_toward`, `trim`), `core/src/markets.rs` (`in_reach`, `merge`),
`core/src/stats.rs` (`trails`), `core/src/land.rs` (`tend`), `core/src/data.rs` (`Config`),
`web/src/lib.rs` (the view's clans and homes), `core/src/stage.rs`, `core/src/world.rs` (`SAVE_VERSION`),
`cli/tests/save.rs`.

## What to build

1. **`core/src/homes.rs`**, the module for where clans live. `clans.rs` is 621 lines, so it doesn't grow:
   first move dividing (`halve`, `divide_land`, `split`) from `clans.rs` into `core/src/dividing.rs`, as is.
   - `homes(w) -> BTreeMap<usize, Vec<u32>>`: each home tile and the clans living on it, ids ascending.
     Derived from `w.clans`, never saved.
   - `claim_land(w, id, territory)`: writes the clan's claim on each tile of `territory` that is unheld or
     already its own, and never on a tile another clan holds. Every place that writes `Tile.claim = Some(id)`
     for a clan's land goes through it: `adjust`, `divide_land`, `split`, `move_on`, `seed`.
   - `gathers(w, id, t) -> bool`: `w.tiles[t].claim == Some(id)`.
   Build-4 adds the home rule here.
2. **A clan's territory is its home and the tiles it claims.** The home stays in `territory`, so `Walks`,
   `connected`, `best_drop` (which never drops home) and the field job read it as today. But `Tile.claim` on
   the home may be another clan's or nobody's. `free` clears only the clan's own claims, as now.
   `actors::edge` still offers only unheld tiles. `sites` still offers only unheld tiles, so daughters and
   clans moving on settle as today.
3. **Gathering only on one's own claim.** `reap` learns which of its tiles the clan may gather. Give it the
   clan's id, or a mask from `gathers`. A tile it doesn't claim gets no gathering share: the gathering job
   divides over the claimed tiles only, each paying its walk. `site_value` reckons a site as claimed, since
   `sites` only offers unheld tiles. Today every tile of every territory is its clan's claim (`probe.md`
   checked 960,000 territory tiles on the retro maps: none another's), so `reap` gives what it gives now, bit
   for bit.
4. **Fields on the home, whoever claims it.** `fields::jobs` takes the field tiles from every territory tile
   with ground that yields, the home included, as now.
5. **Shared field ground.**
   - `fields::room(w, id, t) -> f64`: `ground(w, t)` less the days the other clans sowed on `t` last turn,
     at least 0. Only clans that still have `t` in their territory count (the build-1 correction: `Tile.field`
     less the clan's own entry let a tile that a clan had let go keep its old days, and the first sweep moved
     every map). `sow` caps each tile at its room, not its ground. A tile only one clan sows has room equal to
     its ground, as now. `harvest` still resets `Tile.field` in pass two, where the total is written.
   - `Clan.sown: Vec<(usize, f64)>` (`#[serde(default)]`): the clan's field days per tile this turn, sorted by
     tile, only tiles with days.
   - **The harvest step reaps every clan first, then shares, then takes.** Pass one reaps each clan in the
     shuffled order, reading the stock before any take. Gathering tiles are exclusive, so each tile's stock is
     read by one clan and the takes come out as today's. Then, for each tile where the clans' field days add
     to more than its ground, every clan's days and grain on it scale by `ground ÷ Σ days`. Pass two applies
     the takes, writes `Tile.field` (the total), `Tile.take`, `Clan.sown`, the traffic and the baskets, in the
     shuffled order. Keep the rng draws and float operations in today's order where nothing is shared.
   - `fields::plots(w, c)` reads `c.sown`, not `Tile.field`, and the grain is the clan's own days × the yield.
     The judge (`judge_fields.rs`) and the view read `plots` and keep working.
   - `home_toward` uses the same room.
6. **The home index replaces the claim as a finder of homes.** `markets::in_reach` (a home within reach of
   a tile) reads `homes`, and counts every clan living on a tile in reach. `stats::trails`' worked tiles are
   held tiles that are nobody's home. Search for any other `claim … home == t` pattern and use `homes`.
7. **`Config.towns`** (`#[serde(default = "yes")]`, on in `Default`), with a doc comment: off is the control,
   in which a farmer's home never steps onto a tile another clan holds. Nothing reads it yet: build-4's home
   rule does.
8. **`SAVE_VERSION` bumps** for `Clan.sown` and `Config.towns`, and the pins in `cli/tests/save.rs` move.
   `engine/CLAUDE.md`'s layout gains `homes.rs` and `dividing.rs`, and `clans.rs`'s line says a clan gathers
   only what it claims.

## Tests: `core/tests/towns.rs`

Build each on a stage (`Stage::flat`, or the `map` text, with `Stage::civs` for a founded civ). Place clans
with `world_mut`: set `home`, `territory` and `Tile.claim` by hand to put two or three homes on one tile with
river ground, each clan claiming its own tiles round it, one of them claiming the shared home tile.

- **Each gathers its own claim.** Over a turn, the shared home tile's stock falls only by its claimant's take.
  The others take nothing there, and each takes from its own claimed tiles.
- **Ground is shared.** Three clans sowing one tile: `Tile.field` never exceeds `ground`, each clan's `sown`
  on it is in proportion to the days it asked, and the next turn each plans on its room.
- **A single sower is as before.** A clan alone on its tiles: `reap` and `sow` give the same numbers as a
  copy of today's (keep a small copy of the old `sow` and gathering in the test, or compare against values
  printed from main before the change).
- **Claims hold.** `free` for a clan living on another's tile leaves that tile's claim alone. `claim_land`
  never takes a held tile. A clan whose claimant neighbour dies claims the shared home tile on its next move.
- **The index.** `homes` lists every clan on a shared tile, ids ascending. `in_reach` counts them all.
- **`plots`** gives each clan its own days on a shared tile, not the tile's total.

## Proof and record

- `engine/check.sh` green.
- **The game is main's.** Run `sansfiat sweep --judge --json` on the branch and on a worktree of the branch's
  base. Every map's `standing`, `doubt`, `decisions`, `motion`, `land`, `economy`, `supply`, `fields` and
  `health` blocks are equal. The digest moves only because the save holds two new fields. To show that,
  `digest --state` with `sown` and `towns` removed from the JSON matches the base on small 3 and large 8
  past the pick (a scratch build is fine; don't commit it).
- Write the comparison to `relay/specs/156/homes.md`: the equal blocks, the stripped digests, and the bench
  (`sansfiat bench --size large 8 --after 50`, branch and base, two runs each).

## What later steps rely on

`homes::{homes, claim_land, gathers}`, `fields::room`, `Clan.sown`, `fields::plots` per clan,
`Config.towns`, the harvest's reap-share-take order, and `core/tests/towns.rs` with its staged shared home.
Build-2 keeps this step's digest exactly. Build-3's town reading and page read `homes`. Build-4's home rule
lives in `homes.rs` and moves a farmer's home onto a tile another clan claims.

## Done

`check.sh` green, the tests above, every map's scores equal to the base's, and `homes.md` written.
