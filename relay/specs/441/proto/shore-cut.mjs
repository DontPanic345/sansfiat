// A sea hex filled from a repeated water block, and the bank cut as a strip and turned to each hex edge, composed on
// a seven-hex patch (a land tile ringed by water on three sides) at 3×. From the repo root:
//   node relay/specs/441/proto/shore-cut.mjs   → relay/specs/441/proto/{sea.png,bank.png,patch3x.png}
import { readFileSync, writeFileSync } from 'node:fs';
import { decode, encode } from '../../../../engine/www/assets/png.mjs';
const src = decode(readFileSync('docs/proposals/map-art/terrain.png'));
const S = (x, y) => { const o = (y * src.width + x) * 4; return [src.data[o], src.data[o + 1], src.data[o + 2], src.data[o + 3]]; };
const R = 74, A = R * Math.sqrt(3) / 2, W = 128, H = 148, SS = 4, SCALE = 74 / 109;
const inHex = (dx, dy) => { const u = dx / A, v = dy / R; return Math.abs(u) <= 1 && Math.abs(v) + Math.abs(u) / 2 <= 1; };
// Water: the hex's samples read the block [bx, by, bw, bh] tiled, every other copy mirrored, so the copies seam.
const BLOCK = [1270, 832, 137, 60];
const fold = (t, n) => { const m = ((t % (2 * n)) + 2 * n) % (2 * n); return m < n ? m : 2 * n - 1 - m; };
function sea(off = [0, 0]) {
  const d = new Uint8Array(W * H * 4);
  for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
    let r = 0, g = 0, b = 0, n = 0;
    for (let j = 0; j < SS; j++) for (let i = 0; i < SS; i++) {
      const cx = x + (i + .5) / SS, cy = y + (j + .5) / SS;
      if (!inHex(cx - W / 2, cy - H / 2)) continue;
      const sx = BLOCK[0] + fold(Math.floor(cx / SCALE + off[0]), BLOCK[2]), sy = BLOCK[1] + fold(Math.floor(cy / SCALE + off[1]), BLOCK[3]);
      const p = S(sx, sy); r += p[0]; g += p[1]; b += p[2]; n++;
    }
    const o = (y * W + x) * 4; if (n) { d[o] = Math.round(r / n); d[o + 1] = Math.round(g / n); d[o + 2] = Math.round(b / n); d[o + 3] = Math.round(255 * n / 16); }
  }
  return d;
}
// The bank: the level run at x 1040–1240; a strip from 22 px above the water line to 4 px below it, water line at y 902.
const BANK = { x0: 1060, x1: 1190, top: 878, line: 902, below: 4 };
const isBank = ([r, g, b, a]) => a > 200 && r > g * 0.9 && r > b + 30; // tan blocks and their rim, not grass or water
// A bank cell of `len` cell px along the edge, `th` px thick, turned so its water side faces angle `deg` (0 = E, 90 = S).
function bank(deg, len = 84, th = 26) {
  const Wc = 112, Hc = 112, d = new Uint8Array(Wc * Hc * 4);
  const t = (deg - 90) * Math.PI / 180, c = Math.cos(t), s = Math.sin(t);
  for (let y = 0; y < Hc; y++) for (let x = 0; x < Wc; x++) {
    let r = 0, g = 0, b = 0, a = 0;
    for (let j = 0; j < SS; j++) for (let i = 0; i < SS; i++) {
      const dx = x + (i + .5) / SS - Wc / 2, dy = y + (j + .5) / SS - Hc / 2;
      const u = (dx * c + dy * s) / SCALE, v = (-dx * s + dy * c) / SCALE; // along, across (+ toward the water)
      if (Math.abs(u) > len / 2 / SCALE) continue;
      const sx = Math.floor((BANK.x0 + BANK.x1) / 2 + u), sy = Math.floor(BANK.line + v);
      if (sy < BANK.top || sy > BANK.line + BANK.below) continue;
      const p = S(sx, sy); if (!isBank(p)) continue;
      r += p[0]; g += p[1]; b += p[2]; a += 255;
    }
    const o = (y * Wc + x) * 4; if (a) { d[o] = Math.round(r * 255 / a); d[o + 1] = Math.round(g * 255 / a); d[o + 2] = Math.round(b * 255 / a); d[o + 3] = Math.round(a / 16); }
  }
  return { w: Wc, h: Hc, data: d };
}
writeFileSync('relay/specs/441/proto/sea.png', encode({ width: W, height: H, data: Buffer.from(sea()) }));
const b0 = bank(0); writeFileSync('relay/specs/441/proto/bank.png', encode({ width: b0.w, height: b0.h, data: Buffer.from(b0.data) }));
// A patch: the plains cell from the atlas for land, sea elsewhere; land at the centre with water E, SE and NE.
const atlas = decode(readFileSync('engine/www/assets/atlas.png'));
const cells = JSON.parse(readFileSync('engine/www/assets/atlas.json', 'utf8')).cells;
const PW = 420, PH = 420, out = new Uint8Array(PW * PH * 4);
for (let i = 0; i < PW * PH; i++) { out[i * 4 + 3] = 255; out[i * 4] = 11; out[i * 4 + 1] = 14; out[i * 4 + 2] = 18; }
const blit = (img, w, h, x0, y0) => { for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) { const X = x0 + x, Y = y0 + y; if (X < 0 || Y < 0 || X >= PW || Y >= PH) continue; const s = (y * w + x) * 4, o = (Y * PW + X) * 4, al = img[s + 3] / 255; for (let c = 0; c < 3; c++) out[o + c] = Math.round(out[o + c] * (1 - al) + img[s + c] * al); } };
const plains = (() => { const c = cells['terrain.plains.0'], d = new Uint8Array(c.w * c.h * 4); for (let y = 0; y < c.h; y++) for (let x = 0; x < c.w; x++) { const s = ((c.y + y) * atlas.width + c.x + x) * 4, o = (y * c.w + x) * 4; for (let k = 0; k < 4; k++) d[o + k] = atlas.data[s + k]; } return d; })();
const DIRS = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];
const at = (q, r) => [PW / 2 + A * Math.sqrt(1) * (2 * q + r) * 1 , PH / 2 + R * 1.5 * r];
const water = new Set(['1,0', '0,1', '1,-1']);
const tiles = [[0, 0], ...DIRS];
for (const [q, r] of tiles) { const [x, y] = at(q, r); blit(water.has(`${q},${r}`) ? sea([q * 37, r * 23]) : plains, W, H, Math.round(x - W / 2), Math.round(y - H / 2)); }
DIRS.forEach(([q, r], d) => { if (!water.has(`${q},${r}`)) return; const [x, y] = at(q / 2, r / 2); const bk = bank(d * 60); blit(bk.data, bk.w, bk.h, Math.round(x - bk.w / 2), Math.round(y - bk.h / 2)); });
const Z = 3, big = new Uint8Array(PW * Z * PH * Z * 4);
for (let y = 0; y < PH * Z; y++) for (let x = 0; x < PW * Z; x++) { const s = (Math.floor(y / Z) * PW + Math.floor(x / Z)) * 4, o = (y * PW * Z + x) * 4; for (let k = 0; k < 4; k++) big[o + k] = out[s + k]; }
writeFileSync('relay/specs/441/proto/patch3x.png', encode({ width: PW * Z, height: PH * Z, data: Buffer.from(big) }));
console.log('ok');
