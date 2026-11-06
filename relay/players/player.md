# You play a civilisation in Sans Fiat

Sans Fiat is a hex strategy game set among the first civilisations, and it has no money. Nothing is
bought or sold. Your civ is a capital with its store, a scout, the armies and guard you raise, and the
clans that have joined you; the rest of the world is clans living off the land, some aligned with a
rival civ, some with none. Clans pay a levy into your store at your festival, and each turn you give
some of it away as a gift. A clan that is fed and leans your way joins you; one drawn to a rival may
leave.

Your levers, each an order the view lists:

- **The gift**: how much food the store gives at your festival.
- **The land**: the claim you paint out from your capital, and the **guard** you raise to hold it.
  Clans on your land lean your way and pay your levy.
- **Armies**: raised from your clans as warriors, or archers where you hold forest, into a new
  army or one you have. An army moves, spreads over a zone, forages, keeps its line home supplied,
  and pillages a rival's tile. Where two civs' people stand on one tile they fight; the side that
  holds is stronger, a cut-off army starves and surrenders, and a capital with no militia left
  falls. Disbanding sends people home.
- **The scout**: moves, and searches a route from one tile to another, building trails as it goes.

**You play for Standing**: your civ's share of all the people in the world, the people of your clans
and your units over everyone's. It is the first line of the view. Raise it; above all, don't let it
fall while you watch.

## How you play

You play through one command and nothing else:

    BINARY play --game GAME

where BINARY and GAME are the paths your prompt names. Every call is one shell command with
one or more lines piped into it, for example:

    printf '%s\n' 'note meant: look at the camp did: view surprised: nothing yet' 'view' | BINARY play --game GAME

Each call resumes the game from GAME, applies your lines in order, saves, and prints the view: once
when it starts and again after each line it accepts. A shell call is a separate process, so nothing is
kept between calls except GAME. The view lists **legal orders**: copy one, filling in the
placeholder (`<tile>` is a tile number from the list beside it).

If GAME does not exist yet, start it once with

    BINARY play --game GAME --seed SEED --size SIZE --civ CIV

using the seed, size and civ your prompt names (CIV is the civ's place in the list of cards,
1 for the first). That puts you at the pick, the turn your
civ becomes yours. If GAME exists, the game is already started: resume it, never start it again.

### The lines

- an order, as JSON copied from the legal orders, e.g. `{"type":"setGift","food":4}`;
- `end N`: end N turns (`end` alone ends one);
- `why ID`: the reasons behind a clan, unit, market or civ, newest first;
- `view`: the view again; `map`: the map as text, with your places marked;
- `note TEXT`: a note kept in the game's record, which is read after the game.

A line the game refuses prints `refused: …` and changes nothing; read why and try something else.

## What you must do

1. **Before each decision** (each order, and each time you choose how many turns to end), write a
   note line first, in this form, in the same call:

       note meant: <what you are trying to achieve> did: <what you are about to do and why> surprised: <what in the view you did not expect, or "nothing">

   Then the order or `end N` line. A reader judges your play from these notes alone, so say what
   you saw that made you choose.
2. **Play 100 turns from the pick.** The view's first line says how many turns since the pick. Where
   nothing calls for a decision, use `end N` with a larger N (5 or 10) rather than one turn at a
   time; where something changed (a clan close to joining, a hungry army, Standing falling), look,
   decide, and end fewer turns. Look at `why` when something surprises you.
3. **When the view says 100 or more turns since the pick, stop**, and write one last note:

       note could not tell: <lever>: <what you needed to know to use it well that the view did not show you>

   one line per lever you tried.

   Then reply with a short report: the turn you reached, your Standing at the pick and at the end,
   your three most important decisions, and what you could not tell.

## Three habits

- When a number stays at zero after an order meant to move it, look at `why` on the order's
  subject before your next order, and say in your note what it told you.
- Try every family of lever at least once: the gift, the land and the guard, an army (raise, move,
  zone, forage or supply, pillage, disband), the scout.
- Your last note says what you could not tell for each lever you tried, one `could not tell:` line
  per lever.

## Rules

- Use only `BINARY play` (and `cat` or `ls` of GAME's directory if you need to). Do not read the
  game's source code, its design documents or anything else in the repository: you play from what a
  player sees.
- Do not print environment variables or anything about the machine. Your transcript is public.
- You have about **45 minutes**. If time runs short, end more turns per call; reaching +100 matters
  more than any one turn.
