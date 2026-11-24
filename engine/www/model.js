// The view model: everything the map's layers and the panels need from one `Sim.view()`, worked out
// once per change. Layers and panels read the model; the `Sim` is called for orders and per-selection
// queries only.

/** A civ's claim as a count: every claimed tile is its land. */
function claimCounts(c) {
  return { claimed: c.claim.length };
}

/**
 * @param sim the wasm `Sim`; @param layout `Sim.layout()` parsed; @param rules `rules()` parsed.
 */
export function build(sim, layout, rules) {
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

  // Each store (the capital of a civ, or a settlement) by its tile, with its lots (`lots.rs`).
  const lotsAt = new Map();
  for (const c of civs) if (c.capital != null) lotsAt.set(c.capital, c.lots ?? []);
  for (const s of v.settlements ?? []) lotsAt.set(s.tile, s.lots ?? []);

  const claimOf = new Map(v.claims);
  // Each tile a clan works as fields this turn: its clan, field days and grain.
  const fieldOf = new Map();
  for (const k of Object.values(v.clans)) for (const [t, days, grain] of k.plots) fieldOf.set(t, { clan: k.id, days, grain });
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
    /** Good `g` (an index into `goods()`) in the store at tile `i`, as its chains: one per lot, `{share, path}`,
     *  where `path` is the tiles it went through, from its source, through the tile of each market it changed
     *  hands at, to the store. Empty for no store at `i` or no lot of `g`. */
    chainsOf(i, g) {
      return (lotsAt.get(i) ?? [])
        .filter((l) => l.good === g)
        .map((l) => ({
          share: l.share,
          path: [l.source, ...l.markets.map((m) => v.markets[m]?.tile).filter((t) => t != null), i],
        }));
    },
    /** The clan whose territory holds tile `i`, or null. */
    clanAt: (i) => claimOf.get(i) || null,
    /** The field work on tile `i` this turn, `{clan, days, grain}`, or null. */
    fieldAt: (i) => fieldOf.get(i) ?? null,
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
