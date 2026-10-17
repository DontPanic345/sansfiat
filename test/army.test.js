import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/army/rules.js';
import { newGame } from '../src/game.js';

test('army module is registered and initialises its state', () => {
  assert.equal(mod.name, 'army');
  const s = newGame({ seed: 1 });
  assert.ok(s.mods.army, 'init should create s.mods.army');
});
