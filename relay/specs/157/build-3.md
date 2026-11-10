# #157 build-3 (easel): the armies

Build-2 gave the AI a planner (`core/src/units/plan.rs`): five needs in order, the first with something
affordable to do giving its orders, and the land and gift needs built. Build the three army needs and what
they read of war, so the AI raises armies it can feed, hurts its nearest rival, takes a capital it outweighs
and pulls back from one it can't. Build-4 tunes it. Read `relay/specs/157/spec.md` in full (its settled
rules and decisions 7 and 8 are this step's), `probe.md` (a planner that only marches: games end, people
are thrown away), build-1's `judge.md` and build-2's `plan.md`.

The design: `docs/design/war.md` (Armies, Fighting, Supply and surrender, and its Decided ledger: how a
fight, a retreat, a cut, a siege, a surrender and a fall work) and `docs/design/supply.md` (the pack, the line,
the ring).

## The reads (`core/src/units/reads.rs`)

What the planner reads of war, each a function on `&World`, tested on its own:

- **`attack(w, unit) -> f64`**: the army's attack, each kind's people times its attack (`Fighter::rules`), halved
  when it is hungry, as `fight.rs` counts it (reuse its strength reading; don't copy the numbers).
- **`defence(w, civ, tile) -> f64`**: what a rival army meets on `tile`: the people of `civ`'s clans living on
  it times `FIGHT.militia` (homes, `homes::homes`), its guard's share there entrenched to the cap (on a capital,
  the whole guard, since it is drawn back), and `civ`'s armies on the tile or within `PLAN.near` steps (start
  2) at their defence and entrenchment.
- **`outweighs(w, unit, civ, tile) -> bool`**: `attack ≥ PLAN.outweigh × defence` (start 2.0: an army under half
  the other side's strength retreats, `FIGHT.retreat`).
- **`reaches(w, unit, tile) -> bool`**: the tile is in the civ's ring (`line::ring`, share under one), or the march
  there (`line::turns` on the path) leaves at least `PLAN.spare` (start 2) turns of the army's pack.
- **`rival_line(w, unit)`**: a rival army's line home (`line::line`), as tiles.
- **`cut(w, unit)`**: whether the army is cut off (`line::cut_off`) and by whom (`cut_by_rival`), and the tile
  that cuts it (`cut_at`).

## 1. Feed the armies

For each of the civ's armies, in id order; the first with something to do gives the orders:

- **Withdraw**: an army hungry (`Unit.hungry > 0`), or whose pack is under `PLAN.low` (start 0.4) of its cap
  outside its ring, moves toward its capital: to the nearest tile of its ring whose line carries it, else
  the capital. It keeps both switches on. A march on a capital ends this way when the army goes hungry.
- **Answer a cut**: a cut army that outweighs what holds the cutting tile moves onto it (breaks out); else it
  marches home.
- **Shed**: an army at home that is hungry with the store empty disbands a person a turn.
- The guard's half is build-2's.

## 4. Hurt the nearest rival

The nearest rival is the standing civ whose capital is fewest steps from the civ's.

- **Raise**: while the store holds `PLAN.packs` (start 4) packs past its keep and the civ has fewer than
  `PLAN.armies` (start 2) armies, raise every warrior the store fills (one person from each giving clan,
  `givers`) into a new army at the capital, or into its army at home; switches on. `wants_army` (build-2's keep
  hook) is true while there is a target for this need or need 5 inside `reaches`, and the store is short of
  the packs. #152's stories found the store banks only with the keep at its cap (`relay/specs/152/stories.md`).
- **Pillage**: an idle army sent to a tile of the rival's land where a rival clan lives (`homes`), the nearest
  one it `reaches` and `outweighs` the defence of; on arrival, `Order::Pillage` each turn while it feeds;
  it withdraws by need 1.
- **Cut**: where a rival army's line passes within the civ's ring and the army can stand on it, an army moves
  there and paints a zone across the line (`Order::Zone`, up to `zone_cap` tiles of the line, connected, no
  capital), and keeps it while its own line carries it (`ai_zone`'s afford test). It undeploys (an empty zone)
  when it can't.
- **Meet a rival on its land**: a rival army on the civ's land (an army standing on its claim) is met by an army
  that outweighs it there. Where it holds forest, archers are raised into the army that stays on its land
  (decision 8); armies that march are warriors.

## 5. Take a capital it outweighs

An idle army that `outweighs` the defence of a standing rival's capital and `reaches` it marches on it
(`Order::Move` to the capital, switches on). The nearest such capital first. It pulls back by need 1 when it
goes hungry. After a fall, the next.

## Idle armies

An army at home with nothing to do for `PLAN.idle` (start 20) turns disbands, so its people go back to the
civ's clan.

## Proof, and what later steps rely on

- `engine/check.sh` green; `SAVE_VERSION` and the pins move.
- `core/tests/plan.rs` gains, on stages, one test a lever:
  - `outweighs` on a capital with militia and a guard: an army of twice the defence outweighs, one of less
    doesn't; a hungry army counts half;
  - raising: with packs in the store and a target in reach it raises, with too few packs it doesn't, and it
    never raises past `PLAN.armies`;
  - marching on a capital it outweighs and reaches, and not on one it doesn't outweigh;
  - withdrawing when hungry, toward its ring;
  - answering a cut: breaking out where it outweighs the cutter, marching home where it doesn't;
  - pillaging a rival clan's home tile it reaches;
  - painting a zone across a rival army's line and undeploying when it can't afford it;
  - an idle army disbands after `PLAN.idle` turns;
  - no planner order is refused over 100 turns of a three-civ retro stage.
- `sansfiat sweep --judge --json --floor` on the seven maps: no panic, every game ends. In
  `relay/specs/157/armies.md`: the led block, lead changes, ends, capitals fallen, clans left, warriors
  starved, camps lost, surrendered, orders per 100 by kind, beside `judge.md`'s stand-in and `probe.md`'s
  marching planner. Every lever must be pulled at least once on the seven maps: say which aren't and why.
  Not a bound: the canvas tunes.
- `sansfiat bench --size large 8 --after 50` on the branch and its parent, two runs each: the planner's cost.
  If it is over 1.25 times, cache the ring per civ per turn (it is a whole-map search) before the canvas.
- Build-4 relies on: every `PLAN` number in `data.rs` (each with a note), the reads in `reads.rs`, and the tests.

`engine/CLAUDE.md`'s layout gains `units/reads.rs`.
