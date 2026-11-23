const {decode,encode}=require('./png.cjs');const im=decode('../../../../docs/proposals/map-art/terrain.png');
const [CX,CY,RY,RX]=[551.5,854,109-3,(648-455)/2-3]; // inset hex
// working copy of a region
const get=(x,y)=>{const i=(y*im.w+x)*4;return [im.data[i],im.data[i+1],im.data[i+2]]};
const src=(x,y)=>get(x,y);
// patch: dest rect [455,830,70,68] from source offset (+100,-40)? feathered
const P={dx:452,dy:828,w:72,h:70,sx:555,sy:790,f:8};
function px(x,y){let c=src(x,y);const u=x-P.dx,v=y-P.dy;if(u>=0&&v>=0&&u<P.w&&v<P.h){const e=Math.min(u+P.f,v,P.w-u,P.h-v);const t=Math.max(0,Math.min(1,e/P.f));const d=src(P.sx+u,P.sy+v);c=c.map((a,k)=>a*(1-t)+d[k]*t)}return c}
// inside test for pointy hex of radii RX (half width), RY (circumradius)
const inside=(x,y)=>{const dx=Math.abs(x-CX)/RX,dy=Math.abs(y-CY)/RY;return dx<=1&&dy<=1-dx*0.5};
const W=128,H=148,SS=4,o=Buffer.alloc(W*H*4);
for(let y=0;y<H;y++)for(let x=0;x<W;x++){let r=0,g=0,b=0,n=0,cov=0;
 for(let j=0;j<SS;j++)for(let i=0;i<SS;i++){const u=(x+(i+.5)/SS)/W*2-1,v=(y+(j+.5)/SS)/H*2-1; // -1..1
  const sx=CX+u*RX,sy=CY+v*RY;const inh=Math.abs(u)<=1&&Math.abs(v)<=1-Math.abs(u)*0.5;if(!inh)continue;cov++;
  const c=px(Math.floor(sx),Math.floor(sy));r+=c[0];g+=c[1];b+=c[2];n++}
 const k=(y*W+x)*4;if(n){o[k]=r/n;o[k+1]=g/n;o[k+2]=b/n}o[k+3]=Math.round(255*cov/(SS*SS))}
encode('plains.png',W,H,o);
// preview: 3x, a ring with placeholder plains colour neighbours
const S=3,PW=W*S,PH=H*S,pv=Buffer.alloc(PW*PH*4);for(let y=0;y<PH;y++)for(let x=0;x<PW;x++){const k=((y/S|0)*W+(x/S|0))*4,a=o[k+3]/255,d=(y*PW+x)*4;const bg=[0x9c,0xbf,0x5a];for(let c=0;c<3;c++)pv[d+c]=o[k+c]*a+bg[c]*(1-a);pv[d+3]=255}
encode('plains3x.png',PW,PH,pv);
