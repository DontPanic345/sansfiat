// The placeholder art: every atlas cell drawn from the page's flat colours and shapes. `draw-atlas.mjs`
// renders these into `atlas.png`; paid art replaces the PNG and keeps the cells `layout()` names.

export const TERRAINS = ['sea', 'lake', 'coast', 'plains', 'forest', 'hills', 'mountain', 'desert', 'marsh'];
export const TERRAIN_COLOR = {
  sea: '#1f4f7a',
  lake: '#3f8fc4',
  coast: '#d6c487',
  plains: '#9cbf5a',
  forest: '#3f7a3a',
  hills: '#8a7a5c',
  mountain: '#7c7a85',
  desert: '#efe3b8',
  marsh: '#5f8a6a',
};
export const WIDTH = 1024;
/** The circumradius of a terrain cell's hex, in cell pixels (the cell is 2 × this tall). */
export const HEX_R = 74;
/** How many world units (a hex has circumradius 1) a 64 px icon cell spans. */
export const ICON_WORLD = { 'icon.festival': 2.5, 'icon.centre': 4, 'icon.hearth': 1.6, 'icon.pass': 1, 'icon.town': 1.1 };

const UNITS = ['camp', 'scout', 'guard'];
const ICONS = ['capital', 'festival', 'centre', 'hearth', 'pass'];

/** Every cell with its place in the sheet, its size and a note: `{width, height, cells}`. */
export function layout() {
  const cells = {};
  let x = 0, y = 0, rowH = 0;
  const put = (name, w, h, note) => {
    if (x + w > WIDTH) {
      x = 0;
      y += rowH;
      rowH = 0;
    }
    cells[name] = { x, y, w, h, note };
    x += w;
    rowH = Math.max(rowH, h);
  };
  const row = () => {
    if (x > 0) {
      x = 0;
      y += rowH;
      rowH = 0;
    }
  };
  const hexNote = (what) => `${what}: pointy-top hex, circumradius ${HEX_R}, centred, transparent outside`;
  for (const t of TERRAINS) {
    for (let v = 0; v < 3; v++) put(`terrain.${t}.${v}`, 128, 148, hexNote(`${t}, variant ${v}`));
  }
  row();
  put('overlay.floodplain', 128, 148, hexNote('floodplain wash, translucent, drawn over the tile'));
  put('overlay.desert-dots', 128, 148, hexNote('desert stipple, translucent, drawn over a desert tile'));
  put('overlay.fields', 128, 148, hexNote('field furrows: three short dark ochre strokes in the lower half, translucent, drawn over a tile with fields'));
  row();
  for (const u of UNITS) {
    put(`unit.${u}`, 64, 64, `${u} figure: dark casing and edge, transparent inside (the civ's colour shows through from the mask)`);
    put(`unit.${u}.mask`, 64, 64, `${u} silhouette in opaque white: tinted in the civ's colour and drawn under the figure`);
  }
  row();
  put('icon.capital', 64, 64, "capital: a house with a door, casing and white edge, transparent inside (the civ's colour shows through from the mask)");
  put('icon.capital.mask', 64, 64, "capital silhouette in opaque white: tinted in the civ's colour");
  put('icon.festival', 64, 64, 'festival: a white ring with a dark casing');
  put('icon.centre', 64, 64, 'centre: a gold ring with a dark casing, round the festival that is a centre');
  put('icon.hearth', 64, 64, 'hearth: a small yellow square with a dark edge');
  put('icon.pass', 64, 64, 'pass: a pale dot with a dark edge');
  put('icon.town', 64, 64, 'town: a white diamond with a dark casing, at the top of a festival tile that several clans live on');
  row();
  return { width: WIDTH, height: y, cells };
}

const corners = (cx, cy, r) =>
  [0, 1, 2, 3, 4, 5].map((i) => [cx + r * Math.cos((Math.PI / 3) * i - Math.PI / 6), cy + r * Math.sin((Math.PI / 3) * i - Math.PI / 6)]);

