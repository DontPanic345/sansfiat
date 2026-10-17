import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as mod from '../src/modules/supply/rules.js';
import { newGame } from '../src/game.js';

test('supply module is registered and initialises its state', () => {
  assert.equal(mod.name, 'supply');
  const s = newGame({ seed: 1 });
  assert.ok(s.mods.supply, 'init should create s.mods.supply');
});
