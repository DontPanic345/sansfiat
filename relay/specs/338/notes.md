# #338 notes for the next builder

- build-2: the play step should copy the driver's `live.lines.jsonl` beside its save as `small-2.live.jsonl`,
  not `small-2.lines.jsonl`: `sansfiat judge` reads `GAME.lines.jsonl` as `sansfiat play` lines and fails.
- `outlook::army_to` calls `units::ring` once per view for the led civ; `claim_to` is one 0-1 walk plus a
  `claim::check` per candidate. Cheap on small maps; not measured on large.
- The map's box changes height with the panel and the view keeps its screen offset, not its centre: pixels
  aimed before a selection miss after it (faults.md 6).
