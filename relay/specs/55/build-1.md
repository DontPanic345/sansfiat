# Build 1 (easel): posting, gifts, leaning and joining in the engine; the sweep

Piece #55, "Clans are the prize". Read `relay/specs/55/spec.md` first. Its "What's settled"
sections for build-1 (the civ's new state, keeping, posting and gifts, leaning and joining,
the player's lever, the sweep) and its "Tests the piece must have" are your exact done. The
design is `docs/design/founding.md` and `docs/design/actors.md` ("Markets"). `engine/CLAUDE.md`
has the commands and invariants.

## Where things go

- `core/src/data.rs`: a `LEANING` rules struct with doc comments, and `Config.gifts`
  (`#[serde(default = "yes")]`).
- `core/src/civs.rs`: the new `Civ` fields (each `#[serde(default)]`), `keep`, and founding
  setting `gift` to `LEANING.gift_default`.
- `core/src/markets.rs` `trade`: at a civ's own market, the gifts, the civ's basket in the
  clearing, its return to the store, and trade leaning. Keep the per-clan carry cap as it is.
- A new `core/src/leaning.rs` (or a section of `civs.rs`, your choice): the gift's leaning
  formula, the `lean` step (fade, floor, join), and `ClanJoined`. Add the step to `TURN` after
  `people`.
- `core/src/clans.rs`: `Clan.leaning` (`BTreeMap<u32, f64>`, `#[serde(default)]`); `divide`
  and `split` copy the mother's leanings to the daughter.
- `core/src/world.rs`: `Action::SetKeep { food: Option<f64> }` and `Action::SetGift { food:
  f64 }`, validated (the leader, `Phase::Main`, finite, ≥ 0, a gift ≤ `CIV.store_cap`) and
  applied; `EventKind::ClanJoined` and its text, "Clan {clan} joins {civ name}" (use
  `Event.civ` and `Event.clan`).
- `cli/src/main.rs`: the sweep's new columns and totals, `--no-gifts` and `--lever G` as the
  spec says (make `units::post` public for it); `lead_orders` gets the three new orders, and
  `engine/cross-build.mjs` `leadOrders` mirrors them exactly. Update both doc comments and the
  `USAGE` text.
- Tests under `core/tests/` (a new `leaning.rs`), covering every item in the spec's list.

## Numbers

Pick starting numbers that run, and say what you picked. Aim for clans joining on a small seed
within 200 turns of the pick, but don't touch `CIV.*` to get there: with `store_cap` 20 and a
3-warrior camp's keep of 15, a civ has at most 5 food spare a turn, so few or no joins is a
fine baseline for build-3. Say so if it happens. A reasonable start is `trade 1.0`, `gift 0.3`,
`gift_scale 0.5`, `starving 5.0`, `fade 0.03`, `join 1.0`, `gift_default 2.0` and
`floor 0.001`. Don't tune: build-3 does. Leave `CIV.*` and `UNIT.*` as they are.

## Done

- Everything in the spec's build-1 sections is built as written. Where it can't be, make the
  smallest change that works and record it as a decision.
- `./engine/check.sh` passes (fmt, clippy, tests, the web build, `cross-build.sh` with the
  `lead` line, `check.mjs`). `check.mjs` must still pass: the view gains fields but loses none.
- Write a first run to `relay/specs/55/sweep.md`. It holds `sweep --after 200 --maps retro`,
  the same with `--no-gifts`, and `--lever 6`, each as the command and its output, with one
  line on what it shows. This is the canvas's baseline, not a judgement.
- Your summary names the new functions build-2 and build-3 will call (`civs::keep`, the
  leaning step, `units::post`) and the view-facing fields.

Not yours: the page (build-2), the numbers (build-3).
