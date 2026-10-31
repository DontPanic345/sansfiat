# Build 5 (ford): the scripted game into core

Piece #100. Read `relay/specs/100/spec.md` item 14. The scripted game of `sansfiat digest --lead` is
the engine's second player: it exists twice, as `lead_orders` in `cli/src/main.rs` and as `leadOrders` in
`engine/cross-build.mjs`. It becomes one function in core that both the CLI and the wasm build run. **The
script's behaviour does not change, rule for rule;** the led baseline proves it.

## The work

1. **`core/src/script.rs`** (add `pub mod script;` to `lib.rs`): `pub struct Lead { pick: Option<u32> }`
   (`Lead::default()`), and `Lead::orders(&mut self, w: &mut World)` — the body of the CLI's `lead_orders`
   as it stands after build-4, with its doc comment of eight rules moved to the type. It uses
   `w.act(Action::Order { civ, order })` for every order and ignores refusals as the CLI does. Where the CLI
   code used `w.units.values().any(|u| u.civ == me && u.dest.is_none() && …Camp)`, use the camp from
   `units::camp(w, me)` and its state. `GIFT_DEFAULT` is `LEANING.gift_default` (the mjs copy goes).
2. **CLI**: `digest --lead` makes a `Lead` and calls `lead.orders(&mut w)` before each `EndTurn`; delete
   `lead_orders`.
3. **`web/src/lib.rs`**: `Sim` holds a `Lead`; `Sim::lead_orders(&mut self)` calls it. Doc comment: the
   scripted game, for the cross-build check and scripts.
4. **`cross-build.mjs`**: drop `leadOrders`, `GIFT_DEFAULT`, `byId`, `pick`, `act` and the rule comment.
   The loop is `if (lead) sim.lead_orders(); else if (sim.phase() === 'choose') sim.act('{"type":"chooseCiv","civ":null}');
   sim.act('{"type":"endTurn"}');` and the header comment points at `core/src/script.rs`. `www/play-scout.mjs`'s
   comment that names `lead_orders` is updated.
5. **Test** (`core/tests/script.rs`): two equal fresh worlds led by two `Lead`s for the small map's pick and 80
   turns end with equal logs and equal worlds; the log holds `Action::Order`s of the kinds the script gives
   (a `Raise`, a `Move`, a `Route`, a `Zone`, a `Disband`) and `SetGift`/`SetKeep`.

## Done

`./check.sh` green (cross-build included: native and wasm agree on the led run). The led run's `--state`
digest is identical to `baseline/seed2-r16-t400-lead.state.txt` on every turn (`diff` empty), the watching runs
too, and the led run's plain `digest` equals what build-4 pinned. `grep -rn lead_orders cli` is empty.

The comparison with the baselines is `bash relay/specs/100/baseline/compare.sh`: six `ok` lines.
