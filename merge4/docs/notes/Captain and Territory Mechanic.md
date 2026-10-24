# Captain and Territory Mechanic

Prototype: `D:\lab\sandbox\paint-army\index.html` (single-file Civ-style 4X, two sides, a city each, win by taking the enemy city).

## Idea

Hold ground without putting a unit on every tile. A **captain** is the one visible unit on the map, and he carries an **army** of troops. The player **paints** territory and the army spreads across it and holds it.

## Captain and army

- The captain is the icon. He does no fighting himself and is mostly organisation.
- Most of the army travels with him. Troops split off as **detachments** that stand on painted tiles.
- Every troop belongs to a captain. What happens to a detachment when cut off from its captain is an open question.
- The captain and his troops move one tile per turn.
- Captains can stack on their own troops.

## Territory

- Territory is held only while a troop stands on it. When the last troop leaves, the tile reverts to neutral.
- Nothing is owned permanently, so holding ground costs troops.

## Painting a zone

- A captain has a **zone**: a connected chain of painted tiles (diagonals count) that always includes his own tile.
- Painting only **adds** tiles to the zone, and unpainting only **removes** them. The zone is never redrawn from scratch.
- At end of turn the whole army **spreads evenly** over the zone, creeping at their movment speed. Dug-in troops stay where they are.
- If the zone changes or the army loses troops, it re-spreads..
- To advance, unpaint the rear tiles and paint tiles in front. The whole army marches forward.

## Deployed and undeployed

- **Deployed:** the captain has painted tiles, so the army is spread over them. When he steps off a tile, it joins his zone and he leaves a trail.
- **Undeployed:** the zone is empty and the whole army is gathered on and follows the captain. This is the marching state, with nothing to clear each turn.
- Painting a tile deploys him. Unpainting everything undeploys him, clearing the zone or recall is the same idea.

## Combat

- **Contested tiles**, Tiles painted by both, get a light skirmish between only the troops on those tiles, not an all-out battle.
- **Entrenchment:** defenders who stay put get steadily stronger, up to a cap.
- **Retreat:** defenders facing a losing battle have a chance to retreate and survive. This lets a bunched army take single-held tiles.

## Open questions

- Whether a deployed captain's old tile should join the zone automatically when he moves.
- How strong entrenchment and retreat should be.
