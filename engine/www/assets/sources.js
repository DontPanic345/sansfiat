// The source art: named regions of the studies in `docs/proposals/map-art/`, and the atlas cells cut
// from them. `draw-atlas.mjs` reads these; the browser never does.
import { CLAN, FRINGE, HEX_R, TREE, UNIT } from './placeholders.js';

/** Source studies, by name: repo paths, outside the site. */
export const FILES = { terrain: 'docs/proposals/map-art/terrain.png', actors: 'docs/proposals/map-art/actors.png' };

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

// The trees: whole trees from the source's tree groups, each keep-polygon round its own crown so no tree behind it
// comes along. The big ones are the left clump's front tree and two of the band's front row; the small ones are the
// right group's thinning tail and its mid tree, cut smaller. Ground points are the trunks' feet.
REGIONS['tree.big.0'] = {
  file: 'terrain', rect: [130, 556, 68, 92],
  keep: [[163, 560], [178, 575], [190, 596], [194, 612], [178, 612], [172, 618], [174, 640], [152, 640], [154, 618], [146, 612], [132, 612], [135, 596], [149, 575]],
  key: 'tree', ground: [160, 637], scale: 0.9,
  note: "the left clump's front tree, whole: its crown and trunk, the trees behind it cut off",
};
REGIONS['tree.big.1'] = {
  file: 'terrain', rect: [448, 562, 50, 76],
  keep: [[473, 566], [486, 580], [493, 592], [491, 604], [484, 606], [478, 616], [479, 630], [468, 630], [466, 616], [458, 606], [452, 596], [455, 582], [464, 576]],
  key: 'tree', ground: [473, 632], scale: 0.9,
  note: "the band's front tree, the one whose trunk stands at (473, 632)",
};
REGIONS['tree.big.2'] = {
  file: 'terrain', rect: [600, 542, 62, 96],
  keep: [[630, 546], [644, 560], [656, 580], [658, 594], [646, 600], [642, 610], [640, 624], [632, 632], [622, 632], [618, 622], [614, 610], [606, 600], [602, 586], [612, 568], [622, 556]],
  key: 'tree', ground: [627, 632], scale: 0.9,
  note: "the band's front tree at its right end, its trunk at (627, 632)",
};
REGIONS['tree.small.0'] = {
  file: 'terrain', rect: [1396, 558, 36, 50],
  keep: [[1416, 562], [1426, 576], [1430, 588], [1426, 597], [1419, 600], [1417, 606], [1409, 606], [1407, 599], [1401, 592], [1402, 581], [1409, 570]],
  key: 'tree', ground: [1412, 603], scale: 0.85,
  note: "the right group's tail: an isolated small tree, its trunk at (1412, 603)",
};
REGIONS['tree.small.1'] = {
  file: 'terrain', rect: [1432, 538, 48, 66],
  keep: [[1452, 541], [1466, 560], [1473, 578], [1476, 590], [1466, 596], [1456, 598], [1455, 606], [1449, 606], [1449, 598], [1440, 594], [1434, 582], [1438, 566], [1446, 552]],
  key: 'tree', ground: [1452, 600], scale: 0.85,
  note: "the right group's tail: a slim small tree, its trunk at (1452, 600)",
};
REGIONS['tree.small.2'] = {
  file: 'terrain', rect: [1336, 528, 52, 82],
  keep: [[1364, 530], [1378, 552], [1384, 572], [1380, 586], [1368, 590], [1366, 600], [1358, 600], [1356, 590], [1346, 586], [1341, 572], [1348, 552], [1356, 538]],
  key: 'tree', ground: [1362, 598], scale: 0.65,
  note: 'the mid tree of the right group, cut smaller than the big trees',
};

// The reeds: the plains hex's yellow tuft blades, kept by `tuft` and stretched tall (scaleY is twice scale), then
// ramped from the trees' dark to a straw colour taken from a tuft (`RAMP.straw`). Two tufts, and the first mirrored.
REGIONS['reed.0'] = { file: 'terrain', rect: [496, 800, 60, 40], keep: [[496, 800], [556, 800], [556, 840], [496, 840]], key: 'tuft', ground: [524, 836], scale: 0.75, scaleY: 1.2, note: 'a tuft of the plains hex, stretched tall: its base at (524, 836)' };
REGIONS['reed.1'] = { file: 'terrain', rect: [556, 786, 48, 30], keep: [[556, 788], [600, 788], [600, 814], [556, 814]], key: 'tuft', ground: [578, 811], scale: 1.1, scaleY: 2.0, note: 'the small tuft by the plains hex, enlarged (it is small in the source): its base at (578, 811)' };

