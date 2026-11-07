// The start screen: New game (size, seed, a random seed, Start), Continue (the autosave, greyed with
// "No saved game" when there is none) and Load a file. An old-version save shows the engine's error here.

import { app } from '../app.js';
import { autosave, continueLine } from '../save.js';

const $ = (id) => document.getElementById(id);

export function init() {
  for (const s of app.data.SIZES) $('size').add(new Option(`${s.name} (radius ${s.radius})`, s.name));
  $('dice').onclick = () => {
    $('seed').value = Math.floor(Math.random() * 100000);
  };
  $('newgame').onclick = startNew;
  $('seed').addEventListener('keydown', (e) => {
    if (e.key === 'Enter') startNew();
  });
  $('continue').onclick = continueGame;
  $('start-loadButton').onclick = () => $('start-load').click();
  $('start-load').onchange = loadFile;
}

export function show() {
  $('start-error').textContent = '';
  const e = autosave.read();
  $('continue').disabled = !e;
  $('continue-line').textContent = e ? continueLine(e) : 'No saved game';
}

export function hide() {}

const chosenSize = () => app.data.SIZES.find((s) => s.name === $('size').value) ?? app.data.SIZES[0];

/** Starts the pre-game at `fast` on the play screen. */
function startNew() {
  const seed = Math.max(0, Math.floor(Number($('seed').value) || 0));
  app.show('play', { sim: new app.data.Sim(seed, chosenSize().radius), speed: 'fast', play: true });
}

function continueGame() {
  const e = autosave.read();
  if (!e) return;
  open(() => app.data.Sim.load(JSON.stringify(e.save)));
}

async function loadFile(e) {
  const file = e.target.files[0];
  e.target.value = '';
  if (!file) return;
  const text = await file.text();
  open(() => app.data.Sim.load(text));
}

/** Loads a game (`load` gives the Sim, or throws why) and shows the play screen with it, or the error here. */
async function open(load) {
  let sim;
  try {
    sim = await load();
  } catch (err) {
    $('start-error').textContent = String(err);
    return;
  }
  app.show('play', { sim });
}
