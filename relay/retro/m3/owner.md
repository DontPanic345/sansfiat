# M3: the owner's notes

Dictated after playing main (ec1e47b, Armies merged; #105 R unbuilt and paused), 6 Nov 2026.
The retro's angles read these as evidence and cite them by section and number.

## 1. Play notes

1. The scout should have three buttons: Route from…, Route to… and Move to….
2. The army's Disband all doesn't work: it disbands one at a time. It should be the same as
   the guard's.
3. The scout didn't check all the mountains. It checked one, checked one possible, then
   ignored the other two that may have had a trail on them. The debug view showed they were
   impassable, but the scout never went near them, so the scout wasn't working properly.
4. A trail should always make a difference. Even on plains, it should make a difference.
5. I want component and integration tests: a controlled set-up where the scout (or anything)
   is placed, scripted instructions are given, and an outcome is tested. Real units and the
   real engine running.

## 2. Time and distance

How far is the next town? "A day's walk." That is a distance measured in time and pace.

A person who walks from town to town uses up all of their labour for that day. They
couldn't carry much more than what it took to sustain themselves. This is what naturally
leads to camps, towns and villages being spaced a tile apart, which is what we see in the
real world: villages spaced by the mode of transport. That's possibly the yardstick.

How far can a person walk if they devote a whole day to walking, carrying no luggage, along
a marked route? For the sake of argument, and for developing a game, say that day, as a
relative measure, is 1/100th of the year. That gives the labourer 99 other days to do their
work, whether that is working their tile or carrying something on their back. If they
carried 20 kilos, it would take them a whole extra day.

Land area matters too. Correct me, because I'm probably wrong: an acre or a hectare was
defined as the amount of land that could be ploughed in a day, or in some given time, and I
can't remember whether that was with an ox or a horse.

- How much land, and how many days of the year, does a person need to spend tending it?
  Probably a lot.
- How much of the tile's land area does that use?
- How much yield does it give, if it's growing some kind of grain?
- Per day, how much does that person need to eat?
- How much can they carry? Limited by two things: a maximum weight or a maximum volume.
  Beyond a volume it becomes impractical to carry anything more.

Run the numbers for a farmer living and working one tile away from the market. The case to
work with is the low-yield one: no canals, no irrigation, only a patch by the river. With a
river, canals and irrigation the area extends, and a floodplain is a bonus, but assume none
of that here.

- What is the unirrigated area?
- How many food calories does it yield?
- How many food calories a day does the farmer need to eat?
- By the end of the year, how much harvest does he have to take to market?
- How much is he hauling back?

## 3. Supply as a flow

The supply line is modelled as a flow: a rate or capacity, separate from but derived from
the movement speed of a person moving "per day" over that tile. Kilograms per labour-day.

A ring drawn around the possible centres the army can draw from (the civ as it stands).
Inside this ring the army can supply itself with more than it eats a day; outside it, it is
relying on foraging and its pack. At the edge, every person's labour is devoted to resupply:
useless for fighting.

## 4. The retro

Pick the top set of retro angles that lead to changes and run them at Opus medium.

## 5. #101 held (PR 124): the bands, the ledger lines, the prompts and the trial's record

- I like the player, `relay/players/player.md`.
- I don't think the bands' health was right. I do want a judge, and I want it to have taste,
  and it runs at review time. It should confirm that the game has got better by the measures
  that matter to the game, not to an engine. I don't think running it during CI would have
  caught anything.
- Strike "An army mustn't starve its own side".
- The proposal says what to do with #101: change now and merge, close, or remake into the
  next milestone. Interview me if more is needed about the direction.

## 6. On the horizon

Now or later, retros start automatically as part of the milestone. Feedback from the player
and the retros is automatically fed back into new work items. Why: I am still the bottleneck.
If Relay identifies something that obviously should be done, don't wait for me.
