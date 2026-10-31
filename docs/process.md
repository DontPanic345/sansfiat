# How Sans Fiat gets built

One person with a few hours a week, Claude Code on a Pro plan, and money for art and sound
once the game earns them. The process turns those hours into a finished game by spending
the owner's time on taste and decisions, and the plan's tokens on everything else.

## The loop

Vision, design, proposal, pieces, Relay, play, retro, and round again.

1. **Vision.** `docs/design/vision.md` says what the game is, and one vision per system
   says how that system should feel to play. The owner tells them as stories; Claude drafts
   and the owner corrects.
2. **Design.** One file per mechanic in `docs/design/`, with its rules and the numbers that
   are decided. The owner owns the prose. A piece's review adds what the piece decided to a
   Decided ledger at the foot of the file, so the design keeps up with the code and the
   owner promotes or strikes each line when they next plan.
3. **Proposal.** A milestone in `docs/proposals/`, written in a planning session: the pieces,
   each as the issue Relay reads, with the question each answers. Game pieces and Relay
   changes are separate files.
4. **Pieces.** Opened once the owner has read the proposal. Big by area: a piece is a system
   the player meets, not a fix around an edge.
5. **Relay.** Builds each piece in cold sessions and merges it when its review says so. The
   owner's only stops are a blocker's reply and a merge the crew wouldn't make.
6. **Play.** A session plays each piece before it merges, and its note sits on the PR. The
   owner plays main at the end of a milestone.
7. **Retro.** A planning session reads the milestone back from several angles and writes
   `docs/proposals/<m>-retro.md`. The next proposal starts from it.

## The owner's hours

| When | What | How long |
|---|---|---|
| Weekly | A planning session: the proposal, or the retro | 1–2 hours |
| Between things | A blocker's reply; a merge the crew held back; a look at a PR's screenshots and play note | Minutes, from a phone |
| End of a milestone | Play main; dictate what was good and what wasn't | An hour |

Everything else is a session's job. If something needs more of the owner than this, the
process is wrong, not the owner.

## Visions

A vision is one page per system: a story of play first, then the rules that fall out of it,
then what it needs before it can exist, then what it isn't. It guides a planner cutting
pieces and a designer settling a spec, so it says how the system should feel, not what its
numbers are.

| Vision | File | State |
|---|---|---|
| The game | `vision.md` | Written |
| Land and war | `war.md` | Drafted from the owner's story |
| Technology | `technology.md` | Drafted from the owner's story |
| The page | `page.md` | Drafted |
| Winning | `vision.md`, Winning | The first way to win is chosen in M3 |
| Settling and farming: fields, the home choice, towns | `farming.md` | Needed before M4; `actors.md` Later has the seed |
| Animals: herds, pack animals, horsemen, the camel | `animals.md` | Needed before M5 |
| Goods: beer first, then salt, obsidian, shells, beads | `goods.md` | Needed before M5; `relay/retro/m2/history.md` has the material |
| The wide world: biomes, deserts, ranges, steppe | `map.md`, grown | Needed before M7 |
| Routes with rests: campsites, caches, caravanserai | `routes.md` | Needed before M7 |

A design file holds rules and relations, never values: "a floor", "more than", "a small
share". The values live in `engine/core/src/data.rs` so a build can tune them, and a design
file names a number only to say that one exists.

## Scores

A piece is judged on what moves the game, in this order:

1. **Standing.** A civ's share of the world's people who live on its land or in its clans.
   The score the game is played for, until the vision names another.
2. **Doubt.** Whether a led game is ever in doubt: the lead changes hands after the pick, the
   spread between first and last opens and closes, and some games end before the limit.
3. **Decisions.** What the player can do each turn that changes Standing, and what the AI
   does: orders per hundred turns, and levers pulled more than once.
4. **Motion.** The world keeps moving after the pick: people, land and clans change hands.
5. **Health.** Nobody starves who shouldn't, the pre-game ends on its own, nothing panics,
   and a build gives the digest it gave before. Health is a band the judge holds, not a goal.

A piece's question is asked in the first four. Camps kept and clans joined are health: they
say the machinery runs, not that the game moved. `sansfiat sweep --judge` measures all five
on the retro maps against a control, and each piece adds its check.

## Pieces

- **Big by area.** A piece is a system: land, armies, the page. Its builds are cut by area
  too, each with room to spare; M2's longest session used a third of its context and half
  its time.
- **One question**, in the scores, with a control, answered on the retro maps. A piece
  without a question is a refactor, and its proof is the digests.
- **The owner's stop is the merge.** The review's PR description carries the steps table,
  the answer, the play note, the design edits and the screenshots, so a merge can be judged
  from a phone.
- **No surprises.** A piece lists what's decided and what the designer settles. The designer
  settles by reading first and running second, and asks only when a choice would change the
  goal.
- **Design keeps up.** What a piece decides, its review appends to the `## Decided` ledger
  at the foot of the design file, one line per decision with the piece's number, in the
  same PR. The prose above the ledger is the owner's: in the weekly planning session they
  promote a line into it, reword it, or strike it. Designers read the whole file, ledger
  included, so nothing waits.

## The plan

Relay runs on the owner's Pro plan, beside the owner's own sessions. Its pacing is in
`relay/README.md`: a step starts only if the 5-hour window is expected to stay under 80%
after it, and a stop restarts when the window resets. The week is the real limit: an
M2-sized piece took about 4 weekly points, and a retro or a proposal session takes 10–15.
Plan on half the week for Relay and half for the owner: about six M2-sized pieces a week, or
three M3-sized.

A planning session reads the retro and the proposals, not the angle files. Those are the
evidence, and the retro cites them.

## Art and sound

The owner pays for them once a system's look has stood through a milestone, not before
(`roadmap.md`, The look and the sound). Until then the page draws placeholders to the atlas
the paid art will fill, so the swap is a file, not a piece.

The contract with an artist or a sound designer is a manifest, not a conversation:
`www/assets/atlas.json` names every cell and its size, and `www/assets/sounds.json` maps
each feed event kind to a file name. The brief is that manifest, the concept art in
`docs/art/` and a style line; what comes back is files dropped into `www/assets/`, checked
by the play scripts' screenshots. Licences go in `docs/art/LICENSES.md`. Nothing runs
against a service at play time: the page is files on Pages.

## Engineering under scarcity

- A refactor is digest-safe, or it's a rule change in disguise.
- Judges over eyes. Whatever a session can measure, it measures; the owner's eyes go on feel.
- One door for orders, for the player, the AI and the automatic player alike.
- Relay stays small. When something goes wrong, look first for what could come out.
