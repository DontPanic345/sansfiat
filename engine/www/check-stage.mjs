// The page's panel arm, over staged games (`Sim.stage`): the camp, scout, civ, clan and tile
// shapes are checked as data, and Disband all is pressed as the page presses it. Imported at
// the end of check.mjs, which has already initialised the wasm.
import { fileURLToPath, pathToFileURL } from 'node:url';
import assert from 'node:assert/strict';

const here = fileURLToPath(new URL('.', import.meta.url));
const { Sim, rules, terrain, goods } = await import(pathToFileURL(`${here}pkg/web.js`).href);
const { describe } = await import(pathToFileURL(`${here}panels.js`).href);
const { panelCtxFor } = await import(pathToFileURL(`${here}check-ctx.mjs`).href);

const RULES = JSON.parse(rules());
const TERRAIN = JSON.parse(terrain());
const GOODS = JSON.parse(goods());

// A two-civ game on a radius-2 map: plains inside, sea on the rim. Civ A (the lead) and civ B have their
// capitals on marks A and B; A's army of 4.6 people stands at its capital; food (grain) lies on it.
const MAP = '  ~ ~ ~\n ~ . . ~\n~ . . . ~\n ~ . . ~\n  ~ ~ ~\n';
const MARKS = '  . . .\n . . . .\n. A . B .\n . . . .\n  . . .\n';
const SPEC = {
  map: MAP,
  marks: MARKS,
  civs: [{ mark: 'A' }, { mark: 'B' }],
  people: 12,
  lead: 0,
  armies: [{ civ: 0, at: { mark: 'A' }, people: 4.6 }],
  food: [{ at: { mark: 'A' }, good: 1, amount: 2 }],
};

/** A fresh staged Sim of `spec`. */
const staged = (spec = SPEC) => Sim.stage(JSON.stringify(spec));
const ctxOf = (sim) => panelCtxFor(sim, { rules: RULES, terrain: TERRAIN, goods: GOODS });
const shapeOf = (sim, sel) => describe(ctxOf(sim)(JSON.parse(sim.view())), sel);

// The staged game itself: it is staged, it has the army and the lead, and the same spec gives the same world.
assert.ok(staged().staged(), 'a staged game says so');
assert.equal(new Sim(3, 2).staged(), false, 'a played game is not staged');
assert.equal(staged().digest(), staged().digest(), 'the same spec gives the same digest');
{
  const sim = staged();
  const view = JSON.parse(sim.view());
  assert.equal(view.phase, 'main', 'a staged game is in its main phase');
  assert.ok(view.player != null, 'the lead is set');
  const army = Object.values(view.units).find((u) => u.kind === 'camp' && u.civ === view.player && u.people === 4.6);
  assert.ok(army, 'the lead has a camp of 4.6 people');
  assert.equal(army.tile, view.civs[view.player].capital, 'the army stands at the capital');
}

/** Every button of a shape: a label, and exactly one of action, arm, paint, stepper; a stepper's options each
 *  have a label, a cap and a make that gives an object with a type. */
function checkButtons(shape, name) {
  assert.ok(shape && Array.isArray(shape.buttons), `${name}: a shape with buttons`);
  for (const b of shape.buttons) {
    assert.equal(typeof b.label, 'string', `${name}: a button has a label`);
    const kinds = ['action', 'arm', 'paint', 'stepper'].filter((k) => b[k] != null);
    assert.equal(kinds.length, 1, `${name}: ${b.label} is exactly one of action, arm, paint, stepper`);
    for (const o of b.stepper?.options ?? []) {
      assert.equal(typeof o.label, 'string', `${name}: ${b.label}'s option has a label`);
      assert.ok(Number.isFinite(o.cap), `${name}: ${b.label}'s ${o.label} has a cap`);
      const made = o.make(1);
      assert.equal(typeof made?.type, 'string', `${name}: ${b.label}'s ${o.label} makes an order with a type`);
    }
  }
}

// The panels: each kind of selection gives a shape whose buttons are all well formed.
{
  const sim = staged();
  const view = JSON.parse(sim.view());
  const led = view.player;
  const army = Object.values(view.units).find((u) => u.kind === 'camp' && u.civ === led && u.people === 4.6);
  const scout = Object.values(view.units).find((u) => u.kind === 'scout' && u.civ === led);
  const clan = Object.values(view.clans).find((c) => c.civ === led);
  const shapes = {
    camp: shapeOf(sim, { kind: 'unit', id: army.id }),
    scout: shapeOf(sim, { kind: 'unit', id: scout.id }),
    civ: shapeOf(sim, { kind: 'civ', civ: led }),
    clan: shapeOf(sim, { kind: 'clan', id: clan.id }),
    tile: shapeOf(sim, { kind: 'tile', tile: army.tile }),
  };
  for (const [name, shape] of Object.entries(shapes)) {
    assert.ok(shape && shape.name, `${name}: a shape with a name`);
    checkButtons(shape, name);
  }
  assert.ok(shapes.camp.buttons.some((b) => b.stepper && b.label === 'Disband…'), 'a camp has Disband…');
  assert.ok(shapes.scout.buttons.some((b) => b.stepper && b.label === 'Disband…'), 'a scout has Disband…');
}

