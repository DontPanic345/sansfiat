# Time, distance and labour

How far is the next town? A day's walk. Distance is measured in time and pace, and so is
everything else a person does: the day is the yardstick, and every rule that spends labour,
walks a route or carries a load is priced in it.

## The day

- A **day** is one person's labour for one day. A turn is a year, and a year is a fixed
  count of days (`engine/core/src/data.rs`); nothing in the rules counts seasons, and
  `clans.md`'s rule stands: a turn is however long the seasons take to average out.
- Each person gives the actor they belong to that many days a turn. Walking, gathering,
  tending a field, carrying, guarding, searching and fighting all spend them, and a day
  spent one way isn't spent another.
- A person eats every day whether they work or not.

## Distance

- A **tile** is a day's walk: what a person crosses in a day along a marked route, carrying
  nothing. Rough ground takes more days; a trail brings a tile back toward one day,
  whatever the ground, so a trail makes a difference everywhere, plains included.
- A route's cost is days there and back, as `actors.md` says, and every actor pays it the
  same way.

## Carrying

- A person carries a load up to a weight, and up to a bulk past which more is awkward
  whatever it weighs. Grain and most food hit the weight first; fodder and straw the bulk.
- Carrying a full load takes more of the day than walking empty: a loaded walk costs more
  days per tile.
- A full load is some days' food. So food walks a few tiles at most before the carrier has
  eaten what he carries, and a surplus is worth more the nearer the mouth it feeds. That is
  why towns sit a day apart, and why the market comes to the farmer before the farmer walks
  far to the market (`farming.md`).

## Land

- A field is sized by the days it takes, not by the tile: a person tends only so much land
  in a year with hands and a hoe, and more once an animal pulls for him (`animals.md`).
- A tile is far bigger than anyone works. What limits a tile's fields is the land that will
  take them (river-side, floodplain, later irrigated ground), and the days its people have.

## Supply as a flow

- A supply line is a **flow**: food per turn, which is the days put on it over the days a
  round trip takes, times what one person carries. Longer and rougher lines carry less for
  the same labour; trails raise the flow, pack animals raise it more (`animals.md`).
- Round a civ there is a **ring**: inside it an army's line brings more than the army eats
  a turn, with days to spare for fighting; at its edge every day goes on staying fed and
  none on fighting; outside it the army lives on its pack and the land. The ring is drawn
  from the civ as it stands, from every tile a line may start at, and it moves as the civ
  does. The page draws it when an army is selected.

## What it needs first

Every actor's labour, walk, pack and store already run through `actors.rs`. This file
changes what the units of those numbers mean, so that a day is one number everywhere and
the move cost of a tile, the labour of a person and the food of a load are read against it.

## What it isn't

- Not a calendar. There are no seasons, and no turn is longer or shorter than another.
- Not a speed stat. Nothing moves faster than a person except what carries him (`animals.md`).
- Not a hard limit on reach. Reach runs out because days do.

## Decided

- A trail on flat land costs one day's walk; bare plains cost a day and a third, forest and hills two and two thirds, marsh and a pass four. No reach changed with the unit. (#154)
- The page, its tooltips and the text view give labour in days. (#154)
- A walk is paid by every person who walks it. A clan's festival walk takes one walker for each 2 food it carries there, and at least one, so a clan of twenty carrying its fill pays more than a clan of two. (#154)
- A load of 2 food is about nine days' eating. Clans on the maps carry less than a load, so nearly every clan sends one walker. (#154)
- A clan's walk to each of its tiles is paid by the people who work that tile, and so is the guard's walk over the claim. (#154)
