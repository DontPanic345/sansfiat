// pop: puts tile T at css x (or y) just past each canvas edge and draws with culling on and off; reports the pixels that differ.
async function pop(view, tile) {
  const { page } = await setUp(view);
  try {
    for (const [side, off] of [['left', -90], ['right', 90], ['top', -90], ['bottom', 90], ['left', -130], ['right', 130], ['top', -130], ['bottom', 130]]) {
      const r = await page.eval(`(async () => {
        const m = game.map, t = ${tile};
        const x = m.px[t], y = m.py[t];
        const sx = ${JSON.stringify(side)} === 'left' ? ${off} : ${JSON.stringify(side)} === 'right' ? m.w + ${off} : m.w / 2;
        const sy = ${JSON.stringify(side)} === 'top' ? ${off} : ${JSON.stringify(side)} === 'bottom' ? m.h + ${off} : m.h / 2;
        m.cx = x - (sx - m.w / 2) / m.k; m.cy = y - (sy - m.h / 2) / m.k;
        const grab = () => { m.dirty = true; m.drawNow(); const c = m.canvas; return c.getContext('2d').getImageData(0, 0, c.width, c.height).data; };
        m.noCull = false; const a = grab(); m.noCull = true; const b = grab(); m.noCull = false;
        let d = 0; for (let i = 0; i < a.length; i += 4) if (a[i] !== b[i] || a[i+1] !== b[i+1] || a[i+2] !== b[i+2]) d++;
        return d;
      })()`);
      console.log(`${view.name} tile ${tile} ${side} ${off}px: ${r} px differ`);
    }
  } finally { await page.close(); }
}
const [name, tile] = process.argv.slice(2);
await pop(VIEWS.find((v) => v.name === name), Number(tile));
