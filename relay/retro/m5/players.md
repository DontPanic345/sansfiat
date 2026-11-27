# M5 retro: the players

Verdict: a player now meets two decisions, and both are too easy once seen and fatal or
invisible until then. The first is the opening: a civ starts with no army, a rival's first
army can stand on a near capital two to four turns after the pick, and the view's warning
("a rival army that ends its turn there takes it, and the game") is one clause in a long
line. The second is the claim: the Reach line names a claim that brings a clan every turn, and
a player who sends it every turn wins by two to three times the AI's Standing with nobody
contesting the land. Between the two there is still little: the gift reaches nobody on small
2, the store sits full, no pillage order is ever offered, and salt and obsidian rarely reach
a civ's store.

This session played the text view (`sansfiat play`), by hand for the opening and the scout and
by a two-rule script for whole games (`game.md` §2). Nobody played M5's page for the retro: the
owner sent no notes (`owner.md`), and P's played page is M5's only browser play.

Sources: `play.md` and `play/` (the check, small 2 as civ 1, 110 turns); the play steps in the
M5 PRs (L #349, S #360, R #365, C #369, G #375, P #380, X #409, Q #432); `game/play/`; the ideas
issue #351; `game/sweep.txt` for the AI.

## 1. What a player meets

| Turns | What there is to do | Source |
|---|---|---|
| The pick | Three cards: clans, people, ground, the nearest rival capital's distance. The card doesn't say a rival army is coming | `game/play/s3.*` |
| +0 to +4 | Raise, every turn the store allows, or lose. On small 3 as Gold the capital falls at +3 idle; one raise holds it. On small 6 as Red one raise falls at +4; a raise a turn holds (10 warriors by +5) | `game/play/s3*`, `r6.*`, `t6.*` |
| +1 onward | Send the claim the Reach line names. Each one brings a clan the turn it lands (J), and Standing climbs a clan at a time | check: 11 claims, 36.3%; script: 142 claims, 89.5% |
| Scout | Route it to a rival capital and it walks home once for food and builds the route by about +20 (small 61288) | `game/play/scout-61288.*` |
| After the claims run out | End turn. The gift reaches no clan at the market, the store reads "posted 50.0", the army has no target in reach | check, turns 291 to 391 |

What a player never meets: a pillage order (none listed in the check's 110 turns, in R's play
or in the AI's 56.7 orders per 100 turns; M4's AI gave 0.07), an obsidian-armed army of their
own unless a clan of theirs holds a source (G's play), a full salt store (X's play: Gold held
0.0 salt for 100 turns), and a settlement doing anything they can see (S's play: the signpost
drew nobody; fixed since as a reading, #350).

## 2. The play steps, beside their controls

Every M5 play step played small 2, as civ 1 or Blue, about 100 turns, on the piece's branch.

| Piece | Player | AI control | Idle | Beat the AI? |
|---|---|---|---|---|
| L #336 | 28.9% | 32.8% | 30.8% | no |
| S #353 | 4.9% | 9.7% | 4.9% | no |
| R #354 | 29.4% | 42.3% | 29.1% | no |
| C #337 (Blue) | 29.0%, conquest at +63 | 35.7% | 27.1% | no |
| G #355 (Blue) | 28.7% | 36.6% | 27.6% | no |
| P #338 (the page) | 47.4% | 45.9% | 30.1% | by 1.5, inside the floor |
| Q #427 | 32.0% | 37.5% | 28.0% | no |
| The check (main) | 36.3% | 32.7% | 27.1% | by 3.6 |
| The script (main) | 89.5% | 33.8% | 28.2% | by 55.7 |

Six of the eight played sessions lost to the AI on its own civ; the two that won claimed what
the Reach line named. The play step's player gives orders it reads in the legal list, a few
each, and ends turns: it plays like a new player, which is its use, and never found that the
claim is the game. M4's players.md said the gap between the AI and a player was levers not
shown; P's Reach line showed the lever, and it is now worth more than everything the AI does.

## 3. The same findings, every piece

| Finding | Times in M5 | Where it stands |
|---|---|---|
| The gift reaches no clan | L play, R play, P spec-review (#351), the check | Readout fixed (P): "reaches no clan · gave 0.0 · left 2.0". The rule is open. On small 61288 as Red it does reach clans (`game.md` §4) |
| The store idles, or falls with nothing to show | P play (#351), the check | Open |
| No pillage order | R play (#351), the check; M4's check | Pillage is listed only for an army standing on a rival's tile. Open |
| A short scout route shows no result | the check; M4 four times | Open: a one-tile route reads "0 tiles to search" for 110 turns |
| Rivals' joins bury the player's alerts | P build-2 (#351) | Open: J's feed line per clan, about 60 alerts by +85 on small 61288 |
| A claim brings a dozen clans on the first turn | P build-2 (#351) | Open, and now the whole game (`game.md` §2) |

The play step's prompt carries the open ideas, and the gift still came back four times
(`relay.md` §3).

## 4. What the AI does that a player doesn't

Per 100 turns the planner gives 56.7 orders: 30.8 moves, 10.6 claims, 6.9 forages, 6.5
raises, 1.0 gift changes, 0.4 signposts, 0.4 disbands, 0.1 unposts and 0.02 zones. It claims a
tile or two a turn and no further than `PLAN.hold` tiles a person of its armies; it raises at
whichever store is nearest a rival and marches on a capital it outweighs. That is how 6
capitals fall in the sweep. It doesn't claim as fast as the rules allow, so a player who does
outgrows it, and it never takes back a wide, empty claim: on medium 8 the script held 219
tiles with four warriors and lost none in 250 turns.

## 5. The play step

M5's play steps took 1 to 3 minutes and $0.23 to $0.38 each where their PRs give figures, on small 2 every time; M4's retro
proposed varying the map and civ (`relay/retro/m4/players.md`), and only civ varied (Blue for
C and G). A play step that loses to idle-plus-a-bit tells the piece little about its rule. Two
cheap things would make it count: play a map the piece's rule bites on (a settlement near a
front for R, a source in the civ's land for G), and run the scripted Reach player beside it,
so the judge has a led game that plays as well as the view lets it.

## For other angles

- **retro and proposal:** the led game is decided by the claim; the opening is a raise race
  the card doesn't warn of. Both are rule questions for the owner. The page wasn't played for
  the retro; P is the last page play.
- **relay:** the play step still plays small 2 nine times in nine.

## Ideas

| Idea | Source | Value | Cost | Call |
|---|---|---|---|---|
| The pick's card and the first view say which rival army can reach the capital, and in how many turns | §1; `game/play/s3*` | high: a third of the cards stop losing by +6 | small: the view has the Capital line already | take |
| Bound the player's claim as the planner's is bound | §2, §4; `game.md` §2 | high | one rule | take (with `game.md`) |
| The play step plays a map chosen for the piece's rule, not always small 2 | §5; M4 players.md | medium | one line in `play.md` | take |
| A gift that reaches nobody says where it could: the nearest market with clans that aren't yours | §3; the check | medium | small | try |
| Pillage listed for an army one step from a rival's tile, as move-then-pillage | §3 | low until armies go abroad | small | later |
