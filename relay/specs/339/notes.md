# #339 notes for the next builder

- `www/play/all.mjs` on main took 174.7 s on this runner (`allmjs-main.txt`); `layout.mjs` (31 s, it stays on
  real maps) and `buttons.mjs` (23 s, it opens small 2 only to have a page) are the slowest.
- The four scout tests in `scout_feed.rs` stock the civ's store to 50 grain before every turn (`stock`); the data
  form keeps that as `stores[].everyTurn`, which the Tests screen must say aloud.
