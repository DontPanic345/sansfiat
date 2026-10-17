import { civCities, humanCiv } from '../../core.js';
import { CIV } from './data.js';

export const z = 10;
export function drawLayer() {}
export function onClick() { return false; }

const fmt = (x) => (x >= 10 ? x.toFixed(0) : x.toFixed(1));

export function panel(s) {
  const civ = humanCiv(s);
  if (!civ) return '';
  const store = Object.entries(civ.store).map(([g, v]) => `${g} ${fmt(v)}`).join(', ') || 'empty';
  const cities = civCities(s, civ.id).map((c) => {
    const built = Object.keys(c.buildings ?? {}).join(', ') || 'none';
    const proj = c.project ? `${c.project.kind} ${fmt(c.project.labour)}/${CIV.projects[c.project.kind].labour}`
      : Object.keys(CIV.projects).filter((p) => !c.buildings?.[p]).map((p) =>
        `<button data-action='${JSON.stringify({ type: 'setProject', civ: civ.id, city: c.id, project: p })}'>${p}</button>`).join(' ');
    return `<div>${c.name} (built: ${built}) ${proj}</div>`;
  }).join('');
  const lean = Object.values(s.clans).filter((c) => c.owner == null && (c.leaning[civ.id] ?? 0) > 0)
    .map((c) => `${c.name} ${fmt(c.leaning[civ.id])}`).join(', ') || 'none';
  return `<h3>${civ.name} civ</h3><div>Store: ${store}</div>${cities}<div>Leaning toward us: ${lean}</div>`;
}
