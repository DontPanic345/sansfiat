# #342 Herds: spec

**Question:** Standing. Do civs on grass gain against the no-herds control, as farming civs gained
against no fields? On main (`main.md`, this session) 10 of 21 civs farm and gain +0.049 of Standing
at +200 against no fields; the 11 that don't farm gain +0.012. Herds are for those eleven.

## What the piece delivers

After the pick, a clan may keep animals. Its days split three ways: gathering over its territory,
fields on its river ground, and herding over its grass. A herd is the animals in the clan's store.
They breed by the logistic rule game breeds by, toward what the clan's herding days and its grass
together carry, and the clan eats the increase. Animals are eaten, and the clan brings what it
doesn't eat to its festival like any food. A herder holds wider land than a gatherer, and its herd
goes wherever the clan goes. A farmer doesn't herd.

1. **Animals, a good** (`data/goods.rs`). A new row, `animals`, after the five foods (the eaten goods
   come first: `FOOD_N` becomes 6, `SALT` 6, `OBSIDIAN` 7, `N` 8), with a new use, `Use::Herd`:
   eaten, weight `0.5` (they walk, but need driving), spoil `0.0` (alive in any store). Every tile
   basket and terrain array grows a slot that stays 0: no tile has a wild herd. Every `match` on
   `Use` treats `Herd` as `Food` except where this spec says. `SAVE_VERSION` bumps, and the pins.
2. **The rules** (`data/herds.rs`, new): `HERD`, with a `note`, holding:
   - `graze: [f64; 8]`, animals a tile carries by terrain before fertility: sea 0, coast 1,
     plains 6, forest 1, hills 4, mountain 0, desert 1, marsh 2. Grass is plains and hills.
   - `tend` 3.0: animals one herding day keeps a turn.
   - `breed` 0.3: the herd's logistic rate, as `FOOD[GAME].regen` is game's.
   - `start` 0.5: animals a person a clan takes up herding with, caught from the game on its tiles.
   - `lean` 0.25: the most of its herd a clan short of food eats a turn beyond the increase.
   - `notch` 0.1, `margin` 0.01: as `FIELD`'s.
   - `herder` 0.2: the share from which a clan is a herder.
   - `radius` 3: a herder's territory radius (a gatherer's is `CLAN.territory_radius`, 2).
   - `walk` 0.5: the weight of a herder's walks inside its territory, against `CLAN.inside`: the
     camp moves with the herd, so it walks out and back less.
   These start numbers run; the glaze tunes them.
3. **`Config.herds`**: on by default and in old saves; off is the control, in which no clan herds.
   Herds open at the pick, as fields do (`herds::open`: `config.herds` and civs founded), so the
   pre-game is byte for byte and the control differs only from the pick.
4. **The herding job** (`core/src/jobs.rs`, new). `fields::jobs` moves here as `jobs::jobs` and
   gains a fourth job: herding, at `Clan.herds` (a new saved share, 0 to 1, `#[serde(default)]`)
   over the territory's tiles with `graze > 0`, paying each tile's walk as the other jobs do (at
   `HERD.walk × CLAN.inside` for a herder). The source job's share comes out of all three as it
   comes out of two now. `fields.rs` keeps ground, yield, sow, room, the notch and the farmer.
5. **What a herd carries and how it breeds** (`core/src/herds.rs`, new). On each herding tile the
   herd's room is `min(graze × fertility, days there × HERD.tend)`; `carries` is their sum, `K`.
   Each harvest a herd `H` grows by `HERD.breed × H × (1 − H / K)`, and above `K` it falls toward
   `K` by the same rule, never below it. With `K` 0 (no herding days) the herd falls to 0 by the
   same rule. The clan takes the increase (`herds::offtake`): to start, all of it, which the
   probe's best setting does (`probe.md`, p4); the glaze may change it to only what the herd grows
   past `K ÷ 2` (p6), which grows real herds but doubles starving. What it takes is the harvest's
   animals: it counts in `Reaped::food`, against the surplus target with gathering and field grain
   by the one factor, and what the target leaves is not slaughtered: it stays in the herd. The herd
   itself does not count against the target: the budget is `surplus_target × people` less the
   store's food *without* its animals.
