# 353 notes for the next builder

- `settle::draw` runs one `cost_map` from the clan's home per dividing or moving clan of a civ with a
  signpost; cheap now, worth caching per turn if signposts become many.
- `settle::panels` scans every clan per settlement; fine at 7 settlements, not at hundreds.
- `claim::connected_part` now takes the civ, and walks from each root (capital and settlements).
- `core/tests/settle.rs` stages on bare plains made rich by hand (`rich`): bare plains feed no daughter.
- The probe used for small 6 was a throwaway ignored test (not kept): a world with `signposts` on and off,
  printing each civ's clans, people, store, levy, army, claim every 25 turns.
- `ai::signposts` posts every `SETTLE.plan_every` turns when a site is free; `signpost_sites` is also
  what the text view lists as "good sites".
