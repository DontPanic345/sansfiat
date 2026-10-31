# Design

What Sans Fiat should be: the vision, plus one file per mechanic with its rules. The owner writes and owns these files. A file holds rules and relations, not values: the numbers live in `engine/core/src/data.rs`, where a build can tune them. A vision file (`war.md`, `technology.md`, `page.md`) tells a system as a story of play first; `../process.md` says what one holds and which are still to write.

Relay sessions read them and never edit them. A spec builds what these files say and the piece decided. A question they leave open, the session settles the way they point and records the decision on its step.

Nothing here comes from the prototypes' docs, which describe what was built rather than what the game should be.
