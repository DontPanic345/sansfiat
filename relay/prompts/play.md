# Play step

You play the branch as a player would, then judge the game you played, so the review reads a
played game beside the numbers. The player's rules and the judge's are below this; they hold for
the whole step.

## Setting up

- Build the game once: `cargo build --release -p sansfiat --manifest-path engine/Cargo.toml`.
  BINARY is `engine/target/release/sansfiat`.
- The maps are the retro maps the spec names for play, else in this order: small 2, medium 8,
  small 6. Play the first one, or as many as your step says. CIV is 1.
- GAME for each is `relay/specs/<piece>/play/<size>-<seed>.json`. Its lines file sits beside it.
- Split your time: about 45 minutes of play for one map, and less each for more, with 15 left for
  the judging.

## Playing

Play each map as the player's rules say: 100 turns from the pick, a note before each decision.
While you play you are the player: the view, `why` and `map` are all you read.

## Judging

Then judge each game as the judge's play note says, running `BINARY judge GAME` once per game.
Write `relay/specs/<piece>/play.md`: per map, the judge's Standing line against the AI control and
the idle control, then the findings by kind. A page piece also runs `node engine/www/play/all.mjs`
and puts its screenshots in `relay/specs/<piece>/play/`.

A bad result is a finding, not a block: finish `done` whatever the game showed.

## Fix or ask

Each finding is one line with a class, `fix` or `ask`, and a repro: the lines file and the turns
that show it, or a play script.

A finding is `fix` when all four hold:

1. it names what it breaks: a line of a design file or its ledger, a piece's Goal or Decided line,
   a touch point in `docs/design/page.md`, or health;
2. its repro shows it on `main` too, or on the branch where the piece's own Decided line breaks;
3. the fix restores that line and edits no design prose;
4. it fits one Ford build, with no new order, kind, module, score or `data.rs` number a design line
   doesn't already fix.

Everything else is `ask`: a new or changed rule, prose, a new lever, a band, two lines that
conflict, or "less fun" with no line named.

Put every finding in `result.json` as well, under `findings`, each with `title`, `body`, `class`
and `repro`. The crew puts each `fix` on the milestone's fixes piece and opens each `ask` as a
proposed issue for the owner.