const hexPath = (ctx, w, h, r = HEX_R) => {
  ctx.beginPath();
  corners(w / 2, h / 2, r).forEach(([x, y], i) => (i ? ctx.lineTo(x, y) : ctx.moveTo(x, y)));
  ctx.closePath();
};

const SHIELD_S = 22; // half the shield's height, in cell px
const shield = (ctx, w, h) => {
  const s = SHIELD_S, x = w / 2, y = h / 2;
  ctx.beginPath();
  ctx.moveTo(x - s * 0.8, y - s);
  ctx.lineTo(x + s * 0.8, y - s);
  ctx.lineTo(x + s * 0.8, y + s * 0.1);
  ctx.lineTo(x, y + s);
  ctx.lineTo(x - s * 0.8, y + s * 0.1);
  ctx.closePath();
};

const HOUSE_S = 22;
const house = (ctx, w, h) => {
  const s = HOUSE_S, x = w / 2, y = h / 2 + s * 0.075;
  ctx.beginPath();
  ctx.moveTo(x - s, y + s * 0.8);
  ctx.lineTo(x - s, y - s * 0.1);
  ctx.lineTo(x, y - s * 0.95);
  ctx.lineTo(x + s, y - s * 0.1);
  ctx.lineTo(x + s, y + s * 0.8);
  ctx.closePath();
};

const ring = (ctx, w, h, r, colour, line, casing) => {
  for (const [col, lw] of [['rgba(0,0,0,0.7)', line + casing], [colour, line]]) {
    ctx.strokeStyle = col;
    ctx.lineWidth = lw;
    ctx.beginPath();
    ctx.arc(w / 2, h / 2, r, 0, 2 * Math.PI);
    ctx.stroke();
  }
};

