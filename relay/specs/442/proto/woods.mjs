// The woods and marshes of the seven retro maps, at the end of the pre-game: each forest tile's forest neighbours
// (off the map is not forest), its cluster, rivers and fields on forest and marsh, the kinds forest and marsh meet,
// and the tile of each class nearest world (0, 0) (ties to the lower index). From engine/:
//   node ../relay/specs/442/proto/woods.mjs > ../relay/specs/442/proto/woods.json
import { open } from '../../../../engine/www/play.mjs';
const MAPS = [['small', 2], ['small', 3], ['small', 6], ['small', 9], ['medium', 8], ['medium', 9], ['large', 8]];
const out = {};
for (const [size, seed] of MAPS) {
  const page = await open(`?size=${size}&seed=${seed}&run=1`);
  await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
  const r = await page.eval(`(() => {
    const m = game.map, l = m.l, n = m.px.length, F = m.byName.forest, MA = m.byName.marsh;
    const kind = (i) => l.terrain[i] === 0 && l.lake[i] ? 'lake' : m.names[l.terrain[i]];
    const nb = (i, d) => m.nb[i * 6 + d];
    const plots = new Set(); for (const k of Object.values(game.view().clans)) for (const [i] of k.plots) plots.add(i);
    const river = (i) => l.river[i] > 0 && l.flow[i] >= 0;
    const intoRiver = (i) => { for (let d = 0; d < 6; d++) { const j = nb(i, d); if (j >= 0 && river(j) && l.flow[j] === i) return true; } return false; };
    const near = {}; const put = (name, i) => { const d = m.px[i] ** 2 + m.py[i] ** 2; if (!near[name] || d < near[name].d) near[name] = { tile: i, d: +d.toFixed(2) }; };
    const hist = [0, 0, 0, 0, 0, 0, 0]; const c = {}; const inc = (k, by = 1) => c[k] = (c[k] || 0) + by;
    const meets = {}, marshMeets = {};
    for (let i = 0; i < n; i++) {
      if (l.terrain[i] === F) {
        let f = 0; for (let d = 0; d < 6; d++) { const j = nb(i, d); if (j >= 0 && l.terrain[j] === F) f++; else if (j >= 0) meets[kind(j)] = (meets[kind(j)] || 0) + 1; }
        hist[f]++;
        put(f === 0 ? 'lone' : f === 6 ? 'interior' : 'edge', i);
        if (river(i) || intoRiver(i)) { inc('forest with a river'); put('forest-river', i); }
        if (plots.has(i)) { inc('forest with fields'); put('forest-fields', i); }
      }
      if (l.terrain[i] === MA) {
        inc('marsh');
        for (let d = 0; d < 6; d++) { const j = nb(i, d); if (j >= 0) marshMeets[kind(j)] = (marshMeets[kind(j)] || 0) + 1; }
        if (river(i) || intoRiver(i)) inc('marsh with a river');
        if (plots.has(i)) inc('marsh with fields');
        put('marsh', i);
      }
      if (plots.has(i)) { inc('field tiles'); }
    }
    // clusters of forest
    const seen = new Uint8Array(n); const sizes = [];
    for (let i = 0; i < n; i++) if (l.terrain[i] === F && !seen[i]) { let s = 0; const st = [i]; seen[i] = 1;
      while (st.length) { const a = st.pop(); s++; for (let d = 0; d < 6; d++) { const b = nb(a, d); if (b >= 0 && !seen[b] && l.terrain[b] === F) { seen[b] = 1; st.push(b); } } }
      sizes.push(s); }
    sizes.sort((a, b) => b - a);
    return { n, forest: hist.reduce((a, b) => a + b, 0), neighbours: hist, clusters: sizes.length, biggest: sizes.slice(0, 5), count: c, meets, marshMeets, near };
  })()`);
  out[`${size}${seed}`] = r;
  console.error(size, seed, JSON.stringify(r.neighbours), JSON.stringify(r.count));
  await page.close();
}
console.log(JSON.stringify(out, null, 1));