// The water: clean blocks of the sea (no foam arc, no bank), each repeated and mirrored into a hex. Variant 0 is open
// water with faint streaks, 1 and 2 carry a few light specks, so a bay is not one ripple repeated; the scales differ
// so the copies' seams do not line up from one variant to the next.
const SEA = { file: 'terrain', note: 'the sea: a clean block of water, repeated and mirrored, filled into the hex' };
REGIONS['sea.0'] = { ...SEA, fill: { rect: [1296, 814, 70, 28], scale: 1.0, offset: [0, 0] } };
REGIONS['sea.1'] = { ...SEA, fill: { rect: [1312, 852, 70, 40], scale: 1.0, offset: [10, 5] } };
REGIONS['sea.2'] = { ...SEA, fill: { rect: [1304, 846, 74, 26], scale: 1.0, offset: [40, 9] } };

// The hills: the bottom-left row's first two hexes. Hex 2's left-lower part is the first hill's tail: grass over it.
// The marsh: the plains' clean hex (no rocks) with four pools patched in from the water block, large enough to read
// as water at play zoom, so the marsh is wet ground with reeds standing in it.
REGIONS['marsh.grass'] = {
  ...REGIONS['plains.clean'],
  patches: [
    ...REGIONS['plains.clean'].patches,
    { from: [1305, 852], to: [470, 838], w: 120, h: 44, feather: 5, note: 'a pool of the sea block, left: the largest' },
    { from: [1330, 858], to: [562, 794], w: 80, h: 38, feather: 5, note: 'a second pool, upper right' },
    { from: [1300, 852], to: [525, 892], w: 84, h: 36, feather: 5, note: 'a third pool, below' },
    { from: [1380, 862], to: [500, 778], w: 52, h: 26, feather: 4, note: 'a fourth pool, top left' },
  ],
  note: 'bottom-left row, third hex, rocks patched out, with four large pools of water patched in: the marsh',
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
// variant, turned to each edge; the corners bend the same run (`in` fans it, `out` mitres two strips). Each keeps the
// bank's tan (key `bank`). Its scale sets the bank's thickness: the run's tan is 18 source px deep.
const BANK_SCALE = 0.95;
const RUN = [[1040, 878], [1240, 878], [1240, 906], [1040, 906]];
REGIONS['bank.left'] = { file: 'terrain', rect: [1040, 878, 200, 29], keep: RUN, key: 'bank', ground: [1102, 902], scale: BANK_SCALE, note: 'the shore bank, the level run\'s left part, turned to each edge' };
REGIONS['bank.right'] = { ...REGIONS['bank.left'], ground: [1178, 902], note: 'the shore bank, the level run\'s right part, turned to each edge' };

// The actors: the study's figures and banner on transparent ground (key `actor`). Each stands alone in its box, the
// box with a margin of 3 px as its keep-polygon. A figure's ground point is its feet; the flag's is its pole's top,
// so it hangs from the point it is set on.
REGIONS['actor.camp'] = {
  file: 'actors', rect: [580, 69, 135, 193], keep: [[580, 69], [715, 69], [715, 262], [580, 262]],
  key: 'actor', ground: [645, 256], scale: 0.48,
  note: "the round-shield spearman, standing: his feet at (645, 256), the spear's head at the box's top",
};
REGIONS['actor.scout'] = {
  file: 'actors', rect: [190, 104, 117, 155], keep: [[190, 104], [307, 104], [307, 259], [190, 259]],
  key: 'actor', ground: [246, 253], scale: 0.6,
  note: "the walker with a pack and a staff, standing: his feet at (246, 253), the staff's top at about (294, 149)",
};
REGIONS['actor.flag'] = {
  file: 'actors', rect: [1357, 97, 110, 163], keep: [[1357, 97], [1467, 97], [1467, 260], [1357, 260]],
  key: 'actor', ground: [1372, 106], scale: 0.2,
  note: "the study's flag, its pole's top at (1372, 106): the cloth hangs to the right of the pole, the pole runs down past it",
};
// The clans: the study's second row, each group on transparent ground (key `actor`). Each keep-polygon is its box with a
// 3 px margin, less what the box holds that is not the group. `floor` 128 clears the row's soft fringe (alpha under 128).
REGIONS['clan.tent'] = {
  file: 'actors', rect: [76, 349, 257, 194], keep: [[76, 349], [333, 349], [333, 390], [316, 390], [316, 492], [333, 492], [333, 543], [76, 543]],
  key: 'actor', ground: [205, 528], scale: 0.43, floor: 128,
  note: 'the hide tent, a clan that gathers: its basket at the foot kept, the drying rack and the barrel left out, its ground at its middle',
};
REGIONS['clan.hearth'] = {
  file: 'actors', rect: [1169, 335, 286, 221], keep: [[1169, 335], [1452, 335], [1452, 400], [1436, 430], [1412, 468], [1404, 488], [1404, 556], [1169, 556]],
  key: 'actor', ground: [1320, 540], scale: 0.38, floor: 128,
  note: 'the left tent, the seated man, the fire with its pot and the smoke, a settled clan: the right tent and the woman left out, its ground at its middle',
};
REGIONS['clan.walk'] = {
  file: 'actors', rect: [614, 383, 255, 157], keep: [[614, 383], [869, 383], [869, 540], [614, 540]],
  key: 'actor', ground: [742, 534], scale: 0.44, floor: 128,
  note: 'the man, the woman and the child walking, a clan that came to its home: the donkey left out, its ground at its middle',
};

// The grounds the source lacks, as ramps between colours the source has: the tree's dark floor and leaf, the grass,
// the bank's shaded and lit sand and the water. Each end is a source rectangle's mean.
const RAMP = {
  grassDark: [60, 700, 80, 40],
  treeFloor: [600, 600, 60, 20],
  treeDark: [150, 540, 20, 20],
  treeLeaf: [370, 510, 20, 20],
  sandShade: [1080, 884, 100, 12],
  sandLit: [1070, 893, 30, 8],
  waterDark: [919, 902, 16, 8],
  waterLight: [919, 902, 4, 4],
  straw: [520, 817, 8, 8], // the bright core of a plains tuft's blades: the reeds' light end
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
  CELLS[`terrain.lake.${v}`] = { region, ops: [{ ...ramp(RAMP.waterDark, RAMP.waterLight, 1), lum: [45, 110] }], anchor: 'centre' };
});
// Coast: grass warmed toward the sand, a band along the water that still reads as land. Its flat is the shaded sand.
ground('coast', ramp(RAMP.grassDark, RAMP.sandLit, 0.8), RAMP.sandShade);
// Forest: the grass darkened to the trees' floor and leaf, until the trees are planted.
ground('forest', ramp(RAMP.treeDark, RAMP.grassDark, 0.7), RAMP.treeFloor);
// The wood and the marsh: the trees are cut from the studies' tree groups as they are; the reeds are a tuft ramped from
// the trees' dark to the straw, the first of them mirrored for the third variant.
for (let v = 0; v < 3; v++) {
  CELLS[`tree.big.${v}`] = { region: `tree.big.${v}`, ops: [], anchor: [TREE.big.ax, TREE.big.ay] };
  CELLS[`tree.small.${v}`] = { region: `tree.small.${v}`, ops: [], anchor: [TREE.small.ax, TREE.small.ay] };
}
const REED = [
  { region: 'reed.0', ops: [] },
  { region: 'reed.1', ops: [] },
  { region: 'reed.0', ops: ['mirror'] },
];
REED.forEach((r, v) => {
  CELLS[`reed.${v}`] = { region: r.region, ops: [ramp(RAMP.treeLeaf, RAMP.straw, 1), ...r.ops], anchor: [TREE.reed.ax, TREE.reed.ay] };
});
// Desert: the grass recoloured onto the sand, shaded to lit; the stipple no longer draws.
ground('desert', ramp(RAMP.sandShade, RAMP.sandLit, 1), RAMP.sandLit);
// Marsh: the grass with pools of the sea's water, murked toward the trees' dark teal.
const MARSH = { ...ramp(RAMP.treeDark, RAMP.grassDark, 0.45), keepWater: true };
CELLS['terrain.marsh.0'] = { region: 'marsh.grass', ops: [MARSH], anchor: 'centre' };
CELLS['terrain.marsh.1'] = { region: 'marsh.grass', ops: [MARSH], anchor: 'centre' };
CELLS['terrain.marsh.2'] = { region: 'marsh.grass', ops: [MARSH, 'mirror'], anchor: 'centre' };
const HILL_FLAT = [170, 845, 40, 10];
CELLS['terrain.hills.0'] = { region: 'hills.a', ops: [], anchor: 'centre', flat: HILL_FLAT };
CELLS['terrain.hills.1'] = { region: 'hills.b', ops: [], anchor: 'centre', flat: HILL_FLAT };
CELLS['terrain.hills.2'] = { region: 'hills.a', ops: ['mirror'], anchor: 'centre', flat: HILL_FLAT };

