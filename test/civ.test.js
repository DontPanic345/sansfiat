import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/civ/rules.js';
import { newGame } from '../src/game.js';

test('civ module is registered and initialises its state', () => {
  assert.equal(mod.name, 'civ');
  const s = newGame({ seed: 1 });
  assert.ok(s.mods.civ, 'init should create s.mods.civ');
});
