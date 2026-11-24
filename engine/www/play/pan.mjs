// The pan cache (map/pan.js): a pan shifts the static layer and draws only the strips it uncovers, and everything else that
// changes the layer still redraws it whole. On a small map at play zoom, each case pans, does its change, and compares the
// static layer with a fresh full redraw at the same camera: the pan alone draws strips only (no whole redraw) and matches
// a full redraw but for anti-aliasing at the seam; a pan by half a device px, a zoom, a resize, a layer toggled, a stale
// layer after a turn, and a new map each draw whole and match exactly.
//   node www/play/pan.mjs
import { open, lead } from '../play.mjs';

const inPage = async () => {
  const m = game.map;
  const draws = { whole: 0, strips: 0 };
  const draw = m.drawStatic.bind(m);
  m.drawStatic = () => (m.strip ? draws.strips++ : draws.whole++, draw());
  const settle = () => { m.drawNow(); return { ...draws }; };
  const grab = () => m.layer.getContext('2d').getImageData(0, 0, m.layer.width, m.layer.height).data;
  const differ = (a, b) => { let n = 0; for (let i = 0; i < a.length; i += 4) if (a[i] !== b[i] || a[i + 1] !== b[i + 1] || a[i + 2] !== b[i + 2]) n++; return n / (a.length / 4); };
  const reset = () => { m.k = 18; m.cx = 0; m.cy = 0; m.viewChanged(); m.drawNow(); draws.whole = draws.strips = 0; };
  const cases = {
    pan: () => { for (const [x, y] of [[24, 12], [-10, 7], [0, -9], [13, 0]]) { m.panBy(x, y); m.drawNow(); } },
    'pan by half a px': () => { m.panBy(0.5, 0.25); m.drawNow(); },
    zoom: () => { m.panBy(24, 12); m.drawNow(); m.zoomBy(1.25); },
    resize: () => { m.panBy(24, 12); m.drawNow(); const box = m.canvas.parentElement; box.style.width = `${m.w - 100}px`; m.resize(); },
    'layer on': () => { m.panBy(24, 12); m.drawNow(); m.setLayer('country', true); },
    'layer off': () => { m.setLayer('country', false); m.panBy(24, 12); m.drawNow(); m.setLayer('rivers', false); m.setLayer('rivers', true); },
    stale: () => { m.panBy(24, 12); m.drawNow(); m.woodsKey = 'stale'; m.setModel(m.model); },
    'new map': () => { m.panBy(24, 12); m.drawNow(); const model = m.model; m.setLayout(m.l); m.setModel(model); m.k = 18; m.cx = 0; m.cy = 0; m.viewChanged(); },
  };
  const out = {};
  for (const [name, run] of Object.entries(cases)) {
    reset();
    run();
    const d = settle();
    const got = grab();
    m.dirty = true;
    m.drawNow();
    out[name] = { ...d, differ: differ(got, grab()) };
  }
  return out;
};

const page = await open('');
let failed = false;
const need = (ok, msg) => {
  if (!ok) {
    console.error(`pan: ${msg}`);
    failed = true;
  } else console.log(`ok      ${msg}`);
};
try {
  await lead(page);
  const r = await page.eval(`(${inPage})()`);
  for (const e of page.errors()) throw new Error(`page error: ${e}`);
  for (const [name, c] of Object.entries(r)) {
    if (name === 'pan') {
      need(c.whole === 0 && c.strips > 0, `a pan draws strips only (${c.strips} strips, ${c.whole} whole redraws)`);
      need(c.differ < 0.001, `a panned layer is a full redraw's but for anti-aliasing (${(c.differ * 100).toFixed(3)}% differ)`);
    } else {
      need(c.whole > 0 && c.differ === 0, `${name}: a whole redraw, identical to a fresh one (${c.whole} whole, ${c.strips} strips, ${(c.differ * 100).toFixed(3)}% differ)`);
    }
  }
} finally {
  await page.close();
}
process.exit(failed ? 1 : 0);