// The bank's edge pieces: the strip turned to edge d (the source run faces south: 60 d − 90 degrees), on the
// land side of the edge with its anchor at the edge's midpoint, ending at the edge's two vertices (half the edge,
// HEX_R / 2, and a half px over). The corners: water's corner c fans the run round the vertex; land's corner c
// mitres the strips of its two edges.
for (let d = 0; d < 6; d++) {
  for (let v = 0; v < 2; v++) {
    CELLS[`shore.edge.${d}.${v}`] = { region: v ? 'bank.right' : 'bank.left', ops: [], anchor: [56, 56], rotate: 60 * d - 90, shape: { type: 'edge', half: HEX_R / 2 + 0.5 } };
  }
}
for (let c = 0; c < 6; c++) {
  CELLS[`shore.in.${c}`] = { region: 'bank.left', ops: [], anchor: [24, 24], shape: { type: 'in', c, span: 30.5 } };
  CELLS[`shore.out.${c}`] = { region: 'bank.left', ops: [], anchor: [24, 24], shape: { type: 'out', c, reach: 0.5 } };
}
// The fringes: each is its kind's variant cell, masked to a ragged strip along the edge.
for (const kind of FRINGE) {
  for (let d = 0; d < 6; d++) {
    for (let v = 0; v < 2; v++) CELLS[`fringe.${kind}.${d}.${v}`] = { from: `terrain.${kind}.${v}`, fringe: { dir: d, variant: v } };
  }
}

