// Hex geometry and colours the map's view and layers share.

/** The six hex corners (pointy top) and, for the edge from corner k to k + 1, the neighbour across it. */
export const CORNER = [0, 1, 2, 3, 4, 5].map((i) => [Math.cos((Math.PI / 3) * i - Math.PI / 6), Math.sin((Math.PI / 3) * i - Math.PI / 6)]);
export const EDGE_DIR = [[1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]];

/** A clan's festival route as tiles: home, then the path to the festival (just home with none). */
export const walkOf = (c) => [c.home, ...(c.routes.length ? c.routes[0].path : [])];

/** A colour from a clan's id. */
export const hue = (id) => `hsl(${(id * 137.508) % 360} 70% 55%)`;

