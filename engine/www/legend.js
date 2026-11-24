// The map's legend: each mark the map draws, with its swatch. Hidden country adds its three rows.

/** The legend's rows as HTML; `country` adds the closed-country rows. */
export function legendHtml(country) {
  const sw = (css) => `<i style="${css}"></i>`;
  const items = [
    [sw('background:#1f4f7a'), 'ocean'],
    [sw('background:#3f8fc4'), 'lake'],
    [sw('background:#d6c487'), 'coast'],
    [sw('background:#9cbf5a'), 'plains'],
    [sw('background:#3f7a3a'), 'forest'],
    [sw('background:#8a7a5c'), 'hills'],
    [sw('background:#7c7a85;color:#c4c2cc;text-align:center;font-size:10px;line-height:12px').replace('></i>', '>▲</i>'), 'mountain'],
    [sw('background:#efe3b8 radial-gradient(#b39a55 1px,transparent 1.5px) 0 0/5px 5px'), 'desert'],
    [sw('background:#5f8a6a repeating-linear-gradient(90deg,#2f5a46 0 1px,transparent 1px 5px)'), 'marsh (delta)'],
    [sw('background:linear-gradient(#9cbf5a,#9cbf5a);border:0;height:4px;margin:5px 0;background:#58b8f2;border-radius:2px'), 'river'],
    [sw('background:rgba(96,235,70,.6);border:1px solid #1c6e18'), 'floodplain'],
    ...(country
      ? [
          [sw('background:transparent;border:2px solid #f0c030'), 'valley'],
          [sw('background:transparent;border:2px dashed #b0583a'), 'sealed valley'],
          [sw('background:transparent;border:2px solid #b070e0'), 'neck'],
        ]
      : []),
    [sw('background:#fff2b0;border-radius:50%;width:10px;height:10px;margin:2px 4px;border-color:#3a2c00'), 'pass'],
    [sw('background:hsla(200,70%,55%,.35);border:1px solid hsl(200,70%,55%)'), 'territory'],
    [sw('background:rgba(26,15,7,.6)'), 'worn land'],
    [sw('background:transparent;border:0;border-top:3px dashed hsl(120,70%,55%);height:0;margin:5px 0'), 'festival route'],
    [sw('background:#f2dca4;border:0;height:3px;margin:5px 0'), 'trail'],
    [sw('background:#ffd400;width:9px;height:9px;margin:2px 4px;border-color:#3a2c00'), 'tamed grain (a yellow square on the tile)'],
    [sw('background:transparent;border:2px solid #fff;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'market'],
    [sw('background:transparent;border:3px solid #ffc800;border-radius:50%;width:14px;height:14px;margin:0 2px'), 'centre'],
    [sw('background:hsl(120,70%,55%);clip-path:polygon(50% 0,100% 100%,0 100%);width:12px;height:12px;margin:1px 3px'), 'clan that gathers (a tent, its cloth in its civ\'s colour)'],
    [sw('background:linear-gradient(#2a1a0a,#2a1a0a) 50% 100%/4px 4px no-repeat,hsl(120,70%,55%);clip-path:polygon(50% 0,100% 100%,0 100%);width:12px;height:12px;margin:1px 3px'), 'settled clan (a hearth: a tent by its fire)'],
    [sw('background:hsl(120,70%,55%);border-radius:3px 3px 0 0;width:8px;height:12px;margin:1px 4px'), 'clan that came to its home this turn (walkers)'],
    [sw('background:hsl(120,70%,55%);border:2px solid #ff2a2a;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'starving clan (a red edge)'],
    [sw('background:#d8d2c4;clip-path:polygon(50% 0,100% 100%,0 100%);width:12px;height:12px;margin:1px 3px'), 'clan of no civ (pale cloth)'],
    [sw('background:#d64545;border:2px solid #fff;outline:2px solid #000;width:10px;height:10px;margin:2px 4px;clip-path:polygon(0 25%,50% 0,100% 25%,100% 100%,0 100%)'), 'capital'],
    [sw('background:#d64545;border:2px solid #222;width:10px;height:10px;margin:2px 4px;clip-path:polygon(0 35%,50% 0,100% 35%,100% 100%,0 100%)'), 'settlement'],
    [sw('background:#c9a66b;border:2px solid #3a2c00;width:12px;height:9px;margin:2px 3px;clip-path:polygon(50% 0,100% 45%,100% 100%,0 100%,0 45%)'), 'town: a festival several clans live on'],
    [sw('background:#d64545;border:2px solid #222;width:6px;height:12px;margin:0 6px'), 'signpost'],
    [sw('background:transparent;border:3px solid #3b6fd6;box-shadow:0 0 0 1px #000;width:12px;height:12px;margin:1px 3px'), 'civ land (its claim)'],
    [sw('background:#3b6fd6;border:2px solid rgba(0,0,0,.75);border-radius:50%;width:12px;height:12px;margin:1px 3px;opacity:.85'), 'unaligned clan: a disc under it in the civ it leans to'],
    [sw('background:#3b6fd6;border:2px solid #000;width:10px;height:12px;margin:0 4px;clip-path:polygon(0 0,100% 0,100% 60%,50% 100%,0 60%)'), 'warrior'],
    [sw('background:#3b6fd6;border:2px solid #ff2a2a;width:10px;height:12px;margin:0 4px;clip-path:polygon(0 0,100% 0,100% 60%,50% 100%,0 60%)'), 'hungry warrior'],
    [sw('background:transparent;border:0;border-top:3px dashed #fff;height:0;margin:5px 0'), 'route (selected unit)'],
    [sw('background:hsl(120,85%,55%);height:3px;margin:5px 0;border:0'), 'camp line (green: cheap to feed, red: cut off)'],
    [sw('background:#f4f7fa;height:4px;margin:5px 0;border:0;box-shadow:0 0 0 1px #000'), 'a picked good’s chain from its source to its store (width by share)'],
  ];
  return items.map(([s, name]) => `<span class="item">${s}${name}</span>`).join('');
}
