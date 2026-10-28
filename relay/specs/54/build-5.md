# Build 5 (canvas): a pre-game that levels off and forms its own centres

Piece #54, "Actors, and clans that live off their land". Builds 1–4 put every rule in place
with starting numbers chosen to run, not to be right: the actor model and clans on it
(territory, divided labour, walks, growth on surplus, one festival route), depletion, trails
from walking, open-ended taming, festivals forming alone, people moving and small clans
merging, the page. Read their summaries and the sweeps they left
(`relay/specs/54/clay-sweep.md`, `land-sweep.md`, `festival-sweep.md`). Read
`relay/specs/54/spec.md` for every rule and the decisions behind them, and `engine/CLAUDE.md`.

You find the numbers that make the pre-game work, and you answer the piece's question.

## The question (from `docs/design/clans.md`)

> Does a sparse world of clans, paying real labour to reach their food, spread across the
> land, level off, and form centres with festivals around fertile areas on its own, with no
> rule placing them?

And what the design wants it to look like:

> The pre-game spreads a light population over the world before play starts, so wherever a
> player could get to, people already live. As they settle, small centres grow up around
> fertile land on their own, with festivals where people gather. … A clan grows only on what
> it harvests beyond its need. One that just meets its need holds; well short, it starves and
> shrinks. So the pre-game settles on a light population the land carries without strain.

## What proves it

`cargo build --release -p sansfiat && ./target/release/sansfiat sweep --size all` (seeds
1–10 on small, medium and large), with these as the bar:

- **Centres on their own**: on each size, at least as many seeds end by centres (not the limit
  and top-up) as main does today on small: 8 of 10. Today's small: median end turn 382.
- **People level off**: in most runs, people change by under 10% over the last 100 turns
  before the end (the sweep's levelled-off line). Not growing until the end, not collapsing.
- **Light and spread**: the land cover reached (`cover` ≥ `PREGAME.cover`), clans everywhere
  they can reach, territories small enough that a walk matters (a clan doesn't hold every tile
  in its radius).
- **The new rules bite**: some tiles depleted, not most; people moved at festivals, not none;
  some festivals held by one clan and some by many; trails along festival routes.
- **Centres around fertile land**: `centre_report`'s `fertility_near / fertility_land` above 1
  on average (the evidence #13 and #31 used; `sansfiat run` reports it).
- **Speed**: ms a turn on small within 2× today's 3–5 ms (`sansfiat bench`).

Where a bar can't be met, say how near you got and why, rather than bending a rule out of the
design.

## What you may change

- Every number the piece added or reads: `CLAN.inside`, `grow_full`, `territory_radius`,
  `land_margin`, `min_pop`, the growth rates; `DEPLETION.*`; `TRAIL.*`, `TENDING.*`;
  `MARKET.desire`, `company`, `company_cap`, `switch_margin`, `reach_walk`, `spacing`,
  `form_after`, `min_attendance`, `carry`, `shed`, `gather_bonus`; `PREGAME.*` if the centres'
  test itself needs it. Start from what the sweeps show is off.
- A rule's form, where a number can't fix it: you may change the clay or a kiln. Keep the
  actor model's shape (spec.md, "What the clay leaves the kiln") and every rule the design
  states; record each change of form as a decision, saying what the sweep showed.
- Tests that pin a number you changed: fix them to the new number, or better, to what must hold.

Change no map rule (`MAP`, terrain, foods' caps): the map is #31's and settled.

## Method

Yours. The sweep runs seeds in parallel; small takes seconds. A quick loop on small, then
`--size all` to confirm. `sansfiat run --seed N --every 25` and the page (`node www/play.mjs`)
show one world over time if a number's effect isn't clear from the sweep.

## Done

- `engine/check.sh` passes.
- `relay/specs/54/sweep.md`: the final sweep on every size (its lines and series), the bar
  above with each point met or not and by how much, the numbers you changed from where and
  why (a table), each rule form you changed, and a short answer to the question with the
  evidence. The review copies the answer to the PR.
- Your summary: the answer in one sentence, the numbers that mattered most, and anything the
  review should play to see it.
