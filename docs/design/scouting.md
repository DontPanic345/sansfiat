# Scouting and trails

A scout finds and improves routes across hard country. Every rough tile hides how cheap a
trail could make it, and a mountain hides whether it can be crossed at all. Give a scout two
points and it works out the best route between them, revealing only the tiles that could
change the answer, then builds trails along it.

`scout/` is the starting point. Its rules are kept unless this file changes them, and its
numbers are a place to start tuning from.

## Who scouts

Only a civ scouts, so scouting comes after founding. A scout is a civ's unit, an actor
(`actors.md`), made, fed, moved and disbanded like any unit (`founding.md`, `supply.md`); clans don't scout. Each civ
starts with one scout. It forages along its route, so it seldom draws on its line.

## Hidden costs

Each forest, hills and mountain tile has a hidden trail cost, fixed when the map is made:
what a trail brings its move cost down to. A trail can bring forest and hills down a long way,
and costs work to build, more on rougher ground. A mountain keeps a high cost even with a
trail, and takes the most work.

A mountain can be crossed only where a bridge would span it, and only a few can. Bridge span
is a technology later; for now every civ has the shortest span. A trail brings flat land down a
little; walking wears one there, and a scout builds trails only on rough ground and mountains.

## Trails

A tile has one trail, however it got it. Clans wear one by walking a round (`clans.md`), or a
scout builds one. Either way the trail brings the tile to its hidden trail cost; a worn trail
can't cross a mountain, since clans can't walk there.

Trails belong to the tile, so everyone uses them, enemies included. Any trail nobody uses
wears away, as worn trails do now.

## Knowledge

What a civ's scouts reveal is known to the civ, its allied clans and its allied civs.
Revealed costs are the only thing hidden: there's no fog over terrain or food.

## A route

The player selects a scout, picks a start (a city, say), then a target: an ally for a trade
route, an enemy for a sneak attack. The scout keeps two routes in mind:

- **known**: the best route it can be sure of, pricing every unrevealed tile at its bare cost.
- **possible**: the best route the unrevealed tiles allow, pricing each at its cheapest trail.

While possible is cheaper than known, it reveals tiles on the possible route that could
close the gap: impassable tiles first, since they decide whether a route exists, then the
biggest saving. Once no reveal can help, it builds trails along the known route. A route with
no way through ends with "no route can exist".

Revealing a forest or hills tile takes a little labour; revealing a mountain takes the
scout's whole turn.

The scout should work outward from where it stands. Along a mountain range it checks the
nearby mountains before crossing to try the far end, rather than walking back and forth
between the ends as `scout/` does.

## What the player sees

The page shows the revealed tiles, both routes with their costs, and trails being built. The
event feed carries what the scout finds: a route found and its cost, no route can exist, a
trail built.

The player gives every order through the page's own controls: select a scout, set its start,
set its target. Debug views stay, but nothing in play needs them.

## Out of scope

- Fog of war.
- Bridge span as a technology.
- Alliances themselves: until they exist, knowledge is the civ's alone.
- Several scouts sharing one route.

## Decided

- The scout has the camp's switches and forages round it. (M2 retro, owner note 9)
- Trails, worn or built, make a route cheaper; a marsh trail only bridges its river. (#74, #76)
- A trail on a route the civ uses, such as a camp's line, counts as walked. (#78)
- The player hears of a route when it's found and again when it's built. (#74, #77)
- A scout forages its own tile and the tiles next to it, and pays the walk to each. (#104)
- A scout keeps its route until the route is cleared, and the trails along it count as walked while it does. (#104)
- A trail brings plains and coast to three quarters of their bare cost, worn by traffic as on rough ground; marsh and desert gain nothing. The scout builds no flat trail. (#153)
- The scout's row is Move to… (it walks to a tile and stays there, foraging), Route from… (a ring marks the start) and Route to…, with Clear route and Stop. (#153)
- Along a range the scout checks the mountain with the best case first: the cost through it plus its walk there. The panel lists the mountains it leaves, each with its cost through against the known route. (#153)
- A low scout leaves its errand for the nearest ground within eight steps where a turn's foraging brings a fifth more than it eats, fills its pack and goes back to its errand; an idle scout waits on such ground. (#153)
- A scout's built trails count as walked while it works its route, so they don't wear away behind it. (#154)
