import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/scout/rules.js';
import { newGame } from '../src/game.js';

test('scout module is registered and initialises its state', () => {
  assert.equal(mod.name, 'scout');
  const s = newGame({ seed: 1 });
  assert.ok(s.mods.scout, 'init should create s.mods.scout');
});
