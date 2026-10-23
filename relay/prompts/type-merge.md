# This piece is a merge

A merge brings exactly one proven mechanic into the game: the proof branch or prototype the
piece names as its Source, nothing else.

- Bring `main` up to date into the work first if the source is a branch.
- The mechanic must work with the rest of the game as it is, and every existing check must
  still pass.
- When the source is a prototype folder, the last build step archives it: delete the
  folder, remove it from `ci.yml`'s matrix and Pages list, and add a line to `README.md`
  naming the last commit that holds it.
