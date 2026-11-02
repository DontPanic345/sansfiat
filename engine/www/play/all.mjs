// Runs every other script in www/play/ with a temporary --out and prints a table: the script, pass or
// fail, its seconds. Exits non-zero if any fails. Without Chrome it fails under CI and skips with a
// warning otherwise.   node www/play/all.mjs
import { spawnSync, execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = fileURLToPath(new URL('.', import.meta.url));

const haveChrome = () => {
  if (process.env.CHROME) return existsSync(process.env.CHROME);
  for (const name of ['google-chrome', 'google-chrome-stable', 'chromium', 'chromium-browser']) {
    try {
      execFileSync('sh', ['-c', `command -v ${name}`], { stdio: 'ignore' });
      return true;
    } catch {
      // not this one
    }
  }
  return [
    'C:/Program Files/Google/Chrome/Application/chrome.exe',
    'C:/Program Files (x86)/Google/Chrome/Application/chrome.exe',
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
  ].some(existsSync);
};

if (!haveChrome()) {
  if (process.env.CI) {
    console.error('play/all: no Chrome found (set CHROME): failing under CI');
    process.exit(1);
  }
  console.warn('play/all: no Chrome found (set CHROME): skipping the page scripts');
  process.exit(0);
}

const scripts = readdirSync(here).filter((f) => f.endsWith('.mjs') && f !== 'all.mjs').sort();
const out = mkdtempSync(join(tmpdir(), 'play-all-'));
const rows = [];
for (const script of scripts) {
  const t0 = Date.now();
  const r = spawnSync(process.execPath, [join(here, script), '--out', out], { encoding: 'utf8', timeout: 300_000 });
  const ok = r.status === 0;
  rows.push({ script, ok, seconds: (Date.now() - t0) / 1000 });
  if (!ok) process.stderr.write(`--- ${script} failed (${r.status ?? r.signal})\n${r.stdout.slice(-1500)}${r.stderr.slice(-1500)}\n`);
}
rmSync(out, { recursive: true, force: true });
const width = Math.max(6, ...rows.map((r) => r.script.length));
console.log(`${'script'.padEnd(width)}  result  seconds`);
for (const r of rows) console.log(`${r.script.padEnd(width)}  ${r.ok ? 'pass  ' : 'FAIL  '}  ${r.seconds.toFixed(1)}`);
process.exit(rows.every((r) => r.ok) ? 0 : 1);
