// Deterministic RNG (mulberry32). The seed lives in state.rng so games replay exactly.
export function rand(state) {
  let t = (state.rng = (state.rng + 0x6d2b79f5) >>> 0);
  t = Math.imul(t ^ (t >>> 15), t | 1);
  t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
}

export const randInt = (state, n) => Math.floor(rand(state) * n);
export const pick = (state, arr) => arr[randInt(state, arr.length)];

export function weighted(state, pairs) {
  let x = rand(state) * pairs.reduce((sum, [, w]) => sum + w, 0);
  for (const [value, w] of pairs) if ((x -= w) < 0) return value;
  return pairs.at(-1)[0];
}
