# #426 progress

## Plan

1. `markets::load`: a clan brings its wares and its surplus (food beyond a food a person) a load (`CARRY.clan.load`, 2) at a time; the
   first load rides with the walker who goes anyway, each further load goes while the food it trades
   for (`markets::gain` at the market's last prices, × `CLAN.harvest_labour`) is worth more than the
   walk days of the walker it adds. Walk 0 (on the festival's tile) brings the whole surplus. `MARKET.carry` goes.
2. `worths` prices each festival at the load it would bring there; `go` sets the route's cost to the
   walk × the load's walkers and keeps them on `Clan.walkers`; `trade` brings the load on the basket it
   has, at the walk of its route (cost ÷ walkers).
3. The home step (`homes::step`) values a home with the gain its load would trade for from there, so
   "feeds it better" counts the market at the door. No tie rule.
4. A judge count of capital towns at +200 (`capital_towns_200`).
5. Sweep the branch beside main (`sweep.md`), tests, check.sh, hand on to the review.

## Done

- 1–5 built and swept (`sweep.md`), with prices bounded in the clearing (NaN prices broke replay).
  Variants weighed there: the whole basket, and a first load plus the surplus (both end medium 9's
  pre-game by the backstop); the cap gone with homes valued by their walk alone gave 7 towns at +200,
  so step 3 went in. check.sh passes.
- Tests: `markets.rs`, `festivals.rs`, `towns.rs`. `spec.md` holds the decisions; `steps.json` hands on
  to the review.

## Second whole step (after the owner's call on the review)

The owner took the agreed rule and widened the piece: the first load whatever a clan has, growth by
way of life, a child's days, and crowding.

1. Merge main (#427's `Clan.settled`). Done.
2. `markets::load`: the first load whatever the clan has; further loads from the surplus; at the door
   the whole surplus or the first load if more. Done (`tests/markets.rs`).
3. `clans::growth(settled)`: gathering, settled; the ramp on `since` goes (`CLAN.settle_turns` gone).
   Done.
4. A child's days (`CLAN.child_days`, `Clan.nursing`) come off the next harvest's labour in
   `fields::jobs`. Done (`tests/clans.rs`).
5. Crowding (`clans::crowding`, `Clan.crowded`), a clan note; the judge's `settling` group counts births
   a person a turn (settled, gathering) and crowding deaths. Done; main's sweep has the same counts
   patched in (judge-only, digest unchanged). `goal.py` makes the Goal's table.
6. Tune growth and crowding on the sweep; SAVE_VERSION and pins; check.sh; sweep.md; hand on.
