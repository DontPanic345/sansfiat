// A page held across shell calls, for a session that plays it as a player does. One command per call.
//
//   node www/play/live.mjs start [QUERY] [--dir D] [--size laptop|wide|phone]   # run in the background
//   node www/play/live.mjs CMD ARGS… [--dir D]                                   # one command; prints what the page shows
//
// `start` opens the page (QUERY defaults to "?size=small&seed=2&run=1"), writes its port to D/port (D defaults to
// /tmp/sansfiat-live) and stays up until `stop`. A Relay session starts it with run_in_background. Commands:
//   look | lead NAME|watch | click T | right T | drag T1 T2 | key NAME | button LABEL | tip T|SELECTOR
//   stepper LABEL N [OPTION] [go] | end [N] | shot NAME [SCALE] | save FILE | load FILE | stop
// A tile T is `q,r` as the panel and tooltip write it, a tile index (the text view's), or `@x,y[,SCALE]` in a screenshot's
// pixels (SCALE is that shot's, 0.5 by default). After each command it prints the turn, the Standing, the panel, the feed
// items new since the last command, an open alert and page errors; each command and its printout is a JSON line in
// D/live.lines.jsonl. `save` writes a file `sansfiat judge FILE` reads. Screenshots are PNGs in D.
import { createServer, request } from 'node:http';
import { appendFileSync, copyFileSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { open } from '../play.mjs';

const argv = process.argv.slice(2);
const take = (name) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv.splice(i, 2)[1] : null;
};
const dir = resolve(take('--dir') ?? '/tmp/sansfiat-live');
const size = take('--size') ?? 'laptop';
const [cmd, ...args] = argv;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

if (fileURLToPath(import.meta.url) !== resolve(process.argv[1])) process.exit(0);
if (cmd === 'start') await serve(args[0] ?? '?size=small&seed=2&run=1');
else if (cmd) await send(cmd, args);
else console.error('usage: live.mjs start [QUERY] | look | lead NAME | click T | … | stop (see the header)');

function send(command, rest) {
  const port = readFileSync(join(dir, 'port'), 'utf8').trim();
  // File names are the client's, so they are made absolute here.
  if (['save', 'load'].includes(command) && rest[0]) rest[0] = resolve(rest[0]);
  return new Promise((done) => {
    const req = request({ host: '127.0.0.1', port, method: 'POST' }, (res) => {
      let body = '';
      res.on('data', (d) => (body += d));
      res.on('end', () => {
        console.log(body);
        done();
      });
    });
    req.on('error', () => {
      console.error(`no live page on port ${port}: start it with "live.mjs start" (--dir ${dir})`);
      process.exitCode = 1;
      done();
    });
    req.end(JSON.stringify({ cmd: command, args: rest }));
  });
}

