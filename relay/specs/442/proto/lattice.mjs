// The slot lattice at the starting numbers: slots a tile, a lone tile's big and sparse slots, and how many slots a
// tile with one open edge (E) has in the band.   node relay/specs/442/proto/lattice.mjs
const C=[0,1,2,3,4,5].map(i=>[Math.cos(Math.PI/3*i-Math.PI/6),Math.sin(Math.PI/3*i-Math.PI/6)]);
// edge d from corner d to d+1; normal = midpoint direction
const N=[0,1,2,3,4,5].map(d=>{const mx=(C[d][0]+C[(d+1)%6][0])/2,my=(C[d][1]+C[(d+1)%6][1])/2;const l=Math.hypot(mx,my);return [mx/l,my/l]});
const ap=Math.sqrt(3)/2;
function slots(gap,inset){const o=[];const rh=gap*Math.sqrt(3)/2;for(let r=-5;r<=5;r++)for(let c=-6;c<=6;c++){const x=c*gap+(Math.abs(r)%2?gap/2:0),y=r*rh;const s=N.map(n=>ap-(n[0]*x+n[1]*y));if(Math.min(...s)>=inset)o.push({x,y,s});}return o}
for(const [g,i] of [[0.42,0.1],[0.36,0.1]]){const S=slots(g,i);const big=S.filter(p=>Math.min(...p.s)>=0.32).length;console.log('gap',g,'slots',S.length,'lone: big',big,'band',S.length-big);
 // edge tile with one open edge (d=0): slots whose nearest edge is forest but open edge within band
 let miss=0;for(const p of S){const m=Math.min(...p.s);const near=p.s.indexOf(m);if(p.s[0]<0.32&&near!==0)miss++;}console.log(' one open edge E: within band of E but nearest edge other:',miss, 'within band of E:',S.filter(p=>p.s[0]<0.32).length);}
