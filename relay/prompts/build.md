# Build step

Your brief is the job: build what it says, with its tests, and leave every check passing
after your last edit. When the brief or the spec turns out wrong, correct them in your
commit and say so in your summary, since later steps read them. Do only your step's work:
the next steps are other sessions'.

A test you add runs in seconds. While you work, run the tests your change touches; run
`engine/check.sh` whole once, before you finish, and wait on it with one call. A test that
carries a rule's bound is not yours to loosen: if you can't meet it, stop and say so.
