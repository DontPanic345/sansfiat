// The actor study's second row: connected blobs at alpha >= 128 (8-connected), their boxes, and per blob the count
// of halo pixels (red-dominant: r > 150, g < 90, b < 90, at any alpha) inside the box grown by 12 px.
// From the repo root:  node relay/specs/444/proto/boxes.mjs
import { readPng } from '../../../../engine/www/assets/png.mjs';
const img = readPng('docs/proposals/map-art/actors.png'); const { width: W, height: H, data } = img;
const a = (x, y) => data[(y * W + x) * 4 + 3];
const seen = new Uint8Array(W * H); const blobs = [];
for (let y = 300; y < 600; y++) for (let x = 0; x < W; x++) {
  if (seen[y * W + x] || a(x, y) < 128) continue;
  let [x0, y0, x1, y1, n] = [x, y, x, y, 0]; const st = [[x, y]]; seen[y * W + x] = 1;
  while (st.length) { const [px, py] = st.pop(); n++; x0 = Math.min(x0, px); x1 = Math.max(x1, px); y0 = Math.min(y0, py); y1 = Math.max(y1, py);
    for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) { const qx = px + dx, qy = py + dy; if (qx < 0 || qy < 0 || qx >= W || qy >= H || seen[qy * W + qx] || a(qx, qy) < 128) continue; seen[qy * W + qx] = 1; st.push([qx, qy]); } }
  if (n > 200) blobs.push({ x: x0, y: y0, w: x1 - x0 + 1, h: y1 - y0 + 1, n });
}
blobs.sort((p, q) => p.x - q.x);
for (const b of blobs) {
  let halo = 0, haloSolid = 0;
  for (let y = Math.max(0, b.y - 12); y < Math.min(H, b.y + b.h + 12); y++) for (let x = Math.max(0, b.x - 12); x < Math.min(W, b.x + b.w + 12); x++) {
    const i = (y * W + x) * 4; if (data[i + 3] > 0 && data[i] > 150 && data[i + 1] < 90 && data[i + 2] < 90) { halo++; if (data[i + 3] >= 128) haloSolid++; } }
  console.log(JSON.stringify({ ...b, halo, haloSolid }));
}
