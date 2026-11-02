// Drives the page in headless Chrome, for sessions that need to play it: clicks by tile and by
// button label, waits on page state, and small screenshots. Needs www/pkg (./build-web.sh).
//
//   node www/play.mjs "?size=small&seed=2&run=1" [--until "document.body.dataset.phase === 'choose'"] [--shot out.png] [--scale 0.5]
//
// prints `body.dataset` once the page is ready (and the --until expression is true). From a script:
//
//   import { open } from './play.mjs';
//   const page = await open('?size=small&seed=2&run=1');
//   await page.until("document.body.dataset.phase === 'choose'");
//   await page.button('Lead Red');
//   await page.click(812);          // left-click a tile: select
//   await page.command(830);        // right-click a tile: the selected unit's order
//   await page.shot('turn.png');    // half size by default
//   const page = await open('?size=small&seed=2', { size: 'phone' });   // SIZES: phone, laptop (default), wide
//   await lead(page, { seed: 2 });  // to the main game, leading a civ (a name, or the first card)
//   await page.key('Space'); await page.reload(); await page.file('#load', 'game.json');
//   await page.drag(812, 830); await page.tip(812);   // a tile drag; a tile's or a selector's tooltip text
//   await page.close();
//
// Chrome is found on the PATH (google-chrome, chromium) or in the usual places; CHROME overrides.
import { spawn, execFileSync } from 'node:child_process';
import { createServer } from 'node:http';
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { extname, join, normalize } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = fileURLToPath(new URL('.', import.meta.url));
const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.mjs': 'text/javascript', '.wasm': 'application/wasm', '.css': 'text/css', '.json': 'application/json', '.png': 'image/png', '.svg': 'image/svg+xml' };

const chrome = () => {
  if (process.env.CHROME) return process.env.CHROME;
  for (const name of ['google-chrome', 'google-chrome-stable', 'chromium', 'chromium-browser']) {
    try {
      return execFileSync('sh', ['-c', `command -v ${name}`], { encoding: 'utf8' }).trim();
    } catch {
      // not this one
    }
  }
  const known = [
    'C:/Program Files/Google/Chrome/Application/chrome.exe',
    'C:/Program Files (x86)/Google/Chrome/Application/chrome.exe',
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
  ];
  const found = known.find(existsSync);
  if (!found) throw new Error('no Chrome found: set CHROME to its path');
  return found;
};

// www/ over HTTP, since the page's modules and wasm don't load from file://.
const serve = () =>
  new Promise((resolve) => {
    const server = createServer((req, res) => {
      const path = normalize(join(here, decodeURIComponent(new URL(req.url, 'http://x').pathname)));
      const file = path.endsWith('/') || path.endsWith('\\') ? join(path, 'index.html') : path;
      if (!file.startsWith(normalize(here)) || !existsSync(file)) {
        res.writeHead(404).end();
        return;
      }
      res.writeHead(200, { 'Content-Type': TYPES[extname(file)] ?? 'application/octet-stream' }).end(readFileSync(file));
    });
    server.listen(0, '127.0.0.1', () => resolve(server));
  });

// One DevTools connection to the browser, with the page's session flattened onto it.
const connect = async (url) => {
  const ws = new WebSocket(url);
  await new Promise((resolve, reject) => {
    ws.onopen = resolve;
    ws.onerror = () => reject(new Error(`couldn't reach Chrome at ${url}`));
  });
  let id = 0;
  const waiting = new Map();
  const listeners = [];
  ws.onmessage = (m) => {
    const msg = JSON.parse(m.data);
    if (msg.id && waiting.has(msg.id)) {
      const { resolve, reject } = waiting.get(msg.id);
      waiting.delete(msg.id);
      if (msg.error) reject(new Error(msg.error.message));
      else resolve(msg.result);
    } else if (msg.method) {
      for (const l of listeners) l(msg);
    }
  };
  const send = (method, params = {}, sessionId) =>
    new Promise((resolve, reject) => {
      waiting.set(++id, { resolve, reject });
      ws.send(JSON.stringify({ id, method, params, sessionId }));
    });
  return { send, on: (l) => listeners.push(l), close: () => ws.close() };
};

