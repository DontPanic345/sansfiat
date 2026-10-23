# This piece is a proof

A proof answers the piece's one question about a mechanic, on its own branch, without the
rest of the game in the way. Its PR is never merged: the branch is the proof.

- Build only enough to answer the question. Shortcuts outside the question are fine, and
  say so.
- The review step writes the findings into the PR description with `gh pr edit --body`:
  the answer to the question, the evidence (numbers, seeds, screenshots), and what a merge
  would need. Keep the first line, "Proof for #<piece>".
