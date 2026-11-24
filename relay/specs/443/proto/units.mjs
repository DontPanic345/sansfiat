// The units on a retro map: at the pick (leading the first civ), after 8 turns, after the player raises an army of 3
// warriors (play.mjs's raiseArmy), and 10 turns after that. Each unit's civ, kind, tile, terrain, people, stack and
// hunger; the tiles units share; and the player's and a rival's unit nearest world (0, 0) on each terrain.
// From engine/:  node ../relay/specs/443/proto/units.mjs small 2   (one map a process; run the seven side by side)
import { open, lead, raiseArmy } from '../../../../engine/www/play.mjs';
const MAPS = [[process.argv[2], Number(process.argv[3])]];
const PROBE = `(() => { const m = game.map, l = m.l, v = game.view(), names = Object.fromEntries(Object.entries(m.byName).map(([k, x]) => [x, k]));
  const us = Object.values(v.units), by = {}; for (const u of us) (by[u.tile] ??= []).push(u.id);
  const near = {}; const put = (k, u) => { const d = m.px[u.tile] ** 2 + m.py[u.tile] ** 2; if (!near[k] || d < near[k].d) near[k] = { unit: u.id, tile: u.tile, kind: u.kind, d: +d.toFixed(1) }; };
  for (const u of us) { const t = names[l.terrain[u.tile]]; if (v.civs[u.civ]?.fallen) continue; put((u.civ === v.player ? 'mine-' : 'rival-') + t, u); }
  return { turn: document.body.dataset.turn, player: v.player, civs: Object.values(v.civs).map((c) => c.color + (c.fallen ? '(fallen)' : '')),
    units: us.map((u) => [u.id, u.civ, u.kind, u.tile, names[l.terrain[u.tile]], +u.people.toFixed(1), u.stack ? u.stack.map((x) => +x.toFixed(1)).join('+') : '', u.hungry > 0 ? 'hungry' : '']),
    shared: Object.entries(by).filter(([, ids]) => ids.length > 1), near, digest: game.digest() }; })()`;
for (const [size, seed] of MAPS) {
  const page = await open('');
  await lead(page, { size, seed });
  const turns = async (n) => {
    for (let t = 0; t < n; t++) {
      const before = (await page.data()).turn;
      await page.key('Space');
      try { await page.until(`document.body.dataset.turn !== '${before}'`, 30000); } catch { await page.button('End turn').catch(() => {}); }
    }
  };
  const out = { size, seed, at0: await page.eval(PROBE) };
  await turns(8);
  out.at8 = await page.eval(PROBE);
  out.raised = await raiseArmy(page, { warriors: 3, turns: 20 });
  out.atRaised = await page.eval(PROBE);
  await turns(10);
  out.atRaised10 = await page.eval(PROBE);
  console.log(JSON.stringify(out));
  await page.close();
}
