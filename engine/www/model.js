// The view model: everything the map's layers and the panels need from one `Sim.view()`, worked out
// once per change. Layers and panels read the model; the `Sim` is called for orders and per-selection
// queries only.

/** A civ's claim as counts: tiles claimed, held, paper (claimed, not held), and the thinnest guard
 *  share on a held tile (0 with none held). */
function claimCounts(c) {
  const share = new Map(c.claim.map((t, k) => [t, c.guard[k] ?? 0]));
  const held = c.land.map((t) => share.get(t) ?? 0);
  return {
    claimed: c.claim.length,
    held: c.land.length,
    paper: c.claim.length - c.land.length,
    thinnest: held.length ? Math.min(...held) : 0,
  };
}

/**
 * @param sim the wasm `Sim`; @param layout `Sim.layout()` parsed; @param rules `rules()` parsed;
 * @param prev the model before this change (for the Standing history), or null for a new game.
 */
export function build(sim, layout, rules, prev = null) {
  const v = JSON.parse(sim.view());
  const civs = Object.values(v.civs || {});
  const clanCiv = new Map(v.clan_civ || []);

  const stacks = new Map();
  for (const u of Object.values(v.units)) {
    let list = stacks.get(u.tile);
    if (!list) stacks.set(u.tile, (list = []));
    list.push(u);
  }
  for (const list of stacks.values()) list.sort((a, b) => a.id - b.id);

  const heldBy = new Map();
  for (const u of Object.values(v.units)) {
    if (u.kind !== 'camp') continue;
    for (const t of u.held ?? []) {
      let list = heldBy.get(t);
      if (!list) heldBy.set(t, (list = []));
      list.push(u);
    }
  }

  const claims = new Map(civs.map((c) => [c.id, claimCounts(c)]));

  const ranked = [...civs].sort((a, b) => b.standing - a.standing || a.id - b.id);
  const standing = new Map(
    ranked.map((c, k) => [c.id, { total: c.standing, clans: c.standing_clans, land: c.standing_land, rank: k + 1 }]),
  );
  const leader = ranked.length && ranked[0].standing > 0 ? ranked[0].id : null;

  const walkers = new Map(Object.values(v.markets).map((m) => [m.id, m.clans]));
  const mine = v.player != null ? Object.values(v.units).filter((u) => u.civ === v.player) : [];

  // The Standing of every civ for each turn the page has seen since the pick.
  let history = prev?.history ?? [];
  if (history.length && history[history.length - 1][0] > v.turn) history = [];
  if (v.phase === 'main') {
    const at = Object.fromEntries(civs.map((c) => [c.id, c.standing]));
    history = history.filter(([t]) => t !== v.turn);
    history.push([v.turn, at]);
  } else {
    history = [];
  }

  const claimOf = new Map(v.claims);
  return {
    v,
    layout,
    rules,
    clanCiv,
    stacks,
    /** The camps whose warriors hold tile `i`. */
    holders: (i) => heldBy.get(i) ?? [],
    claims,
    standing,
    leader,
    walkers,
    mine,
    history,
    /** The clan whose territory holds tile `i`, or null. */
    clanAt: (i) => claimOf.get(i) || null,
    /** The civ an unaligned clan leans to most, with the tint it draws: its colour, the more opaque the
     *  nearer its leaning is to the join line. Null for a clan with a civ, or leaning to none. */
    tintOf(c) {
      if (c.civ != null) return null;
      let best = null;
      for (const [civ, x] of Object.entries(c.leaning)) if (best == null || x > best[1]) best = [civ, x];
      if (!best) return null;
      const civ = v.civs[best[0]];
      if (!civ) return null;
      return { color: civ.color, alpha: 0.85 * Math.min(1, best[1] / rules.join) };
    },
  };
}
