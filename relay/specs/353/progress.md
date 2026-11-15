# 353 Signposts and settlements: progress

## Plan and state

1. Clay: `settle.rs` (signposts, draw, found, take, upkeep, panels), the claim's roots (`claim.rs`), the
   levy at a settlement's market (`civs::levy`), the orders `signpost` / `unpost`. DONE, tested.
2. Kiln: a daughter (`dividing::split`) and a clan moving on (`clans::move_on`) draw a signpost first;
   the planner posts and removes (`ai::signposts`, `signpost_sites`). DONE, tested.
3. The judge's `settle` group (`judge_settle.rs`) and the sweep's no-signposts control
   (`cli/src/sweep_settle.rs`). DONE.
4. Stage tests `core/tests/settle.rs` (8), save version 22 and its pins. DONE.
5. The text view and `play`: signpost and settlement lines, the two legal orders with good sites, map
   marks `S` and `P`. DONE (not yet played by a player: the crew's play step does).
6. The page: layer, settlement panel, Signpost… and Remove signpost buttons, play script. NOT DONE:
   `build-1.md` hands it on. The wasm view already carries `signposts` and `settlements`.
7. Sweep against main: `scores.md`, `branch-sweep.json`, `main-sweep.json`, `compare.txt`. DONE.
8. Glaze: `SETTLE.draw` and `SETTLE.walk` are first numbers, not tuned. OPEN: one sweep was run.
9. Design ledger lines: left to the review (the decisions are in the result).
10. PR: opened by this step; the verdict is the review's.

## Open, plainly

- Settlements cost their civ Standing as built (scores.md): the store takes levy and feeds nothing.
- The planner finds few sites (10 signposts over 21 civs).
- The open idea "'gave 0.0' stays zero" was not taken: it is outside this piece's goal.