/** Draws cell `name` into `ctx`, filling (0, 0, w, h); the cell is clear before. */
export function draw(name, ctx, w, h) {
  ctx.lineJoin = 'round';
  ctx.lineCap = 'round';
  const [kind, id] = name.split('.');
  if (kind === 'terrain') {
    ctx.fillStyle = TERRAIN_COLOR[id];
    hexPath(ctx, w, h, HEX_R * 1.0);
    ctx.fill();
    const u = HEX_R, cx = w / 2, cy = h / 2;
    if (id === 'mountain') {
      ctx.beginPath();
      ctx.moveTo(cx, cy - 0.62 * u);
      ctx.lineTo(cx + 0.58 * u, cy + 0.38 * u);
      ctx.lineTo(cx - 0.58 * u, cy + 0.38 * u);
      ctx.closePath();
      ctx.fillStyle = '#c4c2cc';
      ctx.fill();
      ctx.strokeStyle = '#3c3a46';
      ctx.lineWidth = 4;
      ctx.stroke();
    } else if (id === 'marsh') {
      ctx.strokeStyle = '#2f5a46';
      ctx.lineWidth = 5;
      ctx.beginPath();
      for (const [dx, dy] of [[-0.4, 0.2], [0, -0.05], [0.4, 0.25]]) {
        const x = cx + dx * u, y = cy + dy * u;
        ctx.moveTo(x, y + 0.3 * u);
        ctx.lineTo(x, y - 0.3 * u);
        ctx.moveTo(x - 0.12 * u, y - 0.12 * u);
        ctx.lineTo(x, y - 0.3 * u);
        ctx.lineTo(x + 0.12 * u, y - 0.12 * u);
      }
      ctx.stroke();
    }
  } else if (name === 'overlay.floodplain') {
    ctx.fillStyle = 'rgba(96,235,70,0.42)';
    hexPath(ctx, w, h, HEX_R * 1.02);
    ctx.fill();
  } else if (name === 'overlay.fields') {
    ctx.strokeStyle = '#7a5a1e';
    ctx.lineWidth = 6;
    ctx.lineCap = 'round';
    ctx.beginPath();
    for (const y of [0.18, 0.4, 0.62]) {
      ctx.moveTo(w / 2 - 0.5 * HEX_R, h / 2 + y * HEX_R);
      ctx.lineTo(w / 2 + 0.5 * HEX_R, h / 2 + y * HEX_R);
    }
    ctx.stroke();
  } else if (name === 'overlay.desert-dots') {
    ctx.fillStyle = '#b39a55';
    let s = 12345;
    const next = () => {
      s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
      return ((s >>> 8) % 1000) / 1000 - 0.5;
    };
    for (let d = 0; d < 7; d++) {
      const ax = next(), ay = next();
      if (Math.abs(ax) * 2 + Math.abs(ay) * 1.6 < 1.3) ctx.fillRect(w / 2 + ax * 1.4 * HEX_R - 3, h / 2 + ay * 1.4 * HEX_R - 3, 6, 6);
    }
  } else if (kind === 'unit') {
    shield(ctx, w, h);
    if (name.endsWith('.mask')) {
      ctx.fillStyle = '#fff';
      ctx.fill();
    } else {
      ctx.strokeStyle = '#000';
      ctx.lineWidth = 5;
      ctx.stroke();
      ctx.strokeStyle = 'rgba(0,0,0,0.9)';
      ctx.lineWidth = 1.5;
      ctx.stroke();
    }
  } else if (name === 'icon.capital.mask') {
    house(ctx, w, h);
    ctx.fillStyle = '#fff';
    ctx.fill();
  } else if (name === 'icon.capital') {
    house(ctx, w, h);
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 5;
    ctx.stroke();
    ctx.strokeStyle = '#fff';
    ctx.lineWidth = 1.8;
    ctx.stroke();
    const s = HOUSE_S, x = w / 2, y = h / 2 + s * 0.075;
    ctx.fillStyle = 'rgba(0,0,0,0.75)';
    ctx.fillRect(x - s * 0.22, y + s * 0.1, s * 0.44, s * 0.7);
  } else if (name === 'icon.festival') {
    ring(ctx, w, h, 0.9 * (w / ICON_WORLD[name]), '#fff', 4, 4);
  } else if (name === 'icon.centre') {
    ring(ctx, w, h, 1.7 * (w / ICON_WORLD[name]), '#ffc800', 5, 4);
  } else if (name === 'icon.hearth') {
    const q = 0.4 * (w / ICON_WORLD[name]);
    ctx.fillStyle = '#ffd400';
    ctx.strokeStyle = '#3a2c00';
    ctx.lineWidth = 2;
    ctx.fillRect(w / 2 - q / 2, h / 2 - q / 2, q, q);
    ctx.strokeRect(w / 2 - q / 2, h / 2 - q / 2, q, q);
  } else if (name === 'icon.town') {
    const q = 0.55 * (w / ICON_WORLD[name]);
    ctx.beginPath();
    ctx.moveTo(w / 2, h / 2 - q);
    ctx.lineTo(w / 2 + q, h / 2);
    ctx.lineTo(w / 2, h / 2 + q);
    ctx.lineTo(w / 2 - q, h / 2);
    ctx.closePath();
    ctx.fillStyle = '#fff';
    ctx.strokeStyle = '#1a1a1a';
    ctx.lineWidth = 3;
    ctx.fill();
    ctx.stroke();
  } else if (name === 'icon.pass') {
    ctx.fillStyle = '#fff2b0';
    ctx.strokeStyle = '#3a2c00';
    ctx.lineWidth = 3;
    ctx.beginPath();
    ctx.arc(w / 2, h / 2, 0.3 * (w / ICON_WORLD[name]), 0, 2 * Math.PI);
    ctx.fill();
    ctx.stroke();
  } else {
    throw new Error(`no placeholder for ${name}`);
  }
}
