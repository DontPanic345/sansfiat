# #445 build-3: the places on the map, judged beside main

Main is `55709fb` (the branch's base) in a worktree (`/tmp/b3/main`, the branch's `shots.mjs` and `shots-views.mjs` copied in,
`www/pkg` copied from the branch: no Rust changed). Both trees shot the ten place views (`--out`, `--only`). Captures are in
`shots/<view>-main.png` and `<view>-branch.png` (`proto/crops.mjs`: the same rectangle in both, 1× to 3×): `places-lineup-close`,
`places-lineup-play`, `medium9-places-play`, `medium9-places-close`, `small2-places-close`, `large8-places-close`,
`large8-places-play`. `git diff main --stat -- engine/core engine/cli engine/web` is empty. Raw outputs: `places.txt`
(`--places`), `diff.txt`, `time.txt`.

## What I changed, and why

Build-2's first captures showed one thing that did not read and one that did not tell the civ.

1. **The clans stand in the place's yard** (`PLACE.yard`, read by `clans.js`: `layout` asks `places.js`'s `yardTiles` which
   tiles draw a place). On build-2's ring (0.42 from the centre, anchors down to 0.82 below it) a town of 8 was a heap of
   tents with its hamlet hidden behind (`medium9-places-close`: only a thatch edge showed), and a lone clan on a capital or
   settlement stood over its foot. On a place's tile the clans now stand in rows in front of it, the back row first: the
   back row 0.66 below the tile's centre, each row 0.3 further, five to a row spread over 1.3, each at most 0.58 tall in a
   crowd (a lone clan 0.85, at 0.84, which is #444's size for a 4-person clan so `CLAN_BAR` is not loosened: the clans' bars
   hold on every clan view, `--clans` exits 0). Every clan still draws, in id order, with its starving edge and its click;
   none is hidden, and the hall, palisade or hamlet shows above. Off a place's tile, and at the flat zoom, the clans are as
   #444 drew them. `play/places.mjs` checks every clan on a place's tile has its foot below the tile's centre.
2. **`PLACE`:** `w` town 1.6 → 1.9, settlement 1.8 → 1.9, capital 2.0 → 2.4; `feet` 0.3 → 0.2. At 1.6 the hamlet was under the
   town diamond (1.1 wide, over the roof); at 1.9 its huts stand out each side. The hall is the largest place. `feet` 0.2
   lifts the places off the yard. `from` stays 28 (the clans' zoom, k 13.6): the places are main's marks at phone k 6 and k 5,
   so `medium9-places-edge` and `places-lineup-edge` are pixel for pixel main's, as `medium9-places-flat` is.
3. **The capital's pennant** (a variant, authored from the study's pixels: `atlas.md`, "Build-3"). The hall's two banners are
   4 px wide at k 35 and Gold's read as thatch; with a town diamond on top of the hall (Red's capital 47, a town of 3) they are
   gone. The study's flag, at 0.3 with its pole, stands on the roof's right shoulder, clear of the diamond, in the civ's cloth
   (a flag of 10 px at k 35, 5 px at k 18). The palisade's flag stays as cut.
4. **The bar** (`PLACE_BAR` in `shots.mjs`; `--places` exits 1 under it): **80 tinted pixels at k 35 and 30 at k 18**, per capital
   or settlement. The fewest in the views are 89 (a settlement's flag at k 35, medium 9) and 37 (a settlement's flag at k 18,
   the lineup; 39 on medium 9). A settlement's flag is 6 px across at k 18 and reads as its colour on plains and in the wood,
   Gold's as yellow not ochre (its grey floor stays at 190, #443's fix); below 30 the flag is under a 5 px square. A capital
   has 117 to 120 at k 18 and 274 to 279 at k 35. A town has none: the `--places` check is zero.
5. **`--places` prints each place's `share`** (tinted over opaque) and, per view, each owned role's least tinted pixels and
   share range; `--time` prints `stats.overlayMs` beside `terrainMs`, since the places draw in the overlay, not the terrain.

