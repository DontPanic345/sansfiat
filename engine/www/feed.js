// The event feed: newest first, "turn · text", an icon and colour by kind, a filter by kind.
// Events come from `Sim.events(from)`; the page keeps the cursor and hands each new batch to `add`.

const CAP = 300; // entries in the DOM
const KEEP = 6000; // entries in memory, for re-rendering after a filter change

export const KINDS = {
  clanDivided: { icon: '÷', color: '#8fd18f', label: 'Divisions' },
  clanStarving: { icon: '!', color: '#ff7b72', label: 'Starving' },
  clanMoved: { icon: '→', color: '#9ecbff', label: 'Moves' },
  marketFormed: { icon: '◉', color: '#ffd479', label: 'Markets formed' },
  marketFaded: { icon: '○', color: '#a8a8a8', label: 'Markets faded' },
  hearthTamed: { icon: '■', color: '#ffd400', label: 'Hearths' },
  centreFormed: { icon: '★', color: '#ffc800', label: 'Centres' },
  pregameEnded: { icon: '⚑', color: '#ffffff', label: 'End' },
  civFounded: { icon: '♜', color: '#ffffff', label: 'Civs' },
  unitMade: { icon: '+', color: '#8fd18f', label: 'Units made' },
  unitHungry: { icon: '!', color: '#ffb347', label: 'Units hungry' },
  unitDisbanded: { icon: '−', color: '#a8a8a8', label: 'Units disbanded' },
  unitStarved: { icon: '✕', color: '#ff7b72', label: 'Units starved' },
};
/** Rare, important kinds: the last ones trimmed from the DOM. */
const PINNED = new Set(['centreFormed', 'pregameEnded', 'civFounded']);

export class Feed {
  /** @param {{list: HTMLElement, filters: HTMLElement, onClick(item: object): void, mine?(item: object): boolean}} o */
  constructor({ list, filters, onClick, mine = () => false }) {
    this.mine = mine;
    this.list = list;
    this.onClick = onClick;
    this.items = [];
    this.next = 0;
    this.shown = new Set(Object.keys(KINDS));
    this.selected = null;
    for (const [kind, k] of Object.entries(KINDS)) {
      const label = document.createElement('label');
      const box = document.createElement('input');
      box.type = 'checkbox';
      box.checked = true;
      box.addEventListener('change', () => {
        if (box.checked) this.shown.add(kind);
        else this.shown.delete(kind);
        this.render();
      });
      const icon = document.createElement('span');
      icon.textContent = k.icon;
      icon.style.color = k.color;
      icon.setAttribute('aria-hidden', 'true');
      label.append(box, ' ', icon, ' ', k.label);
      filters.append(label);
    }
    this.list.addEventListener('click', (e) => {
      const li = e.target.closest('li');
      if (!li) return;
      const item = this.items.find((it) => it.id === Number(li.dataset.id));
      if (!item) return;
      this.select(item.id);
      this.onClick(item);
    });
  }

  reset() {
    this.items = [];
    this.selected = null;
    this.list.replaceChildren();
  }

  select(id) {
    this.selected = id;
    for (const li of this.list.children) li.classList.toggle('selected', Number(li.dataset.id) === id);
  }

  /** Appends a batch of events (oldest first, as the engine gives them). Starving clans of one turn become one line. */
  add(events) {
    const fresh = [];
    const starving = new Map();
    for (const e of events) {
      if (e.kind === 'clanStarving') {
        const have = starving.get(e.turn);
        if (have) {
          have.clans.push(e.clan);
          continue;
        }
        const item = { ...e, id: this.next++, clans: [e.clan] };
        starving.set(e.turn, item);
        fresh.push(item);
      } else {
        fresh.push({ ...e, id: this.next++ });
      }
    }
    for (const item of fresh) {
      if (item.clans && item.clans.length > 1) {
        const n = item.clans.length;
        const some = item.clans.slice(0, 4).join(', ');
        item.text = `${n} clans are starving (${some}${n > 4 ? ', …' : ''})`;
      }
    }
    this.items.push(...fresh);
    if (this.items.length > KEEP) this.items.splice(0, this.items.length - KEEP);
    for (const item of fresh) {
      if (this.shown.has(item.kind)) this.list.prepend(this.row(item));
    }
    this.trim();
  }

  row(item) {
    const k = KINDS[item.kind] || { icon: '·', color: '#ccc' };
    const li = document.createElement('li');
    li.dataset.id = item.id;
    li.dataset.kind = item.kind;
    li.dataset.turn = item.turn;
    if (item.id === this.selected) li.classList.add('selected');
    if (this.mine(item)) li.classList.add('mine');
    const b = document.createElement('button');
    b.type = 'button';
    const turn = document.createElement('span');
    turn.className = 'turn';
    turn.textContent = `${item.turn} ·`;
    const icon = document.createElement('span');
    icon.className = 'icon';
    icon.textContent = k.icon;
    icon.style.color = k.color;
    icon.setAttribute('aria-hidden', 'true');
    const text = document.createElement('span');
    text.className = 'text';
    text.textContent = item.text;
    b.append(turn, icon, text);
    li.append(b);
    return li;
  }

  /** Keeps the DOM to the latest `CAP` lines, dropping the oldest common ones before the rare ones. */
  trim() {
    let n = this.list.children.length;
    if (n <= CAP) return;
    for (let li = this.list.lastElementChild; li && n > CAP; ) {
      const prev = li.previousElementSibling;
      if (!PINNED.has(li.dataset.kind)) {
        li.remove();
        n--;
      }
      li = prev;
    }
    while (n > CAP) {
      this.list.lastElementChild.remove();
      n--;
    }
  }

  /** Rebuilds the list from memory under the current filter. */
  render() {
    const rows = [];
    // Newest first, the latest CAP lines that pass the filter.
    for (let i = this.items.length - 1; i >= 0 && rows.length < CAP; i--) {
      const item = this.items[i];
      if (!this.shown.has(item.kind)) continue;
      rows.push(this.row(item));
    }
    this.list.replaceChildren(...rows);
  }
}
