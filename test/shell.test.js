// The shell's DOM-free pieces: what the human may see on the map and in debug views.
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { newGame, runPregame } from '../src/game.js';
import { dispatch } from '../src/rules.js';
import { fogOf } from '../src/render.js';
import { viewValues } from '../src/views.js';

const base = runPregame(newGame({ seed: 1, players: 2 }));
const choose = (civ) => dispatch(structuredClone(base), { type: 'chooseCiv', civ }).state;

test('no fog before the main game or when watching all-AI', () => {
  assert.equal(fogOf(base), null);
  assert.equal(fogOf(choose(null)), null);
});

test('debug views show only tiles the human civ has seen; trail costs only once revealed', () => {
  const s = choose(Number(Object.keys(base.civs)[0]));
  const fog = fogOf(s);
  assert.ok(fog, 'human civ in the main game has fog');
  const unseen = Object.keys(s.map.tiles).filter((k) => !fog.seen(k));
  assert.ok(unseen.length > 0, 'some tiles are unseen at the start');
  for (const mode of ['population', 'fertility', 'occupation', 'tame']) {
    for (const k of viewValues(s, mode, fog).keys()) assert.ok(fog.seen(k), `${mode} leaks ${k}`);
  }
  const all = viewValues(s, 'trailCost', null);
  assert.ok(all.size > 0);
  for (const k of viewValues(s, 'trailCost', fog).keys()) assert.ok(fog.revealed(k), `trailCost leaks ${k}`);
});
