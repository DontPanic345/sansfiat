# M2 retro: history (the real economics the game draws on)

## Verdict

Sans Fiat's actor model (labour divided over a territory, every route a walk there and back,
festivals clearing without money) already matches how pre-money economies worked: most goods
moved by gift, reciprocity and feast, not by price. The next milestones should add **goods that
differ in weight, durability and where they come from**, because that is what made real
exchange worth the walk. The strongest topics for a no-money game are obsidian, salt,
herding with pack animals, farming with the home-versus-market choice, and sieges. Shells and
beads are fine under vision rule 2 if they are an ornament good with their own demand and
nothing in the engine treats them as a unit of account. Paper is outside the era; its
sans-fiat stand-in is the clay token (see below).

Sourcing: obsidian, beer and bead/gift-exchange claims below were checked by web search this
session (links in the topics). Everything else is from my memory of the standard literature,
named by author and year; I did not re-verify those this session, so check a title before
quoting it.

Design context read: `docs/design/vision.md`, `actors.md`, `clans.md`, `supply.md`,
`docs/notes/*.md`, `docs/proposals/m2.md` section 4 "Later". Already planned there: farmer
actor, steward, secondary resources, traders between festivals, paid army. Already out of
scope in `supply.md`: porters, caches, depots, roads, pack animals, fighting. Festivals today
trade "people, and later ideas and small goods such as obsidian" (`clans.md`).

## Topics

