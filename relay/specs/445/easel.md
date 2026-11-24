# #445 build-2: the places' views on the branch

Shot by `node www/shots.mjs --places DIR` on `relay/445` (the branch's `shots.mjs` and `shots-views.mjs`): all ten place views,
each with its probe per place and its digest and role counts. Every place passed: each capital and settlement nearest its own civ,
each place's role its tile's, each town with no tint. The run exits 0. The digests are the spec's: small 2 +40 `4ee21ce6517e1cc5`,
medium 9 +40 `c38cfa6eb0163d07`, large 8 +40 `37104de33cd8e31b`, the place lineup after one turn `514f01f255b6d28f`.

| View | Digest | Places on screen (roles) |
| --- | --- | --- |
| `medium9-places-play` | `c38cfa6eb0163d07` | 4 (capital 1, settlement 2, town 1) |
| `medium9-places-close` | `c38cfa6eb0163d07` | 4 (capital 1, settlement 2, town 1) |
| `medium9-places-edge` | `c38cfa6eb0163d07` | 8 (mark 6) |
| `medium9-places-flat` | `c38cfa6eb0163d07` | 8 (mark 6) |
| `small2-places-close` | `4ee21ce6517e1cc5` | 3 (capital 1, settlement 1, town 1) |
| `large8-places-close` | `37104de33cd8e31b` | 2 (capital 1, town 1) |
| `large8-places-play` | `37104de33cd8e31b` | 4 (capital 1, town 3) |
| `places-lineup-close` | `514f01f255b6d28f` | 5 (capital 3, settlement 1, town 1) |
| `places-lineup-play` | `514f01f255b6d28f` | 5 (capital 3, settlement 1, town 1) |
| `places-lineup-edge` | `514f01f255b6d28f` | 5 (mark 4) |

## The places, per view

Tile, role, civ (`none` for a town), whether the tile reads as a town, its clans, `tinted` pixels and the probe's `nearest`.

**`medium9-places-play`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 566 | capital | 592 | false | 255 | 41 | 592 |
| 602 | settlement | 592 | false | 671 | 37 | 592 |
| 919 | settlement | 592 | true | 689,690,722 | 36 | 592 |
| 735 | town | none | true | 208,223,609,621,633,679,683,720 | 0 | - |

**`medium9-places-close`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 566 | capital | 592 | false | 255 | 78 | 592 |
| 602 | settlement | 592 | false | 671 | 91 | 592 |
| 919 | settlement | 592 | true | 689,690,722 | 91 | 592 |
| 735 | town | none | true | 208,223,609,621,633,679,683,720 | 0 | - |

**`medium9-places-edge`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 1510 | mark | 590 | false | 63 | 0 | - |
| 40 | mark | 591 | false | 227,603 | 0 | - |
| 566 | mark | 592 | false | 255 | 0 | - |
| 602 | mark | 592 | false | 671 | 0 | - |
| 919 | mark | 592 | true | 689,690,722 | 0 | - |
| 46 | mark | 591 | false | 746 | 0 | - |

**`medium9-places-flat`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 1510 | mark | 590 | false | 63 | 0 | - |
| 40 | mark | 591 | false | 227,603 | 0 | - |
| 566 | mark | 592 | false | 255 | 0 | - |
| 602 | mark | 592 | false | 671 | 0 | - |
| 919 | mark | 592 | true | 689,690,722 | 0 | - |
| 46 | mark | 591 | false | 746 | 0 | - |

**`small2-places-close`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 506 | capital | 454 | false | 48 | 80 | 454 |
| 288 | settlement | 454 | false | - | 81 | 454 |
| 505 | town | none | true | 69,162,330,472 | 0 | - |

**`large8-places-close`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 2689 | capital | 1600 | false | 65 | 81 | 1600 |
| 2787 | town | none | true | 107,401,1218,1632,1685,1777,1864,1911 | 0 | - |

**`large8-places-play`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 2689 | capital | 1600 | false | 65 | 37 | 1600 |
| 2787 | town | none | true | 107,401,1218,1632,1685,1777,1864,1911 | 0 | - |
| 2871 | town | none | true | 597,1095,1250 | 0 | - |
| 2524 | town | none | true | 243,264,350,414,754,1742,1747,1775,1965 | 0 | - |

**`places-lineup-close`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 47 | capital | 10 | true | 7,16,17 | 81 | 10 |
| 102 | capital | 11 | false | 8 | 77 | 11 |
| 97 | capital | 12 | false | 9 | 75 | 12 |
| 140 | settlement | 11 | true | 19,20,21 | 83 | 11 |
| 79 | town | none | true | 22,23,24,25 | 0 | - |

**`places-lineup-play`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 47 | capital | 10 | true | 7,16,17 | 37 | 10 |
| 102 | capital | 11 | false | 8 | 38 | 11 |
| 97 | capital | 12 | false | 9 | 39 | 12 |
| 140 | settlement | 11 | true | 19,20,21 | 32 | 11 |
| 79 | town | none | true | 22,23,24,25 | 0 | - |

**`places-lineup-edge`**

| tile | role | civ | reads as a town | clans | tinted | nearest |
| --- | --- | --- | --- | --- | --- | --- |
| 47 | mark | 10 | true | 7,16,17 | 0 | - |
| 102 | mark | 11 | false | 8 | 0 | - |
| 97 | mark | 12 | false | 9 | 0 | - |
| 140 | mark | 11 | true | 19,20,21 | 0 | - |

## Notes

- **`large8-places-play` has no capital at 2524.** Its civ's capital has fallen (`civ.fallen` is set), so by the rule in `spec.md` (a capital place only for a civ that hasn't fallen) the tile draws its town's hamlet: 9 clans, a town. `proto/scenes.txt` listed the tile as a capital because it reads `civ.capital` without the fallen filter. The view still proves the 9-clan town; a living capital that reads as a town is tile 47 in `places-lineup-close`. Build-3 should read that view knowing this.
- The `between` views print tile `-1`: their camera is the mean of two tiles.
- `medium9-places-flat` (phone, k 5) and `places-lineup-edge` (phone, k 6) are at the flat zoom: every entry is a mark, with no cloth to probe.
- `medium9-places-edge` (phone, k 6) is also at the flat zoom: all eight places on screen are marks, as the spec's "flat zoom" rule has it at `PLACE.from` 28 css px.
- Not run here: `--time`, `--diff` against main and the captures under `relay/specs/445/shots/`; those are build-3's and the review's (`spec.md`, "For the review").
