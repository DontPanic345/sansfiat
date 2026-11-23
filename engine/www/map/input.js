// The map canvas's pointer input: a press, drag, pinch and wheel, picking, painting and the hover. `MapView.attach()`
// calls `attach(view)`, which listens on the view's canvas and calls the view's callbacks (`view.opts`).
// Pure DOM, no drawing.

export function attach(view) {
  const c = view.canvas;
  c.style.touchAction = 'none';
  c.addEventListener('pointerdown', (e) => {
    c.setPointerCapture(e.pointerId);
    view.pointers.set(e.pointerId, { x: e.clientX, y: e.clientY });
    view.opts.onHoverEnd();
    view.drag = { moved: 0, pinch: view.pointers.size > 1 ? pinchState(view) : null, button: e.button };
    if (view.painting && e.button === 0 && view.pointers.size === 1) {
      const rect = c.getBoundingClientRect();
      const i = view.tileAt(e.clientX - rect.left, e.clientY - rect.top);
      view.drag.paint = i;
      if (i >= 0) view.opts.onPaint?.(i);
    }
  });
  c.addEventListener('pointermove', (e) => {
    const p = view.pointers.get(e.pointerId);
    const rect = c.getBoundingClientRect();
    if (!p) {
      const i = view.tileAt(e.clientX - rect.left, e.clientY - rect.top);
      view.opts.onHover(i, e.clientX, e.clientY);
      return;
    }
    const dx = e.clientX - p.x, dy = e.clientY - p.y;
    p.x = e.clientX;
    p.y = e.clientY;
    if (view.pointers.size >= 2) {
      const st = pinchState(view);
      if (view.drag.pinch) {
        view.zoomBy(st.d / view.drag.pinch.d, st.x - rect.left, st.y - rect.top);
        view.panBy(st.x - view.drag.pinch.x, st.y - view.drag.pinch.y);
      }
      view.drag.pinch = st;
      view.drag.moved += 100;
      return;
    }
    if (view.drag.paint !== undefined) {
      const i = view.tileAt(e.clientX - rect.left, e.clientY - rect.top);
      if (i >= 0 && i !== view.drag.paint) view.opts.onPaint?.(i);
      view.drag.paint = i;
      return;
    }
    view.drag.moved += Math.abs(dx) + Math.abs(dy);
    if (view.drag.moved > 4) {
      view.opts.onHoverEnd();
      view.panBy(dx, dy);
    }
  });
  const up = (e) => {
    const was = view.pointers.delete(e.pointerId);
    if (!was || !view.drag) return;
    if (view.pointers.size === 0) {
      if (view.drag.moved <= 4 && e.type === 'pointerup' && view.drag.paint === undefined) {
        const rect = c.getBoundingClientRect();
        const i = view.tileAt(e.clientX - rect.left, e.clientY - rect.top);
        // Left picks, right commands; other buttons do neither.
        if (view.drag.button === 0) view.opts.onPick(i);
        else if (view.drag.button === 2) view.opts.onCommand?.(i);
      }
      view.drag = null;
    } else {
      view.drag.pinch = null;
    }
  };
  c.addEventListener('contextmenu', (e) => e.preventDefault());
  c.addEventListener('pointerup', up);
  c.addEventListener('pointercancel', up);
  c.addEventListener('pointerleave', (e) => {
    if (!view.pointers.has(e.pointerId)) view.opts.onHoverEnd();
  });
  c.addEventListener(
    'wheel',
    (e) => {
      e.preventDefault();
      view.opts.onHoverEnd();
      const rect = c.getBoundingClientRect();
      const unit = e.deltaMode === 1 ? 16 : e.deltaMode === 2 ? 400 : 1;
      view.zoomBy(Math.exp(-e.deltaY * unit * 0.0015), e.clientX - rect.left, e.clientY - rect.top);
    },
    { passive: false },
  );
}

export function pinchState(view) {
  const [a, b] = [...view.pointers.values()];
  return { d: Math.max(1, Math.hypot(a.x - b.x, a.y - b.y)), x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 };
}
