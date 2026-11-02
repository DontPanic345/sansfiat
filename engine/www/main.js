// The page's router. It loads the wasm, the atlas and the sounds, then shows one screen at a time: start,
// play (with the pick over its map) or end, each a module with show(args) and hide(). `?size&seed` goes
// straight to play (`run=1` skips the pre-game, `play=1` runs it, `speed` sets its pace); a bare URL
// opens start. `window.game` is what the scripts in www/play/ read.

import init, { Sim, terrain, goods, rules, sizes } from './pkg/web.js';
import { loadAtlas } from './atlas.js';
import { Sound } from './sound.js';
import { app } from './app.js';
import { autosave } from './save.js';
import * as start from './screens/start.js';
import * as play from './screens/play.js';
import * as end from './screens/end.js';

await init();
app.data = {
  Sim,
  TERRAIN: JSON.parse(terrain()),
  GOODS: JSON.parse(goods()),
  RULES: JSON.parse(rules()),
  SIZES: JSON.parse(sizes()),
};
app.atlas = await loadAtlas();
app.sound = new Sound(await (await fetch('./assets/sounds.json')).json());

const SCREENS = { start, play, end };

/** Shows the screen `name` (its section, and body.dataset.screen), handing it `args`. */
app.show = (name, args) => {
  if (app.screen) SCREENS[app.screen].hide();
  app.screen = name;
  document.body.dataset.screen = name;
  for (const n of Object.keys(SCREENS)) document.getElementById(`screen-${n}`).hidden = n !== name;
  SCREENS[name].show(args);
};

start.init();
play.init();
end.init();

window.game = {
  screen: () => app.screen,
  view: () => play.state().v,
  model: () => play.state().model,
  layout: () => play.state().layout,
  get map() {
    return play.map();
  },
  get feed() {
    return play.feedList();
  },
  atlas: app.atlas,
  sound: app.sound,
  autosave: () => autosave.read(),
  tileCiv: (i) => play.state().sim.tile_civ(i),
  save: () => JSON.parse(play.state().sim.save()),
  digest: () => play.state().sim.digest(),
  border: (civ) => play.map().border(civ),
  notes: (id) => play.state().sim.notes(id),
};

const params = new URLSearchParams(location.search);
if (params.has('size') || params.has('seed')) {
  const size = app.data.SIZES.find((s) => s.name === params.get('size')) || app.data.SIZES[0];
  const seed = Math.max(0, Math.floor(Number(params.get('seed')) || 1));
  app.show('play', {
    sim: new Sim(seed, size.radius),
    speed: params.get('speed'),
    run: params.get('run') === '1',
    play: params.get('play') === '1',
  });
} else {
  app.show('start');
}
