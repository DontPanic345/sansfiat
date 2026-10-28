# Sans Fiat

Prototypes for a hex 4X with no money: goods are physical and worth what people will trade for them.
Each folder stands alone. Live builds are on [GitHub Pages](https://dontpanic345.github.io/sansfiat/).

| Folder | What |
| --- | --- |
| [`engine/`](engine/) | The game: a Rust engine with a CLI and a wasm web front end. New work goes here. |
| [`merge4/`](merge4/) | hex-economy, hex4x, scout and supply-lines merged into one game: pre-game economy, civs, scouts, armies, supply. |
| [`scout/`](scout/) | Hidden trail costs and scouting routes. |
| [`supply-lines/`](supply-lines/) | Units carrying food, resupply lines, forage and pillage. |
| [`paint-army/`](paint-army/) | Single-page territory-painting army experiments. |

The prototypes not listed are archived under `prototype/<name>` tags.

The JS prototypes are plain ES modules: `npm start`, `npm test`, `npm run sim`.
