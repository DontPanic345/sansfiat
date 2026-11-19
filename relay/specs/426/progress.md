# #426 progress

## Plan

1. `markets::load`: a clan brings its wares and its food a load (`CARRY.clan.load`, 2) at a time; the
   first load rides with the walker who goes anyway, each further load goes while the food it trades
   for (`markets::gain` at the market's last prices, × `CLAN.harvest_labour`) is worth more than the
   walk days of the walker it adds. Walk 0 (on the festival's tile) brings everything. `MARKET.carry` goes.
2. `worths` prices each festival at the load it would bring there; `go` sets the route's cost to the
   walk × the load's walkers and keeps them on `Clan.walkers`; `trade` brings the load on the basket it
   has, at the walk of its route (cost ÷ walkers).
3. The home step (`homes::step`) values a home with the gain its load would trade for from there, so
   "feeds it better" counts the market at the door. No tie rule.
4. A judge count of capital towns at +200 (`capital_towns_200`).
5. Sweep the branch beside main (`sweep.md`), tests, check.sh, hand on to the review.

## Done

- 1–4 built. Tests: `markets.rs` (the load rule, walkers, worths and routes), `festivals.rs` (trade at the
  door and on a long route).
