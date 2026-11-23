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
    insetRight: 5,                                     // the right edge keeps a dark line at 3
    patches: [{ from: [555, 790], to: [452, 828], w: 72, h: 70, feather: 8,
                note: "hex 2's hill runs into the lower left: grass and a tuft over it" }],
    note: 'bottom-left row, third hex: grass, tufts and rocks',
  },
  'plains.clean': {
    file: 'terrain',
    hex: { cx: 551.5, cy: 854, rx: 96.5, ry: 109 },
    inset: 3,
    insetRight: 5,
    patches: [
      { from: [555, 790], to: [452, 828], w: 72, h: 70, feather: 8,
        note: "hex 2's hill runs into the lower left: grass and a tuft over it" },
      { from: [541, 806], to: [530, 862], w: 99, h: 58, feather: 6,
        note: 'the rocks at the lower right: clean grass from above them' },
    ],
    note: 'the same hex with the rocks patched out: a plains tile that is not a grey blob at play zoom',
  },
  'peak.left': {
    file: 'terrain',
    rect: [110, 140, 280, 180],                 // x, y, w, h in source px
    keep: [[110, 140], [300, 140], [300, 150], [385, 258], [372, 300], [110, 312]], // drops the next peak and the hill below it
    key: 'ground',                              // clears grass, tufts and hill
    ground: [262, 292],                         // the source point that lands on the cell's anchor
    scale: 0.74,                            // cell px per source px
    note: "the range's left peak, with its foot's boulders",
  },
  'peak.right': {
    file: 'terrain',
    rect: [1125, 140, 285, 170],
    keep: [[1180, 232], [1262, 140], [1385, 140], [1385, 250], [1360, 300], [1130, 300]],
    key: 'ground',
    ground: [1263, 290],
    scale: 0.72,
    note: "the range's right peak and its skirt of rocks",
  },
  'peak.front': {
    file: 'terrain',
    rect: [740, 150, 310, 190],
    keep: [[897, 164], [1000, 240], [1050, 300], [1050, 340], [745, 340], [745, 310], [790, 255], [850, 200]],
    key: 'ground',
    ground: [890, 318],
    scale: 0.7,
    note: "the range's front peak, low in the middle",
  },
  'low.mid': {
    file: 'terrain',
    rect: [520, 190, 190, 95],
    keep: [[620, 193], [700, 268], [700, 280], [525, 280], [525, 268]],
    key: 'ground',
    ground: [605, 272],
    scale: 0.7,
    note: 'the low peak between the big ones',
  },
  'low.spur': {
    file: 'terrain',
    rect: [580, 165, 275, 120],
    keep: [[715, 168], [790, 225], [850, 272], [590, 272], [640, 230]],
    key: 'ground',
    ground: [720, 272],
    scale: 0.66,
    note: 'the lower peak right of the low one',
  },
};
REGIONS['low.spur.s'] = { ...REGIONS['low.spur'], scale: 0.52, note: 'the lower peak, small: the diagonal ridges' };
REGIONS['low.mid.s'] = { ...REGIONS['low.mid'], scale: 0.52, note: 'the low peak, small: the diagonal ridges' };

/** The atlas cells cut from regions, by atlas key: each replaces that placeholder in place. */
export const CELLS = {
  'terrain.plains.0': { region: 'plains.clean', ops: [], anchor: 'centre' },
  'terrain.plains.1': { region: 'plains.grass', ops: [], anchor: 'centre' },
  'terrain.plains.2': { region: 'plains.clean', ops: ['mirror'], anchor: 'centre' },
  'terrain.mountain.0': { region: 'plains.clean', ops: [], anchor: 'centre' },
  'terrain.mountain.1': { region: 'plains.grass', ops: [], anchor: 'centre' },
  'terrain.mountain.2': { region: 'plains.clean', ops: ['mirror'], anchor: 'centre' },
  'mountain.peak.0': { region: 'peak.left', ops: [], anchor: [112, 128] },
  'mountain.peak.1': { region: 'peak.right', ops: [], anchor: [112, 128] },
  'mountain.peak.2': { region: 'peak.front', ops: [], anchor: [112, 128] },
  'mountain.lone.0': { region: 'peak.left', ops: [], anchor: [112, 128] },
  'mountain.lone.1': { region: 'peak.right', ops: ['mirror'], anchor: [112, 128] },
  'mountain.ridge.e.0': { region: 'low.mid', ops: [], anchor: [112, 128] },
  'mountain.ridge.e.1': { region: 'low.spur', ops: [], anchor: [112, 128] },
  'mountain.ridge.se.0': { region: 'low.spur.s', ops: ['mirror'], anchor: [112, 128] },
  'mountain.ridge.se.1': { region: 'low.mid.s', ops: [], anchor: [112, 128] },
  'mountain.ridge.sw.0': { region: 'low.spur.s', ops: [], anchor: [112, 128] },
  'mountain.ridge.sw.1': { region: 'low.mid.s', ops: ['mirror'], anchor: [112, 128] },
};
