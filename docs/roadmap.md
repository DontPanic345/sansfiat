# Road map

Each milestone ends in something the owner plays, and each piece in it answers a question
in the scores (`process.md`). The order is a plan, not a promise: the retro after each
milestone re-cuts the next. Years don't differ: every year is the same, and the game's
motion comes from people, land and what they learn, not from the weather.

## What a finished game has

| It needs                                                                          | Milestone                      |
| --------------------------------------------------------------------------------- | ------------------------------ |
| A way to win and a way to lose                                                    | M4                             |
| A rival that plays for the same goal                                              | M4                             |
| Land that is yours, and armies that take and hold it                              | M3                             |
| A page that is the game: start, play, end, save                                   | M3, then grown every milestone |
| A judge on every change, and a player that plays every piece                      | M3, then M4: the stage, the compare and the play step |
| The day as the yardstick everything is priced in                                  | M4; the clock closed as built in M5 |
| A loop that closes: settlements, reach, goods that are wanted, units that trade   | M5                             |
| New ways to feed a civ: fields, herds, beer                                       | M4, M6, M6                     |
| Knowledge that spreads                                                            | M6                             |
| A world worth crossing: biomes, deserts, ranges, steppe, the routes across them   | M7                             |
| Goods worth the walk                                                              | M5, then M8                    |
| Surplus: cloth, the steward, great works                                          | M9                             |
| Water and walls: irrigation, sieges, cities                                       | M10                            |
| Every way to win; learning; balance from automatic players; art, sound, a release | M11                            |

## Milestones

### M3: A game

The engine shaped for many units and many players; a judge that reports and a player that
plays; land you claim and guard; armies of kinds that skirmish, march, besiege and surrender;
a page that is the game. `proposals/m3.md` has the pieces, and `proposals/m3-retro.md` what
they answered. The rival that plays and the game that ends (R) didn't run: without an economy
that feeds an army there was nothing for it to plan, so it moves to M4.

### M4: Settling

A game that ends; the game proving itself, with a stage that places units on a known map
and a judge that compares against main with the drift measured; scouts that work; the day
as the yardstick, with labour split across jobs and the supply ring drawn; the farmer, whose
clan puts its days to fields as its grain tames and fills a store worth taking;
towns where many clans live on one tile; and then the rival that plays for Standing on that
economy. `proposals/m4.md` has the pieces. You play: a game that ends, against a rival that
settles and comes for your store. Visions: `design/labour.md` and `design/farming.md`,
drafted.

### M5: A loop that closes

Explore, expand, exploit, exterminate and exchange, each with the least that makes it work.
Land that is held by being claimed and lost by being walked on, with the guard and the militia
gone; clans on a civ's land that are plainly its own; signposts that draw clans to settle, and
settlements with a market, a store and land; lines that end at the nearest store, so an army
reaches a rival fed; scouts that eat at a store, finish their errands and can be killed. Then
salt and obsidian from sources on the map, units that trade at markets, and a good's path
drawn from its source. `proposals/m5.md` has the pieces, and `proposals/m4-retro.md` what M4
answered. You play: a small map where you settle toward a rival and a capital falls. Vision:
`design/goods.md`, to draft before the goods pieces.

### M6: Herding and knowing

First what M5 set aside: the page played and its worst faults fixed; every staged test listed
and watched on the page; herds as a store that walks and breeds, pastoral clans with wide,
thin land, pack animals that carry a line further, and horsemen. Vision: `design/animals.md`,
drafted.

Then beer, from grain: a good that isn't eaten, which draws clans to a feast and feeds people
put to a work. Vision: `design/goods.md`.

Technology as drift (`design/technology.md`): knacks that grow by practice, luck and contact,
so an advantage leaks and lagging is a strategy. The camel is one: a knack that turns up
among clans at a desert's edge and spreads. You play: a civ that lags on purpose and
copies. Needs: unit kinds and fighting (M3), herds (M5).

### M7: The wide world

A new map: larger biomes; deserts no one crosses until the camel; alpine ranges with few
passes; steppe that feeds herds and little else. Routes with rests: campsites a line can
stage through, caches an army leaves behind it, caravanserai where routes cross. A line
reaches as far as its rests do. You play: a steppe civ reaching across a desert. Needs:
knacks (M6), pack animals (M5). Vision: `design/map.md` grows, and `design/routes.md`, to
write.

### M8: Things worth the walk

More goods from fixed sources across the biomes, beside M5's salt and obsidian: wood, stone
and shells from a few tiles; beads as an ornament good. Clans specialise by what lies near,
traders walk between festivals, and standing partnerships form. You play: a civ whose trade
crosses a desert. Needs: goods and exchange (M5), the map (M7). Vision: `design/goods.md`.

### M9: Surplus and the steward

Weaving as a long labour chain; the steward who levies a painted area and the records that
widen his reach; great works that take the world's labour, fed with beer. You play: a great
work. Needs: a grain surplus (M4), processed goods (M5).

### M10: Water and walls

Irrigation as an improvement bought and kept with labour; walls that raise a defender's
entrenchment; sieges of a walled centre; more than one city. You play: a siege. Needs: tile
improvements; a city as an actor.

### M11: Finished

Every way to win weighed and chosen; the start screen's options and settings; learning the
touch points; balance from automatic players across strategies; paid art and sound in; a
save that survives versions; a release on Pages and itch.io.

## Alongside every milestone

- The judge grows by one check per piece, and the play step plays every piece.
- The page grows with every mechanic it shows, to `design/page.md`, and each mechanic's
  page step brings its motion: a march, a skirmish, a feast, drawn so the eye catches it.
- A milestone is built when none of its pieces can run; the crew tags it `m<n>`, the owner
  plays, and the retro runs, from M4 as a Relay run.

## The look and the sound

| What                                                            | When                        | Why then                                                     |
| --------------------------------------------------------------- | --------------------------- | ------------------------------------------------------------ |
| The HUD and menus: top bar, bottom panel, start and end screens | M3, piece P                 | The game needs them to be played                             |
| Placeholder art on the atlas                                    | M3, piece P                 | So paid art is a file swap                                   |
| Sound hooks: feed events to a sound bus, silent                 | M3, piece P                 | So sound is a file swap                                      |
| Motion for each mechanic                                        | With its page step, from M4 | Judged by eye, so it rides on a piece the owner plays anyway |
| Paid tile art                                                   | After M7                    | The biomes are settled then, not before                      |
| Paid unit and icon art                                          | After M5                    | Kinds, herds and carriers exist then                         |
| Paid sound                                                      | After M7                    | The feed's events have stood for several milestones          |
| Settings screen, music, a polish pass                           | M11                         | There's something to set by then                             |
