// The page's panel arm, over staged games (`Sim.stage`): the camp, scout, guard, civ, clan and tile
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
  const guard = Object.values(view.units).find((u) => u.kind === 'guard' && u.civ === led);
  const clan = Object.values(view.clans).find((c) => c.civ === led);
  const shapes = {
    camp: shapeOf(sim, { kind: 'unit', id: army.id }),
    scout: shapeOf(sim, { kind: 'unit', id: scout.id }),
    guard: shapeOf(sim, { kind: 'unit', id: guard.id }),
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
  assert.ok(shapes.guard.buttons.some((b) => b.stepper && b.label === 'Disband…'), 'a guard has Disband…');
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

// Disband all: the camp of 4.6, the scout (2) and the guard (3) each go home whole, and their people go to their clan.
for (const kind of ['camp', 'scout', 'guard']) pressAll(kind);

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
  const order = JSON.stringify({ type: 'order', civ: view.player, order: { type: 'raise', kind: 'guard', n: 1 } });
  assert.equal(sim.check(order), 'the game is over', 'an order is refused once the game is over');
  assert.throws(() => sim.act(order), /the game is over/, 'and an order is not taken');
  sim.act('{"type":"endTurn"}');
  assert.equal(JSON.parse(sim.view()).turn, picked + 3, 'the world runs on after the end');
  assert.ok(army, 'the army is still on the map');
}

console.log('ok: staged arm (shapes, digest, Disband all) and the end');
