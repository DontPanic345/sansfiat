# The trial: one game played and judged by Sonnet

**Map** small seed 2 (pick at turn 283). **Civ** Red (civ 716, the first card, `--civ 1`): 12 clans,
79.1 people, 10.6% Standing, rank 1. Red because on this map the AI loses Red's lead: with every civ
on the AI, Red falls to 8.8% at +100 while Blue climbs from 7.5% to 17.6% (`probe.md`). So the control
is a clear yardstick: a player who just ends turns lands near 8.9%, and holding the lead takes play.

Files: `trial/g.json` (the game), `trial/g.lines.jsonl` (every line the player sent),
`trial/judge-findings.md` (the judge's report as it returned it),
`trial/judge.txt` (`sansfiat judge trial/g.json`). The prompts are `relay/players/player.md` and
`relay/players/judge.md`, passed verbatim with the paths appended.

## Cost

Both ran as Agent subagents (`general-purpose`, `model: "sonnet"`; the judge at `effort: "medium"`).
The Agent tool's own figures are below. Its token count looks like the context's size at the end, not
what was billed, so the billed tokens come from the subagents' transcripts, summed over API responses.
Dollars are an **estimate** at Sonnet 5.5's list price: $2 per million input tokens, $10 output,
$0.20 cache read, $2.50 cache write (1.25 × input).

| Session | Agent tool: tokens, tool calls, minutes | Billed tokens (in / cache write / cache read / out) | Est. list $ |
|---|---|---|---|
| Player, attempt 1 | 14,266, 3, 0.4 | not kept | under 0.04 |
| Player, attempt 2 | not given (it ended on an API error; see below) | 24 / 18,619 / 227,511 / 3,188 over 13 responses, 12 tool calls, 3.9 min | 0.12 |
| Player, resumed for its last note | 26,770, 2, 0.2 | in the line above | in the line above |
| Judge | 22,024, 7, 2.2 | 12 / 13,463 / 81,014 / 2,568 over 6 responses | 0.08 |

The plan points are this step's own, which the crew records.

What happened:

- **Attempt 1** stopped at once: the prompt gave `--civ 716` (the civ's id) where `play` takes the
  card's place, 1 to 3. The player refused to guess and reported it. `player.md` now says CIV is the
  card's place.
- **Attempt 2** played to turn 391 (+108) in under four minutes. Then an API safeguard error ended the
  session before its last note: a false positive (`reasoning_extraction`) on an ordinary turn. I resumed
  it once with SendMessage, telling it the game was at +108, and it wrote the `could not tell` note and
  its report.

## The game

**Turns reached:** 391, which is 108 past the pick. It ran past +100 because its last `end 10`
started at +98.

**Decisions** (6 orders, 19 notes, 0 refusals, 2 `why`/`map` looks):

| Turn | Order | The player's note (meant / did / surprised) |
|---|---|---|
| 283 | forage 719 | start feeding clans / forage and gift 4 / store is empty at the pick |
| 283 | setGift 4 | gift food to nearby clans |
| 288 | setGift 12 | actually give food, gave 0 so far / store grows but gave 0.0 |
| 323 | setGift 30 | store is idle at 47, spend on clans / Blue passed me |
| 333 | move 719 to 194 (the AI's post) | reach the unaligned clan, after `why 321` and `map` / gifts give nothing from the capital |
| 341 | move 719 to 166 | stop the pack draining / gifts still 0 |

Between 293 and 323, and again from 341 to 391, it ended turns in 10s with the note "keep feeding" or
"reach +100 while watching Standing".

**`sansfiat judge`'s score lines** (`trial/judge.txt`):

    turn 283 (+0): civ 716 10.6%, civ 717 7.5%, civ 718 5.1%; leader civ 716
    turn 303 (+20): civ 716 9.6%, civ 717 10.4%, civ 718 5.8%; leader civ 717
    turn 343 (+60): civ 716 10.0%, civ 717 13.5%, civ 718 7.8%; leader civ 717
    turn 363 (+80): civ 716 10.8%, civ 717 15.0%, civ 718 8.7%; leader civ 717
    turn 383 (+100): civ 716 9.8%, civ 717 14.4%, civ 718 11.1%; leader civ 717
    led civ 716: Standing at +108 10.0% (rank 2) in the game, 8.9% (rank 2) with every civ on the AI
    doubt (game / control): lead changes 1 / 1, spread +108 0.052 / 0.133, swings 4 / 1
    motion (game / control): clans changed 25 / 25, people 123.2 / 117.9, land tiles 404 / 337
    player orders: 6 (forage 1, move 2, setGift 3), 5.56 per 100 turns; turns with none 103 of 108, longest run 49

**Led civ against the control:** Red ends on 10.0% against the control's 8.9%, rank 2 in both. The
player beat the AI by 1.1 points and still lost the lead to Blue by turn 303. Blue gains less in the
game (14.9%) than in the control (21% at +200, 17.6% at +100), so the spread halves (0.052 against 0.133).

## The judge's findings

Condensed from the judge's report; each finding names its turn and evidence:

- **no choice**, 283: `forage 719`, with no other order weighed. Turns 283–323: five of the six
  orders in the first 40 turns, then one policy ("keep feeding").
- **reason**, 288 and 323: `setGift 12`, then `setGift 30` "because gave 0.0". The judge calls the
  first a misreading (the feed shows "291 gave food to 1 unaligned clan", and one clan joined at 289).
  It is half right: after 291, gave really was 0 for 100 turns (see below).
- **reason**, 333: the move to the AI's post on a guess ("gifts give nothing from the capital"),
  undone at 341.
- **reason**, 351–381: the note "reach +100 while watching Standing" is about the turn counter, not
  a reason.
- **reason**, 391: the player's `could not tell` note. The view gives no reason a gift is skipped, and
  the leans don't say what raises them.
- **no result**, 283, 323, 333, 341: `setGift 4`, `setGift 30` and both moves. Nothing visible
  followed: "last turn gave 0.0", and no lean moved.
- **idle**, 341–391: 50 turns with no order (the judge's longest run is 49). Standing fell from 10.8%
  to 10.0%, the feed showed "people leave Red's clans" (367–372) and "Clan 61 is starving" (380), and
  no decision followed. Also 293–323, while Blue passed Red.
- **idle**: the game gave little to decide. Only setGift points at Standing, only one unaligned clan
  is in reach, and `raise` was never tried.
- One misreading of the judge's own: it says `why 321` printed "only the standard view". The reasons
  were there, after the view the call prints at its start. `play` now heads them
  `why ID: its reasons, newest first`.

## What the player couldn't tell

> why gifts went to 0 (gave 0.0 from turn 291 on despite a 47 store and gift set to 30), which clans a
> gift reaches, why my clans starved short of shellfish, and what raises the lean of unaligned clans

The first two are one gap in the easel, and it is real. A gift goes only to unaligned clans that
**trade at your market** (`markets::trade`). The view listed clan 321 as "in reach of your market",
but 321 traded at market 696, so no gift ever reached it. The view now says, beside each clan in
reach, whether it trades at your market and so whether your gift reaches it, and its header says a
gift reaches only those (`cli/src/view.rs`). At Red's +108:

    unaligned clans in reach of your market, leaning to each civ (join at 1.0; a gift reaches only those that trade there):
      clan 321 at 249 (-5, -4), 6.6 people: civ 716 0.02, civ 717 0.00, civ 718 0.00; trades at market 696, so your gift doesn't reach it

The other two (why clans starve, what raises a lean) are in `why` on the clan, which the player used
once and read only in part.

## The answer

Yes, the game can judge itself, and cheaply: a Sonnet player reached +108 on small seed 2 in under
four minutes and about $0.12 of tokens at list price. `sansfiat judge` put its Red at 10.0% against the
AI's 8.9%, and a Sonnet judge at about $0.08 returned turn-stamped findings in all four kinds. What the
trial found is mostly about the game, not the player. After one join at turn 289 a gift had no one to
reach, and the view didn't say why. So 103 of 108 turns had no order, and the one lever that points at
Standing went dead with no sign. That is the kind of finding the piece exists to surface.
