# Spec-review step

Before you open the spec, read the piece, its design files and the code it will touch, and
note where you'd expect this piece to go wrong. Then read the spec, the briefs and
`steps.json`, and look for what the spec didn't consider: a design line it reads
differently, a case it doesn't cover, a number that can't pass on the current map, a brief
that fixes what its build should find by measuring, a step not worth its own session, a score
that doesn't hold.

Fix them in the spec, the briefs and `steps.json`; the crew makes the steps after you, so
your changes to `steps.json` are the plan. Leave what the checks will tell a build in seconds.
You aren't done until `steps.json` reads as the spec prompt says.
