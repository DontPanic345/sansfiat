// What a script asks the page, through `window.game`: what one thing in the world is (`Sim.inspect`, the same JSON
// as `sansfiat inspect`), where it is on the screen, and what can be pressed. A reading: it never changes the page.

/** What `ui()` lists: everything a player can press, type in or pick. */
const PRESSABLE = 'button, input, select, textarea, a[href], [role="button"]';

/** The attributes a selector is made from, most telling first. */
const KEYS = ['data-role', 'data-action', 'data-label', 'data-list', 'data-civ', 'data-id', 'data-lead', 'data-show', 'data-watch', 'aria-label'];

/** One thing as JSON: `target` is `#ID`, `Q,R`, a tile index, or `{id}` / `{tile}`. Throws the engine's reason when there is none. */
export function inspect(sim, target) {
  return JSON.parse(sim.inspect(text(target)));
}

/** Every subject of `kind` (`units`, `clans`, `civs`, `markets`, `settlements`): `[{id, at, ...}]`. */
export function list(sim, kind) {
  return JSON.parse(sim.list(kind));
}

/** Where `target` stands on the screen: its tile, and the client point of the tile's centre, and whether that point
 *  is on the map's canvas now. */
export function where(sim, map, target) {
  const v = inspect(sim, target);
  const { x, y } = map.screenOf(v.at.tile);
  const r = map.canvas.getBoundingClientRect();
  const onScreen = x >= r.left && x <= r.right && y >= r.top && y <= r.bottom;
  return { kind: v.kind, id: v.id ?? null, ...v.at, x: Math.round(x), y: Math.round(y), onScreen };
}

/** What can be pressed now, in page order: each one's selector (unique on the page), role, accessible name, state and
 *  box. `filter`, if given, is a string matched case-blind against the role, name and selector, or a function of a row. */
export function ui(filter) {
  const rows = [];
  for (const e of document.querySelectorAll(PRESSABLE)) {
    if (!shown(e)) continue;
    const b = e.getBoundingClientRect();
    const row = {
      selector: selector(e),
      role: e.dataset.role || e.getAttribute('role') || (e.type && e.tagName === 'INPUT' ? e.type : e.tagName.toLowerCase()),
      name: name(e),
      disabled: e.disabled || e.getAttribute('aria-disabled') === 'true',
      x: Math.round(b.left + b.width / 2),
      y: Math.round(b.top + b.height / 2),
      w: Math.round(b.width),
      h: Math.round(b.height),
    };
    for (const k of ['action', 'kind', 'civ', 'id', 'label']) if (e.dataset[k] != null) row[k] = e.dataset[k];
    const pressed = e.getAttribute('aria-pressed') ?? e.getAttribute('aria-expanded');
    if (pressed != null) row.pressed = pressed === 'true';
    if (e.type === 'checkbox' || e.type === 'radio') row.checked = e.checked;
    else if (e.tagName === 'SELECT' || e.tagName === 'INPUT' || e.tagName === 'TEXTAREA') row.value = e.value;
    const why = e.dataset.why || (row.disabled ? e.title : '');
    if (why) row.why = why;
    rows.push(row);
  }
  if (filter == null) return rows;
  if (typeof filter === 'function') return rows.filter(filter);
  const f = String(filter).toLowerCase();
  return rows.filter((r) => [r.role, r.name, r.selector].some((s) => s.toLowerCase().includes(f)));
}

/** `target` as `Sim.inspect` takes it. */
function text(target) {
  if (target && typeof target === 'object') return target.id != null ? `#${target.id}` : String(target.tile);
  return String(target);
}

/** Whether `e` is drawn: laid out, with a size, and not inside anything hidden. */
function shown(e) {
  if (e.checkVisibility && !e.checkVisibility({ visibilityProperty: true })) return false;
  const b = e.getBoundingClientRect();
  return b.width > 0 && b.height > 0;
}

/** The name a screen reader gives `e`: its aria-label, its label, its text (not a select's: that is its options), or
 *  its title. */
function name(e) {
  const text = e.tagName === 'SELECT' ? '' : e.textContent;
  // A label that wraps its control: its own text, not the control's.
  const own = (l) => [...l.childNodes].filter((n) => n !== e && !n.contains?.(e)).map((n) => n.textContent).join('');
  const label = e.getAttribute('aria-label') || (e.labels?.[0] ? own(e.labels[0]) : '') || text || e.title || e.id || '';
  return label.replace(/\s+/g, ' ').trim();
}

/** A CSS selector that picks `e` alone: its id; else its telling attributes, as few as do it, under its nearest
 *  ancestor with an id if they are not enough alone; else its place among its siblings, from that ancestor. */
function selector(e) {
  if (e.id) return `#${CSS.escape(e.id)}`;
  const tag = e.tagName.toLowerCase();
  let attrs = '';
  for (const k of KEYS.filter((k) => e.hasAttribute(k))) {
    attrs += `[${k}="${e.getAttribute(k).replace(/["\\]/g, '\\$&')}"]`;
    if (unique(tag + attrs, e)) return tag + attrs;
  }
  const own = tag + attrs;
  const anchor = e.parentElement?.closest('[id]');
  const under = anchor ? `#${CSS.escape(anchor.id)} ${own}` : own;
  if (attrs && unique(under, e)) return under;
  const steps = [];
  for (let n = e; n && n !== anchor && n !== document.body; n = n.parentElement) {
    const i = [...n.parentElement.children].indexOf(n) + 1;
    steps.unshift(`${n.tagName.toLowerCase()}:nth-child(${i})`);
  }
  return [anchor ? `#${CSS.escape(anchor.id)}` : 'body', ...steps].join(' > ');
}

function unique(sel, e) {
  const all = document.querySelectorAll(sel);
  return all.length === 1 && all[0] === e;
}
