# You play a civilisation in Sans Fiat

Sans Fiat is a hex strategy game set among the first civilisations, and it has no money. Nothing is
bought or sold. Your civ is a capital, a camp of warriors, a scout and the clans that have joined you;
the rest of the world is clans living off the land, some aligned with a rival civ, some with none.
Clans join a civ that feeds them: your camp forages, its food goes into your store, and each turn you
give some of it away as a gift. A clan that is fed and leans your way joins you.

**You play for Standing**: your civ's share of all the people in the world, the people of your clans
and your units over everyone's. It is the first line of the view. Raise it; above all, don't let it
fall while you watch.

## How you play

You play through one command and nothing else:

    BINARY play --game GAME

where BINARY and GAME are the paths at the end of this prompt. Every call is one shell command with
one or more lines piped into it, for example:

    printf '%s\n' 'note meant: look at the camp did: view surprised: nothing yet' 'view' | BINARY play --game GAME

Each call resumes the game from GAME, applies your lines in order, saves, and prints the view: once
when it starts and again after each line it accepts. A shell call is a separate process, so nothing is
kept between calls except GAME. The view lists **legal orders**: copy one, filling in the
placeholder (`<tile>` is a tile number from the list beside it).

If GAME does not exist yet, start it once with

    BINARY play --game GAME --seed SEED --size SIZE --civ CIV

using the seed, size and civ at the end of this prompt (CIV is the civ's place in the list of cards,
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
   time; where something changed (a clan close to joining, a hungry camp, Standing falling), look,
   decide, and end fewer turns. Look at `why` when something surprises you.
3. **When the view says 100 or more turns since the pick, stop**, and write one last note:

       note could not tell: <what you needed to know to play well that the view did not show you>

   Then reply with a short report: the turn you reached, your Standing at the pick and at the end,
   your three most important decisions, and what you could not tell.

## Rules

- Use only `BINARY play` (and `cat` or `ls` of GAME's directory if you need to). Do not read the
  game's source code, its design documents or anything else in the repository: you play from what a
  player sees.
- Do not print environment variables or anything about the machine. Your transcript is public.
- You have about **45 minutes**. If time runs short, end more turns per call; reaching +100 matters
  more than any one turn.
