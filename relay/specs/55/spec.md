# Spec: #55, Clans are the prize

## What the piece delivers

After founding most clans are still unaligned: on small seed 2, 33 of 153 clans belong to a
civ at the pick. This piece lets a civ win the others over:

1. **Posting.** At its capital's market the civ trades as a clan does: it brings what its store
   holds beyond what it **keeps**, the market clears it with the clans' baskets, and what comes
   back goes into the store.
2. **Keeping.** By default a civ keeps what its camp needs. The player can set what their civ
   keeps, or put it back to the default.
3. **Gifts.** A civ gives up to its **gift** of food a turn to the unaligned clans at its
   market, from what it would otherwise post. The player sets their civ's gift.
4. **Leaning.** Each unaligned clan has a leaning towards each civ. It rises with what the clan
   gains by trading at the civ's market and with the food the civ gives it, and it fades each
   turn. A gift buys less leaning the more is given, and a starving clan values it more.
5. **Joining.** A clan joins the civ it leans to most once that leaning reaches the join level.
   From then on it is a civ clan: it is drawn to the capital (`CIV.pull`), pays the levy, and
   its daughters belong to the civ (as they already do).
6. **The AI civs** post, keep and give by the same rules, on the default keep and gift.
7. **The page.** The civ panel lists the clans leaning towards the civ, with the player's keep
   and gift controls. A clan's panel shows its leanings. The feed reports clans joining.
8. **`sansfiat sweep --after N`** reports the clans that joined each civ. `--no-gifts` is the
   control run with gifts off, and `--lever G` leads one civ with gift G against the AI.

The piece's question is the Shape line's: **is joining a lever worth pulling?** The canvas
(build-3) answers it with the sweep in `relay/specs/55/sweep.md` and by playing. The review
writes the answer and the evidence into the PR description.

Out of scope: more cities, materials and crafts, treaties, clans leaving a civ, a civ posting at
any market but its own, clans of one civ leaning to another, drawing leanings on the map.

## The shape: Easel and Canvas (as the piece suggests)

The design and the piece settle the mechanics: posting, keeping, gifts, leaning, joining, the
actions and the events. They're groundwork. What they can't settle is the part the player
meets: whether clans join at a pace that matters, whether giving more draws more clans than it
costs, and whether a civ can give without starving its camp. That's judged by the sweep and by
playing, after everything exists, so it all goes in one canvas step at the end.

| Step | Part | What | Model |
|---|---|---|---|
| build-1 | easel | Engine: keep, posting, gifts, leaning, joining, actions, `Config.gifts`, the event, counters, tests; the sweep's columns, `--no-gifts` and `--lever`; `lead_orders` and `cross-build.mjs` | sonnet medium (off the ladder) |
| build-2 | easel | The page: the civ panel's leaning clans and the keep and gift controls, the clan's leanings, the feed; `check.mjs`; a play script and screenshots; `engine/CLAUDE.md` | haiku high |
| build-3 | canvas | Tune the leaning, the gifts, the keep and the store so that joining is a lever worth pulling, or show that it isn't; `relay/specs/55/sweep.md` | opus low |
| review | | The piece against the design, the sweep's answer, playing the page | sonnet high (by coin) |

Each step leaves `engine/check.sh` green.

## What's settled

### The civ's new state (build-1)

`Civ` gets these fields, each `#[serde(default)]` so old saves load:

- `keep: Option<f64>` is the food the player has set to keep. `None` means the default.
- `gift: f64` is the most food the civ gives a turn. It starts at `LEANING.gift_default` for
  every civ when founded, the player's civ included.
- `posted: f64` and `given: f64` are what it posted and gave last turn, for the panel.
- `joined: u32` and `gifted: f64` count clans joined and food given since founding, for the
  sweep.

`Clan` gets `leaning: BTreeMap<u32, f64>` (civ id to leaning, `#[serde(default)]`). It's
non-empty only for unaligned clans.

The new numbers go in one `LEANING` struct in `data.rs`: `trade`, `gift`, `gift_scale`,
`starving`, `fade`, `join`, `gift_default` and `floor` (below), each with a doc comment.
`Config` gets `gifts: bool` (`#[serde(default = "yes")]`, on by default). With it off, no civ
gives, whatever its `gift` is.

