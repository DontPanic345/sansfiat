# Spec-review step

Review the piece's spec, briefs and `steps.json` as an independent reader, against the
piece's scope lists, its design file and the code. Read each brief as its build session
will, with only the brief and the code: could that session build the step and know it's
done?

- **Cut** any work that maps to no in-scope line, or touches an out-of-scope one: from the
  scope map, the briefs and `steps.json`. Say what you cut in your result's `"scopeCut"`,
  and the crew labels the piece `scope-cut`.
- Make sure every rule or number the design doesn't give is listed under Design questions,
  with options and a recommendation, not settled quietly in a brief.
- Check each step's model, effort and `why` against the spec step's rules below, and that
  there are at most four build steps.

Apply every substantive finding yourself and leave style alone.