// The unit figures: the study's round-shield spearman (the camp) and walker (the scout), each standing 88 px tall with
// its feet at the anchor (48, 92), and the study's flag cut small, its pole's top on the spear's shaft below the head
// (the camp) or on the staff's top (the scout). A mask is its figure's spec with `cloth` and the `grey` op: the flag's
// cloth, and for the camp the round shield's face (an ellipse in the study, 31 by 34 px about (675.5, 193.5)).
const UNIT_ANCHOR = [UNIT.ax, UNIT.ay];
const FLAG_CLOTH = [[1380, 132], [1394, 133], [1427, 132], [1429, 126], [1438, 124], [1451, 124], [1454, 130], [1454, 180], [1450, 184], [1446, 188], [1428, 191], [1421, 195], [1397, 195], [1380, 191]];
const SHIELD_FACE = [[706.5, 193.5], [705.4, 202.3], [702.3, 210.5], [697.4, 217.5], [691, 222.9], [683.5, 226.3], [675.5, 227.5], [667.5, 226.3], [660, 222.9], [653.6, 217.5], [648.7, 210.5], [645.6, 202.3], [644.5, 193.5], [645.6, 184.7], [648.7, 176.5], [653.6, 169.5], [660, 164.1], [667.5, 160.7], [675.5, 159.5], [683.5, 160.7], [691, 164.1], [697.4, 169.5], [702.3, 176.5], [705.4, 184.7]];
// The tunic: the study's ochre cloth, a polygon round the body (the skin, hair, boots, pack and gloves lie outside it), less the
// outline's darkest pixels. Points are read off a 6x crop of each figure: its origin plus the crop's px over 6.
const crop6 = (ox, oy, pts) => pts.map(([x, y]) => [ox + x / 6, oy + y / 6]);
const lit = (r, g, b) => 0.299 * r + 0.587 * g + 0.114 * b >= 60;
const CAMP_TUNIC = { poly: crop6(580, 69, [[150, 560], [230, 490], [330, 480], [430, 540], [420, 640], [370, 700], [360, 930], [240, 930], [190, 880], [190, 780], [140, 770], [140, 590]]), where: lit };
const SCOUT_BODY = { poly: crop6(190, 104, [[200, 340], [250, 290], [320, 285], [335, 360], [435, 360], [455, 335], [500, 325], [545, 420], [495, 475], [490, 580], [310, 580], [305, 430], [175, 420]]), where: lit };
const SCOUT_SHORTS = { poly: crop6(190, 104, [[190, 590], [500, 590], [505, 710], [300, 715], [185, 690]]), where: lit };
// The cloth's grey runs from this up to white: high, so the civ's colour stays bright under the art's shading.
const GREY = { op: 'grey', lo: 190 };
const CAMP_FLAG = { region: 'actor.flag', at: [25, 34] };
const SCOUT_FLAG = { region: 'actor.flag', at: [75.5, 30] };
CELLS['unit.camp'] = { region: 'actor.camp', over: [CAMP_FLAG], ops: [], anchor: UNIT_ANCHOR };
CELLS['unit.camp.mask'] = { region: 'actor.camp', over: [CAMP_FLAG], cloth: { 'actor.camp': [SHIELD_FACE, CAMP_TUNIC], 'actor.flag': [FLAG_CLOTH] }, ops: [GREY], anchor: UNIT_ANCHOR };
CELLS['unit.scout'] = { region: 'actor.scout', over: [SCOUT_FLAG], ops: [], anchor: UNIT_ANCHOR };
CELLS['unit.scout.mask'] = { region: 'actor.scout', over: [SCOUT_FLAG], cloth: { 'actor.scout': [SCOUT_BODY, SCOUT_SHORTS], 'actor.flag': [FLAG_CLOTH] }, ops: [GREY], anchor: UNIT_ANCHOR };

