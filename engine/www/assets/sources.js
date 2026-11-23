// The source art: named regions of the studies in `docs/proposals/map-art/`, and the atlas cells cut
// from them. `draw-atlas.mjs` reads these; the browser never does.
import { FRINGE } from './placeholders.js';

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

// The water: the clean block of the sea (no ripple crosses it), filled into a hex at the plains' scale. Three
// offsets give three cells that differ.
const WATER = { file: 'terrain', rect: [1270, 832, 137, 60] };
const SEA = { file: 'terrain', fill: { rect: WATER.rect, scale: 74 / 109 }, note: 'the sea: a clean block of water, repeated and mirrored, filled into the hex' };
REGIONS['sea.0'] = { ...SEA, fill: { ...SEA.fill, offset: [0, 0] } };
REGIONS['sea.1'] = { ...SEA, fill: { ...SEA.fill, offset: [37, 23] } };
REGIONS['sea.2'] = { ...SEA, fill: { ...SEA.fill, offset: [71, 41] } };

// The hills: the bottom-left row's first two hexes. Hex 2's left-lower part is the first hill's tail: grass over it.
// The marsh: the plains' grass hex with two pools patched in from the water block, so the marsh has water in it.
REGIONS['marsh.grass'] = {
  ...REGIONS['plains.grass'],
  patches: [
    ...REGIONS['plains.grass'].patches,
    { from: [1280, 836], to: [520, 826], w: 30, h: 18, feather: 4, note: 'a pool of the sea block, in the marsh' },
    { from: [1300, 840], to: [585, 880], w: 26, h: 16, feather: 4, note: 'a second pool of the sea block' },
  ],
  note: 'bottom-left row, third hex, with two pools of water patched in: the marsh',
};

const HILLS = { file: 'terrain', hex: { cx: 164.5, cy: 854, rx: 96.5, ry: 109 }, inset: 3, insetRight: 5 };
REGIONS['hills.a'] = { ...HILLS, note: 'bottom-left row, first hex: a hill with grass round it' };
REGIONS['hills.b'] = {
  ...HILLS,
  hex: { ...HILLS.hex, cx: 358 },
  patches: [{ from: [262, 760], to: [262, 820], w: 46, h: 80, feather: 6, note: "hex 1's hill runs into the lower left: grass over it" }],
  note: 'bottom-left row, second hex: a hill, with the first hill\'s tail patched out',
};

// The bank: the level run of tan blocks on the land side of the water line (y 902). Two strips of it, one per
// variant, 124 source px long, and a disc of it for the corners. Each keeps the bank's tan (key `bank`).
const RUN = [[1040, 878], [1240, 878], [1240, 906], [1040, 906]];
REGIONS['bank.left'] = { file: 'terrain', rect: [1040, 878, 200, 29], keep: RUN, key: 'bank', ground: [1102, 902], scale: 74 / 109, note: 'the shore bank, the level run\'s left part, turned to each edge' };
REGIONS['bank.right'] = { ...REGIONS['bank.left'], ground: [1178, 902], note: 'the shore bank, the level run\'s right part, turned to each edge' };
const DISC = Array.from({ length: 32 }, (_, i) => [1102 + 8.8 * Math.cos((i / 32) * 2 * Math.PI), 898 + 8.8 * Math.sin((i / 32) * 2 * Math.PI)]);
REGIONS['bank.disc'] = { file: 'terrain', rect: [1040, 878, 200, 29], keep: DISC, key: 'bank', ground: [1102, 898], scale: 74 / 109, note: 'a disc of the shore bank, for a corner of the shore' };

// The grounds the source lacks, as ramps between colours the source has: a shade of the woods, the grass, the
// hill's shaded ochre and the bank's tan, the water and its pale rim. Each is a source rectangle's mean.
const RAMP = {
  grassDark: [60, 700, 80, 40],
  grassLight: [1000, 650, 60, 40],
  woodsShade: [360, 615, 80, 18],
  hillShade: [230, 862, 40, 16],
  bankTan: [1080, 884, 100, 12],
  waterDark: [919, 902, 16, 8],
  waterLight: [919, 902, 4, 4],
};
const ramp = (dark, light, mix) => ({ op: 'ramp', dark, light, mix });

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

// The grounds the source lacks are the plains' recipes with a ramp; a variant is the plains' three (clean,
// grass, clean mirrored), so the three cells differ as the plains' do.
const RECIPES = [
  { region: 'plains.clean', ops: [] },
  { region: 'plains.grass', ops: [] },
  { region: 'plains.clean', ops: ['mirror'] },
];
const ground = (kind, rampOf, flat) => RECIPES.forEach((r, v) => {
  const ops = [...(rampOf ? [rampOf] : []), ...r.ops];
  CELLS[`terrain.${kind}.${v}`] = { region: r.region, ops, anchor: 'centre', ...(flat ? { flat } : {}) };
});
['sea.0', 'sea.1', 'sea.2'].forEach((region, v) => {
  CELLS[`terrain.sea.${v}`] = { region, ops: [], anchor: 'centre' };
  CELLS[`terrain.lake.${v}`] = { region, ops: [ramp(RAMP.waterDark, RAMP.waterLight, 1)], anchor: 'centre' };
});
ground('coast', ramp(RAMP.grassDark, RAMP.bankTan, 0.35), RAMP.bankTan);
ground('forest', ramp(RAMP.woodsShade, RAMP.grassLight, 0.7));
ground('desert', ramp(RAMP.hillShade, RAMP.bankTan, 1));
CELLS['terrain.marsh.0'] = { region: 'marsh.grass', ops: [ramp(RAMP.woodsShade, RAMP.grassLight, 0.4)], anchor: 'centre' };
CELLS['terrain.marsh.1'] = { region: 'marsh.grass', ops: [ramp(RAMP.woodsShade, RAMP.grassLight, 0.4)], anchor: 'centre' };
CELLS['terrain.marsh.2'] = { region: 'marsh.grass', ops: [ramp(RAMP.woodsShade, RAMP.grassLight, 0.4), 'mirror'], anchor: 'centre' };
const HILL_FLAT = [160, 845, 60, 25];
CELLS['terrain.hills.0'] = { region: 'hills.a', ops: [], anchor: 'centre', flat: HILL_FLAT };
CELLS['terrain.hills.1'] = { region: 'hills.b', ops: [], anchor: 'centre', flat: HILL_FLAT };
CELLS['terrain.hills.2'] = { region: 'hills.a', ops: ['mirror'], anchor: 'centre', flat: HILL_FLAT };

// The bank's edge pieces: the strip turned to edge d (the source run faces south: 60 d − 90 degrees), on the
// land side of the edge with its anchor at the edge's midpoint. The corners are a disc of the same bank.
for (let d = 0; d < 6; d++) {
  for (let v = 0; v < 2; v++) {
    CELLS[`shore.edge.${d}.${v}`] = { region: v ? 'bank.right' : 'bank.left', ops: [], anchor: [56, 56], rotate: 60 * d - 90 };
  }
}
for (let c = 0; c < 6; c++) {
  CELLS[`shore.in.${c}`] = { region: 'bank.disc', ops: [], anchor: [24, 24] };
  CELLS[`shore.out.${c}`] = { region: 'bank.disc', ops: [], anchor: [24, 24] };
}
// The fringes: each is its kind's variant cell, masked to a ragged strip along the edge.
for (const kind of FRINGE) {
  for (let d = 0; d < 6; d++) {
    for (let v = 0; v < 2; v++) CELLS[`fringe.${kind}.${d}.${v}`] = { from: `terrain.${kind}.${v}`, fringe: { dir: d, variant: v } };
  }
}
