# #441 spec-review

Checked against the code on `relay/441` (main `f5f4620` plus the spec):

- **Geometry holds.** `geom.js`'s `CORNER` is pointy-top from upper right clockwise; edge `d` (corner `d` to
  `d + 1`) faces `EDGE_DIR[d]`. Corner 1 of a tile is its E neighbour's corner 3 and its SE neighbour's corner 5;
  corner 2 is SE's corner 4 and SW's corner 0, so each vertex is enumerated once. The bank's turn
  `60 d − 90` takes the source's south-facing run onto edge `d`'s normal.
- **Data is there.** `Sim.layout` gives `river`, `flow` (−1 for none) and `lake`; `terrain.js` already splits
  terrain 0 into `lake` and `sea` as `kindOf` will. `rivers.js` runs a line 0.62 into a water tile, so a gap in
  the bank at a mouth shows it.
- **No sea touches a lake** on any retro map (`proto/shore.json` has no `lake|sea` edge), so water meets water
  without a seam case.
- **Scale.** The pieces draw at `cell.h × H / 148`, as `layers/ridges.js` does; a world edge is about 72 cell
  px, so the 84 px bank run covers it with a little past each end.
- **The flat colours (changed the plan).** Measured on `terrain.png`: hill hexes average `(127, 135, 48)` and
  `(126, 134, 48)`, the plains hex `(122, 133, 50)`, 6 apart in RGB. Main's nine flat colours are at least 28
  apart (hills and mountain closest). With every ground's `flat` the mean of a recoloured grass cell, hills,
  coast, marsh and forest would merge at the flat zoom. The spec now lets a cell take its `flat` from a source
  rectangle (build-1 adds it and starts hills on its mound's ochre), and build-3 holds no pair under 28.
- **The cut.** Two Haiku-high easels by area (the atlas, the map) and a Sonnet-high canvas stand: merged, the
  easels would hold about 17 files and shoot 33 views on two trees in one 90-minute Haiku session.
- `view.js` is 607 lines; this piece adds an import and a `LAYERS` entry only, so no split is asked here.
