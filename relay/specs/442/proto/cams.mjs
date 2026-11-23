// The new views' cameras (tiles of proto/woods.json; the turns views are proto/fields.mjs's), set on main: each reads back
// unclamped. The laptop's minK is 7.47 and the phone's 3.34. From engine/: node ../relay/specs/442/proto/cams.mjs
import { open } from '../../../../engine/www/play.mjs';
const V = [['medium',9,827,[['laptop',35],['laptop',18],['laptop',12],['laptop',11],['phone',6],['phone',5]]],['small',2,439,[['laptop',35],['laptop',18]]],['small',9,258,[['laptop',35]]],['small',2,506,[['laptop',12],['laptop',11],['phone',6],['phone',5]]]];
for (const [size, seed, tile, list] of V) for (const [win, k] of list) {
  const page = await open(`?size=${size}&seed=${seed}&run=1`, { size: win });
  await page.until("document.body.dataset.phase && document.body.dataset.phase !== 'pregame'", 120000);
  const r = await page.eval(`(() => { const m = game.map; const cx = m.px[${tile}], cy = m.py[${tile}]; m.cx = cx; m.cy = cy; m.k = ${k}; m.viewChanged(); return { ok: m.cx === cx && m.cy === cy && m.k === ${k}, minK: m.minK }; })()`);
  console.log(size, seed, tile, win, k, JSON.stringify(r));
  await page.close();
}
