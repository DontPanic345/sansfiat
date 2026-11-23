// The mountain tiles of the seven retro maps by how many mountain neighbours they have. From engine/: node ../relay/specs/440/proto/classes.mjs
import { open } from '../../../../engine/www/play.mjs';
const MAPS = [['small',2],['small',3],['small',6],['small',9],['medium',8],['medium',9],['large',8]];
const out = {};
for (const [size, seed] of MAPS) {
  const page = await open(`?size=${size}&seed=${seed}&run=1`);
  await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
  const r = await page.eval(`(() => {
    const m = game.map, l = m.l, M = m.byName.mountain, n = m.n;
    const cls = {iso:0,end:0,straight:0,gentle:0,sharp:0,fork3:0,more4:0};
    const ex = {}; let edge = 0, mtn = 0, rowMajor = true;
    for (let i = 1; i < n; i++) if (m.py[i] < m.py[i-1] - 1e-9) { rowMajor = false; }
    for (let i = 0; i < n; i++) {
      if (l.terrain[i] !== M) continue; mtn++;
      const ds = []; let e = false;
      for (let d = 0; d < 6; d++) { const j = m.nb[i*6+d]; if (j < 0) e = true; else if (l.terrain[j] === M) ds.push(d); }
      if (e) edge++;
      let c;
      if (ds.length === 0) c = 'iso'; else if (ds.length === 1) c = 'end';
      else if (ds.length === 2) { const g = Math.min((ds[1]-ds[0]+6)%6, (ds[0]-ds[1]+6)%6); c = g===3?'straight':g===2?'gentle':'sharp'; }
      else if (ds.length === 3) c = 'fork3'; else c = 'more4';
      cls[c]++; (ex[c] ??= []).length < 3 && ex[c].push(i);
    }
    return { n, mtn, edge, rowMajor, radius: m.radius, cls, ex, minK: m.minK };
  })()`);
  out[`${size}${seed}`] = r; console.log(size, seed, JSON.stringify(r));
  await page.close();
}