// The clans' cloth, read off 4x crops of the study (source px): the hide's two flaps (the opening's dark, the poles and
// the basket left out), the hearth's tent canvas (the poles, the man, the fire and the smoke left out), and the walkers'
// tunics, dress and shawl and the child's tunic (skin, hair, packs, staff and boots left out, by `isCloth`).
const CLAN_ANCHOR = [CLAN.ax, CLAN.ay];
const isCloth = (r, g, b) => lit(r, g, b) && b < 0.5 * r; // not skin, which is pinker: its blue is over half its red
const TENT_LEFT = [[242.5, 387.5], [220, 391.3], [190, 411.3], [152.5, 420], [132.5, 426.3], [130, 470], [132.5, 520], [152.5, 530], [185, 530], [187.5, 490], [210, 490], [230, 500], [230, 415]];
const TENT_RIGHT = [[250, 400], [260, 422.5], [282.5, 440], [302.5, 457.5], [322.5, 485.5], [310, 488], [290, 483.8], [270, 481.3], [252.5, 481.3]];
const HEARTH_CANVAS = [[1295, 401.3], [1340, 466.3], [1315, 462], [1290, 462], [1260, 469], [1230.5, 473], [1230.5, 490], [1223, 490], [1223, 470], [1195, 469], [1192.5, 465], [1202.5, 450], [1230, 417.5], [1240, 410], [1285, 402.5]];
const MAN_TUNIC = [[647.5, 430], [672.5, 428.8], [692.5, 430], [696.3, 445], [697.5, 462.5], [697.5, 497.5], [682.5, 502.5], [667.5, 500], [651.3, 497.5], [647.5, 475]];
const WOMAN_DRESS = [[735, 442.5], [750, 440], [770, 437.5], [782.5, 438.8], [805, 438.8], [810, 455], [807.5, 480], [805, 500], [802.5, 518.8], [770, 518.8], [745, 518.8], [740, 497.5], [733.8, 475], [732.5, 455]];
const CHILD_TUNIC = [[817.5, 478.8], [857.5, 478.8], [861.3, 485], [862.5, 507.5], [860, 520], [817.5, 520], [815, 505], [816.3, 487.5]];
const CLAN_CLOTH = {
  tent: [{ poly: TENT_LEFT, where: lit }, { poly: TENT_RIGHT, where: lit }],
  hearth: [{ poly: HEARTH_CANVAS, where: lit }],
  walk: [{ poly: MAN_TUNIC, where: isCloth }, { poly: WOMAN_DRESS, where: isCloth }, { poly: CHILD_TUNIC, where: isCloth }],
};
for (const kind of ['tent', 'hearth', 'walk']) {
  CELLS[`clan.${kind}`] = { region: `clan.${kind}`, ops: [], anchor: CLAN_ANCHOR };
  CELLS[`clan.${kind}.mask`] = { region: `clan.${kind}`, cloth: { [`clan.${kind}`]: CLAN_CLOTH[kind] }, ops: [GREY], anchor: CLAN_ANCHOR };
}
