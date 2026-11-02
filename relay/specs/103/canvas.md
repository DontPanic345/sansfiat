# #103 build-4 (canvas): how the page reads

Before shots are `screens/build-4-before/<size>-<screen>.png` (the easel as build-3 left it); after shots are
`screens/<size>-<screen>.png`, written by `layout.mjs` at the end of this step.

## What changed, and the shot that showed it

| Change | Before → after | Why |
|---|---|---|
| The feed opens by default only at 1400 px and wider; at laptop it is closed, with Show feed and the unseen-alert count on the bar's Feed. | `before/laptop-camp.png` → `laptop-camp.png` | A 340 px feed left the laptop map a 940 px column; with the panel under it the map was 44%. |
| The panel is two columns at laptop and wide: what it is (doing, numbers, lists) on the left, why (reasons, hint, refusals) on the right; lists run inline; the button row wraps instead of scrolling; the reasons list scrolls past three lines; max height 34% (45% at phone, one column). | `before/laptop-camp.png`, `before/laptop-civ.png`, `before/phone-camp.png` → the same names | The panel took ~40% of the laptop window; at phone the row scrolled sideways and hid Move to… and Paint. Now every button shows at phone in three short rows. |
| Every clan's territory tint is off by default (View → Territories brings it back); a selected clan's territory is still drawn. | `before/laptop-large-fit.png` → `laptop-large-fit.png` | At fit the tints and their edges hid the terrain; "no ring, radius or circle that isn't a rule". |
| Clan circles are capped at 0.75 of a hex (were 1.4) and scale 0.16·√people (was 0.22). | same | Big clans covered their neighbours' tiles, so the placeholders read as speckle. |
| The end chart's scale runs to the next of 10/25/50/100% over the highest Standing; labels sit at the right edge kept 16 px apart; one turn is a dot; under 10% shows a decimal. | `before/laptop-end.png` → `laptop-end.png` | Standing near 5% sat on the 0% line with three labels on top of each other. |
| The start card's size select and seed are narrower, so Start sits on the seed row. | `before/laptop-start.png` → `laptop-start.png` | Start wrapped under the row. |
| The driver's `tile()` clicks only where the map canvas is the element under the point. | (pick.mjs, camp.mjs) | With the feed closed, map tiles lie under the zoom and Show feed buttons. pick.mjs now clicks the card's Show (Show feed matched first). |

Left alone, with reasons: the centres' gold rings and the festival rings are rules (the win's sites, the markets),
so they stay on; festival routes were already off. `placeholders.js` and the atlas are unchanged: with the tints
off and the clans smaller, the terrain cells read at low zoom (`laptop-large-fit.png`). The bar already fit one
line at laptop and wide and one short line with ☰ at phone (35 px). On phone, Move to… covers the missing
right-click; the bottom sheet and drawer need no other touch handling.

## The map's share (camp selected, the feed as it opens)

| Size | Bar | Before | After |
|---|---|---|---|
| phone | 35 px | n/a (drawer) | n/a (drawer) |
| laptop | 41 px | 44% | **73.4%** (main 67%) |
| wide | 41 px | 58% | **66.7%** (main 79%) |

`all.mjs` now runs `layout.mjs` with its 60% check on.

## The played game (`www/play/game.mjs`, small seed 2, Red)

Led at turn 237, played to 267 with buttons, keys and clicks only (about 9 s; `all.mjs` runs it).

| Turn | Step | What the page said |
|---|---|---|
| 237 | Lead | "You lead Red: Standing 5.6%, 3rd of 3"; the civ is selected. |
| 237 | Gift +1 | Gift 2 → 3. |
| 237 | Raise… → a guard → Go | Guard raised. |
| 237 | P, click, Esc | Claim 24 → 25; hint "Click or drag on the map; Esc stops"; Standing now 6.6%, 2nd. |
| 240 | Alert followed | "Red's scout at (-3, -11) goes hungry" → selects the scout. The panel says "Idle", **not why it's hungry** (no reason row for hunger). Laptop: the feed is closed, so the alert was reached by Show feed; the Feed button's count is the only cue. |
| 242 | Camp, Move to…, click | "Marching to -9, -5: 4 turns". |
| 246 | P, two clicks, Esc | Zone 3 tiles; doing says "Cut off: its line can't feed it" — the page says it, but nothing warned before the order that the tile was past the line's reach. |
| 252 | Reload, Continue | Start reads "Continue Red, turn 252, small seed 2"; digest 7a57dac92542676e before and after. |
| 267 | Game → Save to file | `sansfiat-seed2-turn267.json` |

Digests: page `267 cf9039e9bc0cc311`, `sansfiat digest --game` `267 cf9039e9bc0cc311` — match.

`sansfiat judge`:
```
turn 267 (+30): civ 472 7.9%, civ 473 10.1%, civ 474 14.6%; leader civ 474
led civ 472: Standing at +30 7.9% (rank 3) in the game, 3.6% (rank 3) with every civ on the AI
claim (game / control): 25 / 26 tiles claimed, 23 / 1 held, guard 2.0 / 0.0 people
doubt (game / control): lead changes 1 / 1, spread +30 0.067 / 0.107, swings 0 / 0
motion (game / control): clans changed 21 / 17, people 111.1 / 94.0, land tiles 68 / 99, clans left 0 / 0, claimed tiles taken 0 / 0
player orders: 6 (claim 1, move 1, raise 1, setGift 1, zone 2), 20.00 per 100 turns; turns with none 27 of 30, longest run 20
```

## What still reads badly

- Wide is 66.7%, under main's 79%: the feed's 340 px column is open by default there.
- Phone: the sheet is still ~40% of the screen with the civ selected; the map is the top half.
- Move to… does not show a target's reach before the click; the camp walks out past its line and is cut off.
- A hungry scout's panel has no reason row for its hunger.
- Clan circles still crowd the map at fit on a large map; a zoom-dependent clan mark is the next step.
