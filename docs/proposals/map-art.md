# Illustrated map rollout

Use the approved map screenshot in `docs/proposals/map-art/ui.png` as the visual target. The terrain and actor PNGs alongside it are source studies. Inspect their alpha, crop by silhouette, clean and downsample at game scale. Keep source references outside the site's runtime payload. Build missing pixels, edge masks and variants deterministically in the atlas tool or as checked-in art. The GitHub runner can work from these inputs without access to the desktop image tool.

## Relay pieces

1. **Build the map-art atlas and ship the first plains tile.** Produce a deterministic atlas build from named, explicitly cropped source regions, alpha cleanup, stable anchors and existing atlas keys. Ship one playable grassland cell. Verify repeat builds and capture the same seed, camera and zoom on main and branch. 

2. **Connect mountains into ranges across hexes.** Ship isolated peaks, end caps, straight, bending and branching ridges. Select joins from the six neighbors, anchor all pieces to the ground, and draw overhangs in depth order. Prove bends, a fork and map-edge culling on real maps at two zoom levels. Depends on 1.

3. **Connect coastlines, lakes and the remaining ground types.** Cover sea, lake, coast, plains, hills, mountain ground, forest ground, desert and marsh. Compose edge and corner transitions with stable variants; verify narrow isthmuses, bays, inland lakes, river mouths and the existing flat zoom path. Depends on 1.

4. **Blend woodland and marsh into adjacent terrain.** Build woodland interiors, sparse edges and reeds from the source palette. Preserve river and field visibility. Show a dense forest, a lone wooded hex and a marsh across a zoom boundary. Depends on 2, 3.

5. **Give map units distinct silhouettes and faction colours.** Inventory the actual unit types on current main. Produce and wire sprites and faction masks for the supported actors, preserving selection, movement and stacking signals. Compare friendly and foreign units on grass and forest at play zoom. Depends on 1.

6. **Draw clans, camps and hearths from live state.** Map existing clan and camp states to readable silhouettes, using the actor sheet and current layers. Verify moving and settled examples, co-location and faction readability against current gameplay. Illustrate only distinctions the live model exposes. Depends on 5.

7. **Draw towns, cities and capitals on the living map.** Use existing settlement roles to select thatch, palisade and civic silhouettes. Preserve ownership, town labels, frontier and capital markers. Show a crowded settlement beside a unit and forest ridge. Author any missing variants in the same palette. Depends on 5, 6.

8. **Verify the complete illustrated map alongside gameplay.** Integrate against current main and inspect representative retro maps across close, normal and strategic zoom. Compare frame timing on identical views, selections, overlay legibility and map-edge popping. Fix regressions and publish before/after captures plus measured results. Depends on 4, 7.

## Delivery

Use one normal Relay piece and PR per milestone. Fit each change to the map layers on current main. Keep simulation and save semantics stable and compare real maps with identical seeds and viewport settings. The art track uses the shared crew, pacing, CI, and review verdict. Independent gameplay PRs can continue to land. Source studies can evolve through ordinary reviewable commits. The UI image sets art direction; HUD redesign is a separate scope.

