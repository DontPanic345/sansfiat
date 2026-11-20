# #426: What a clan carries is what its days carry

Built whole in one session; the review is the only step after it.

## What the piece delivers

- `MARKET.carry` is gone. `markets::load` decides what a clan brings to a festival: its salt and
  obsidian (less the salt its store will use), and its surplus (the food beyond a food a person), a
  load of `CARRY.clan.load` (2) at a time, each food in the same share. The first load rides with the
  walker who goes anyway; each further load goes while the food it trades for (`markets::gain` at the
  market's last prices, × `CLAN.harvest_labour`) is worth more than the walk days of the walker it adds,
  and the clan stops at the first load that is not. Walk 0 (a clan on its festival's tile) brings its
  whole surplus. Walkers stay one for each 2 of weight, at least one (`actors::carriers`).
- `markets::worths` weighs each festival at the load the clan would bring there (from its store and
  last harvest); `go` sets the route's cost to the walk times that load's walkers and keeps them on
  `Clan.walkers` (new field, serde default). `trade` applies `load` to the basket the clan holds after
  harvest, at its route's walk (route cost ÷ walkers). `civs.rs` (a clan sent to its capital's market)
  and `homes.rs` price routes through `markets::route`.
- `homes::step` weighs a home with the gain its load would trade for from there (`markets::load_to`,
  ÷ people, beside `clans::could_reap`). "Feeds it better" counts the market at the door; no tie rule.
- The judge counts `capital_towns_200`.
- `market::clear` bounds every price within `PRICE_SPAN` (10^12) of its goods' geometric mean: markets
  where clans brought salt and next to no food drove prices to NaN, which broke the save's replay.
- `SAVE_VERSION` 27 and the digest pins.

## Proof

`sweep.md`: a town clan trades 0.161 food a person a turn against 0.049 for a farmer outside a town
(main 0.043 against 0.038), and more markets hold at +200 with more clans at them. But towns at +200
are within main's floor (11 against 10) and capital towns fewer (3 against 5): the market at the door
pays, and does not yet draw farmers. Health is at main's floor. Starving rises (3.3% of clan-turns
against 2.2%), most among gatherers, which no longer bring food they need for variety.

The "first load + surplus" variant in `sweep.md` draws farmers (15 towns, 5 capital towns) and starves
less than main, but runs medium 9's pre-game to the backstop: it is the owner's call, raised as an idea.

Tests: `core/tests/markets.rs` (the load rule load by load, walkers, worths and routes),
`core/tests/festivals.rs` (trade at the door and on a long walk), `core/tests/towns.rs` (a farmer
steps toward its festival when its load trades for more from nearer, and stays when it trades for
nothing).

## Decisions the piece left open

- What a clan brings is its surplus, the food beyond a food a person, not its whole basket. "Whole
  surplus" is the Decided line's word, and bringing the whole basket ran medium 9's pre-game to the
  backstop (`sweep.md`, variants).
- The first load rides with the walker who goes anyway: a clan going to a festival brings up to one
  load of its surplus whatever the walk; the walk's days are paid in going (`worths`).
- A load's worth is what it trades for at the market's last prices, in food at `CLAN.harvest_labour`
  days a food, the same pricing `worths` used for the whole trip.
- The home step counts the load's trade gain. Without it, towns at +200 fell to 7: an outside clan
  now pays one walker, so the walk alone pulled less to the door.
- A clan with no surplus brings no food, only its salt and obsidian. Letting it bring one load anyway
  (the variant) breaks the health line on medium 9.
- Prices in a clearing are bounded within 10^12 of their mean.

## What the review should check

- Read `markets::load` against the piece's Decided lines.
- Read the starving cost in `sweep.md` against what it buys, and the variant the owner is asked about.
- `tests/cases.rs` and `tests/scout_errand.rs` changed with the new picks (`notes.md`): one errand named
  as unsettled, a trapped scout treated as the war's, and Lead's target chosen among reachable tiles.
- Play: none (the piece's `Play:` line).