6. **The herd in the store, through a turn.** At the harvest the clan's breeding stock stays in `store[ANIMALS]`; the basket gets the increase taken, not the herd, so `trade`
   and the levy never see the breeding stock. At `eat`, the clan eats its basket as now; if it is
   still short of `CLAN.stable_at` of its need it eats from its herd, at most `HERD.lean` of it.
   Animals left uneaten in the basket go back to the herd. Nothing spoils a herd (spoil 0).
7. **Starting a herd.** When a clan's herding share rises from 0 and it holds no animals, it starts
   a herd of `HERD.start × people`. The game on its own tiles pays for it as far as it goes (taken in
   proportion to their stock); the rest it brings in from the wild beyond its land. The probe found
   the game on a clan's tiles hunted out by the pick (0.1 to 7 game on a clan of four to seven), so
   a herd caught from game alone never starts (p1–p3: 13 herder clans on seven maps).
8. **The notch reckons the herd it would keep, not this turn's increase.** `value_at` at a herding
   share counts the herd's **steady yield**, `HERD.breed × K ÷ 4` (what a herd at half its land
   yields a turn), in place of this turn's increase, as `worth` counts a source's dig whether or
   not the clan keeps it. Weighed on this turn's increase, a herd of four on new land yields too
   little to beat the days it takes, and no civ herds (p1–p3); on its steady yield, 9 civs herd
   and 8 of them gain (p4). The harvest itself uses the herd's real increase.
9. **The notch** (`herds::notch`, in the `move` step beside `fields::notch`). A clan weighs its
   herding share ± `HERD.notch` by `value_at` and moves toward the better if it beats the current by
   `HERD.margin`, as the field notch does. **Fields or herds, not both:** a clan with fields above 0
   does not raise herds, and one with herds above 0 does not raise fields; with both at 0 each notch
   may raise its own, the field notch first. A starving clan with herds drops a notch. A share rising
   from 0 emits `ClanHerding` ("Clan 140 takes up herding at (3, −2)"); falling to 0, a note.
10. **Wide land.** A herder (`herds::herder`, share ≥ `HERD.herder`) holds land to `HERD.radius`:
   every site that reads `CLAN.territory_radius` for a clan's territory (`dividing.rs`, `homes.rs`,
   `clans::adjust`) reads `herds::radius(c)`. Its home follows the middle of its territory as every
   clan's does; the farmer's home rule doesn't apply to it.
11. **The herd goes with the clan.** It is the store, so a daughter takes her share of it, a clan
    that moves on takes it, a merged clan brings it, and a clan that dies loses it. A daughter
    starts at her mother's herding share; a clan that moves on keeps its share (its herd walks with
    it), unlike fields, which it leaves.
12. **The judge** (`core/src/judge_herds.rs`, `cli/src/sweep_herds.rs`, new): the `herds` group, and
    the no-herds control as one more run a map in `sweep --judge` (measured below).
13. **The page and the text view**: a herds layer, the clan panel's Herd row and doing line, the civ's
    Living row with herding, the tile tooltip's grazing, the feed's `ClanHerding`, the text view's
    herd line, and `www/play/herds.mjs`.

## Shape: Clay, Kiln and Glaze

As the issue says. The hard part is the clay: a store that breeds inside the clans' turn, through
harvest, trade, eat, divide and move, where every food sum now has a good it must leave out, and a
third job in a split that two pieces built. Once the herd lives on the actor, the judge and the page
are settled. The numbers — grazing, tending, breeding, the radius and the walk — only work together,
and only the retro maps say whether herders gain Standing, hold wide land and starve no more.

The spec step sketched the clay on main and ran nine settings on the retro maps (`probe.md`,
`probe.diff`). Two rules came out of it, now settled above: the herd's start doesn't wait on game,
and the notch reckons the herd's steady yield. At those rules and the start numbers (p4) 9 civs
herd and 8 gain, +0.033 against herds off, with starving at 1.15 times. That is the clay's shape,
held. What's left is what the glaze is for: the herds are a trickle (341 animals over seven maps),
no setting gives herders wide land, and the rule that grows real herds doubles starving.

- **build-1 (clay):** the good, `HERD`, `Config.herds`, `jobs.rs`, `herds.rs`, the herd through
  the turn, the notch, the radius, `ClanHerding`, tests and a stage test, with the start numbers.
