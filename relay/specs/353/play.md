# Play note, #353 (small, seed 2, civ 3 Gold, 100 turns)

Judge output: `play/small-2.judge.txt`. Lines: `play/small-2.json.lines.jsonl` (or `.lines.jsonl` beside the game).

**Standing at +100:** 4.9% played (rank 3) against 9.7% with every civ on the AI, and 4.9% idle led civ. The led play equals the idle control. Started at 7.4%.

## Findings
- **fix**, turns 262-362: the signpost at 235 drew no clan in 100 turns and no settlement was founded. The view said "would feed a daughter 106%" at turn 262 and "0%" from turn 277 on, with no site listed after. Breaks the piece's Goal (clans weigh a signpost as a site). Repro: `small-2.json.lines.jsonl`, turns 262-362; `why 426` shows only the post. The 106% to 0% swing is unexplained (the neighbouring unaligned clans or rival claims may have taken the tile); needs a look at why the weight is zero.
- **ask**, turns 262-362: 'gave 0.0' stays zero with gift 6 then 15 and 55 posted a turn. Already open (#336 idea), not repeated in result.json.
- **ask**, turn 277: a claim naming tiles beyond the claim is refused with "must be joined" and no tile named; the player can't tell which tile is the unjoined one.
- **ask**, turn 277-362: standing fell from 7.4% to 4.9% with nothing a lever could do about it; gift, claim of one tile, a lone army, and the scout moved Standing by under 1 point. The view doesn't say what raises it.
- **no result**: `route` for scout 429 and `move` of army 481 left no line in the view; `view` listed "no route" for the scout throughout.
- **idle**: 95 of 100 turns with no order; nothing in the view called for one beyond the falling Standing.
