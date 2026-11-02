# #103 build-3: the player's levers and the button or key that reaches each

The spec's question counts 15 levers: the 11 `Order` kinds (`engine/core/src/orders.rs`), `setGift`,
`setKeep`, `chooseCiv` and `endTurn`. Main gives a button for 14; `move` was right-click only, and no
lever had a key. After build-3, each has a button, and the four with a key say which.

| # | Lever | Button (or key) on the page | Where it sits | Script that presses it |
|---|---|---|---|---|
| 1 | `raise` (warriors or a guard) | Raise… (warriors, or a guard), a stepper with a count and Go | the civ's panel | `civ.mjs`, `camp.mjs` (the hold) |
| 2 | `disband` | Disband…, a stepper | a camp's, a guard's and a scout's panel | `camp.mjs` (opens the count) |
| 3 | `claim` | Claim (P), Unclaim (U); P and U paint, and drag | the civ's panel, or P/U with the civ selected | `civ.mjs` |
| 4 | `move` | Move to… (arms a click), Stop while marching; right-click | a camp's panel | `camp.mjs`, `camp.mjs` (hold) |
| 5 | `zone` | Paint (P), Unpaint (U), Undeploy; P and U, and drag | a camp's panel | `camp.mjs` |
| 6 | `switches` | Auto-forage: on/off, Auto-supply: on/off | a camp's panel | `camp.mjs` |
| 7 | `resupply` | Resupply | a camp's panel | `camp.mjs` |
| 8 | `forage` | Forage | a camp's panel | `camp.mjs` |
| 9 | `pillage` | Pillage (greyed with its reason where it can't) | a camp's panel | `camp.mjs` |
| 10 | `route` | Move to… (arms the end), Route from… (arms the start) | a scout's panel | `scout.mjs` |
| 11 | `clearRoute` | Clear route | a scout's panel | `scout.mjs` |
| 12 | `setGift` | Gift −1, Gift +1 | the civ's panel | `civ.mjs` |
| 13 | `setKeep` | Keep −1, Keep +1, Keep default | the civ's panel | `civ.mjs` |
| 14 | `chooseCiv` | Lead, on the pick's cards (Watch is the pick with no civ) | the pick | `pick.mjs`, and every script through `lead` |
| 15 | `endTurn` | End turn, and space | the bar | `bar.mjs`, `camp.mjs` |

Main reached 14 with buttons (no `move` button); after build-3 all 15 have a button, and space ends
the turn, P and U paint a zone or a claim.
