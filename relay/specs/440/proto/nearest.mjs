// The mountain tile of each class nearest world (0, 0) on the view maps. From engine/: node ../relay/specs/440/proto/nearest.mjs
import { open } from '../../../../engine/www/play.mjs';
for (const [size, seed] of [['small',2],['small',9],['medium',9],['large',8]]) {
  const page = await open(`?size=${size}&seed=${seed}&run=1`);
  await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
  const r = await page.eval(`(() => {
    const m = game.map, l = m.l, M = m.byName.mountain, n = m.n;
    const best = {};
    for (let i = 0; i < n; i++) {
      if (l.terrain[i] !== M) continue;
      const ds = []; for (let d = 0; d < 6; d++) { const j = m.nb[i*6+d]; if (j >= 0 && l.terrain[j] === M) ds.push(d); }
      let c = ds.length===0?'iso':ds.length===1?'end':ds.length===2?(()=>{const g=Math.min((ds[1]-ds[0]+6)%6,(ds[0]-ds[1]+6)%6);return g===3?'straight':g===2?'bend':'sharp';})():ds.length===3?'fork':'more';
      const d2 = m.px[i]**2 + m.py[i]**2;
      if (!best[c] || d2 < best[c].d2) best[c] = { i, d2: +d2.toFixed(1), px: +m.px[i].toFixed(2), py: +m.py[i].toFixed(2), ds: ds.join('') };
    }
    return best;
  })()`);
  console.log(size, seed, JSON.stringify(r));
  await page.close();
}
