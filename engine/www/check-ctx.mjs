// The context the panels (panels.js) are described with, over a Sim: what check.mjs and check-stage.mjs
// both build it from. `panelCtxFor(sim, {rules, terrain, goods})` gives `(view, extra) => ctx`.

export function panelCtxFor(sim, { rules, terrain, goods }) {
  const layout = JSON.parse(sim.layout());
  return (view, extra = {}) => ({
    v: view,
    model: {
      claims: new Map(),
      standing: new Map(Object.values(view.civs).map((c) => [c.id, { total: 0, clans: 0, land: 0, rank: 1 }])),
      holders: () => [],
      clanAt: () => null,
      fieldAt: () => null,
    },
    layout,
    terrain,
    goods,
    rules,
    sim: { notes: (id) => sim.notes(id), route: (id) => sim.route(id), tile: (i) => sim.tile(i), tile_civ: (i) => sim.tile_civ(i) },
    view: null,
    routing: { arm: null, start: null, unit: null },
    outcome: null,
    painting: null,
    country: false,
    ...extra,
  });
}
