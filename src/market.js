import { FOODS } from './data.js';

// Each trader values a basket at Σ sqrt(amount), so given exchange ratios `prices`
// it wants amount_f ∝ 1/p_f², spending exactly what its own basket is worth.
export function demand(basket, prices, goods = FOODS) {
  const wealth = goods.reduce((s, f) => s + prices[f] * (basket[f] ?? 0), 0);
  const denom = goods.reduce((s, f) => s + 1 / prices[f], 0);
  return Object.fromEntries(goods.map((f) => [f, wealth / (prices[f] ** 2 * denom)]));
}

export const utility = (basket) => Object.values(basket).reduce((s, v) => s + Math.sqrt(Math.max(0, v)), 0);

// Finds exchange ratios at which posted supply meets demand, then hands out the goods.
// Prices are relative only (geometric mean 1); no trader holds or receives them.
// baskets: {id: {food: amount}}. Returns {prices, after: {id: basket}, volume: {food: amount}}.
export function clear(baskets, startPrices = {}, { iterations = 300, step = 0.5, tolerance = 1e-5 } = {}) {
  const ids = Object.keys(baskets);
  const supply = Object.fromEntries(FOODS.map((f) => [f, ids.reduce((s, id) => s + (baskets[id][f] ?? 0), 0)]));
  const goods = FOODS.filter((f) => supply[f] > 1e-9);
  const prices = Object.fromEntries(goods.map((f) => [f, startPrices[f] ?? 1]));
  if (ids.length < 2 || goods.length < 2) return { prices, after: structuredClone(baskets), volume: {} };

  for (let i = 0; i < iterations; i++) {
    const want = Object.fromEntries(goods.map((f) => [f, 0]));
    for (const id of ids) {
      const d = demand(baskets[id], prices, goods);
      for (const f of goods) want[f] += d[f];
    }
    let worst = 0;
    for (const f of goods) {
      const excess = (want[f] - supply[f]) / supply[f];
      worst = Math.max(worst, Math.abs(excess));
      prices[f] *= Math.exp(step * Math.max(-1, Math.min(1, excess)));
    }
    const mean = Math.exp(goods.reduce((s, f) => s + Math.log(prices[f]), 0) / goods.length);
    for (const f of goods) prices[f] /= mean;
    if (worst < tolerance) break;
  }

  // Ration whichever side of each good is long, so goods are conserved exactly.
  const net = Object.fromEntries(ids.map((id) => {
    const d = demand(baskets[id], prices, goods);
    return [id, Object.fromEntries(goods.map((f) => [f, d[f] - (baskets[id][f] ?? 0)]))];
  }));
  const volume = {};
  const after = {};
  for (const id of ids) after[id] = { ...baskets[id] };
  for (const f of goods) {
    const buys = ids.reduce((s, id) => s + Math.max(0, net[id][f]), 0);
    const sells = ids.reduce((s, id) => s + Math.max(0, -net[id][f]), 0);
    const v = Math.min(buys, sells);
    if (v <= 0) continue;
    volume[f] = v;
    for (const id of ids) {
      const z = net[id][f];
      after[id][f] = (after[id][f] ?? 0) + (z > 0 ? z * (v / buys) : z * (v / sells));
    }
  }
  return { prices, after, volume };
}
