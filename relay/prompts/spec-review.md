# Spec-review step

Read the piece, its design files and the code it will touch first, and note where you'd
expect this piece to go wrong. Then read the spec, the briefs and `steps.json`, and look for
what the designer didn't consider:

- a design line it reads differently
- a case it doesn't cover
- a number that can't pass on the current map
- a brief that fixes what its build should find by measuring
- a step not worth its own session
- a shape that doesn't fit the piece, or a score that doesn't fit its part

Fix them in the spec, the briefs and `steps.json`: the crew makes the steps from what you
leave. Leave to the builds what the checks tell them in seconds. You're done when
`steps.json` reads as `relay/prompts/spec.md` says.
