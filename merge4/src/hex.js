// Axial coordinates, pointy-top. A hex is {q, r}.
export const DIRS = [[1, 0], [1, -1], [0, -1], [-1, 0], [-1, 1], [0, 1]];

export const key = (h) => `${h.q},${h.r}`;
export const eq = (a, b) => a.q === b.q && a.r === b.r;
export const neighbors = (h) => DIRS.map(([dq, dr]) => ({ q: h.q + dq, r: h.r + dr }));
export const distance = (a, b) =>
  (Math.abs(a.q - b.q) + Math.abs(a.q + a.r - b.q - b.r) + Math.abs(a.r - b.r)) / 2;

export function within(h, radius) {
  const out = [];
  for (let dq = -radius; dq <= radius; dq++)
    for (let dr = Math.max(-radius, -dq - radius); dr <= Math.min(radius, -dq + radius); dr++)
      out.push({ q: h.q + dq, r: h.r + dr });
  return out;
}

// odd-r offset (rectangular maps) <-> axial
export const fromOffset = (col, row) => ({ q: col - (row - (row & 1)) / 2, r: row });
export const toOffset = (h) => ({ col: h.q + (h.r - (h.r & 1)) / 2, row: h.r });

export const toPixel = (h, size) => ({
  x: size * Math.sqrt(3) * (h.q + h.r / 2),
  y: size * 1.5 * h.r,
});

export function fromPixel(x, y, size) {
  const q = ((Math.sqrt(3) / 3) * x - y / 3) / size;
  const r = ((2 / 3) * y) / size;
  return round(q, r);
}

function round(q, r) {
  const s = -q - r;
  let rq = Math.round(q), rr = Math.round(r);
  const rs = Math.round(s);
  const dq = Math.abs(rq - q), dr = Math.abs(rr - r), ds = Math.abs(rs - s);
  if (dq > dr && dq > ds) rq = -rr - rs;
  else if (dr > ds) rr = -rq - rs;
  return { q: rq, r: rr };
}