/** The window sizes the page is judged at. */
export const SIZES = { phone: { width: 390, height: 844 }, laptop: { width: 1280, height: 800 }, wide: { width: 1920, height: 1080 } };

const KEYS = { Space: [' ', 'Space'], Escape: ['Escape', 'Escape'], Enter: ['Enter', 'Enter'], Tab: ['Tab', 'Tab'], ArrowLeft: ['ArrowLeft', 'ArrowLeft'], ArrowRight: ['ArrowRight', 'ArrowRight'], ArrowUp: ['ArrowUp', 'ArrowUp'], ArrowDown: ['ArrowDown', 'ArrowDown'] };

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

/** Opens the page with `query` (like "?size=small&seed=2") in a headless Chrome of `size` (a SIZES name,
 *  laptop 1280×800 by default, or `width` and `height`). `path` and `ready` open another page of www/ and
 *  say when it is up (for scripts that draw on a blank page). */
export const open = async (query = '', { size = 'laptop', path = 'index.html', ready = '!!window.game && !!document.body.dataset.screen', ...dims } = {}) => {
  const { width, height } = { ...SIZES[size], ...dims };
  const server = await serve();
  const profile = mkdtempSync(join(tmpdir(), 'play-'));
  const proc = spawn(chrome(), [
    '--headless=new', '--remote-debugging-port=0', `--user-data-dir=${profile}`, `--window-size=${width},${height}`,
    '--no-first-run', '--no-default-browser-check', '--disable-gpu', 'about:blank',
  ], { stdio: ['ignore', 'ignore', 'pipe'] });
  const wsUrl = await new Promise((resolve, reject) => {
    let err = '';
    proc.stderr.on('data', (d) => {
      err += d;
      const m = /DevTools listening on (ws:\S+)/.exec(err);
      if (m) resolve(m[1]);
    });
    proc.on('exit', (code) => reject(new Error(`Chrome exited (${code}): ${err.slice(-500)}`)));
  });
  const cdp = await connect(wsUrl);
  const { targetId } = await cdp.send('Target.createTarget', { url: 'about:blank' });
  const { sessionId } = await cdp.send('Target.attachToTarget', { targetId, flatten: true });
  const send = (method, params) => cdp.send(method, params, sessionId);
  const errors = [];
  cdp.on((msg) => {
    if (msg.sessionId !== sessionId) return;
    if (msg.method === 'Runtime.exceptionThrown') errors.push(msg.params.exceptionDetails.exception?.description ?? msg.params.exceptionDetails.text);
    if (msg.method === 'Runtime.consoleAPICalled' && msg.params.type === 'error') errors.push(msg.params.args.map((a) => a.value ?? a.description).join(' '));
  });
  await send('Runtime.enable');
  await send('Page.enable');
  await send('Emulation.setDeviceMetricsOverride', { width, height, deviceScaleFactor: 1, mobile: false });
  const { port } = server.address();
  await send('Page.navigate', { url: `http://127.0.0.1:${port}/${path}${query}` });

  const evaluate = async (expr) => {
    const r = await send('Runtime.evaluate', { expression: expr, returnByValue: true, awaitPromise: true });
    if (r.exceptionDetails) throw new Error(r.exceptionDetails.exception?.description ?? r.exceptionDetails.text);
    return r.result.value;
  };
  const until = async (expr, timeout = 120_000) => {
    const end = Date.now() + timeout;
    for (;;) {
      try {
        if (await evaluate(expr)) return;
      } catch {
        // the page isn't up yet
      }
      if (Date.now() > end) throw new Error(`timed out waiting for ${expr}${errors.length ? `; page errors: ${errors.join('; ')}` : ''}`);
      await sleep(100);
    }
  };
  await until(ready);

  // A tile's centre on screen, on the map's canvas with nothing over it (the panel, the feed, the zoom or a button): the whole map is
  // fitted when the tile is out of view, and the map centred on it when fitting still leaves it out.
  const tile = async (i) => {
    // A map still easing to a centre (after Lead, say) moves the tile under the pointer: wait for it.
    await until('!game.map.anim', 10_000);
    const at = () => evaluate(`(() => { const r = document.getElementById('map').getBoundingClientRect(); const p = game.map.screenOf(${i});
      const inside = p.x >= r.left && p.x < r.right && p.y >= r.top && p.y < r.bottom;
      return { x: p.x, y: p.y, inside: inside && document.elementFromPoint(p.x, p.y)?.id === 'map' }; })()`);
    let p = await at();
    if (!p.inside) {
      await evaluate('game.map.fit()');
      p = await at();
    }
    if (!p.inside) {
      await evaluate(`game.map.centreOn(${i}, { minK: game.map.k, smooth: false })`);
      p = await at();
    }
    if (!p.inside) throw new Error(`tile ${i} is not on the map's canvas`);
    return { x: p.x, y: p.y };
  };
  const mouse = async (i, button) => {
    const { x, y } = await tile(i);
    const buttons = button === 'left' ? 1 : 2;
    await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x, y });
    await send('Input.dispatchMouseEvent', { type: 'mousePressed', x, y, button, buttons, clickCount: 1 });
    await send('Input.dispatchMouseEvent', { type: 'mouseReleased', x, y, button, buttons: 0, clickCount: 1 });
  };

  /** The pointer's tile-centre coordinates for a selector or a tile. */
  const centreOf = async (target) =>
    typeof target === 'number'
      ? tile(target)
      : evaluate(`(() => { const e = document.querySelector(${JSON.stringify(target)}); if (!e) return null; const r = e.getBoundingClientRect(); return { x: r.left + r.width / 2, y: r.top + r.height / 2 }; })()`);

  return {
    eval: evaluate,
    /** Loads another URL of the page (a query like "?size=small&seed=2&run=1"). */
    goto: async (q) => {
      await send('Page.navigate', { url: `http://127.0.0.1:${port}/${path}${q}` });
      await sleep(200);
      await until(ready);
    },
    /** Reloads the page in the same profile, so `localStorage` stays. */
    reload: async () => {
      await send('Page.reload');
      await sleep(200);
      await until(ready);
    },
    /** A real key press: `name` is a `KeyboardEvent.key` ("a", "Escape", "ArrowLeft") or "Space". */
    key: async (name) => {
      const [key, code] = KEYS[name] ?? [name, name.length === 1 && /[a-z]/i.test(name) ? `Key${name.toUpperCase()}` : name];
      const text = key.length === 1 ? key : undefined;
      await send('Input.dispatchKeyEvent', { type: text ? 'keyDown' : 'rawKeyDown', key, code, text, windowsVirtualKeyCode: key.length === 1 ? key.toUpperCase().charCodeAt(0) : 0 });
      await send('Input.dispatchKeyEvent', { type: 'keyUp', key, code });
      await sleep(50);
    },
    /** Downloads (a Save to file's link) land in directory `dir`. */
    download: (dir) => send('Page.setDownloadBehavior', { behavior: 'allow', downloadPath: dir }),
    /** Sets the files of the `<input type=file>` `selector` to `path` (or a list of paths). */
    file: async (selector, path) => {
      const { root } = await send('DOM.getDocument', { depth: 0 });
      const { nodeId } = await send('DOM.querySelector', { nodeId: root.nodeId, selector });
      if (!nodeId) throw new Error(`no element ${selector}`);
      await send('DOM.setFileInputFiles', { files: [].concat(path), nodeId });
      await sleep(100);
    },
    /** A left-button drag from tile `from` to tile `to`, through the points between. */
    drag: async (from, to) => {
      const a = await tile(from);
      const b = await tile(to);
      await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: a.x, y: a.y });
      await send('Input.dispatchMouseEvent', { type: 'mousePressed', x: a.x, y: a.y, button: 'left', buttons: 1, clickCount: 1 });
      for (let n = 1; n <= 8; n++) {
        await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: a.x + ((b.x - a.x) * n) / 8, y: a.y + ((b.y - a.y) * n) / 8, button: 'left', buttons: 1 });
      }
      await send('Input.dispatchMouseEvent', { type: 'mouseReleased', x: b.x, y: b.y, button: 'left', buttons: 0, clickCount: 1 });
      await sleep(50);
    },
    /** The tooltip's text after hovering a tile or a selector: the page's `#tip` once it shows, else the
     *  element's `title`; null if neither. */
    tip: async (target) => {
      const at = await centreOf(target);
      if (!at) return null;
      await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: at.x - 2, y: at.y });
      await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: at.x, y: at.y });
      const read = `(() => { const t = document.getElementById('tip'); return t && !t.hidden ? t.textContent : null; })()`;
      for (let n = 0; n < 20; n++) {
        const text = await evaluate(read);
        if (text) return text;
        await sleep(100);
      }
      return typeof target === 'string' ? evaluate(`document.querySelector(${JSON.stringify(target)})?.title || null`) : null;
    },
    until,
    tile,
    /** The bottom panel as the page draws it: its name, doing line, numbers (with tooltips), list rows,
     *  reasons, buttons (greyed or not, with their tooltips), the refusal line and the hint. Null if hidden. */
    panel: () =>
      evaluate(`(() => { const d = document.getElementById('detail'); if (!d.firstElementChild) return null;
        const t = (e) => (e ? e.textContent.trim() : null);
        return {
          name: t(d.querySelector('h2 span')),
          doing: t(d.querySelector('.doing')),
          numbers: [...d.querySelectorAll('.num')].map((n) => ({ label: t(n.firstElementChild), value: t(n.lastElementChild), tip: n.title })),
          lists: [...d.querySelectorAll('.list')].map((l) => ({ label: t(l.firstElementChild), rows: [...l.querySelectorAll('.listrow')].map((r) => t(r.querySelector('button'))) })),
          reasons: [...d.querySelectorAll('.why li')].map((l) => l.textContent.trim()),
          buttons: [...d.querySelectorAll('.actions > button, .actions .stepper > button')].map((b) => ({ label: b.textContent.trim(), disabled: b.disabled, title: b.title, pressed: b.getAttribute('aria-pressed') })),
          refused: t(d.querySelector('.refused')),
          hint: t(d.querySelector('.hint')),
          text: d.textContent,
        }; })()`),
    /** Opens a stepper button `label` in the panel, sets its count (and its option, by its text), and
     *  presses Go when `go` is set, else Cancel. Returns the count's max, Go's state and its tooltip. */
    stepper: async (label, { n = 1, option = null, go = false } = {}) => {
      const r = await evaluate(`(async () => {
        const d = document.getElementById('detail');
        const open = [...d.querySelectorAll('.actions > button')].find((b) => b.textContent.trim() === ${JSON.stringify(label)});
        if (!open) return { missing: true };
        if (open.disabled) return { disabled: true, title: open.title };
        open.click();
        await new Promise((r) => requestAnimationFrame(r));
        const sel = d.querySelector('.stepper select');
        if (sel && ${JSON.stringify(option)} != null) {
          sel.value = [...sel.options].find((o) => o.textContent === ${JSON.stringify(option)}).value;
          sel.dispatchEvent(new Event('change'));
          await new Promise((r) => requestAnimationFrame(r));
        }
        const input = d.querySelector('.stepper input');
        if (!input) return { missing: true };
        input.value = String(${Number(n)});
        input.dispatchEvent(new Event('input'));
        const button = (text) => [...d.querySelectorAll('.stepper button')].find((b) => b.textContent.trim() === text);
        const out = { max: Number(input.max), go: !button('Go').disabled, title: button('Go').title, options: sel ? [...sel.options].map((o) => o.textContent) : [] };
        if (${go} && out.go) button('Go').click();
        else button('Cancel').click();
        return out;
      })()`);
      await sleep(50);
      return r;
    },

    /** The page's own errors so far. */
    errors: () => [...errors],
    data: () => evaluate('({ ...document.body.dataset })'),
    /** Left-clicks tile `i`: selects what's there. */
    click: (i) => mouse(i, 'left'),
    /** Clicks tile `i` until a `kind` ("unit", "civ", "clan", "festival", "tile") is selected, since a click on
     *  the selected thing's tile moves to the next thing there. Returns the selection's key. */
    select: async function (i, kind) {
      for (let n = 0; n < 8; n++) {
        const [now, at] = await evaluate(`(() => { const s = document.body.dataset.selected || ''; const [k, id] = s.split(':'); const v = game.view();
          return [s, k === 'unit' ? v.units[id]?.tile : k === 'civ' ? v.civs[id]?.capital : k === 'clan' ? v.clans[id]?.home
            : k === 'festival' ? v.markets[id]?.tile : Number(id)]; })()`);
        if (now.startsWith(`${kind}:`) && at === i) return now;
        await mouse(i, 'left');
      }
      throw new Error(`clicking tile ${i} never selected a ${kind}`);
    },
    /** Right-clicks tile `i`: the selected unit's order. */
    command: (i) => mouse(i, 'right'),
    /** Moves the pointer onto tile `i`: its tooltip shows after a moment. */
    hover: async (i) => {
      const { x, y } = await tile(i);
      await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x, y });
    },
    /** Clicks the first enabled visible button whose text starts with `label`. */
    button: async (label) => {
      const seen = await evaluate(`(() => {
        const all = [...document.querySelectorAll('button')].filter((b) => b.offsetParent);
        const b = all.find((b) => !b.disabled && b.textContent.trim().startsWith(${JSON.stringify(label)}));
        if (b) { b.click(); return null; }
        return all.map((b) => b.textContent.trim() + (b.disabled ? ' (disabled)' : ''));
      })()`);
      if (seen) throw new Error(`no enabled button "${label}"; the page shows: ${seen.join(', ')}`);
      await sleep(50);
    },
    /** A PNG of the page at `scale` of its size. */
    shot: async (file, scale = 0.5) => {
      const { data } = await send('Page.captureScreenshot', { format: 'png', clip: { x: 0, y: 0, width, height, scale } });
      writeFileSync(file, Buffer.from(data, 'base64'));
    },
    close: async () => {
      cdp.close();
      proc.kill();
      server.close();
      await sleep(200);
      rmSync(profile, { recursive: true, force: true, maxRetries: 3 });
    },
  };
};

/** Takes a page to the main game: the pre-game run on small seed `seed` (2 by default), then the pick's
 *  card whose name starts with `civ` (the first card without it) led. Returns the page's data. */
export const lead = async (page, { seed = 2, size = 'small', civ } = {}) => {
  await page.goto(`?size=${size}&seed=${seed}&run=1`);
  await page.until("document.body.dataset.phase === 'choose'");
  await page.button(civ ? `Lead ${civ}` : 'Lead ');
  await page.until("document.body.dataset.phase === 'main'");
  return page.data();
};

if (process.argv[1] && fileURLToPath(import.meta.url) === normalize(process.argv[1])) {
  const args = process.argv.slice(2);
  const flag = (name) => {
    const i = args.indexOf(name);
    return i >= 0 ? args.splice(i, 2)[1] : null;
  };
  const untilExpr = flag('--until');
  const shot = flag('--shot');
  const scale = Number(flag('--scale') ?? 0.5);
  const page = await open(args[0] ?? '');
  try {
    if (untilExpr) await page.until(untilExpr);
    if (shot) await page.shot(shot, scale);
    console.log(JSON.stringify(await page.data()));
    for (const e of page.errors()) console.error(`page error: ${e}`);
  } finally {
    await page.close();
  }
}
