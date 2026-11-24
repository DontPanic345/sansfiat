# Notes for the next builder

- The M5 play games (`relay/specs/33x-35x/play/*.json`) no longer replay on main: their saves are versions 23 to 26.
  To read one, build the commit just before the version moved in a scratch worktree (#350's spec built `da69498^`
  for #354's game). Bumping the version field alone diverges at the pick.
- A line's days are `walk_cost` of the path out (twice it), with each step charged on the tile entered, so a
  line's cost is not symmetric. See the idea in #350's result.
