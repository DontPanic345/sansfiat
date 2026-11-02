// The pick: a card per civ over the map while the phase is `choose`. Built once per game (`sim`), so a
// Show or a Lead doesn't rebuild it. The play screen gives the three actions.

const $ = (id) => document.getElementById(id);

export class Pick {
  /** @param {{lead(civ: number): void, show(civ: number): void, watch(): void}} on */
  constructor(on) {
    this.on = on;
    this.sim = null;
    $('pick').addEventListener('click', (e) => {
      const b = e.target.closest('button');
      if (!b) return;
      if (b.dataset.lead) this.on.lead(Number(b.dataset.lead));
      else if (b.dataset.show) this.on.show(Number(b.dataset.show));
      else if (b.dataset.watch) this.on.watch();
    });
  }

  /** Shows the cards while the phase is `choose`; hides them otherwise. */
  update(sim, phase) {
    const el = $('pick');
    el.hidden = phase !== 'choose';
    if (el.hidden) {
      this.sim = null;
      return;
    }
    if (this.sim === sim) return;
    this.sim = sim;
    $('legend').open = false;
    const cards = JSON.parse(sim.pick());
    const rows = cards.map((c) => {
      const terrain = c.terrain.map(([name, share]) => `${name} ${Math.round(share * 100)}%`).join(', ');
      const rival = c.rival_steps == null ? 'no rival' : `${c.rival_steps} steps to the nearest rival`;
      return `<div class="card" data-civ="${c.civ}"><div class="head"><i class="sw" style="background:${c.color}"></i><b>${c.name}</b> <span class="dim">capital at ${c.q}, ${c.r}</span></div>
<div>${c.clans} clan${c.clans === 1 ? '' : 's'} · ${Math.round(c.people * 10) / 10} people · ${c.hearths} hearth${c.hearths === 1 ? '' : 's'}</div>
<div class="dim">${terrain || 'no land'}</div>
<div class="dim">${rival}${c.rival_by_land ? '' : ' · no land route to a rival'}</div>
<div class="buttons"><button type="button" data-lead="${c.civ}">Lead ${c.name}</button><button type="button" data-show="${c.civ}">Show</button></div></div>`;
    });
    el.innerHTML = `<h2>Choose your civ</h2>${rows.join('')}<div class="buttons"><button type="button" data-watch="1">Watch all three</button></div>`;
  }
}
