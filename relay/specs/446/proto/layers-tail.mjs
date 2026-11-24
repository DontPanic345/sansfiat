// layers: per static layer, its median draw time over five redraws of the view (ms), the layers drawn as drawStatic draws them.
async function layers(view) {
  const { page } = await setUp(view);
  try {
    const r = await page.eval(`(async () => {
      const { LAYERS } = await import('./map/view.js');
      const m = game.map, ctx = m.lctx, t = {};
      for (let i = 0; i < 5; i++) {
        m.collectVisible();
        ctx.setTransform(1, 0, 0, 1, 0, 0); ctx.fillRect(0, 0, m.layer.width, m.layer.height); m.setWorld(ctx);
        for (const l of LAYERS) {
          if (!l.static || !(m.on[l.name] || l.always?.(m))) continue;
          const t0 = performance.now(); l.draw(ctx, m.model, m); (t[l.name] ??= []).push(performance.now() - t0);
        }
      }
      return Object.entries(t).map(([n, xs]) => [n, xs.sort((a, b) => a - b)[2]]).filter(([, x]) => x >= 0.3);
    })()`);
    console.log(`${view.name}  k ${view.k}  ${r.map(([n, x]) => n + ' ' + x.toFixed(1)).join('  ')}`);
  } finally {
    await page.close();
  }
}
for (const name of process.argv[2].split(',')) await layers(VIEWS.find((v) => v.name === name));
