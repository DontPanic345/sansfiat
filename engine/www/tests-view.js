// The Tests screen's text, read from core's state: the list's rows, each hold's mark and the result line.
// The verdict is core's (`test_state()`, `Sim.tests()`); these only say it in words.

/** A hold's mark: pending, held (and the turn it settled), or failed (its turn and the engine's why). */
export function holdMark(h) {
  if (h.state === 'held') return `held at turn ${h.turn}`;
  if (h.state === 'failed') return `failed at turn ${h.turn}: ${h.why}`;
  return 'pending';
}

/** The result line: nothing until the last turn, then Passed, or the first failed hold's words and turn. */
export function resultLine(s) {
  if (!s.done) return '';
  if (s.passed) return 'Passed';
  const f = s.holds.find((h) => h.state === 'failed');
  return f ? `Failed: ${f.words} at turn ${f.turn}` : 'Failed';
}

/** The holds as list items: each one's words, and its mark. Its `data-state` names its state. */
export function holdItems(ol, s) {
  ol.replaceChildren(
    ...s.holds.map((h) => {
      const li = document.createElement('li');
      li.dataset.state = h.state;
      const words = document.createElement('span');
      words.className = 'hold-words';
      words.textContent = h.words;
      const mark = document.createElement('span');
      mark.className = 'hold-mark';
      mark.textContent = holdMark(h);
      li.append(words, mark);
      return li;
    }),
  );
}

/** The list's rows: a button per test, its name and its claim, chosen with `onPick(test)`. */
export function listRows(ul, tests, onPick) {
  ul.replaceChildren(
    ...tests.map((t) => {
      const li = document.createElement('li');
      const b = document.createElement('button');
      b.type = 'button';
      b.className = 'test-row';
      b.dataset.test = t.name;
      const name = document.createElement('b');
      name.textContent = t.name;
      const claim = document.createElement('span');
      claim.textContent = t.claim;
      b.append(name, claim);
      b.onclick = () => onPick(t);
      li.append(b);
      return li;
    }),
  );
}
