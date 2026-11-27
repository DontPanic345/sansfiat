// The bottom panel: one component that draws a selection's shape (spec item 15) into the slot under the
// map. The shape comes from panels.js; this file only lays it out and wires its buttons. Shape:
//
//   { key, name, swatch?, doing, numbers: [{label, value, tip?}], list?: [{label, rows: [{select | centre, label, text?}], empty?}],
//     reasons: [{turn, text}], buttons: [...], hint? }
//
// A button is one of: {label, action} (sent through `order`, greyed with the engine's reason when refused),
// {label, arm} (arms a map click), {label, paint} (P or U's mode), {label, stepper: {options: [{label, make(n),
// cap}]}} (opens a count in the row, with Go and Cancel). A button the kind can't take is left out of the shape.
//
// A list row selects its `select`, or centres the map on its tile `centre`. `hooks`: {check(action) -> reason or '',
// run(action), arm(kind), armed, paint(mode), painting, select(sel), centre(tile), pick({store, name})}.
// A number row with `pick` is a button that picks that good (`picked` marks it); an action with `why` is greyed with it.
//
// Each part carries `data-role` for scripts (`window.game.ui()`): `panel` (with `data-key`), `panel-name`, `panel-close`,
// `doing`, `number` and `list-row` (with `data-label`), `reason` (with `data-turn`), `action` (with `data-action`, the label's
// slug, `data-kind` arm|paint|order|stepper, and `data-why` when refused), and an open stepper's `count`, `option`, `go`, `cancel`.

const TAG = (tag, props = {}, ...kids) => {
  const e = document.createElement(tag);
  for (const [k, v] of Object.entries(props)) {
    if (v == null || v === false) continue;
    if (k === 'class') e.className = v;
    else if (k === 'text') e.textContent = v;
    else if (k === 'onclick') e.onclick = v;
    else e.setAttribute(k, v === true ? '' : String(v));
  }
  e.append(...kids.flat().filter((x) => x != null && x !== false));
  return e;
};

/** A label as `data-action` writes it: lower case, a sign before a number a word, each run of other characters a
 *  hyphen ("Move to…" is `move-to`, "Gift −1" is `gift-minus-1`). */
export const slug = (label) =>
  label
    .replace(/[−-](?=\d)/g, ' minus ')
    .replace(/\+(?=\d)/g, ' plus ')
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '');

/** The stepper open in the row: which button, its option, and its count. Kept across renders of the same
 *  selection, so a turn ending under an open count does not close it. */
let stepping = null;

/** The most a stepper option allows: counts from 1 up to `cap` while `Sim.check` takes each one. */
function most(hooks, option) {
  let n = 0;
  let why = '';
  for (let k = 1; k <= option.cap; k++) {
    why = hooks.check(JSON.stringify(option.make(k)));
    if (why) break;
    n = k;
  }
  return { max: n, why: n ? '' : why };
}

/** Draws `shape` into `slot`; a null shape empties it (and so hides it, by CSS). */
export function render(slot, shape, hooks) {
  if (!shape || (stepping && stepping.key !== shape.key)) stepping = null;
  slot.replaceChildren();
  slot.dataset.role = 'panel';
  slot.dataset.key = shape?.key ?? '';
  if (!shape) return;
  slot.append(
    TAG(
      'h2',
      {},
      TAG('span', { 'data-role': 'panel-name' }, shape.swatch ? TAG('i', { class: 'sw', style: `background:${shape.swatch}` }) : null, shape.name),
      TAG('button', { type: 'button', class: 'close', 'data-role': 'panel-close', 'aria-label': 'Close details', text: '×', onclick: () => hooks.select(null) }),
    ),
  );
  // Two columns at laptop and wide: what it is (doing, numbers, lists) and why (reasons, hint, refusals).
  const info = TAG('div', { class: 'info' });
  const aside = TAG('div', { class: 'aside' });
  slot.append(info, aside);
  if (shape.doing) info.append(TAG('p', { class: 'doing', 'data-role': 'doing', text: shape.doing }));
  if (shape.numbers?.length) {
    info.append(
      TAG(
        'div',
        { class: 'numbers' },
        shape.numbers.map((n) => {
          const label = [TAG('span', { class: 'dim', text: n.label }), ' ', TAG('b', { text: String(n.value) })];
          // A row that picks (a store's ware): a button, its state marked.
          if (!n.pick) return TAG('span', { class: 'num', 'data-role': 'number', 'data-label': n.label, title: n.tip || null }, ...label);
          return TAG('button', { type: 'button', class: `num pick${n.picked ? ' picked' : ''}`, 'data-role': 'number', 'data-label': n.label, title: n.tip || null, 'aria-pressed': String(!!n.picked), onclick: () => hooks.pick(n.pick) }, ...label);
        }),
      ),
    );
  }
  for (const list of [].concat(shape.list ?? [])) {
    if (!list.rows.length && !list.empty) continue;
    const rows = list.rows.length
      ? list.rows.map((r) =>
          TAG('div', { class: 'listrow' }, TAG('button', { type: 'button', class: 'link', 'data-role': 'list-row', 'data-label': r.label, 'data-list': list.label, text: r.label, onclick: () => (r.centre != null ? hooks.centre(r.centre) : hooks.select(r.select)) }), r.text ? TAG('span', { class: 'dim', text: ` ${r.text}` }) : null),
        )
      : [TAG('p', { class: 'dim', text: list.empty })];
    info.append(TAG('div', { class: 'list' }, TAG('span', { class: 'dim', text: list.label }), rows));
  }
  if (shape.reasons?.length) {
    aside.append(
      TAG(
        'div',
        { class: 'why' },
        TAG('span', { class: 'dim', text: 'Why' }),
        TAG(
          'ul',
          {},
          shape.reasons.map((r) => TAG('li', { 'data-role': 'reason', 'data-turn': r.turn }, TAG('span', { class: 'turn', text: `${r.turn} ·` }), ' ', r.text)),
        ),
      ),
    );
  }
  if (shape.hint) aside.append(TAG('p', { class: 'dim hint', text: shape.hint }));
  if (!shape.buttons?.length) return;

  const refused = [];
  const row = TAG('div', { class: 'actions', role: 'toolbar', 'aria-label': 'Actions' });
  for (const b of shape.buttons) {
    if (b.arm) {
      if (b.why) refused.push(`${b.label}: ${b.why}`);
      row.append(TAG('button', { type: 'button', ...role(b, 'arm', b.why), text: b.label, 'aria-pressed': String(hooks.armed === b.arm), disabled: !!b.why, title: b.why || null, onclick: () => hooks.arm(b.arm) }));
    } else if (b.paint) {
      row.append(TAG('button', { type: 'button', ...role(b, 'paint'), text: b.label, 'aria-pressed': String(hooks.painting === b.paint), onclick: () => hooks.paint(b.paint) }));
    } else if (b.action) {
      const why = b.why ?? hooks.check(JSON.stringify(b.action));
      if (why) refused.push(`${b.label}: ${why}`);
      row.append(TAG('button', { type: 'button', ...role(b, 'order', why), text: b.label, disabled: why !== '', title: why || null, onclick: () => hooks.run(b.action) }));
    } else if (b.stepper) {
      row.append(stepper(shape, b, hooks, refused));
    }
  }
  slot.append(row);
  if (refused.length) aside.append(TAG('p', { class: 'dim refused', text: refused.join(' · ') }));
}

