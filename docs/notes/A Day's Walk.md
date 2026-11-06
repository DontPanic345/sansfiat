# A day's walk

The owner's yardstick (`relay/retro/m3/owner.md` §2), run through with real-world numbers.
Rounded; the sources are the usual historical and ethnographic ranges, not measurements of
one place. The design that follows from them is `docs/design/labour.md` and `farming.md`;
the engine's numbers are `engine/core/src/data.rs`, and they're ratios of these, not these.

## The acre, corrected

- An **acre** is the land a yoke of oxen ploughs in a day: a furlong (a "furrow long",
  220 yards) by a chain (22 yards), about 0.4 ha. Oxen, not horses; horses came to the
  plough thousands of years later, with the collar.
- A **hectare** has no such origin: it's 10,000 m², a metric unit from the 1790s.
- The German *Morgen* ("morning") is the same idea as the acre, half a day's ploughing,
  0.25–0.9 ha by region.
- For the first farmers the plough and the ox are still in the future. A person with a
  digging stick or a hoe prepares about 0.05–0.1 ha in a day, so "a day's work of land"
  is a tenth of an acre, not an acre. The ox multiplies it by five to ten: that's M5's
  animals, and it's the single biggest change a herd makes to a farmer.

## The person

| | |
|---|---|
| A day's walk, unladen, on a marked route | 30–40 km; call it 35 |
| Loaded (20–30 kg), the same route | 20–25 km, so a loaded tile costs about 1.5 days |
| A sustainable load | 20–30 kg; a porter carries 30 all day, a soldier 20–25 with kit |
| Bulk past which a load is awkward | 50–60 litres; grain at 0.75 kg/l hits the weight first, fodder the bulk |
| Food a day, working | about 2,500 kcal |
| Grain | about 3,400 kcal/kg, so an all-grain diet is 0.75 kg a day, 270 kg a year; with other foods, 0.5 kg a day and about 200 kg of grain a year |
| A full load of grain | 25 kg, 30–50 days of one person's grain |

So a person carries 30–50 days of food and walks a tile in a day: food walks a very long way
in principle, and the limit isn't the pack. It's that carrying is days not spent working.

## The field, unirrigated

| | |
|---|---|
| Dry-farmed early grain, gross | 500–1,000 kg/ha |
| Seed sown | 100–150 kg/ha; kept back, never eaten |
| Net | about 400–800 kg/ha; call it 500 |
| Days per hectare with a hoe | 70–100 (prepare 20–40, sow and weed 20–30, cut and thresh 15–25) |
| So one person, 99 days | about 1 ha, net 500 kg of grain |

One hoe-farmer feeds two people on grain alone, or himself with a margin, or four if grain is
two fifths of what they eat (`clans.md`, Diet). A tile a day's walk across (35 km) is about
1,000 km², 100,000 ha; the fields are nothing against it. What bounds a tile's fields is the
ground that takes them without irrigation: the river-side strip, a few hundred metres wide,
and the floodplain. A 20 km river across a tile with 300 m of workable bank each side is
1,200 ha: a thousand hoe-farmers, more than a whole clan district has.

## The farmer one tile from market

He has 99 days after the one day the year's reckoning spends on him. Say 80 go on 0.8 ha
and the rest on gathering, and he eats 200 kg of grain. The field nets 400; 200 is his, 200
is surplus. To carry 200 kg to a market a tile away:

| | |
|---|---|
| Loads | 8 of 25 kg |
| Days per load | 1.5 out, 1 back |
| Days carrying | 20, a fifth of his year |
| What he carries back | what he traded for: fish, hides, fruit, light goods; a few loads at most, since his surplus traded at a festival's ratios is lighter than the grain |

A fifth of his year to move a tile is the whole result. Two tiles away it's two fifths and
the field shrinks to pay for it; by three the grain isn't worth walking. So the farmer moves
to the market, or the market forms at the farmers. Fields and festival end on one tile: a
town. A pack animal carries four loads and eats grass: the ring widens to three or four
tiles, which is where village spacing sits once there are carts and donkeys.

## The army

A warrior eats 0.75 kg a day and carries 25 kg: 33 days of his own food if he carries
nothing else. A line a tile long that keeps him fed takes one carrier's 2.5 days per 25 kg,
so about one carrier-day in 13 feeds one warrior-day one tile out; at four tiles out it's
one in three, and at ten the carrier eats his load on the way. That's the ring
(`labour.md`, Supply as a flow): the distance at which a line's labour equals what it
carries. It's a few tiles on foot, more with animals and trails.

## What this says for the engine

- The engine's unit of labour today is `CLAN.work` 3.5 a person a turn, and a plains tile
  costs 1 to enter: a person walks three and a half tiles a year and does nothing else. In
  days that's 100 a person and 1 a tile. The ratio of walking to working is what the design
  needs; `architecture.md`'s angle (`relay/retro/m3/architecture.md`) says whether the
  change is a renaming of `data.rs` or a rule change.
- A person eats 1 food a turn. A load should be a few turns' food on the day scale: a
  load of 25 kg is a tenth of a year's eating, so the pack rule "a full pack holds several
  turns of food" (`supply.md`) is generous by ten, unless the pack stands for the train
  behind the army rather than what each man carries. Worth deciding.
- A field is days, not tiles. The farmer's territory (`actors.md`, Later actors) is the
  labour it puts in, on the ground the tile allows.
