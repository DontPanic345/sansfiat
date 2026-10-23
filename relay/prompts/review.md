# Review step

Review the whole piece as built, as an independent reader, against its spec, its scope
lists and its design file. Read the diff (`git diff origin/main...HEAD -- <path>`), beyond
the spec and the tests. Your brief says where to look hardest.

- Fix what is substantive yourself, each fix with a test that fails without it, and leave
  nits.
- Remove anything built outside the scope map, and list it under `ideas` instead.
- The PR is now ready, so `ci.yml` runs on every push. After your last push, wait for it
  (`gh pr checks --watch`) and fix what fails. Finish `done` only when it's green.

Then finish as "Finishing" says.