/** Disband all as the page presses it: the button sends its order with no count, and the whole unit goes home. */
function pressAll(kind) {
  const sim = staged();
  const view = JSON.parse(sim.view());
  const u = Object.values(view.units).find((x) => x.kind === kind && x.civ === view.player);
  const all = shapeOf(sim, { kind: 'unit', id: u.id }).buttons.find((b) => b.label === 'Disband all');
  const what = kind === 'camp' ? 'army' : kind;
  sim.act(JSON.stringify(all.action));
  const after = JSON.parse(sim.view());
  assert.equal(after.units[u.id], undefined, `Disband all sends the whole ${what} home: the ${what} is still on the map`);
  assert.ok(Math.abs(after.civs[u.civ].people - view.civs[u.civ].people - u.people) < 1e-9, `Disband all sends the ${what}'s ${u.people} people to its clan`);
}

// Disband all: the camp of 4.6, and the scout (2) each go home whole, and their people go to their clan.
for (const kind of ['camp', 'scout']) pressAll(kind);

// The end: a staged game with a limit of two turns is over after the second EndTurn. Its orders are refused with
// "the game is over"; the world runs on, so an EndTurn still goes through.
{
  const sim = staged({ ...SPEC, limit: 2 });
  const picked = JSON.parse(sim.view()).turn;
  sim.act('{"type":"endTurn"}');
  assert.equal(JSON.parse(sim.view()).phase, 'main', 'a game with two turns is still going after one');
  sim.act('{"type":"endTurn"}');
  const view = JSON.parse(sim.view());
  assert.equal(view.phase, 'over', 'the game is over after its limit');
  assert.equal(view.over.why, 'limit', 'the limit ends it');
  assert.equal(view.over.turn, picked + 2, 'it ends on the limit\'s turn');
  assert.deepEqual(view.limit, { turns: 2, at: picked + 2 }, 'the view gives the limit and the turn it falls on');
  assert.equal(view.standing.length, 3, 'Standing at the pick and after each of the two turns');
  assert.equal(view.standing[0][0], picked, 'the first Standing is the pick\'s turn');
  const army = Object.values(view.units).find((u) => u.kind === 'camp' && u.civ === view.player);
  const order = JSON.stringify({ type: 'order', civ: view.player, order: { type: 'raise', kind: 'scout', n: 1 } });
  assert.equal(sim.check(order), 'the game is over', 'an order is refused once the game is over');
  assert.throws(() => sim.act(order), /the game is over/, 'and an order is not taken');
  sim.act('{"type":"endTurn"}');
  assert.equal(JSON.parse(sim.view()).turn, picked + 3, 'the world runs on after the end');
  assert.ok(army, 'the army is still on the map');
}

