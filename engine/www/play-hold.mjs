// Cuts a rival camp's line in the page, headless: leads a civ, raises its camp to a thicker guard
// than an AI camp's, marches it onto the nearest rival camp's tile (the AI's post, three steps out
// from its capital), paints the ring of tiles round it as the zone, and ends turns until the feed
// says the rival's line is cut by our army and its camp goes hungry. Screenshots each stage. Usage:
//
//   node www/play-hold.mjs [out-dir] [--seed N] [--size NAME]   (default: relay/specs/56/screens, small seed 2; needs www/pkg)
//
// The ring works where the line can't get round: the rival camp's line starts at its own tile and
// must leave through a neighbour, and every walkable neighbour is in our zone. Standing on the rival's
// tile, our share there and on the ring (guarding over seven tiles) beats the AI's (over its three)
// once we have more than seven warriors. Uses play.mjs's driver; exits non-zero on a page error or
// if the cut never comes.
import { mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from './play.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const args = process.argv.slice(2);
const flag = (name, dflt) => {
  const i = args.indexOf(name);
  return i >= 0 ? args.splice(i, 2)[1] : dflt;
};
const seed = Number(flag('--seed', '2'));
const size = flag('--size', 'small');
const out = resolve(args[0] ?? join(here, '../../relay/specs/56/screens'));
mkdirSync(out, { recursive: true });
const WARRIORS = 10;

const page = await open(`?size=${size}&seed=${seed}&run=1`);
const log = (o) => console.log(JSON.stringify(o));
try {
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button('Lead ');
  const me = await page.eval('game.view().player');
  const myCamp = () => page.eval(`(() => { const u = Object.values(game.view().units).find((u) => u.civ === ${me});
    return u && { id: u.id, tile: u.tile, people: u.people, dest: u.dest, hungry: u.hungry, pack: u.store.reduce((a, b) => a + b, 0),
      zone: u.territory, held: u.held, guard: u.guard }; })()`);
  const selectMine = async () => {
    const c = await myCamp();
    await page.select(c.tile, 'unit');
    return c;
  };

  // Raise the camp to WARRIORS at the capital, ending turns while the store fills the packs.
  await selectMine();
  for (let turn = 0; turn < 60 && (await myCamp()).people < WARRIORS; turn++) {
    const ok = await page.eval(`(() => { const b = [...document.querySelectorAll('button')].find((b) => b.textContent.trim() === 'Raise 1');
      if (b && !b.disabled) { b.click(); return true; } return false; })()`);
    if (!ok) await page.button('End turn');
    await selectMine();
  }
  const raised = await myCamp();
  log({ raised: raised.people, turn: Number((await page.data()).turn) });
  if (raised.people < 8) throw new Error(`raised only ${raised.people} warriors`);

  // The nearest rival camp at its post (not at its capital), and its ring.
  const plan = await page.eval(`(() => {
    const v = game.view(), L = game.layout(), map = game.map;
    const steps = (a, b) => { const dq = L.q[a] - L.q[b], dr = L.r[a] - L.r[b]; return Math.max(Math.abs(dq), Math.abs(dr), Math.abs(dq + dr)); };
    const mine = Object.values(v.units).find((u) => u.civ === ${me});
    const capitals = Object.values(v.civs).map((c) => c.capital);
    const rivals = Object.values(v.units).filter((u) => u.civ !== ${me} && !capitals.includes(u.tile) && u.line)
      .sort((a, b) => steps(a.tile, mine.tile) - steps(b.tile, mine.tile));
    const r = rivals[0];
    if (!r) return null;
    const ring = [];
    for (let d = 0; d < 6; d++) { const j = map.nb[r.tile * 6 + d]; if (j >= 0 && map.land[j] && !capitals.includes(j)) ring.push(j); }
    return { rival: r.id, civ: r.civ, name: v.civs[r.civ].name, at: r.tile, ring, zone: r.territory, line: r.line.path };
  })()`);
  if (!plan) throw new Error('no rival camp at a post');
  log({ plan });
  const rival = `game.view().units[${plan.rival}]`;
  const rivalState = () => page.eval(`(() => { const u = ${rival}; return u && { tile: u.tile, people: u.people, hungry: u.hungry, cut_off: u.cut_off,
    cut_at: u.cut_at, pack: u.store.reduce((a, b) => a + b, 0), zone: u.territory, held: u.held, guard: u.guard, line: u.line && u.line.path }; })()`);
  await page.click(plan.at);
  await page.eval(`game.map.centreOn(${plan.at}, { minK: 34, smooth: false })`);
  await page.shot(join(out, 'hold-rival.png'), 1);

  // March towards it, stopping on the last tile of our own land on the way to fill the packs along
  // the line, then onto the rival camp's tile with its auto-forage off (pillaging the tile would
  // drive off the rival's clan there, and with it the AI's post).
  await selectMine();
  await page.command(plan.at);
  const stage = await page.eval(`(() => { const u = Object.values(game.view().units).find((u) => u.civ === ${me});
    const r = JSON.parse(JSON.stringify(game.map.routes.find((r) => r.unit === u.id) || { tiles: [u.tile] }));
    return r.tiles.filter((t) => game.tileCiv(t) === ${me}).at(-1); })()`);
  await page.command(stage);
  for (let turn = 0; turn < 30; turn++) {
    const c = await myCamp();
    const full = await page.eval(`(() => { const u = game.view().units[${raised.id}]; return u.store.reduce((a, b) => a + b, 0) + 0.5 >= u.pack_cap; })()`);
    if (c.tile === stage && full) break;
    await page.button('End turn');
  }
  log({ staged: (await myCamp()).tile === stage, stage, camp: await myCamp() });
  await selectMine();
  await page.command(plan.at);
  await page.button('Auto-forage: on');
  for (let turn = 0; turn < 30 && (await myCamp()).tile !== plan.at; turn++) await page.button('End turn');
  const there = await myCamp();
  log({ arrived: there.tile === plan.at, turn: Number((await page.data()).turn), camp: there });
  if (there.tile !== plan.at) throw new Error('the camp never reached the rival camp');

  // Paint the ring, a tile at a time (each is next to the camp, so the zone stays connected).
  await selectMine();
  await page.button('Paint');
  for (const t of plan.ring) await page.click(t);
  await page.button('Paint'); // toggles paint mode off
  await page.eval(`game.map.centreOn(${plan.at}, { minK: 34, smooth: false })`);
  const painted = await myCamp();
  log({ painted: painted.zone, error: await page.eval(`document.getElementById('error').textContent`) });
  if (painted.zone.length !== plan.ring.length + 1) throw new Error('the ring was not painted');
  await page.shot(join(out, 'hold-painted.png'), 1);

  // End turns until the feed says the rival's line is cut by our army and its camp is hungry.
  const feed = () => page.eval(`[...document.querySelectorAll('#feed li, #feed div')].map((e) => e.textContent).join('\\n')`);
  const cutText = `line is cut by`;
  let cut = false, hungry = false, s;
  for (let turn = 0; turn < 25 && !(cut && hungry); turn++) {
    await page.button('End turn');
    s = await rivalState();
    const text = await feed();
    cut = cut || text.includes(`camp's line is cut by`);
    hungry = hungry || (s && s.hungry > 0);
    const m = await myCamp();
    log({ turn: turn + 1, rival: s, mine: m && { people: m.people, held: m.held.length, hungry: m.hungry, pack: m.pack }, cut, hungry });
    if (turn === 0) {
      await page.click(plan.at);
      await page.select(plan.at, 'unit');
      await page.click(plan.at);
      await page.shot(join(out, 'hold-held.png'), 1);
    }
  }
  // The rival camp selected: its line drawn to where our ground stops it, and the feed.
  await page.eval(`(() => { const s = document.body.dataset.selected; return s; })()`);
  for (let n = 0; n < 4 && !(await page.eval(`document.body.dataset.selected === 'unit:${plan.rival}'`)); n++) await page.click(plan.at);
  await page.shot(join(out, 'hold-cut.png'), 1);
  const lines = (await feed()).split('\n').filter((l) => l.includes(cutText) || l.includes('goes hungry'));
  log({ feed: lines.slice(0, 6) });
  if (!cut || !hungry) throw new Error(`the rival camp was not cut and hungry (cut ${cut}, hungry ${hungry})`);
  console.log(`cut: ${plan.name}'s camp at its post is cut by our army and hungry`);
  for (const e of page.errors()) console.error(`page error: ${e}`);
  if (page.errors().length) process.exitCode = 1;
} finally {
  await page.close();
}
