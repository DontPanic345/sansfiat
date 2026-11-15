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

- A civ's land is its claim: every claimed tile joined to the capital, held with nobody standing on it. No guard and no militia: neither is raised, fed or counted, so what a wide claim costs is the armies it takes to answer for it. (#336)
- Claims don't overlap: a claim can't cover a rival's land or its capital, so a rival's tile is had only by taking it. (#102, #336)
- A rival army that ends its turn on a tile of a civ's land where no army of that civ stands takes it, and with it any part of the claim no longer joined to the capital. The tile joins the taker's claim where it touches it; the part cut off is nobody's. (#102, #336)
- Standing counts unaligned clans whose homes are on a civ's land. (#102)
- Every turn, wherever armies of two or more civs stand on one tile, they fight: each side loses a fifth of the others' strength there in people, and the strongest side left holds the tile. (#104, #291, #336)
- Only armies fight, and only an army takes land; a scout neither fights nor takes land, and clans don't defend their homes. (#104, #336)
- A march stops on the first tile where a rival has people. (#104)
- A capital is besieged while rival armies hold every way into it: its own clans harvest nothing, and no levy reaches the store. (#104)
- An army that has gone hungry three turns in a row with its line cut, isn't marching and isn't fighting that turn surrenders whole, with its pack, to the nearest clan of the rival whose land cuts it off. (#104)
- A capital falls when a rival army stands on it and no army of its civ does. The fallen civ's units go home to the nearest clan, its store fills the packs of the armies that took the capital and the rest is lost, and its market becomes an ordinary festival. (#104, #336)
- A game ends by conquest only once two or more civs were founded and at most one capital stands, so a watched game of three goes on after one capital falls. (#152)
- The AI plays every civ the player doesn't lead with the player's orders and no others. Once a turn it sees to the first of these that wants something: its armies fed, its land held, its Standing grown, its nearest rival hurt, a capital it outweighs taken. This ends the stand-in claim of #102 and the AI's raising no army of #152. (#157)
