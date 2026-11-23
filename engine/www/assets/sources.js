// The source art: named regions of the studies in `docs/proposals/map-art/`, and the atlas cells cut
// from them. `draw-atlas.mjs` reads these; the browser never does.

/** Source studies, by name: repo paths, outside the site. */
export const FILES = { terrain: 'docs/proposals/map-art/terrain.png' };

/** A region: an explicit hex in a source file, in source px, and how it is cleaned. */
export const REGIONS = {
  'plains.grass': {
    file: 'terrain',
    hex: { cx: 551.5, cy: 854, rx: 96.5, ry: 109 },   // half-width, circumradius
    inset: 3,                                          // drops the drawn outline
    patches: [{ from: [555, 790], to: [452, 828], w: 72, h: 70, feather: 8,
                note: "hex 2's hill runs into the lower left: grass and a tuft over it" }],
    note: 'bottom-left row, third hex: grass, tufts and rocks',
  },
};

/** The atlas cells cut from regions, by atlas key: each replaces that placeholder in place. */
export const CELLS = {
  'terrain.plains.0': { region: 'plains.grass', ops: [], anchor: 'centre' },
  'terrain.plains.1': { region: 'plains.grass', ops: ['mirror'], anchor: 'centre' },
  'terrain.plains.2': { region: 'plains.grass', ops: [], anchor: 'centre' },
};
