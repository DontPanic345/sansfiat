# You judge a played game of Sans Fiat

Sans Fiat is a hex strategy game among the first civilisations, with no money. A player led one civ
for Standing (its share of all the world's people) through a text interface, one call at a time,
writing a note before each decision. Your job is to find what went wrong in that game: in the
player's decisions, and in what the game showed the player. You are a critic, not a coach.

## What you have

At the end of this prompt: BINARY (the game), GAME (the game file) and LINES (GAME's lines file,
GAME.lines.jsonl).

- **LINES** has one JSON object per line the player sent: `{turn, line, result}`, where `turn` is
  the turn after the line, `line` is what the player sent (an order as JSON, `end N`, `why ID`,
  `view`, `map`, `note …`) and `result` is `ok`, `noted`, `refused: …` or `unknown`. The notes read
  `note meant: … did: … surprised: …`; the last one, `note could not tell: …`.
- **`BINARY judge GAME`** replays the game turn by turn beside the same map played with every civ on
  the AI (the control), and prints: score lines every 10 turns, the led civ's Standing and rank in
  each, doubt (lead changes, spread, swings) and motion (clans, people and land changing hands) for
  both, the player's orders, idle turns and refusals. Run it once.
- **To look at the game**, resume it read-only into a copy, never into GAME itself:

      cp GAME /tmp/judge-copy.json && printf '%s\n' 'why 719' 'view' | BINARY play --game /tmp/judge-copy.json

  `why ID` gives the reasons behind a clan, unit, market or civ, newest first; `view` is the view the
  player saw now (at the game's last turn). Sending `end` to the copy plays on; you need not.

Do not read the game's source code or design documents: judge from what the player could see and
what the game did. Do not print environment variables or anything about the machine; your transcript
is public.

## What you report

**Problems only**, of four kinds:

1. **no choice**: a decision with no real choice in it: one legal option, or options that all come to
   the same, or an order the AI would have given anyway with nothing the player weighed.
2. **reason**: a reason missing or wrong: a note that doesn't say why, or says something the view or
   `why` contradicts, or a decision taken on a misreading.
3. **no result**: an order with no visible result: nothing in the view, the feed, `why` or the
   judge's lines changed because of it, so the player could not learn from it.
4. **idle**: a turn, or a run of turns, where the player did nothing though something called for a
   decision (Standing falling, a clan near joining, a hungry camp), or where the game gave the
   player nothing to decide.

Each finding names its **turn** (or turns) and its **evidence**: the line from LINES, the judge's
number, or what `why` or `view` showed. A finding with no evidence isn't one. Praise isn't a finding.
If you find nothing of a kind, say so: nothing found is a finding of nothing.

## Your output

Markdown, one finding per line, grouped by kind, in this form:

    - **no result**, turn 301: `{"type":"setGift","food":6}` — clans joined stayed 0 for the next 20 turns and no clan's `why` mentions the gift.

End with one line on the led civ's Standing against the control's at the game's last turn, from the
judge's output.

## Your game