- **build-2 (kiln):** the judge's herds group, the no-herds control, the feed's kind and sound.
- **build-3 (kiln):** the page and the text view.
- **build-4 (glaze):** the numbers, tuned on the retro maps against the no-herds control until the
  bounds hold. It may change the clay's and the kilns' rules where a bound needs it, and records each.

## The question and how it is measured

**The control.** The no-herds control is the same map with `Config.herds` off. Herds open at the
pick, so the control differs from the run only from the pick; the floor's nulls play their own
no-herds control. Read with `sweep --judge --json --floor`: one civ's gain swings by up to half a
point on main (`main.md`), so a move counts past the floor.

A civ **herds** when its clans put at least a tenth of their people's days to herding at +200
(herding share: Σ people × herds ÷ Σ people). A **herder** is a clan with share ≥ `HERD.herder`.
A civ's **grass share** is the share of its clans' territory tiles at the pick with `graze` of
plains or hills and no field ground: the land herds are for.

| Score | Measure | Control | Bound |
|---|---|---|---|
| Standing | **Herding civs gain**: per civ, Standing at +200 less the no-herds control's. | No-herds control, per civ | Herding civs gain at least +0.01 on average past the floor, three quarters of them gain, and their mean beats the others' by at least 0.01. At least 4 civs herd. |
| Standing | **Civs on grass**: the third of civs with the highest grass share, their mean gain. Reported beside the herding civs': the goal's words, read on the land. | No-herds control | Reported |
| Standing | **Fields still pay**: farming civs' gain against no-fields. | Main's +0.049 (`main.md`) | Not below +0.03 |
| Motion | **Wide, thin land**: territory tiles a person, herders against gatherers (share 0 of both jobs) and farmers, at +200 over the seven maps. | Gatherers and farmers in the same runs | Herders at least 1.5 times gatherers' |
| Motion | **Animals change hands**: festival-turns where animals traded, pick to +200, and the maps with any. | No-herds control (0) | On at least 4 of 7 maps |
| Motion | Clans, people and land changed hands. | No-herds control | No fewer, less the floor |
| Health | **Nobody starves who shouldn't**: starving clan-turns over clan-turns against the control's, read less its floor; herders' rate beside gatherers'. | No-herds control | At most 1.25 overall, no map over 1.6 |
| Health | Pre-games 7 of 7, panics 0, digest mismatches 0. With herds off, `sweep --judge --json` equals main's in every map's scores. | Main | Equal |
| Health | `sansfiat bench --size large 8 --after 50`. | Main, same session | At most 1.25 times |
| Doubt | Lead changes and the spread. | No-herds control | Reported |

The judge reports at +100 and +200: herders, herd animals, herding people, herded tiles, the herding
share per civ, the store turns at a pack (as fields), animals' festival-turns, tiles a person by kind.

## What I settled

- **Herds open at the pick**, as fields: the pre-game is clans.md's proof and stays byte for byte.
- **Grazing is a table by terrain, not a food cap.** Grass isn't a food anyone eats, so tiles carry
  no wild herd and nothing regrows: `HERD.graze` by terrain, times fertility.
- **Grazing takes no stock and wears nothing**: game and grain on a grazed tile stay to gather. If
  herds pay too well everywhere there is grass, the glaze may make a herd take game (a grazed tile's
  game cap falls by the herd's share of its room); that is its first lever, recorded if pulled.
- **The herd is in `store[ANIMALS]`**, as the piece decided; the breeding stock stays out of the
  basket so it is neither traded nor levied, and only the increase and a short clan's lean are eaten.
- **A herd starts at `HERD.start` a person**, the game on the clan's own tiles paying what it can
  (by running: game is hunted out by the pick, and a herd caught from it alone never starts, p1–p3).
- **The notch reckons the herd's steady yield**, `breed × K ÷ 4` (by running: on this turn's
  increase no civ herds, p1–p3; on the steady yield 9 do, p4).