### Keeping (build-1)

`civs::keep(w, civ) -> f64`: the civ's `keep` if the player has set one. Otherwise it's what
its camp needs: `units::pack_cap(warriors)`, enough to fill an empty pack. With no camp, it's
`pack_cap(UNIT.start)`, so an AI civ can still raise one. With `Config.army` off, it's 0.

### Posting and gifts (build-1), in `markets::trade` at the civ's own market

At each market, after `civs::levy` and before clearing, if a civ's capital is on this market
(`civ.market == mid`, in `Phase::Main`):

1. `spare = max(0, store total − keep)`.
2. **Gifts.** `pool = min(gift, spare)`, or 0 with `Config.gifts` off. It goes to the clans at
   this market this turn with `civ == None`, split in proportion to `v × people`, where
   `v = LEANING.starving` for a clan whose `starving` is set (last turn's eat) and 1 otherwise.
   It comes out of the store in proportion to its goods and goes into each clan's `basket`, so
   the clan eats it this turn. A clan given `g` leans `v × LEANING.gift × ln(1 + g /
   (LEANING.gift_scale × people))` more towards the civ (`libm::log`). With no unaligned clan
   there, nothing is given. `given` and `gifted` record it.
3. **Posting.** What's left of `spare` comes out of the store in proportion to its goods, and
   the civ brings it to the clearing as one more basket after the clans', with no carry cap,
   since the store is there. Its basket after the clearing goes back into the store. `posted`
   records what it brought. The cap applies in `upkeep` as now.
4. **Trade leaning.** If the civ posted anything this turn and the market cleared, each
   unaligned clan there leans `LEANING.trade × max(0, nutrition(after, p) −
   nutrition(brought, p)) / p` more towards the civ, where `p` is its people and `brought` and
   `after` are its basket into and out of the clearing.

Every other market clears as now. The clearing still conserves every good, the civ's included.

### Leaning and joining (build-1): a `lean` step after `people`

- Each unaligned clan's leanings fade: each is multiplied by `1 − LEANING.fade`, and any under
  `LEANING.floor` is dropped.
- Then, in clan id order, a clan whose strongest leaning (ties go to the lower civ id) is at
  least `LEANING.join` joins that civ: `civ = Some(id)`, its leanings are cleared, the civ's
  `joined` goes up by one, and an event `ClanJoined` is recorded (clan, civ, at its home).
  Its wording is "Clan {clan} joins {civ name}".
- Before `Phase::Main` the step does nothing.
- A daughter (from `divide` or `split`) takes a copy of her mother's leanings. A clan that
  merges into another brings no leaning: the host keeps its own.
- A clan that has joined a civ never leans again.

### The player's lever (build-1)

Two new `Action`s, checked in `validate` and applied in `act`, for the civ the player leads, in
`Phase::Main`:

- `SetKeep { food: Option<f64> }` sets `keep`. `None` goes back to the default. A number must
  be finite and at least 0.
- `SetGift { food: f64 }` sets `gift`. It must be finite, at least 0, and at most
  `CIV.store_cap`.

The AI doesn't change its civs' keep or gift: they stay on the defaults, so the AI posts and
gives by the same rules. `lead_orders` (CLI) and `leadOrders` (`cross-build.mjs`) get
`SetGift { food: 2× the default }` at `n == 10` and `SetKeep { food: Some(5) }` at `n == 50`,
then `SetKeep { food: None }` at `n == 70`, so the native and wasm builds compare through them.

### The sweep (build-1)

`sansfiat sweep --after N [--maps retro | seeds…] [--no-army] [--no-gifts] [--lever G]`:

- Each civ's line gains `joined {joined} gifted {gifted:.1} leaning {k}`, where k is the
  unaligned clans with a leaning to it of at least half `LEANING.join` at the end.
- The totals add "clans joined: {total} ({per civ, min/median/max})", "unaligned clans at the
  end: {n} of {clans}", and the civs' people as now.
- `--no-gifts` runs with `Config.gifts` off: the control.
- `--lever G` leads the lowest civ id at the pick and acts `SetGift { food: G }`. Each turn
  the sweep sends the led camp to where the AI would post it (make `units::post` public and act
  `MoveCamp` when the camp is idle), and raises it again (`Raise`) if it's gone and can be. The
  led civ then differs from the AI only in its gift. Its line is marked `lead`, and the totals
  add "led civ joined {a}, its rivals {b} on average".

The output keeps today's lines as they are, so #29's numbers can still be compared.

### The page (build-2)

- **The view** (`web/src/lib.rs`): each civ row carries `keep` (the value in force), `keep_set`
  (null for the default), `gift`, `posted`, `given`, `joined` and `gifted`. Clans already carry
  `leaning`. `rules()` adds `join` (`LEANING.join`) and `gift_max` (`CIV.store_cap`).
- **The civ panel** (`select.js` `capitalPanel`) shows a store line with what's kept, what was
  posted and what was given last turn, and the clans joined since founding. "Leaning clans"
  lists the unaligned clans with a leaning to this civ, strongest first, up to 10: the clan,
  its people, a starving mark, its leaning as a share of `join`, and the civ it leans to more,
  if there is one ("Blue leads"). Clicking a row selects that clan's home tile. For the
  player's civ, there are controls for the gift (− and +, in steps of 1 food, 0 to `gift_max`)
  and the keep (− and + in steps of 1, and "Default", which sends `SetKeep { food: null }`).
- **The clan's tile panel** lists its leanings by civ, as shares of `join`.
- **The feed** shows `ClanJoined`. Its filter marks it mine when the civ is the player's.
- **`check.mjs`** covers the new view fields and rules, `leaning` on clans, and that
  `SetKeep`/`SetGift` go through for the leader and are refused for a watcher.
- **A play script** (`www/play-lean.mjs`) leads a civ, raises its gift, ends 30 turns, and
  screenshots the civ panel and a leaning clan's panel into `relay/specs/55/screens/`.
- **`engine/CLAUDE.md`**: the layout row for civs (keep, posting, gifts) and a new
  `core/src/leaning.rs` row if build-1 made one, the actions, and the sweep's flags.

## What's left unsettled, and where

| Unsettled | Proof | Step |
|---|---|---|
| The numbers: every `LEANING.*`; `CIV.store_cap`, `levy` and `endowment` if posting and gifts need a bigger store; the default keep, if the camp's full pack is wrong | `relay/specs/55/sweep.md`, below | build-3 |
| Whether a rule's form must change (how a gift is split, whether gifts should draw clans to the festival, what trade leaning reads, the AI's gift) for joining to be a lever | the same sweep and play, each change a decision | build-3 |
| How the panel reads: whether a player can see a clan coming and what giving does | by eye, through `play-lean.mjs` | build-3 |

