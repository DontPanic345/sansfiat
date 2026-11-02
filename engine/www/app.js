// What the screens share, filled in by main.js before any screen shows: the wasm data (`data`), the
// atlas and the sound bus; `show(name, args)` is the router, and `screen` the name of the screen up.
// Screens import this rather than main.js, so main.js only routes.

export const app = {
  data: null,
  atlas: null,
  sound: null,
  screen: null,
  show: () => {},
};