async function serve(query) {
  mkdirSync(dir, { recursive: true });
  const page = await open(query, { size });
  await page.download(dir);
  const lines = join(dir, 'live.lines.jsonl');
  let shotScale = 0.5;
  let lastFeed = await page.eval(`Math.max(0, ...[...document.querySelectorAll('#feed li')].map((l) => Number(l.dataset.id)))`);

  // A tile as the page writes it, an index, or a screenshot pixel, as an index.
  const tileOf = async (t) => {
    if (/^\d+$/.test(t)) return Number(t);
    let m = /^(-?\d+),\s*(-?\d+)$/.exec(t);
    if (m) {
      const i = await page.eval(`game.map.at(${m[1]}, ${m[2]})`);
      if (i == null || i < 0) throw new Error(`no tile (${m[1]}, ${m[2]})`);
      return i;
    }
    m = /^@(\d+),(\d+)(?:,([\d.]+))?$/.exec(t);
    if (m) {
      const s = Number(m[3] ?? shotScale);
      const i = await page.eval(`game.map.tileAt(${m[1] / s}, ${m[2] / s})`);
      if (i == null || i < 0) throw new Error(`no tile at pixel ${m[1]},${m[2]}`);
      return i;
    }
    throw new Error(`a tile is q,r, an index or @x,y: "${t}"`);
  };

  // What a player sees.
  const show = async () => {
    const top = await page.eval(`(() => { const t = (id) => document.getElementById(id)?.textContent.trim();
      const s = document.body.dataset.screen;
      return { screen: s, turn: t('turn'), phase: t('phasetext'),
        chips: [...document.querySelectorAll('#chips .chip')].map((c) => c.textContent.replace(/\\s+/g, ' ').trim()),
        pick: document.getElementById('pick').hidden ? null : document.getElementById('pick').innerText.replace(/\\n+/g, ' | '),
        end: s === 'end' ? t('end-head') + ' ' + t('end-line') : null,
        error: t('error'), startError: s === 'start' ? t('start-error') : null,
        alerts: [...document.querySelectorAll('#alerts li')].map((l) => ({ id: l.dataset.id, text: l.textContent.trim() })),
        feed: [...document.querySelectorAll('#feed li')].map((l) => ({ id: Number(l.dataset.id), text: l.textContent.trim() })) }; })()`);
    const out = [];
    if (top.screen === 'start') out.push(`start screen${top.startError ? `: ${top.startError}` : ''}`);
    else {
      out.push(`turn ${top.turn} · ${top.phase}${top.chips.length ? ` · Standing: ${top.chips.join('; ')}` : ''}`);
      if (top.pick) out.push(`pick: ${top.pick}`);
    }
    if (top.end) out.push(`END: ${top.end}`);
    const p = top.screen === 'play' ? await page.panel() : null;
    if (p) {
      out.push(`panel: ${p.name} — ${p.doing ?? ''}`);
      for (const n of p.numbers) out.push(`  ${n.label} ${n.value}`);
      for (const l of p.lists) out.push(`  ${l.label}: ${l.rows.join('; ') || 'none'}`);
      for (const r of p.reasons) out.push(`  because ${r}`);
      if (p.buttons.length) out.push(`  buttons: ${p.buttons.map((b) => `[${b.label}${b.disabled ? ` — greyed${b.title ? `: ${b.title}` : ''}` : ''}${b.pressed === 'true' ? ' — on' : ''}]`).join(' ')}`);
      if (p.refused) out.push(`  refused: ${p.refused}`);
      if (p.hint) out.push(`  hint: ${p.hint}`);
    }
    if (top.error) out.push(`error line: ${top.error}`);
    const fresh = top.feed.filter((f) => f.id > lastFeed).sort((a, b) => a.id - b.id);
    if (fresh.length) lastFeed = Math.max(...fresh.map((f) => f.id));
    if (fresh.length) out.push(`feed (${fresh.length} new): ${fresh.slice(-14).map((f) => f.text).join(' | ')}`);
    for (const a of top.alerts) out.push(`ALERT: ${a.text}`);
    for (const e of page.errors().splice(0)) out.push(`PAGE ERROR: ${e}`);
    return out.join('\n');
  };

  const alertIds = () => page.eval(`[...document.querySelectorAll('#alerts li')].map((l) => l.dataset.id)`);

  const run = {
    look: async () => {},
    lead: async ([name]) => {
      if (!name || name === 'watch') {
        await page.button('Watch');
        await page.until("document.body.dataset.phase === 'main'");
      } else {
        await page.button(`Lead ${name}`);
        await page.until("document.body.dataset.phase === 'main'");
      }
      await page.until('!game.map.anim');
      lastFeed = Math.max(0, ...(await page.eval(`[...document.querySelectorAll('#feed li')].map((l) => Number(l.dataset.id))`)));
    },
    click: async ([t]) => page.click(await tileOf(t)),
    right: async ([t]) => page.command(await tileOf(t)),
    drag: async ([a, b]) => page.drag(await tileOf(a), await tileOf(b)),
    key: async ([name]) => page.key(name),
    button: async (label) => page.button(label.join(' ')),
    stepper: async ([label, n = '1', ...rest]) => {
      const go = rest[rest.length - 1] === 'go';
      if (go) rest.pop();
      const r = await page.stepper(label, { n: Number(n), option: rest.join(' ') || null, go });
      return `stepper ${label}: ${JSON.stringify(r)}`;
    },
    tip: async ([t]) => {
      const target = /^[@\d-]/.test(t) ? await tileOf(t) : t;
      return `tooltip: ${(await page.tip(target)) ?? 'none'}`;
    },
    end: async ([n = '1']) => {
      const before = new Set(await alertIds());
      let done = 0;
      let why = 'done';
      for (; done < Number(n); done++) {
        await page.button('End turn');
        await sleep(30);
        const s = await page.eval(`({ screen: document.body.dataset.screen, error: document.getElementById('error').textContent, open: [...document.querySelectorAll('#alerts li')].map((l) => l.dataset.id) })`);
        if (s.screen !== 'play') {
          why = `the ${s.screen} screen`;
          done++;
          break;
        }
        if (s.error) {
          why = 'refused';
          break;
        }
        if (s.open.some((id) => !before.has(id))) {
          why = 'a new alert';
          done++;
          break;
        }
      }
      return `ended ${done} of ${n}: stopped for ${why === 'done' ? 'no reason: all done' : why}`;
    },
    shot: async ([name, scale = '0.5']) => {
      const file = join(dir, name.endsWith('.png') ? name : `${name}.png`);
      shotScale = Number(scale);
      await page.shot(file, shotScale);
      return `shot: ${file} (${shotScale} scale; @x,y reads this)`;
    },
    save: async ([file]) => {
      const before = new Set(readdirSync(dir));
      await page.eval(`document.getElementById('save').click()`);
      let got = null;
      for (let n = 0; n < 30 && !got; n++) {
        await sleep(100);
        got = readdirSync(dir).find((f) => !before.has(f) && /^sansfiat-.*\.json$/.test(f));
      }
      if (got) copyFileSync(join(dir, got), file);
      else writeFileSync(file, JSON.stringify(await page.eval('game.save()')));
      return `saved ${file}${got ? '' : ' (game.save(): the download did not land)'}`;
    },
    load: async ([file]) => {
      await page.goto('');
      await page.until("document.body.dataset.screen === 'start'");
      await page.file('#start-load', file);
      await page.until("document.body.dataset.screen === 'play'");
      await page.until('!game.map.anim');
      lastFeed = Math.max(0, ...(await page.eval(`[...document.querySelectorAll('#feed li')].map((l) => Number(l.dataset.id))`)));
      return `loaded ${file}`;
    },
  };

  let queue = Promise.resolve();
  const server = createServer((req, res) => {
    let body = '';
    req.on('data', (d) => (body += d));
    req.on('end', () => {
      queue = queue.then(async () => {
        const { cmd: c, args: a } = JSON.parse(body);
        let text;
        let stopping = false;
        try {
          if (c === 'stop') {
            text = 'stopped';
            stopping = true;
          } else if (!run[c]) text = `unknown command "${c}"`;
          else {
            const said = await run[c](a);
            text = [said, await show()].filter(Boolean).join('\n');
          }
        } catch (e) {
          text = `FAILED: ${e.message}\n${await show().catch(() => '')}`;
        }
        appendFileSync(lines, `${JSON.stringify({ cmd: c, args: a, out: text })}\n`);
        res.end(text);
        if (stopping) {
          await page.close();
          server.close();
          process.exit(0);
        }
      });
    });
  });
  server.listen(0, '127.0.0.1', () => {
    writeFileSync(join(dir, 'port'), String(server.address().port));
    console.log(`live page up on port ${server.address().port}, dir ${dir}`);
  });
}
