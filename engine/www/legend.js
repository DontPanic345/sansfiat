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
    [sw('background:#ffd400;width:9px;height:9px;margin:2px 4px;border-color:#3a2c00'), 'hearth'],
    [sw('background:transparent;border:2px solid #fff;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'market'],
    [sw('background:transparent;border:3px solid #ffc800;border-radius:50%;width:14px;height:14px;margin:0 2px'), 'centre'],
    [sw('background:hsl(120,70%,55%);border:2px solid #ff2a2a;border-radius:50%;width:12px;height:12px;margin:1px 3px'), 'clan (red ring: starving)'],
    [sw('background:hsl(120,70%,55%);border:3px solid #d6a63b;border-radius:50%;width:10px;height:10px;margin:2px 3px;box-shadow:0 0 0 1px #000'), 'civ clan (ring in the civ colour)'],
    [sw('background:#d64545;border:2px solid #fff;outline:2px solid #000;width:10px;height:10px;margin:2px 4px;clip-path:polygon(0 25%,50% 0,100% 25%,100% 100%,0 100%)'), 'capital'],
    [sw('background:transparent;border:3px solid #3b6fd6;box-shadow:0 0 0 1px #000;width:12px;height:12px;margin:1px 3px'), 'civ land (its claim)'],
    [sw('background:linear-gradient(90deg,hsl(120,70%,55%),#3b6fd6);border:2px solid #000;width:12px;height:12px;margin:1px 3px'), 'unaligned clan tinted by the civ it leans to'],
    [sw('background:#3b6fd6;border:2px solid #000;width:10px;height:12px;margin:0 4px;clip-path:polygon(0 0,100% 0,100% 60%,50% 100%,0 60%)'), 'warrior'],
    [sw('background:#3b6fd6;border:2px solid #ff2a2a;width:10px;height:12px;margin:0 4px;clip-path:polygon(0 0,100% 0,100% 60%,50% 100%,0 60%)'), 'hungry warrior'],
    [sw('background:transparent;border:0;border-top:3px dashed #fff;height:0;margin:5px 0'), 'route (selected unit)'],
    [sw('background:hsl(120,85%,55%);height:3px;margin:5px 0;border:0'), 'camp line (green: cheap to feed, red: cut off)'],
  ];
  return items.map(([s, name]) => `<span class="item">${s}${name}</span>`).join('');
}
