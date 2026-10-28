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

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

/** Opens the page with `query` (like "?size=small&seed=2") in a 1280×800 headless Chrome. */
export const open = async (query = '', { width = 1280, height = 800 } = {}) => {
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
  await send('Page.navigate', { url: `http://127.0.0.1:${port}/index.html${query}` });

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
  await until('!!window.game && !!document.body.dataset.phase');

  // A tile's centre on screen, fitting the whole map first when it's out of view.
  const tile = async (i) => {
    const at = () => evaluate(`(() => { const p = game.map.screenOf(${i}); return [p.x, p.y]; })()`);
    let [x, y] = await at();
    if (x < 0 || y < 0 || x >= width || y >= height) {
      await evaluate('game.map.fit()');
      [x, y] = await at();
    }
    return { x, y };
  };
  const mouse = async (i, button) => {
    const { x, y } = await tile(i);
    const buttons = button === 'left' ? 1 : 2;
    await send('Input.dispatchMouseEvent', { type: 'mouseMoved', x, y });
    await send('Input.dispatchMouseEvent', { type: 'mousePressed', x, y, button, buttons, clickCount: 1 });
    await send('Input.dispatchMouseEvent', { type: 'mouseReleased', x, y, button, buttons: 0, clickCount: 1 });
  };

  return {
    eval: evaluate,
    until,
    tile,
    /** The page's own errors so far. */
    errors: () => [...errors],
    data: () => evaluate('({ ...document.body.dataset })'),
    /** Left-clicks tile `i`: selects what's there. */
    click: (i) => mouse(i, 'left'),
    /** Clicks tile `i` until a `kind` ("unit", "capital", "tile") is selected, since a click on the
     *  selected thing's tile moves to the next thing there. Returns the selection's key. */
    select: async function (i, kind) {
      for (let n = 0; n < 8; n++) {
        const [now, at] = await evaluate(`(() => { const s = document.body.dataset.selected || ''; const [k, id] = s.split(':'); const v = game.view();
          return [s, k === 'unit' ? v.units[id]?.tile : k === 'capital' ? v.civs[id]?.capital : Number(id)]; })()`);
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
