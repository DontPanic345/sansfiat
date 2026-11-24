// overlays: each View overlay that starts off, turned on alone over the view: DIR/NAME-<layer>.png, and the share of pixels it moves by more than 48 in any channel.
async function overlays(view, out, names) {
  const { page } = await setUp(view);
  try {
    for (const n of names) {
      const r = await page.eval(`(() => {
        const m = game.map, c = m.canvas;
        const grab = () => { m.dirty = true; m.drawNow(); return c.getContext('2d').getImageData(0, 0, c.width, c.height).data; };
        const a = grab(); m.setLayer(${JSON.stringify(n)}, true); const b = grab(); const png = c.toDataURL('image/png'); m.setLayer(${JSON.stringify(n)}, false);
        let d = 0; for (let i = 0; i < a.length; i += 4) if (Math.abs(a[i]-b[i]) > 48 || Math.abs(a[i+1]-b[i+1]) > 48 || Math.abs(a[i+2]-b[i+2]) > 48) d++;
        return { png, share: d / (a.length / 4) };
      })()`);
      writeFileSync(join(out, `${view.name}-${n}.png`), Buffer.from(r.png.split(',')[1], 'base64'));
      console.log(`${view.name} ${n}: ${(100 * r.share).toFixed(1)}% of pixels move by > 48`);
    }
  } finally { await page.close(); }
}
const [name, out, list] = process.argv.slice(2);
mkdirSync(out, { recursive: true });
await overlays(VIEWS.find((v) => v.name === name), out, list.split(','));
