# Road map

Each milestone ends in something the owner plays, and each piece in it answers a question
in the scores (`process.md`). The order is a plan, not a promise: the retro after each
milestone re-cuts the next. Years don't differ: every year is the same, and the game's
motion comes from people, land and what they learn, not from the weather.

## What a finished game has

| It needs                                                                          | Milestone                      |
| --------------------------------------------------------------------------------- | ------------------------------ |
| A way to win and a way to lose                                                    | M3                             |
| A rival that plays for the same goal                                              | M3                             |
| Land that is yours, and armies that take and hold it                              | M3                             |
| A page that is the game: start, play, end, save                                   | M3, then grown every milestone |
| A judge on every change, and a player that plays every piece                      | M3                             |
| New ways to feed a civ: fields, herds, beer                                       | M4, M5                         |
| Knowledge that spreads                                                            | M6                             |
| A world worth crossing: biomes, deserts, ranges, steppe, the routes across them   | M7                             |
| Goods worth the walk                                                              | M8                             |
| Surplus: cloth, the steward, great works                                          | M9                             |
| Water and walls: irrigation, sieges, cities                                       | M10                            |
| Every way to win; learning; balance from automatic players; art, sound, a release | M11                            |

## Milestones

### M3: A game

The engine shaped for many units and many players; a judge and a player on every PR; land
you claim and guard; armies of kinds that skirmish, march, besiege and surrender; a rival
that plays for Standing; a way to win and to lose; a page that is the game.
`proposals/m3.md` has the pieces. You play: a whole game against two rivals.

### M4: Settling

The Neolithic revolution begins: the farmer. A clan splits its labour between gathering and
fields as its grain tames; fields are small, worked hard, and don't wear; the home choice,
by the field or by the market; towns where many clans
choose to live. A settled civ feeds more people on less land and has a store worth taking.
You play: a civ that settles against one that doesn't. Needs first: labour across jobs on
the actor (`architecture.md`), the owner's design call. Vision: `design/farming.md`, to
write.

### M5: Herds and beer

Herds as a store that walks and breeds; pastoral clans with wide, thin land; pack animals
that cut a route's cost and carry a line further; horsemen. Beer from grain: a good that
isn't eaten, which draws clans to a feast and feeds people put to a work. The engine learns
that a good can be something other than food. You play: a pastoral civ against a farming
one, and a feast that draws a rival's clans. Needs: a carrier on the shared actor; goods
that aren't eaten. Visions: `design/animals.md` and `design/goods.md`, to write.

### M6: Knowing things

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

Goods that aren't food from fixed sources across the biomes: salt from springs and coasts,
which slows spoiling; obsidian and shells from a few tiles; beads as an ornament good.
Festivals carry them, clans specialise by what lies near, traders walk between festivals,
and standing partnerships form. You play: a civ whose land holds a salt spring. Needs: the
goods seam (M5), the map (M7). Vision: `design/goods.md`.

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
- A milestone ends with a tag, `m<n>`, the owner's play, and the retro.

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
