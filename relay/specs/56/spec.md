# #56 Armies hold ground: spec

## What it delivers

A camp's warriors can be spread over a **zone** the player paints from the camp's tile. The
zone is the camp's territory (`actors.md`): it divides its guarding evenly over it, and the
warriors spread over it one tile a turn. A tile the warriors stand on is **held**: it counts as
the civ's land while they stand there, and a rival camp's line can't pass it. A camp whose line
is cut eats its pack, then forages, as a camp cut off by land already does. The AI paints a zone
over its own line. `paint-army/` is archived.

## Shape: Magnum

Taken as the piece suggests. Holding and cutting are one rule seen from two sides: whether a
held tile cuts a rival is the same test as whether it guards one's own line, and the zone's size,
the spread, the share of guarding a tile needs to count as held, and the AI's zone all trade
against each other (a big zone guards more line but spreads thin and costs labour the camp
needs to eat). Splitting engine from tuning would leave the easel's numbers wrong and the canvas
rewriting it; the page is small next to that. One Opus step, `build-1`.

## Settled (decisions the owner can overrule)

1. **The zone is the camp's territory**, `actor.territory` itself (sorted, the camp's tile in it,
   as every actor's territory holds its home; today it is `[home]`), so `Walks`, `divide` and
   `connected` work on it unchanged. `Unit` gains `held: Vec<usize>` (the tiles its warriors stand
   on). A zone must be connected through hex neighbours to the camp's tile, over walkable land, and
   have at most `people` tiles counting the camp's (paint-army's `people − 1` painted tiles; one
   warrior stays at the camp). Any land may be painted (own, unclaimed or a rival's) except a
   capital.
2. **One action sets the zone**: `Action::Zone { tiles }` lists the painted tiles (the camp's own
   left out) and replaces the zone, checked like the other unit orders (`check_zone`). The page's
   paint and unpaint build the list; unpainting a tile drops whatever is no longer connected
   (paint-army's `pruneZone`). An empty list undeploys.
3. **Moving undeploys.** A move order clears the zone back to `[home]` and the held tiles back to
   the camp's own (the march already resets `territory` on each step). paint-army kept the zone
   when its captain marched; here a camp guards only where it stands, which keeps the zone a
   territory around home and the move simple.
4. **The spread.** At the end of the turn, after the march and before foraging and supply, each zone
   tile next to a held tile becomes held; tiles no longer in the zone stop being held at once. The
   camp's own tile is held from the start. So the warriors spread one ring a turn: each walks one
   tile a turn, as paint-army's detachments did.
5. **Guarding is labour.** The camp sets aside a guarding labour (the build tunes how much) and
   divides it over its held tiles, the camp's own included, with `actors::divide(guard, &[],
   walks)`: no routes, since the line is paid by `supply` as now, and each tile's share pays its
   walk (`Walks` over the held tiles; 0 at the camp) first. A tile whose share after its walk falls
   below a floor (`UNIT.guard`, tuned) isn't held that turn. What's left of the labour goes on
   foraging and supply as now, so a wide zone leaves less for staying fed.
6. **Held land is the holder's for the line only**, through a new `civs::holder(w, i)`. Design says
   a held tile is the civ's land; here it counts so only in the line test. Forage-vs-pillage keeps
   `tile_civ`: a camp holds its own tile, so counting held land there would turn every pillage
   into foraging. Clans' claims, the pre-game's territory and the AI's `post` keep using land
   without holding.
7. **The line test**: a camp's line can't pass a tile held by a rival civ, nor rival civ land unless
   its own civ holds that tile. Design says the line can't pass a rival civ's land; holding a tile
   there lets one's own line through it, so a guarded line can reach into a rival's country. The
   line's two ends, the camp's tile and the capital, never block it.
8. **Two camps on one tile** (designer's call): the tile goes to the camp whose guarding share on it
   (after its walk) is larger; on a tie it stays with whoever holds it, and a fresh tie goes to the
   lower civ id. The loser's tile stays in its zone, unheld, and it takes the tile if its share
   grows or the holder's shrinks. No fighting: nobody is hurt, the thicker guard simply stands
   there. (The spec had the first holder keep it. AI camps sit 3 steps out (`UNIT.post`) and the
   AI's zone of `people − 1 = 2` tiles covers both tiles between camp and capital, so under
   first-holder an AI's line could never be cut once it deployed, and the review's cut couldn't
   happen. Under the share rule the player cuts by raising a bigger camp and painting tightly.)
9. **Cut by a rival army** is its own feed event: `LineCut { by }` ("X's camp's line is cut by Y's
   army"), alongside the existing `CampCutOff` (which stays for being cut off by distance or land):
   a line is cut by an army when it would carry the camp's food with holding ignored and doesn't
   with it. Also `Deployed` (zone set, with its size) and `Held` when the spread first fills the
   zone. Army counters gain `held` (tile-turns held) and `cut_by_army` (turns cut by a rival's
   ground) for the sweep.
10. **The AI's zone guards its own line**: an AI camp that is posted and idle paints the first tiles
    of its line home outward from the camp, up to what its size allows and the build finds it can
    afford. It never paints to cut a rival; that's left to the player.
11. **On the page**: Paint and Unpaint buttons in the camp's panel (and keys P/U), click or drag on
    the map; the zone drawn as an outline in the civ's colour, held tiles filled faintly, a rival's
    held tiles shown the same in theirs. The panel shows the zone's size, held count and the guard
    share per tile. Selecting a camp still draws its line; a line cut by held ground draws to the
    tile where it stops, marked.
12. **Archive `paint-army/`** (tag `prototype/paint-army` exists): it has no `package.json` and
    `ci.yml`'s `js` matrix doesn't run it, so delete `index.html`, leave a one-line `README.md`
    naming the tag as `hex-economy/README.md` does, and drop its row from the repo's `README.md`.

## Left unsettled, all in build-1

- `UNIT.guard`, the labour guarding takes, the zone cap beyond `people − 1`, and the AI's zone
  length. **Proof**: the sweep (`sweep --after 200 --maps retro`, extended with held and
  cut-by-army columns) shows AI camps holding their lines without starving more than with the zone
  off (a `--no-zone` control), and a scripted cut (see below) starves the rival's camp.
- **The cut, scripted**: a play script `www/play-hold.mjs` that leads a civ, marches its camp next to
  a rival camp's line, paints a zone across it, and ends turns until the rival's camp is reported
  cut and hungry. It may raise warriors first: a ring round a rival camp is six tiles, less where
  water or mountains close it; screenshots under `relay/specs/56/screens/`. The review reruns it.

## Not in this piece

Fighting, entrenchment, forts, sieges, capturing cities, stewards.
