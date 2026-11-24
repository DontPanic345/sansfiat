# #356 build-3 (kiln, page): what the page shows

- `node www/play/exchange.mjs`: passes. Fill salt on the staged army at the capital: the pack's salt 0 → 4, the
  store's 8 → 4, the pack's lot dug at the salt source; Fill salt then greys with "its pack's salt is full". The
  store's Obsidian row picks: `body.dataset.good` is `13:obsidian`, `game.map.flow` has one chain, its first
  point an obsidian source, its last the store; the flow layer draws dark pixels round its midpoint (790 more
  with it on). A second click clears it; the Salt row picks salt's chains, which start on the salt source;
  Escape clears them.
- `node www/play/buttons.mjs`: passes, 35 cases. Fill salt is listed on the camp (all three states) and on the
  scout; off a store it is greyed and its tooltip says why.
- `flow.png`: a generated small map (`?size=small&seed=2`), the led civ Red at turn 288, its capital's store with
  obsidian picked. Four lots, shares 0.51, 0.20, 0.16 and 0.13, each drawn from an obsidian source to the capital,
  width by share, the sources ringed. Competing layers are off in the shot (routes, trails, lines, clans, units,
  and the rest of the civ layers) so the chains read. The shot came from a scratch script that drives the page
  with `open` and `lead` from `www/play.mjs`; `--shot` alone takes no selection, so it could not pick the good.

## Decisions

- The stage's `StoreSpec` takes `salt` and `obsidian` (both default 0), each dug at the stage's first source of
  it, so a staged store can hold a ware with a lot. A store could hold food only before; the play script needs
  salt to fill with and a source for the chain. Stages without them are unchanged.
- The Fill button's grey reason is the engine's, carried in the view as `units[].fill` (`{ok, why}`, from
  `World::validate`), and `panel.js` takes a button's `why` before asking `Sim.check`.
- A picked good belongs to the store it was picked on: selecting anything else clears it, and so does Escape.
- Only a store's wares are picks. A pack's lots are in the view (`units[].lots`) but no panel draws them yet.
