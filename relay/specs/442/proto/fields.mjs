// Fields on forest and marsh after 8 turns of the main game (leading the first civ), and the tile of each class nearest
// world (0, 0). small 2 runs twice: the same digest both times, so a view after N turns is the same view. From engine/:
//   node ../relay/specs/442/proto/fields.mjs
import { open, lead } from '../../../../engine/www/play.mjs';
for (const [size, seed] of [['small', 2], ['medium', 9], ['small', 2]]) {
  const page = await open('');
  await lead(page, { size, seed });
  for (let t = 0; t < 8; t++) {
    const before = (await page.data()).turn;
    await page.key('Space');
    await page.until(`document.body.dataset.turn !== '${before}'`, 60000);
  }
  console.log(size, seed, JSON.stringify(await page.eval(`(() => { const m = game.map, l = m.l, F = m.byName.forest, MA = m.byName.marsh;
    const near = {}; const put = (name, i) => { const d = m.px[i] ** 2 + m.py[i] ** 2; if (!near[name] || d < near[name].d) near[name] = { tile: i, d: +d.toFixed(2) }; };
    const river = (i) => l.river[i] > 0 && l.flow[i] >= 0;
    for (const k of Object.values(game.view().clans)) for (const [i, days] of k.plots) {
      if (l.terrain[i] === F) { let f = 0; for (let d = 0; d < 6; d++) { const j = m.nb[i*6+d]; if (j>=0 && l.terrain[j]===F) f++; } put('forest-fields', i); if (f >= 3) put('forest-fields-3', i); if (river(i)) put('forest-fields-river', i); }
      if (l.terrain[i] === MA) put('marsh-fields', i);
    }
    return { turn: document.body.dataset.turn, near, digest: game.digest() }; })()`)));
  await page.close();
}
