# Notes for later pieces

- Marsh (piece 4): the marsh cell's pools are 2–3 px in a 148 px cell, so at play zoom a delta reads as dark grass. Give
  it reeds and larger water before calling deltas "told apart at a glance" (`map.md`, On the page).
- The edge layer doubles the static draws on a large map at play zoom (+6 ms on large8-play). A composite per tile
  (ground + its fringes + its bank, cached per tile and zoom) would bring it back to one draw a tile.
- The sea's fill still shows a faint regular column of specks per hex (`terrain.sea.*`); a less periodic block would
  hide the lattice in open water.
