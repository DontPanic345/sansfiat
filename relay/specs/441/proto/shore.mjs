// The shores of the seven retro maps: water, lakes, necks, bays, river mouths, shore corners and the land edges
// between ground types, and the tile of each feature nearest world (0, 0). From engine/:
//   node ../relay/specs/441/proto/shore.mjs > ../relay/specs/441/proto/shore.json
import { open } from '../../../../engine/www/play.mjs';
const MAPS = [['small', 2], ['small', 3], ['small', 6], ['small', 9], ['medium', 8], ['medium', 9], ['large', 8]];
const out = {};
for (const [size, seed] of MAPS) {
  const page = await open(`?size=${size}&seed=${seed}&run=1`);
  await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
  const r = await page.eval(`(() => {
    const m = game.map, l = m.l, n = m.px.length;
    const kind = (i) => l.terrain[i] === 0 && l.lake[i] ? 'lake' : m.names[l.terrain[i]];
    const wet = (i) => i >= 0 && l.terrain[i] === 0;
    const nb = (i, d) => m.nb[i * 6 + d];
    const kinds = {}; for (let i = 0; i < n; i++) kinds[kind(i)] = (kinds[kind(i)] || 0) + 1;
    // runs of land / water round a tile (off-map counts as water: the rim is sea)
    const runs = (i) => { const w = []; for (let d = 0; d < 6; d++) { const j = nb(i, d); w.push(j < 0 || wet(j)); }
      let ch = 0; for (let d = 0; d < 6; d++) if (w[d] !== w[(d + 1) % 6]) ch++;
      return { water: w.filter(Boolean).length, runs: ch / 2 }; };
    const near = {}; const put = (name, i) => { const d = m.px[i] ** 2 + m.py[i] ** 2; if (!near[name] || d < near[name].d) near[name] = { tile: i, d: +d.toFixed(2), x: +m.px[i].toFixed(2), y: +m.py[i].toFixed(2) }; };
    const count = {}; const inc = (k) => count[k] = (count[k] || 0) + 1;
    for (let i = 0; i < n; i++) {
      const r = runs(i);
      if (!wet(i)) {
        if (r.water > 0) inc('shore land');
        if (r.runs >= 2) { inc('neck (land, 2+ water runs)'); put('neck', i); }
        if (r.water >= 4 && r.runs === 1) { inc('cape (land, 4+ water in one run)'); put('cape', i); }
        if (r.water === 6) { inc('islet'); put('islet', i); }
      } else {
        if (r.water < 6) inc('shore water');
        if (r.runs >= 2) { inc('strait (water, 2+ land runs)'); put('strait', i); }
        if (r.water <= 2 && r.runs === 1) { inc('bay (water, 4+ land in one run)'); put(l.lake[i] ? 'bay-lake' : 'bay-sea', i); }
      }
      if (l.river[i] > 0 && l.flow[i] >= 0 && wet(l.flow[i])) { const lk = l.lake[l.flow[i]]; inc(lk ? 'mouth into lake' : 'mouth into sea'); put(lk ? 'mouth-lake' : 'mouth-sea', i); if (l.river[i] >= 9) { inc('big mouth (river >= 9)'); put('mouth-big', i); } }
    }
    // lakes: components
    const seen = new Uint8Array(n); const lakes = [];
    for (let i = 0; i < n; i++) if (l.lake[i] && l.terrain[i] === 0 && !seen[i]) { let s = 0; const st = [i]; seen[i] = 1; let best = i;
      while (st.length) { const a = st.pop(); s++; if (m.px[a] ** 2 + m.py[a] ** 2 < m.px[best] ** 2 + m.py[best] ** 2) best = a; for (let d = 0; d < 6; d++) { const b = nb(a, d); if (b >= 0 && !seen[b] && l.lake[b] && l.terrain[b] === 0) { seen[b] = 1; st.push(b); } } }
      lakes.push(s); if (s <= 3) put('small-lake', best); }
    lakes.sort((a, b) => a - b);
    // edges between differing kinds (each once, dirs 0..2) and shore vertices (dirs d, d+1 make a triangle)
    const edges = {}; const verts = { 'one water': 0, 'two water': 0 };
    for (let i = 0; i < n; i++) for (let d = 0; d < 3; d++) { const j = nb(i, d); if (j < 0) continue; const a = kind(i), b = kind(j); if (a !== b) { const k = [a, b].sort().join('|'); edges[k] = (edges[k] || 0) + 1; } }
    for (let i = 0; i < n; i++) for (const d of [0, 1]) { const j = nb(i, d), k = nb(i, d + 1); if (j < 0 || k < 0) continue; const w = [i, j, k].filter(wet).length; if (w === 1) verts['one water']++; if (w === 2) verts['two water']++; }
    for (let i = 0; i < n; i++) { const t = kind(i); if (t === 'marsh' || t === 'desert' || t === 'hills' || t === 'forest' || t === 'coast') put(t, i); }
    const top = Object.fromEntries(Object.entries(edges).sort((a, b) => b[1] - a[1]));
    return { n, kinds, count, lakes, verts, edges: top, near };
  })()`);
  out[`${size}${seed}`] = r;
  console.error(size, seed, JSON.stringify(r.count), 'lakes', JSON.stringify(r.lakes));
  await page.close();
}
console.log(JSON.stringify(out, null, 1));
