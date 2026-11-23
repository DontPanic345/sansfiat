// The archers' bow: the lineup with red's camp given a stack of 4 warriors and 2 archers (set on the drawn model, not the game),
// shot at k 18 and 35. From engine/:  node ../relay/specs/443/proto/bow.mjs OUTDIR
import { writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { open } from '../../../../engine/www/play.mjs';
import { stage } from '../../../../engine/www/play-stage.mjs';
import { LINEUP } from '../../../../engine/www/play-stage.mjs';
const out = process.argv[2];
mkdirSync(out, { recursive: true });
const page = await open('', { size: 'laptop' });
try {
  await stage(page, LINEUP);
  for (const k of [35, 18]) {
    await page.eval(`(() => { const m = game.map; m.cx = (m.px[125] + m.px[170]) / 2; m.cy = (m.py[125] + m.py[170]) / 2; m.k = ${k}; m.viewChanged(); })()`);
    await page.until('!game.map.anim && !game.map.frame', 5000);
    const png = await page.eval(`(() => { const m = game.map; for (const [, list] of m.model.stacks) for (const u of list) if (u.kind === 'camp') { u.stack = [u.people - 2, 2]; } m.dirty = true; m.drawNow(); return m.canvas.toDataURL('image/png'); })()`);
    writeFileSync(join(out, `bow-${k}.png`), Buffer.from(png.split(',')[1], 'base64'));
  }
} finally { await page.close(); }