## Per view: main and the branch

| View | Main | Branch |
|---|---|---|
| `places-lineup-close` (k 35) | Red's capital 47 a red house under a diamond; Blue's capital 102 a blue house; Gold's a gold house; Blue's settlement 140 a small blue house (Red's army 4.0 beside it); the town of 4 pale circles under a diamond | the capitals are the civic hall on its mound among its houses, the civ's flag on its roof's shoulder (red, blue, gold) and the two banners on its front; Red's, a town of 3, shows three red tents in its yard under the hall and its flag clear of the diamond; Blue's settlement is the palisade with its blue flag beside the wood, Red's army figure and "4.0" over its right side, readable; the town of 4 is the hamlet's thatched huts above four pale tents; the ridge, wood, rings and borders are main's |
| `places-lineup-play` (k 18) | the same at 17 px | places 29 to 36 px wide: the hamlet two huts and a ridge of roof over tents, the palisade a ring and a blue speck of flag, the halls a roof and a 5 px flag; hall, palisade and hamlet are told apart by the ring and the flag, less than at k 35; the army's shield and "4.0" sit on the palisade, readable |
| `places-lineup-edge` (phone k 6) | main's houses | identical (0% differ) |
| `medium9-places-play` (k 18) | Gold's capital 566 a house, settlements 602 and 919 small houses, the town 735 a diamond over 8 circles | Gold's three places each a hall or a palisade with a gold flag, small; the town of 8 two huts above two rows of tents in the forest under the ridge; camps, the green land and the rivers are main's |
| `medium9-places-close` (k 35) | the same close | the town of 8 shows its huts, 8 tents in two rows (5 and 3), and the diamond over its roof; 602 (a lone clan) a palisade with its flag above a tent; 919 a palisade over three tents; Gold's flags read gold on the green and the ridge |
| `medium9-places-edge`, `-flat` (phone k 6, k 5) | main's marks | identical (0% differ) |
| `small2-places-close` (k 35) | Blue's capital 506 a house, the scout beside it, the town 505 a diamond over four circles, settlement 288 a house | the hall with its blue flag and the scout's figure and "2.0" in front, readable; the forest town 505 under a diamond, its huts above four tents; 288 a palisade with its blue flag; the signpost is main's |
| `large8-places-close` (k 35) | capital 2689 a house and its scout; the town 2787 a diamond over 8 circles | the hall on the marsh with its flag; the town of 8 two huts above two rows of tents (5 and 3), the starving red edges main's; hills as main |
| `large8-places-play` (k 18) | the fallen capital's tile 2524 a diamond over 9 circles, 2689 a house | 2524 a hamlet over 9 tents (a town, as build-2 says); 2689 a hall with its flag; the forest town 2871 under the mountains a hamlet over three tents |

## Judged

- **Role.** At k 35 the hamlet (thatch, three huts, no wall), the palisade (a ring, huts inside, a flag) and the hall (a mound,
  a hall, a flag, banners, the largest) are three different things, on plains and in the wood. At k 18 they are 29 to 43 px
  wide and the ring and the flag tell the palisade from the hamlet, the hall from both by its size and flag; weaker than k 35,
  and the town diamond (main's, over the roof) takes a part of the hamlet. `PLACE.from` stays 28: at phone k 6 main's marks
  stay, and `medium9-places-edge` and `places-lineup-edge` are pixel for pixel main's.
- **Ownership.** Red, Blue and Gold read as their colours in the flags of the capitals and the settlements on plains, marsh,
  hills and in the forest, at k 35 and at k 18; Gold's flag is yellow on the thatch, a darker yellow than the roof, not ochre
  thatch. The hall's two front banners add little (4 px); the pennant is what reads. On a capital the gold centre ring
  crosses the hall's flag at k 35 where the hall is larger than the ring's centre: the ring is main's and draws over.
- **The crowd.** The town of 8 at 735 and at 2787, the lineup's three crowded places, and 2524's nine each show the place
  and every clan: the clans are small (≤ 0.58 tall) in two rows, the back row part-hidden by the front's, each
  tent, hearth or walker countable at k 35; at k 18 a row of five is a row of tents. A starving clan's red edge still shows
  (`play/places.mjs`, `--clans`).
- **Beside a unit, a forest and a ridge.** In `places-lineup-close` the settlement's right side has Red's army figure and
  "4.0" over it: figure and number readable, the palisade's flag and ring visible above; the trees and the peaks draw as on
  main. In `small2-places-close` the scout's figure and "2.0" stand over the hall's yard, readable, and the forest town beside
  keeps its trees.
- **Markers.** The town diamond, the centre ring, signposts, borders and trails sit where main has them and draw over the
  places. The diamond covers the hall's roof peak on a capital that reads as a town and the hamlet's middle; its ring and
  white disc are untouched.

## `--diff` (`diff.txt`)

Cameras and digests match in every view (exit 0). `medium9-places-flat`, `medium9-places-edge` and `places-lineup-edge`: 0%.

| View | Share that differ |
|---|---|
| medium9-places-play | 0.62% |
| medium9-places-close | 1.34% |
| small2-places-close | 1.37% |
| large8-places-close | 1.20% |
| large8-places-play | 0.56% |
| places-lineup-close | 1.91% |
| places-lineup-play | 0.55% |

## `--places` (`places.txt`)

Exit 0: every place its tile's role, every owned place nearest its own civ, every capital and settlement over `PLACE_BAR`
(80 at k 35, 30 at k 18), every town at zero tinted pixels, at the flat zoom every place a mark. Each role's tinted pixels:

| View | capital (least tinted, share) | settlement | town |
|---|---|---|---|
| medium9-places-play (k 18) | 117, 12.6% | 39, 7.0% | 0 |
| medium9-places-close (k 35) | 275, 8.7% | 89, 4.7% | 0 |
| small2-places-close (k 35) | 277, 8.7% | 91, 4.8% | 0 |
| large8-places-close (k 35) | 279, 8.8% | - | 0 |
| large8-places-play (k 18) | 120, 12.7% | - | 0 |
| places-lineup-close (k 35) | 274, 8.7% | 91, 4.8% | 0 |
| places-lineup-play (k 18) | 117, 12.4% | 37, 7.0% | 0 |
| medium9-places-edge/-flat, places-lineup-edge (k 6, 5) | marks | marks | - |

Every place with its tile, clans, tinted and share:

| view | tile | role | civ | on | reads as town | clans | tinted | share | nearest |
|---|---|---|---|---|---|---|---|---|---|
| medium9-places-play | 566 | capital | 592 | plains | false | 255 | 117 | 12.6% | 592 |
| medium9-places-play | 602 | settlement | 592 | hills | false | 671 | 41 | 7.1% | 592 |
| medium9-places-play | 919 | settlement | 592 | forest | true | 689,690,722 | 39 | 7.0% | 592 |
| medium9-places-play | 735 | town | none | forest | true | 208,223,609,621,633,679,683,720 | 0 | 0.0% | - |
| medium9-places-close | 566 | capital | 592 | plains | false | 255 | 275 | 8.7% | 592 |
| medium9-places-close | 602 | settlement | 592 | hills | false | 671 | 89 | 4.7% | 592 |
| medium9-places-close | 919 | settlement | 592 | forest | true | 689,690,722 | 91 | 4.8% | 592 |
| medium9-places-close | 735 | town | none | forest | true | 208,223,609,621,633,679,683,720 | 0 | 0.0% | - |
| small2-places-close | 506 | capital | 454 | marsh | false | 48 | 277 | 8.7% | 454 |
| small2-places-close | 288 | settlement | 454 | plains | false | - | 91 | 4.8% | 454 |
| small2-places-close | 505 | town | none | forest | true | 69,162,330,472 | 0 | 0.0% | - |
| large8-places-close | 2689 | capital | 1600 | marsh | false | 65 | 279 | 8.8% | 1600 |
| large8-places-close | 2787 | town | none | plains | true | 107,401,1218,1632,1685,1777,1864,1911 | 0 | 0.0% | - |
| large8-places-play | 2689 | capital | 1600 | marsh | false | 65 | 120 | 12.7% | 1600 |
| large8-places-play | 2787 | town | none | plains | true | 107,401,1218,1632,1685,1777,1864,1911 | 0 | 0.0% | - |
| large8-places-play | 2871 | town | none | forest | true | 597,1095,1250 | 0 | 0.0% | - |
| large8-places-play | 2524 | town | none | plains | true | 243,264,350,414,754,1742,1747,1775,1965 | 0 | 0.0% | - |
| places-lineup-close | 47 | capital | 10 | plains | true | 7,16,17 | 277 | 8.7% | 10 |
| places-lineup-close | 102 | capital | 11 | plains | false | 8 | 277 | 8.8% | 11 |
| places-lineup-close | 97 | capital | 12 | plains | false | 9 | 274 | 8.7% | 12 |
| places-lineup-close | 140 | settlement | 11 | plains | true | 19,20,21 | 91 | 4.8% | 11 |
| places-lineup-close | 79 | town | none | plains | true | 22,23,24,25 | 0 | 0.0% | - |
| places-lineup-play | 47 | capital | 10 | plains | true | 7,16,17 | 119 | 12.6% | 10 |
| places-lineup-play | 102 | capital | 11 | plains | false | 8 | 119 | 12.6% | 11 |
| places-lineup-play | 97 | capital | 12 | plains | false | 9 | 117 | 12.4% | 12 |
| places-lineup-play | 140 | settlement | 11 | plains | true | 19,20,21 | 37 | 7.0% | 11 |
| places-lineup-play | 79 | town | none | plains | true | 22,23,24,25 | 0 | 0.0% | - |

## `--time`

`time.txt`: five redraws of each view on both trees. `stats.terrainMs` is the static layer's; the places draw in the overlay, so
the overlay's median (`stats.overlayMs`) is the figure:

| View | branch overlay median | main |
|---|---|---|
| medium9-places-play | 8.1 ms | 8.5 ms |
| medium9-places-close | 4.9 ms | 4.6 ms |
| places-lineup-play | 3.2 ms | 3.5 ms |

The terrain's medians are the same (6.4 / 6.6, 2.0 / 1.9). `medium9-places-play` is 0.95 of main's, under the 1.5× bar. The
atlas caches (`scaled`, `shaded`) hold the places' canvases between redraws.

## `play/places.mjs`

Passes: the roles by the rule; cloth nearest its own civ and a town's tint at zero; the clans after the places and in the
place's yard (new: every clan on a place's tile has its foot below the tile's centre, 12 clans); every clan drawn; the click on 140
reaches the settlement and each clan; at the flat zoom every place a mark and the houses on 47 and 140 drawn (47 and 45 pixels near
the colour). `play/clans.mjs` passes. `node www/shots.mjs --units` fails on `small2-camps-selected: no unit to select` on main
too (this runner: no army to select), so it is not run through.

## Changes to the easel, and the variant

- `www/map/layers/places.js`: `PLACE.yard`, `yardTiles`, the sizes and `feet` above.
- `www/map/layers/clans.js`: `layout` puts the clans of a place's tile in its yard (the one rule beyond #444's).
- `www/assets/sources.js`: region `place.flag` and the capital's pennant (`over`), in the cell and its mask; atlas rebuilt
  (`atlas.md`).
- `www/shots.mjs`: `PLACE_BAR`, `--places`'s share and role summary, `--time`'s overlay. `www/play/places.mjs`: the yard check.
- Nothing under `engine/core`, `engine/cli`, `engine/web`.

## For the review

The hall's pennant crosses the centre ring on a capital at k 35 and the diamond hides the hamlet's middle: both are main's
markers drawn over the places, as the spec says. At k 18 the three roles are weaker than at k 35 (places 29 to 43 px). A place
that draws larger at k 18 (a floor in px) would change the rule "a world size"; not taken.