- **Herds off equals main** in all 19,953 numbers of `sweep --judge --json` (by running the probe).
- **Fields or herds, not both** (the piece's "a farmer doesn't herd", made exact).
- **Moving with the herd**: a turn is a year, so the seasonal walk is inside the turn. A herder holds
  land to radius 3 and walks it at half a gatherer's weight; its home follows the middle as any clan's.
- **Animals weigh 0.5** at a festival (`MARKET.carry` counts weight): they walk, but need driving.
  Carrying anything else is the pack animal piece (#343).
- **Where the code goes**: `jobs.rs` (the split, out of `fields.rs`), `herds.rs`, `data/herds.rs`,
  `judge_herds.rs`, `sweep_herds.rs`; `clans.rs` is 653 lines and only calls into them.

## Unsettled, and which step holds it

All in build-4, the glaze: every `HERD` number, animals' weight and the offtake, tuned together
until every bound above holds, read from `sweep --judge --json --floor` against the no-herds
control. What is open, from the probe (`probe.md`):

- **Wide land**, which no setting met: herders hold 0.36 to 0.64 tiles a person against gatherers'
  0.70 to 0.94. A herding share of a tenth gives few days, so `K` is bound by the days and more
  grass adds nothing. The days against the grass (`tend`, `graze`) are the first look; the radius
  and the walk don't bind yet.
- **Herds that grow** without starving: eating every increase keeps herds at their start; eating
  only past `K ÷ 2` grows them and doubles starving; a fed-only notch kills herding.
- **The Standing bound past the floor**: p4's +0.033 over 9 civs is one draw.
- Whether herds pay so well that every plains gatherer herds (the game lever above).

If no setting meets the Standing bound, the glaze reports the nearest and sends an `ask`; if only
wide land fails, it reports that with the nearest setting and the Standing it gives.

## What the review runs

- `engine/check.sh`.
- `sansfiat sweep --judge --json --floor` on the branch and on main (a worktree), then
  `sansfiat judge --compare main.json branch.json`. The herds group's per-civ table and every bound
  above go in the PR: herding civs' Standing against the control, civs on grass, fields still paying,
  wide land, animals traded, starving.
- With herds off (`Config::default()`'s `herds` false, locally): `sweep --judge --json` equals
  main's in every map's scores.
- `cargo test --release -p engine --test herds -- --nocapture` and the herd's stage test.
- `sansfiat bench --size large 8 --after 50` on the branch and on main, five each, one session.
- `node www/play/herds.mjs`, with a look at its screenshots.
- A played game (`sansfiat play --game g.json`, the retro map whose civ herds most in the glaze's
  sweep, as that civ): 60 turns past the pick, reading the feed's "takes up herding", the clan's Herd
  row and the civ's Living line.

No new button or order: nobody orders a clan to herd.

## Decisions I made that the piece left open

1. **Herds open at the pick**, as fields do: the pre-game stays byte for byte and the no-herds control
   differs only from the pick.
2. **Design says "grows by what the land it grazes can carry".** The land's carrying is a grazing table
   by terrain times fertility (plains 6, hills 4, others 0 to 2), and a herd carries no more than its
   herding days tend (`HERD.tend` a day). Grass is not a food: no tile has a wild herd.
3. **A herd breeds by game's logistic rule** at `HERD.breed`, falls toward what it carries when over it,
   and the clan eats the increase. The breeding stock is never traded or levied.
4. **A clan short of food eats into its herd**, at most `HERD.lean` of it a turn: a herd is a store.
5. **A herd starts at `HERD.start` animals a person**, the game on the clan's tiles paying what it can,
   the rest brought in from beyond its land: by the pick the game is hunted out (`probe.md`).
6. **The notch weighs the herd it would keep**: a herding share is reckoned by the steady yield of
    the land its days tend, `breed × K ÷ 4`, not by this turn's increase (`probe.md`, p1–p4).
7. **Grazing takes no stock and wears nothing** to start; the glaze may make it take game.
8. **Design says "a farmer doesn't herd".** A clan works fields or herds, never both.
9. **Design says "a herding clan holds wide, thin land and moves across it".** A herder's radius is 3
   against a gatherer's 2, and its walks inside its land weigh half a gatherer's: the camp moves with
   the herd. No migration rule: a turn is a year.
10. **A clan that moves on keeps its herding share**, since its herd walks with it; it gives up fields.
11. **Animals spoil in no store and weigh 0.5** at a festival.
12. **Design says animals are "the first good with a use beyond eating: they carry".** In this piece
    they are eaten and traded only; carrying is #343.
13. **"Civs on grass" is read two ways**: civs that herd (the bound, as fields' "farming civs"), and the
    third of civs with the most grass at the pick (reported).

## Ideas taken

None. The open ideas are about signposts, gifts, scouts, the claim and the view; none is within herds.