### Obsidian
A volcanic glass, the best cutting edge before metal, found at few sources (central Anatolia,
the Aegean, the Caucasus). In the early Neolithic Anatolian obsidian reached the Jordan Valley,
over 1,000 km away. Renfrew's "law of monotonic decrement" (1977) says the share of obsidian
in a site's stone falls steadily with distance from the source; he read steep or flat curves
as down-the-line exchange, free-lance trade or centres redistributing. Later agent-based work
(Ortega et al. 2014, J. Archaeological Method and Theory) found pure down-the-line passing
cannot reach past about 300 km; a few well-connected villages with long links explain the
long reach. The fall-off curve alone is contested as evidence of the exchange mode. Moved by:
gift and reciprocity between neighbours, plus journeys by a few. Cost: the quarry trip and
the carry, but it is light and does not spoil, so value per walked tile is high.
Sources: [Ortega et al., PMC4590507](https://pmc.ncbi.nlm.nih.gov/articles/PMC4590507),
[Ortega et al., MDPI Systems 4(2):18](https://www.mdpi.com/2079-8954/4/2/18),
[HAL: modelling obsidian exchange](https://hal.archives-ouvertes.fr/hal-02192853).

Mechanics for Sans Fiat:
- **A fixed-source good.** Obsidian exists only on a few tiles; a clan holding one can post it
  at its festival, others cannot make it. The player decides which obsidian tile to hold or
  reach, and whether a long trail to it is worth the labour.
- **Hand-me-on trade.** A clan that got obsidian at one festival may re-post it at the next.
  Decision: whether to be the clan that carries it onward, since a clan that walks to two
  festivals pays two routes.
- Emergent check, not a rule: the share of obsidian in clans' stores should fall with route
  cost from the source (Renfrew's curve). That is a free measure for the players/judges angle.

### Beer
Cereal brewing is old: starch and phytolith residue in 13,000-year-old Natufian stone mortars
at Raqefet Cave, Israel, tied to burial feasts (Liu et al. 2018, J. Archaeological Science:
Reports). Beer at Göbekli Tepe (Dietrich et al. 2012, Antiquity) is argued from large stone
vats but is not certain. The "beer before bread" idea, that feasting drove grain taming, is
debated. What is firmer: beer turns a bulky, spoiling grain surplus into something that stores
a little, travels as a liquid, and makes people come to a feast and work for the host (later
Mesopotamian texts pay labourers in beer rations). Moved by: feast hospitality and labour
payment, i.e. gift and reciprocal obligation, not sale.
Sources: [Raqefet Cave brewing, Haifa](https://cris.haifa.ac.il/en/publications/fermented-beverage-and-food-storage-in-13000y-old-stone-mortars-a/),
[Stanford summary](https://news.stanford.edu/?p=23025),
[Göbekli Tepe caution](https://novoscriptorium.com/2020/06/27/feasting-in-gobekli-tepe-alcoholic-beverages-in-the-ppna/).

Mechanics:
- **A processed good.** Grain plus labour becomes beer; beer spoils slower than grain in the
  store and weighs less per unit of attraction. Decision: spend grain on the festival's
  gathering (beer raises the "desire to gather" bonus in `clans.md`) or keep it as seed and
  food.
- **Beer as wage.** The player can feed workers on a great work with beer instead of grain.
  This ties to the "Great work" win option in `vision.md`: the labour has to be fed and
  drawn from clans, so the player decides how much surplus to brew to get people to come.

### Economy (the exchange mix itself)
Polanyi (1957) separated reciprocity, redistribution and market exchange; pre-money societies
ran mostly on the first two. Graeber (Debt, 2011) argues that credit and obligation came
before barter, and that pure barter between strangers was rare. Humphrey and Hugh-Jones
(Barter, Exchange and Value, 1992) show barter is mostly between people in an ongoing
relationship. Moved by: all four ways (gift, barter, tribute, credit).

Mechanics:
- The festival clearing algorithm is the market-exchange case. Add **a standing partnership**
  (the kinship idea in `Pre-game Economy.md` open questions): two clans that traded before
  clear with each other at better terms. Decision: keep trading with a known partner who walks
  a long way, or take a new partner nearer.
- **Redistribution.** A civ's capital collects and hands out; the steward in `m2.md` section 4
  is this. Decision: how much to collect, since it costs the walk and the clans' own surplus.

### Beads
Ornaments of ostrich eggshell, shell, bone and stone, from at least tens of thousands of
years ago. In the Kalahari, hxaro is a gift-exchange system: partners are chosen for being
ecologically elsewhere, and about a third of a Ju/'hoãnsi's partners live over 50 km away
(Wiessner 1982, 1994). Strontium analysis of eggshell beads shows networks linking different
ecological zones around 33,000 years ago. The exchange insures against a local bad year:
you hold a claim on a partner's land. Moved by: delayed gift exchange, which is credit.
Sources: [Ostrich eggshell beads, Wikipedia (weaker source)](https://en.wikipedia.org/wiki/Ostrich_eggshell_beads),
[Ga-Mohana Hill beads, PMC9159631](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC9159631/),
[MPI-SHH on Kalahari beads](https://shh.mpg.de/2182268/eggshell-beads-kalahari).

Mechanics:
- **Bead-making as use of spare labour.** A clan with spare labour and a source of shell or
  eggshell makes beads, which are light, do not spoil and are wanted for display.
- **Partnership as insurance.** A partner far away is valuable because its bad years do not
  match yours. Decision: choose partners across terrain types, at a long walk, rather than
  the nearest clan. (Needs good and bad years, which `clans.md` marks out of scope.)

### Shells
Marine shells reached far inland: Spondylus ornaments from the Aegean/Black Sea went up the
Danube into central Europe in the Neolithic (Séfériadès and others; I could not confirm
this by search this session); cowries and others circulated as ornaments and later as
currency (Hogendorn and Johnson, The Shell Money of the Slave Trade, 1986, covers the later
case). Shells are light, hard, do not spoil, cannot be counterfeited by a landlubber, and
have a known source: all the traits of a proto-money. Yet in the Neolithic they mostly moved
as gifts and grave goods, not as payment.

Mechanics:
- **A coast-only good**, like obsidian but from the other side of the map. Decision: send a
  clan to the coast to make or fetch shells, or buy them off the clans that live there at a
  festival.
- See "Does the game want proto-money?" below.

### Salt
Needed by animals and by people eating mostly grain and meat; also a preservative. Salt is
made at brine springs and coasts by boiling or evaporating in pottery, which takes fuel and
labour. Evidence for Neolithic salt-boiling at Romanian springs (Weller, Alexianu and others)
and wider surveys (Nenquin 1961; Harding, Salt on the Move, 2013). I recall these from
memory and did not re-check them. Moved by: regular exchange along set routes from the few
production sites, since everyone needs it and few can make it.

Mechanics:
- **A need, not a luxury.** Add salt to the diet or the store: food in a store with salt
  spoils slower than 30% a turn (`Pre-game Economy.md`). Clans without salt on their land
  want it. Decision: send people to boil it, or send a route to the people who do.
- Makes a **steady, one-sided festival flow**: the salt clans always sell, the rest always
  buy. That is a trade pattern the market algorithm can show without any rule.

### Paper
Paper proper is Chinese, around 100 CE, long after the first civilisations; papyrus came
about 3000 BCE in Egypt (the oldest papyri found are about 2500 BCE, Tallet, Wadi al-Jarf).
In the era Sans Fiat targets, the record-keeper is the **clay token**: Schmandt-Besserat
(Before Writing, 1992) argued small clay tokens stood for counted goods from about 8000 BCE and
led to the sealed envelope and then writing. Her wider chain to writing is disputed, but the
tokens and accounts are real. Moved by: tribute and redistribution; records exist so
temples and palaces can say who owes what. Scott (Against the Grain, 2017) points out the first
written records are grain accounts of states.

Mechanics:
- Treat "paper" as **records**: a ledger that lets a steward collect from a wider area than he
  can walk. Without records, the steward's reach is limited to land he can inspect on foot.
  Decision: pay labour for scribes to widen what the civ can levy, or stay small and personal.
- This is the one topic to leave out of the first milestones; it needs a steward and stored
  surplus to mean anything.

### Weaving
Flax fibres twisted and dyed at Dzudzuana Cave, Georgia, about 30,000 years ago (Kvavadze et
al. 2009, Science); spinning and weaving in the early Neolithic with flax; wool arrives with
woolly sheep and the secondary-products shift (Sherratt 1981; Barber, Prehistoric Textiles,
1991). Cloth takes huge labour (spinning dominates) and lasts, so it stores value. Moved by:
household work, then tribute and trade; textiles became a main export of Mesopotamian cities.

Mechanics:
- **A craft good with a long labour chain.** Fibre (flax from fields, or wool from herds),
  then spinning labour, then cloth. Decision: where to put spare labour, since spinning can be
  done at home between other work.
- Needs fields or herds first.

### Settling down to farming
Sedentism came before agriculture in the Levant: Natufians lived in villages from about 14,500
BCE on wild cereals (Bar-Yosef). Domestication was slow, a thousand years or more (Fuller et
al.), which `clans.md` already models with the 120-turn taming. Scott (2017) argues early
farmers were not clearly better off: more work, more disease, more storage to defend. Garden
farming on small plots, worked intensively by hand, was common (Bogaard 2005). Moved by:
within a household or village, mostly not traded.

Mechanics (already in `actors.md` "The farmer"):
- A farmer's territory is its fields, small and intensive, and does not deplete. The decision
  for the player: is a fixed farming village worth the risk of holding a store that rivals and
  raiders can see?
- **A harvest peak.** Farming concentrates labour in a season, while foraging spreads it. If
  turn length is a year, this only shows up as storage and spoilage; keep that in mind.

### Irrigation
Small canals and flood-recession fields came before large systems; large works, as in
Mesopotamia, grew with the cities. Wittfogel's idea that irrigation forced the state (1957)
is largely rejected: Adams (Heartland of Cities, 1981) found canals grew from local
decisions. Cost: canal digging and upkeep labour, and the fight over who gets the water
downstream.

Mechanics:
- **A tile improvement bought with labour, kept up with labour.** Raises a tile's grain cap
  (the game already says "irrigation tames grain further"). Decision: spend labour now that
  could harvest, for a bigger yield later. If the canal is not maintained, it silts up.
- Because water runs downhill, a canal upstream hurts clans below: a natural border dispute.

### Land use
Boserup (1965) argued that land goes from long fallow to annual cropping when population
presses, with more labour per unit of food. Von Thünen (1826) showed that, around a market,
bulky, perishable goods are grown closest and the rest further out, because transport cost
decides what pays. This is exactly the game's "each tile's share pays the walk first".

Mechanics:
- Land use choice per tile: forage, farm, fallow, graze. The same labour rule decides how
  far out each use goes. Decision: how much of the near ring to put to crops against
  grazing, which the player influences through what the festival wants, not by painting.
- Gives a measure for the players angle: concentric rings around festivals should appear.

### Pack animals
Donkey domesticated in northeast Africa, about 5,000 BCE or later (Rossel et al. 2008, Kimura
et al. 2011, from memory); cattle, sheep and goats from about 8000 BCE. The wheel and wagon
are about 3500 BCE. Before animals, people carried everything; an animal carries several
times a person's load and eats grazing. Moved by: owners; a pack animal is capital, not
traded often.

Mechanics (`supply.md` lists pack animals out of scope):
- **Route cost drops for a clan with animals**, for goods, not for the people. A herd must
  be fed from the territory it passes. Decision: keep animals (labour and grazing) to cut the
  walk to the festival and the supply line.
- Armies with pack animals lengthen their supply line: a direct, countable payoff.

### Herding
Sheep, goat, cattle and pig domestication in the Fertile Crescent, 10,500 to 8,000 BCE.
The **secondary products revolution** (Sherratt 1981): milk, wool, traction came later;
dairy fats found in pots from about 6500 BCE (Evershed et al. 2008). Mobile pastoralism
developed alongside farming and was not a stage before it (Khazanov, Nomads and the Outside
World, 1984): herders depend on farmers for grain and trade with them, and sometimes raid.
Moved by: bride-wealth, gift, tribute, and exchange with farmers. A herd is a store of wealth
that walks.

Mechanics:
- **A herd is a store that grows and walks.** It does not spoil, breeds, and needs grazing
  and a herder's labour. Game already breeds in the engine; a tamed herd is the same rule
  with a keeper. Decision: put labour into herding (fewer tiles to work, a wider range) or
  into fields.
- A herding clan's territory is large and low-yield; it trades milk, wool and animals for grain.
  Tension with farmers over where the herd walks.

### Working the land from home vs living near the market
Von Thünen: distance to the market sorts land use. Adams and Algaze (The Uruk World System,
1993) describe Mesopotamian farmers living in or near cities and walking out to fields;
Scott describes earlier villages scattered among the fields. Both patterns existed; the
trade-off is the daily walk to the field against the periodic walk to market.

Mechanics (the cleanest fit for the actor model):
- A farmer can **commute out to the fields from the market**, or **live at the fields and walk
  to the market**. Both are a route paid there and back; the first is paid daily with little
  to carry, the second is paid occasionally with a load. The game's rule for adding a tile
  already decides, but the *home tile* choice is new. Decision for the player and the
  clans: where a clan's home sits relative to its fields and its festival.
- At scale, this is what grows towns: a market that many clans prefer to live beside.

### Fighting, sieges
Violence is old (Talheim, Germany, about 5000 BCE, a massacre; Keeley, War Before
Civilization, 1996) but organised war needs something worth taking. Jericho's wall and tower
(about 8000 BCE; Kenyon; Bar-Yosef argued it was for flood and mud, which is debated); Tell
Brak and Hamoukar show an attack on a town about 3500 BCE (Ur et al. 2007). A siege works by
cutting supply or the field work, because a walled town can only be starved or breached. Moved
by: raid and tribute; the defender pays with walls and stores.

Mechanics (consistent with `supply.md`: the supply line, pillage):
- **A siege is a supply line cut.** An army holding the ground round a walled centre stops its
  clans working their fields; the centre eats its store. The defender's decision: how much
  labour into walls and stored grain; the attacker's: how long the army's own line holds.
- Fighting itself stays as small as `Captain and Territory Mechanic.md`: contested tiles,
  entrenchment and retreat. Walls raise entrenchment for the defender's tile.

## Does the game want shells and beads (proto-money)?

Vision rule 2 allows it: if something does the job of money, an actor produces it and its
value comes from what other actors give for it. So shells and beads **fit** if:
1. They are made by actors at a labour cost from a located source.
2. They have a use of their own: display, gift, grave goods. That demand sets their ratios in
   the festival clearing.
3. Nothing in the engine treats them as a numeraire, a price, a tax unit or "the player's
   purse". A player cannot hold a pile and spend it at will.

They **break** "sans fiat" if the game starts to read them as a unit of account: levies in
beads, wages in beads, the army paid in beads, or a market quote in beads. Note the
exchange algorithm is already numeraire-free (`Pre-game Economy.md`: ratios are relative,
nobody holds them). Good and bad years are needed to make beads a store of value; without
them there is no reason to hold them.

Recommendation: beads and shells as a good, not a currency, and watch rather than design.
Measure whether ratios against beads become more stable and whether clans hold more than
they use. If that emerges, it is the thing the game is about: it arose, it was not given.
Beer and salt are the safer economy bets, since they have a real need behind them.

## Order of eras (what must come before what)

| Step | Topics | Playable step it makes | Needs |
|---|---|---|---|
| 1. Things worth the walk | obsidian, salt, shells, beads | Festivals carry goods other than food and people; fixed sources; clans specialise by what lies near; trail-making pays | M2 only (festivals, routes, trails) |
| 2. Settling | settling down to farming, land use, home vs market | The farmer actor, fallow and fields, the home-tile choice; harvest piles up in a store | step 1 not strictly, but wants salt for storing |
| 3. Surplus into more | beer, weaving (flax), economy of partnership/redistribution | Processed goods; feasts; steward collects and gives | step 2 (a grain surplus) |
| 4. Animals | herding, pack animals | A walking store; cheaper routes and longer supply lines; pastoral clans | step 2; `supply.md` out-of-scope list lifted |
| 5. Water and walls | irrigation, sieges, wool weaving | Improvements bought with labour; defence of a settled centre; the steward's records ("paper") | steps 2 to 4 |

Fighting in its small form (contested tiles) already works from M2. Sieges need something
worth besieging: a store, fields, a centre, so after step 2.

## Strongest for a game with no money
1. **Salt**: a real need, few sources, forces trade without any luxury story.
2. **Obsidian**: shows a distance fall-off for free and gives clans a reason to walk.
3. **Home vs market and land use**: pure labour-and-walk mechanics already in the model.
4. **Herding and pack animals**: a store that walks; lowers route cost, a direct payoff.
5. **Sieges as supply cut**: reuses `supply.md`.
Beads and shells: strong but the risky one; take as a good, not currency. Paper: leave until
a steward exists. Weaving: strong but late, labour chain is long.

## For other angles
- game: M2's "Later" list (`m2.md` section 4) names farming, steward, traders and secondary
  resources; this file's order gives them a sequence.
- players/judges: Renfrew's fall-off, von Thunen rings and bead-ratio stability are
  automatic checks needing no judge.
- briefs: step 1 is small enough for a single piece; steps 3 and 5 are likely two pieces each.

## Ideas
| Idea | Value | Cost | Recommendation |
|---|---|---|---|
| Fixed-source goods (obsidian, shells, salt) posted at festivals | high | medium | take |
| Salt as a storage need (slows spoilage) | high | small | take |
| Standing partnerships between clans (better terms for repeat traders) | medium | medium | try |
| Beads/shells as an ornament good, never a unit of account | medium | small | try |
| Renfrew fall-off and von Thunen ring as automatic checks | medium | small | take |
| Farmer's home-tile choice (field vs festival) | high | medium | take |
| Herds as a walking store; pack animals cut route cost | high | large | later |
| Beer as processed grain raising desire to gather | medium | small | try |
| Siege as a supply-line cut with walls raising entrenchment | medium | medium | later |
| Irrigation as a maintained tile improvement | medium | medium | later |
| Records/"paper" widening a steward's reach | low | medium | later |
| Weaving as a long labour chain | low | large | later |
| Good and bad years, to make credit and partnership matter | high | medium | take (before beads) |