**What proves the painting** (on the retro's seven maps, `--after 200`):

- **Clans join.** All on the AI, most civs draw at least one clan, and joining matters: the
  civs' people grow beyond the gifts-off control. Not every clan joins either: unaligned clans
  remain within reach of each civ at 200 turns, so the prize lasts.
- **Gifts matter.** The default run draws clearly more clans than `--no-gifts`.
- **The lever pays, at a cost.** With `--lever` at a gift above the default, the led civ
  draws more clans than its AI rivals on most maps. Giving costs something the player can see:
  a store held low, and less food on hand for the camp. Giving everything starves the camp,
  or keeping more draws fewer clans. If no setting makes the lever pay, that's the answer,
  with the evidence.
- **The camps hold.** All on the AI, every civ still keeps its first camp (#29's 21 of 21), or
  any loss is explained.
- **Speed.** ms a turn in the main game within 1.5× of today's.

## Decisions the piece left open

1. **Only unaligned clans lean and join.** Civ clans, rivals' included, don't lean, since
   clans leaving a civ is out of scope. A rival's clans can still trade at your market.
2. **Trade leaning comes only from the civ's own market, and only on turns the civ posted.**
   It's measured by the food a person the clan gained in the clearing (`nutrition`, as
   `markets::gain` measures it). The clearing is pooled, so what came from the civ can't be
   told apart. Where the civ posts, it's most of what's there to gain.
3. **Gifts come out of what the civ would post** (the store beyond keep), before posting. So
   giving and posting share one surplus, and keeping guards the camp against both.
4. **The gift is one amount a turn for the whole civ, not a per-clan order.** The AI can use
   the same lever by the same rule, and the player pulls it once, not every turn. It's split
   over the unaligned clans at the market in proportion to people, weighted towards the
   starving. The decided rule was that a gift buys less the more is given and a starving clan
   values it more. That's a log in food per person (diminishing per turn and per clan), times
   `LEANING.starving` for a starving clan. Choosing which clan gets a gift is an idea for a
   later piece.
5. **Gifts don't draw clans to the festival.** `markets::worths` is unchanged: a clan goes by
   what it expects to gain in trade, and the civ's goods enter that through the market's prices
   and goods. The canvas may add a draw if the sweep shows the lever can't reach anyone.
6. **The default keep is a full pack for the camp.** That's the most the camp's line would
   draw at once. With no camp it's the pack of a new one, so an AI civ can still raise one.
7. **The player's civ starts on the AI's defaults.** So watching (all on the AI) and leading
   without touching the lever play the same rules.
8. **A daughter takes her mother's leanings.** The people lean, and a split shouldn't wipe
   that. A host keeps its own leanings when a clan merges into it.
9. **Leaning fades by a share each turn**, as merge4's did. Steady giving settles at a
   leaning of what a turn buys over the fade, so `join` is reached only by keeping it up.
10. **`Config.gifts`** holds the control, as `Config.army` does. A save replays it, and old
    saves load with gifts on.
11. **Design says (founding.md, Out of scope) that clans joining after founding, posting to
    markets, and anything the AI civs do beyond keeping their units are out.** This piece
    brings exactly those in, as its Decided lines say. Founding's other exclusions still hold.
12. **One new event, `ClanJoined`.** Gifts and posting show on the panel, not the feed: they
    happen every turn.

## Tests the piece must have (build-1)

- **Keep:** the default is the camp's full pack, a new camp's pack with none, and 0 with the
  army off. A set keep overrides it.
- **Posting:** a civ with a store beyond keep brings it to its market. The clearing conserves
  every good with the civ in it. The store gets the civ's basket back. Nothing is posted below
  keep.
- **Gifts:** the pool is at most `gift` and at most the store beyond keep. It's split by
  `v × people`. The clans' baskets get it. With gifts off, nothing is given. A starving clan
  given the same food leans more. Twice the gift buys less than twice the leaning.
- **Leaning:** trade leaning only at the civ's market on a turn it posted; it fades; it drops
  under `floor`; joining at `join` goes to the strongest (ties to the lower id); `ClanJoined`;
  a joined clan's leanings are cleared; a daughter copies them; civ clans never lean.
- **Actions:** `SetKeep`/`SetGift` refused for a watcher, before the pick, and for bad numbers;
  applied for the leader. A save with them replays to the same world.
- **The whole:** a small seed runs 100 turns past the pick with clans joining; native and wasm
  digests agree through `lead`.

Tests on real maps assert what holds, not counts, since build-3 retunes every number. A test
can force a join with hand-set leanings, or build a market by hand on `common::flat()`.

## What the later steps need to know

- `cargo test` runs at `opt-level = 3`. `core/tests/common/mod.rs` builds flat worlds.
- Float maths: only `+ - * /`, `sqrt`, `floor`, `round`, `min`, `max`, `abs`; `ln` through
  `libm::log`. No `HashMap`: use `BTreeMap` and deterministic orders. All randomness goes
  through `rng::rand` on `World.rng`, and this piece draws none.
- `cargo build --release -p sansfiat && ./target/release/sansfiat sweep --after 200 --maps retro`
  takes a few seconds (the seven maps run in parallel).
- `World.events` isn't saved. Anything a rule reads next turn (leanings, keep, gift) is.
- Piece #28 (the scout) may land on `main` before this one. The review merges `main` in.
