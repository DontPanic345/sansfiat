// hits: per drawn unit, clan and place in game.map.stats, the tile under the centre of its drawn box (what a click there
// picks, by tileAt) against the tile it belongs to. Prints each view's counts and every miss.
async function hits(view) {
  const { page } = await setUp(view);
  try {
    const r = await page.eval(`(() => {
      const m = game.map, v = game.view(), d = m.dpr, s = m.stats, out = [];
      const at = (e) => m.tileAt((e.x + e.w / 2) / d, (e.y + e.h / 2) / d);
      for (const e of s.units ?? []) { const u = v.units[e.unit]; if (u) out.push({ what: 'unit ' + e.unit + ' ' + e.kind, own: u.tile, at: at(e) }); }
      for (const e of s.clans ?? []) if (e.w) { const c = v.clans[e.clan] ?? v.clans[e.id]; out.push({ what: 'clan ' + (e.clan ?? e.id), own: c ? c.home : e.tile, at: at(e) }); }
      for (const e of s.places ?? []) if (e.w && e.role !== 'mark') out.push({ what: 'place ' + e.role, own: e.tile, at: at(e) });
      return out;
    })()`);
    const miss = r.filter((e) => e.own !== e.at);
    console.log(`${view.name}  k ${view.k}  drawn ${r.length}  off their tile ${miss.length}  ${miss.map((e) => e.what + ' ' + e.own + '->' + e.at).join(', ')}`);
  } finally {
    await page.close();
  }
}
const names = process.argv[2].split(',');
for (const name of names) await hits(VIEWS.find((v) => v.name === name));
