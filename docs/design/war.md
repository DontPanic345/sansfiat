# Land and war

Red and Blue are rivals. Each claims land out from its capital and sets warriors over it,
spread thin along the border. Blue wants more and paints one of Red's tiles. Where the
claims overlap the guards skirmish: a few losses a turn, costly to both and decisive for
neither. Red answers by gathering every warrior into one army and marching it over the
border. The dispersed Blue guards barely slow it, but Blue's capital is another matter: the
militia there, the guards drawn back to it, and Red's own hunger far from home stop it
short. Then Blue closes round the Red army, cuts its line home, and doesn't fight it.
There's no need. The army eats its pack, goes hungry, and surrenders.

## Land

- A civ's land is what it **claims**: a connected patch painted out from the capital.
  Claiming is a decision; holding is the cost.
- A civ **guards** its land as every actor divides its labour: its guards spread over the
  claim, so a wide claim is thinly held. Unguarded claim is paper: a rival unit walking in
  takes the tile.
- A clan whose home is on a civ's land leans to it a little each turn, and the civ's guard
  protects it. Clans off its land are won at festivals, as now. The people on a civ's land
  and in its clans are its **Standing**.
- Claims may overlap. An overlapped tile is contested.

## Armies

- A civ raises **armies** from its clans' people, as many as it can feed. An army is a unit:
  a stack of people of one or more kinds on a tile, with a pack and a line home
  (`supply.md`).
- **Kinds.** Warriors first. Archers need bows, made with labour on forest the civ holds.
  Horsemen need herds and come with animals. A kind has a strength in attack and in defence,
  and what a civ has learned (`technology.md`) shifts it a little.
- An army **holds** the tiles its people stand on (`supply.md`, the zone) and **marches** as
  a stack.
- **Militia.** A clan defends its home tile with its own people, weakly. The capital's clan
  and its store make the capital the hardest tile to take.

## Fighting

- A **contested tile** skirmishes each turn: each side loses a small share of its people
  there, by the other side's strength. The stronger side holds the tile.
- **Entrenchment.** A defender that stays put gets stronger, up to a cap.
- **Retreat.** A side losing badly falls back to its nearest held tile rather than die
  where it stands.
- A tile changes hands when none of the holder's people are left on it. A capital falls
  when its militia is gone and a rival army stands on it.
- Fighting is always over a tile. An army sent at another fights it on the tile it holds,
  since arriving contests it; an army on a tile nobody enters isn't fighting anyone.

## Supply and surrender

- An army lives on its pack and its line (`supply.md`). A line can't pass a rival's held
  ground, so a surrounded army is cut off.
- Hungry for long enough, an army **surrenders**: its people join the nearest clan of whoever
  holds the ground, and both civs hear of it.
- A siege is a line cut: an army round a capital stops its clans working, and its store
  spoils down.

## Losing and winning

- A civ **loses** when its capital falls. Its land and clans go unaligned and its armies
  disband.
- The first way to **win**: the last capital standing, or the most Standing when the turn
  limit comes.

## What it needs first

Orders addressed to a unit, and many units per civ (`../architecture.md`); the page's unit
panel (`page.md`).

## What it isn't

- No stacks of doom: an army eats, and far from home most of its labour goes on staying fed.
- No fog, and no alliances yet. No walls or cities yet (`../roadmap.md`, M9).

## Decided

- A civ's land is the part of its claim its guard holds: a claimed tile whose guard falls under a floor is paper, and counts for nothing. The capital is always held. (#102)
- A civ raises one guard, which stands at the capital, takes no orders, and eats from the store and then from its clans' stores. Its camp's held tiles add to the guard on its claim. (#102)
- Where two civs hold the same tile, it is the land of the one with the thicker guard there, and a tie goes to the older civ. A claim can't cover a rival's capital. (#102)
- A rival unit that ends its turn on paper takes that tile, and with it any part of the claim no longer joined to the capital. (#102)
- Standing counts unaligned clans whose homes are on a civ's land. (#102)
- Until the AI learns to claim, it claims every land tile within a step of its capital or of any clan's home within four steps of it, and raises a guard while some of that claim is paper and it can feed one. (#102)
- Every turn, wherever people of two civs stand on one tile, they fight: each side loses a fifth of the other side's strength there in people, and the stronger side left holds the tile. (#104)
- Only armies, guards and militia fight. Only an army takes unguarded claim; a scout neither fights nor takes land. (#104)
- Warriors fight at 1 attacking and defending. Archers fight at 0.8 attacking and 1.4 defending, with no range, since a fight is over one tile. (#104)
- Archers can be raised only by a civ that holds forest, and the army they join spends that turn making bows instead of marching. (#104)
- An army that stays put entrenches by a tenth a turn, up to half again its strength. A guard always defends entrenched. A hungry army fights at half strength. (#104)
- A march stops on the first tile where a rival has people. (#104)
- An army left under half the other side's strength retreats one step back. Guards and militia never retreat: a guard loses a tile only when an army kills all its people there. (#104)
- A civ's clans defend their home against a rival army at three tenths of a warrior's strength. While a rival army stands on a capital, the whole guard is drawn back to it, and the rest of the claim is paper until the capital is clear. (#104)
- A capital is besieged while rival armies hold every way into it: its own clans harvest nothing, and no levy reaches the store. (#104)
- An army that has gone hungry three turns in a row with its line cut, isn't marching and isn't fighting that turn surrenders whole, with its pack, to the nearest clan of the rival whose land cuts it off. (#104)
- A capital falls when a rival army stands on it and the civ has no guard, no army and less than one person of militia there. The fallen civ's units go home to the nearest clan, its store fills the packs of the armies that took the capital and the rest is lost, and its market becomes an ordinary festival. (#104)
- A game ends by conquest only once two or more civs were founded and at most one capital stands, so a watched game of three goes on after one capital falls. (#152)
- The AI plays every civ the player doesn't lead with the player's orders and no others. Once a turn it sees to the first of these that wants something: its armies fed, its land held, its Standing grown, its nearest rival hurt, a capital it outweighs taken. This ends the stand-in claim of #102 and the AI's raising no army of #152. (#157)
- A civ is founded with the claim the stand-in was: the land within a step of its capital and of the clans' homes near it. The AI widens it a tile a turn toward the nearest clan's home it doesn't cover, while its guard holds all of it and its levy feeds one more guard; it gives up its farthest paper while it can't raise a guard for it, and it never paints a rival's land. (#157)
- The AI raises a warrior a turn while its levy covers its guard and armies and one more half again over and its store holds the pack: up to a quarter of its people while a rival's capital is within four turns' walk or a rival army is on its claim, else a party of four, which can live off the land. It sends a warrior home when its store can't feed its mouths. (#157)
- An AI army outweighs a capital when the fight there, played forward by the fight's own rule against the whole guard, the militia and any army on it, takes it within eight turns with three tenths of the army left. It marches on the nearest capital it outweighs and pulls back the turn it no longer does. (#157)
- An AI army of three or more marches onto the nearest rival land whose defenders it outweighs by half again, a clan's home before its ground, and pillages it; and it stands across the line of a rival army abroad, with its zone over the line's tiles beside it. (#157)
- A rival army on the AI's claim or beside its capital: its army marches onto it when it outweighs it by half again, else stands across its line, else stands on its capital. (#157)
- An AI army marches home when a rival cuts its line, when it is hungry, and when its pack is under a turn and the tile under it is bare; off its own land with a low pack it first halts and lives off the tile. It leaves its own land only with four fifths of its pack. (#157)
