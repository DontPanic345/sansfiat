# #352 notes for the next builder

- `stats::standing_parts` still splits Standing into clans and land. The land part is the unaligned
  clans on a civ's land, which now exist only between a home moving and the next `lean` step, so it
  reads 0 in every sweep. The judge's `on_land_200` and `land_200` and the page's `standing_land`
  can go when someone next touches Standing.
- `leaning::land` runs after every claim change: the `lean` step, a claim order, the `claim` step
  and founding. A new way to change a claim (S's settlements) calls it too.
- `www/play/` has no script that asserts the clan panel's Civ and Levy rows or the clans' fill by
  civ beyond `towns.mjs`; `check-stage.mjs` only checks the clan shape is well formed.