// The clan's fields and the civ's Living row, as data. A staged game has no ground for a clan to farm, so the
// view is given the share and plots a farmer's view would hold, and describe reads them as the page does.
{
  const sim = staged();
  const view = JSON.parse(sim.view());
  const led = view.player;
  const clan = Object.values(view.clans).find((c) => c.civ === led);
  const sel = { kind: 'clan', id: clan.id };
  const plain = describe(ctxOf(sim)(view), sel);
  assert.ok(!plain.numbers.some((n) => n.label === 'Fields'), 'a clan with no share has no Fields row');
  assert.ok(plain.doing && !plain.doing.startsWith('Farms'), 'a clan with no share does not farm');
  const farmed = JSON.parse(JSON.stringify(view));
  farmed.clans[clan.id].fields = 0.2;
  farmed.clans[clan.id].plots = [[clan.home, 4, 2.6], [clan.territory.find((t) => t !== clan.home) ?? clan.home, 5.4, 3.5]];
  const shape = describe(ctxOf(sim)(farmed), sel);
  const row = shape.numbers.find((n) => n.label === 'Fields');
  assert.ok(row, 'a clan with a share has a Fields row');
  assert.equal(row.value, '2 tiles · 9.4 days · 6.1 grain', 'the Fields row gives the tiles, days and grain');
  assert.ok(row.tip.startsWith('Days a turn on fields, and the grain they yielded'), 'the Fields tooltip says what the days and grain are');
  const lay = JSON.parse(sim.layout());
  assert.ok(row.tip.includes(`(${lay.q[clan.home]}, ${lay.r[clan.home]}): 4.0 days, 2.6 grain`.replace('-', '−')), 'the Fields tooltip lists each tile by its hex');
  assert.ok(row.tip.includes('4.0 days, 2.6 grain'), 'the Fields tooltip gives each tile\'s days and grain');
  assert.ok(shape.doing.startsWith('Farms a fifth of its days; '), `a farmer's doing line leads with its share: ${shape.doing}`);
  const now = plain.doing.charAt(0).toLowerCase() + plain.doing.slice(1);
  assert.equal(shape.doing.slice('Farms a fifth of its days; '.length), now, 'and then says what the clan says now');
  // The civ's Living row: its three parts, in whole people, with the army counted on top of the clans' people.
  const civ = describe(ctxOf(sim)(view), { kind: 'civ', civ: led });
  const living = civ.numbers.find((n) => n.label === 'Living');
  assert.ok(living, 'the civ panel has a Living row');
  const people = civ.numbers.findIndex((n) => n.label === 'People');
  assert.equal(civ.numbers.findIndex((n) => n.label === 'Living'), people + 1, 'Living comes after People');
  assert.match(living.value, /^gathering \d+ · farming \d+ · in the army \d+$/, 'Living reads gathering, farming and in the army, in whole people');
  const L = view.civs[led].living;
  assert.equal(living.value, `gathering ${Math.round(L.gathering)} · farming ${Math.round(L.farming)} · in the army ${Math.round(L.army)}`, 'Living is the view\'s parts');
  assert.ok(Math.abs(L.gathering + L.farming - view.civs[led].people) < 1e-9, 'the clans\' people are gathering or farming');
  const units = Object.values(view.units).filter((u) => u.civ === led).reduce((a, u) => a + u.people, 0);
  assert.ok(Math.abs(L.army - units) < 1e-9 && L.army >= 4.6, 'the army is the people of all its units, the camp of 4.6 among them');
  assert.ok(L.gathering + L.farming + L.army > view.civs[led].people, 'the army\'s people are on top of the clans\' people (People counts the clans)');
}

// A tile's name says its field ground and its fields: a floodplain tile with ground 20 days, worked by clan 140 for 9.4
// days, and a tile with ground but no work (the view's model gives no field there).
{
  const { tileKind } = await import(pathToFileURL(`${here}panels.js`).href);
  const layout = { terrain: [0, 1, 1], river: [0, 0, 0], flood: [0, 1, 1], pass: [0, 0, 0], zone: [0, 0, 0], lake: [0, 0, 0], ground: [0, 20, 20] };
  const terrain = [{ name: 'ocean', land: false }, { name: 'plains', land: true }];
  const model = { fieldAt: (i) => (i === 1 ? { clan: 140, days: 9.4, grain: 6.1 } : null) };
  const ctx = { layout, terrain, country: false, model };
  assert.equal(tileKind(ctx, 1), 'plains, floodplain, field ground 20 days, fields 9.4 days (clan 140)', 'a tile with fields says its ground and its fields');
  assert.equal(tileKind(ctx, 2), 'plains, floodplain, field ground 20 days', 'a tile with ground and no fields says its ground');
}

// A staged town (#156): two clans living on the capital's tile with the founder make three homes on a festival's
// tile, so the view names the town and the tile panel lists its clans. Without them there is no town.
{
  const town = {
    ...SPEC,
    clans: [
      { home: { mark: 'A' }, tiles: [], people: 3 },
      { home: { mark: 'A' }, tiles: [], people: 2 },
    ],
  };
  const sim = staged(town);
  const v = JSON.parse(sim.view());
  const capital = v.civs[v.player].capital;
  assert.equal(v.towns.length, 1, 'a festival with three homes on its tile is a town');
  assert.equal(v.towns[0].tile, capital, 'the town is on the capital');
  assert.equal(v.towns[0].clans.length, 3, 'three clans live there');
  assert.equal(v.towns[0].market, v.civs[v.player].market, 'the town is the capital\'s festival');
  const list = shapeOf(sim, { kind: 'tile', tile: capital }).list;
  assert.equal(list[0].label, 'Clans living here', 'the tile panel lists the clans living there');
  assert.equal(list[0].rows.length, 3, 'and its three clans');
  assert.equal(JSON.parse(staged().view()).towns.length, 0, 'a stage with one home on the tile is no town');
}

console.log('ok: staged arm (shapes, digest, Disband all), the end and a staged town');
