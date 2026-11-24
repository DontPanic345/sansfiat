# #356 salt: the owner's levers, measured

The owner ruled: more salt from the sources, the levy tuned, a civ holds the goods (a larger cap),
and clans use obsidian as gathering tools. Built:

- A pan gives three quarters of a unit a day (`SOURCE.salt_a_day`), three times obsidian's.
- The levy takes all the salt and obsidian a civ clan brings (`CIV.levy_goods` 1), not what it keeps
  at home, whatever room the store has for food.
- A civ's store holds four turns of salt for a full store (24) and obsidian for eighty warriors (40).
- Obsidian is tools: a tenth a person, a fifth more gathering, a tenth wears out a turn; a clan wants
  it as a food and keeps its tools from the festival (`TOOLS`).
- Pay goes to the nearest friendly market that trades a good the store holds, not the nearest.

## army_flow, 200 turns past the pick

| map | past reach | paid | line at a market | food bought | store salt | store obsidian |
|---|---|---|---|---|---|---|
| small 2 | 229 | 2 | 0 | 0.5 | 1.41 | 0.02 |
| small 3 | 295 | 2 | 0 | 0.4 | 6.55 | 0.02 |
| small 6 | 42 | 0 | 0 | 0 | 6.07 | 0.08 |
| small 9 | 186 | 3 | 2 | 5.5 | 0.39 | 10.27 |
| medium 8 | 14 | 0 | 0 | 0 | 1.83 | 14.12 |
| medium 9 | 351 | 0 | 0 | 0 | 10.19 | 0.00 |
| large 8 | 308 | 0 | 0 | 0 | 1.92 | 0.00 |

Before (the rework alone): stores held 0.00–0.08 salt; line at a market 0 on every map.

## What stops pay now (army-turns its line home can't feed, no stall stocked)

| reason | army-turns |
|---|---|
| its store holds no salt or obsidian | 746 |
| no line home | 127 |
| markets in reach, none trades the store's goods | 138 |
| they trade them, for less than their weight in food | 90 |
| no friendly market in reach | 43 |
| pay planned | 64, of which 57 marching with no labour left to carry it |

A store holds salt on average, but keeping a full store uses up to 6 salt a turn (`KEEPING.keeps`), so
the store at an army's line end is empty most turns it needs pay.

## The sweep (`sweep-salt.json` against `sweep-main.json`)

line at a market 2; claims taken 220 → 70; clans changed 410 → 610; armed armies 33 → 10 (clans keep
obsidian as tools); mean store 41.7 → 35.6 food; largest army 14 → 24; capitals fallen 3 → 4;
lead changes 7 → 12; no panics or digest mismatches.

## Checks

Engine tests pass; save version 31 with its digests. `game.mjs` fails: right after raising its three
warriors on seed 9 the store can't fill a scout's pack. `save.mjs` times out under `all.mjs`'s load and
passes alone. `story.rs` runs 61 s in debug here, as main's runs 75 s.
