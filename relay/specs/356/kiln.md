# #356 build-2 (kiln): trade in the sweep, beside the exchange-off control

`sansfiat sweep --judge --json` at the kiln commit, in `sweep-kiln.json` (this folder; 201 s, no panics).
Each map also runs `Config { exchange: false, ..base }`, which is main's world: `digests-off.sh` gives all
21 digests equal to `relay/specs/340/digests-main.txt`. The control rows are that run's trade group.

## The first number (army-turns fed at a market that isn't the civ's)

Counted from the pick to +200 with every civ on the AI. A turn counts when an army bought more than 1e-6
food at a market that is neither its civ's capital market nor a settlement's. "Need" is what those armies
ate that turn (their people × `UNIT.food`); the eating itself is not recorded per turn.

| map | fills | foreign: army-turns | food bought / need | own: army-turns | unit-turns trading | salt, count / worth at +200 | obsidian, count / worth at +200 |
|---|---|---|---|---|---|---|---|
| small 2 | 28 | 1 | 0.00 / 4.0 | 1 | 2 | 3.6% / 1.0% | 0.3% / 0.2% |
| small 3 | 183 | 0 | 0.00 / 0.0 | 1 | 1 | 6.2% / 4.6% | 0.0% / 0.0% |
| small 6 | 24 | 5 | 0.55 / 20.0 | 0 | 5 | 5.6% / 2.3% | 0.4% / 0.2% |
| small 9 | 51 | 4 | 0.97 / 16.0 | 2 | 6 | 4.7% / 3.6% | 0.0% / 0.0% |
| medium 8 | 42 | 5 | 0.30 / 20.0 | 0 | 5 | 4.8% / 1.8% | 0.0% / 0.0% |
| medium 9 | 31 | 5 | 0.33 / 20.0 | 0 | 5 | 1.7% / 0.4% | 0.0% / 0.0% |
| large 8 | 19 | 0 | 0.00 / 0.0 | 0 | 0 | 4.4% / 1.5% | 0.0% / 0.0% |
| **total** | **378** | **20** (control 0) | **2.15 / 80.0** | **4** (control 0) | **24** | | |

- The count is small, as the spec expected: 20 army-turns in seven watches, buying 2.15 food where the
  armies needed 80 (about 3%). A full pack buys a fraction of a turn at the probe's salt prices.
- Fills are common (378); what is rare is an army with salt whose line can't feed it and whose pack is
  under `PLAN.trade_at` turns of food. Most armies that carry salt are fed by their line when they reach a
  market, so they don't trade. That is the gate the glaze should look at first.

## The trade group, at +200 and over the watch

Trades: 337,881 over the seven watches (1,726 at +200). Food is in nearly every trade, so the largest
count share is a food's by construction: fish 20.9% (medium 9) by count and grain 39.8% (small 2) by worth.
The wares are the point: salt is in 1.7–6.2% of trades by count at +200, obsidian in 0–0.4%, so obsidian
almost never changes hands at a market on these maps. The per-good shares are in `sweep-kiln.json`
(`maps[].trade_vs_control.trade.goods`), and the watch's shares too.

## Health against main (`judge --compare sweep-main.json sweep-kiln.json`)

The compare was run without `--floor`, so the tool flags nothing; the floors below are build-1's
(`pass.md`) and are read by hand.

| metric | main | kiln | change | read |
|---|---|---|---|---|
| starving share (total) | 0.0328 | 0.0340 | +0.0013 | within the 0.0014 floor |
| starving clan-turns (`towns` run, all) | 6,194 | 7,428 | +1,234 | **past the 165 floor** |
| worst map's starving ratio | 1.397 | 1.040 | −0.357 | better |
| people over control (goods) | 1.0127 | 1.0350 | +0.022 | within the 0.026 floor |
| armies people over control | 0.9991 | 1.0044 | +0.005 | within the 0.032 floor |
| settle people over control | 0.9892 | 1.0146 | +0.025 | not checked against a floor |

The one line out of band is the starving clan-turns. The likely cause is the civ store's reserve (salt kept
back from keeping, `KEEPING.packs`): the store's food spoils faster when its salt is spared from keeping it.
This is a guess from the mechanism, not measured. The glaze can test it by setting `KEEPING.packs` to 0 on
the same sweep (the Fill and the planner stay on), and the review should weigh it.

The sweep's own controls move too: `control_all` starving goes 5,509 → 8,105 with exchange on, so a change
in the second routes and the reserve moves every run's trajectory, as build-1 saw.