/** A button's `data-` attributes: its role, its label's slug, its kind and the reason it is refused. */
function role(b, kind, why) {
  return { 'data-role': 'action', 'data-action': slug(b.label), 'data-kind': kind, 'data-why': why || null };
}

/** A stepper button: closed, it opens a count in the row; open, the count, its option, Go and Cancel. */
function stepper(shape, b, hooks, refused) {
  const open = stepping && stepping.label === b.label;
  if (!open) {
    // Closed, the button opens while any option is allowed; when none is, it says why (the first option's).
    const all = b.stepper.options.map((o) => most(hooks, o));
    const ok = all.some((x) => x.max > 0);
    const why = ok ? '' : all[0].why;
    if (why) refused.push(`${b.label}: ${why}`);
    return TAG('button', {
      type: 'button',
      ...role(b, 'stepper', why),
      text: b.label,
      disabled: !ok,
      title: why || null,
      onclick: () => {
        stepping = { key: shape.key, label: b.label, option: 0, n: 1 };
        hooks.redraw();
      },
    });
  }
  const opts = b.stepper.options;
  const option = opts[stepping.option] ?? opts[0];
  const { max } = most(hooks, option);
  const n0 = Math.min(Math.max(1, stepping.n), Math.max(1, max));
  const go = TAG('button', { type: 'button', 'data-role': 'go', text: 'Go' });
  const note = TAG('span', { class: 'dim' });
  const count = TAG('input', { type: 'number', 'data-role': 'count', min: 1, max: Math.max(1, max), value: n0, 'aria-label': `How many, for ${b.label}` });
  // The count's order, or the engine's reason it is refused (greyed Go, and the reason beside it).
  const order = () => {
    const n = Math.floor(Number(count.value));
    if (!(n >= 1 && n <= option.cap)) return { n, action: null, why: 'choose a count from 1' };
    const action = option.make(n);
    return { n, action, why: hooks.check(JSON.stringify(action)) };
  };
  const sync = () => {
    const { n, why } = order();
    stepping.n = n >= 1 ? n : 1;
    go.disabled = why !== '';
    go.title = why;
    note.textContent = why ? ` ${why}` : '';
  };
  count.oninput = sync;
  go.onclick = () => {
    const { n, action, why } = order();
    if (why || !action) return;
    stepping = null;
    hooks.run(option.make(n));
  };
  sync();
  const kids = [count];
  if (opts.length > 1) {
    const pick = TAG('select', { 'data-role': 'option', 'aria-label': `What, for ${b.label}` }, opts.map((o, k) => TAG('option', { value: k, text: o.label, selected: k === stepping.option })));
    pick.onchange = () => {
      stepping.option = Number(pick.value);
      hooks.redraw();
    };
    kids.push(pick);
  }
  const cancel = () => {
    stepping = null;
    hooks.redraw();
  };
  kids.push(go, TAG('button', { type: 'button', 'data-role': 'cancel', text: 'Cancel', onclick: cancel }), note);
  return TAG('span', { class: 'stepper', ...role(b, 'stepper') }, kids);
}
