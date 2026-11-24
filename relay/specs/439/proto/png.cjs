const fs=require('fs'),zlib=require('zlib');
function decode(f){const b=fs.readFileSync(f);let p=8,w,h,ct,idat=[];while(p<b.length){const len=b.readUInt32BE(p),type=b.toString('ascii',p+4,p+8),d=b.subarray(p+8,p+8+len);if(type==='IHDR'){w=d.readUInt32BE(0);h=d.readUInt32BE(4);ct=d[9]}else if(type==='IDAT')idat.push(d);p+=12+len}
const raw=zlib.inflateSync(Buffer.concat(idat));const bpp=ct===6?4:3,stride=w*bpp,out=Buffer.alloc(w*h*4);let prev=Buffer.alloc(stride),q=0;
for(let y=0;y<h;y++){const ft=raw[q++];const line=Buffer.from(raw.subarray(q,q+stride));q+=stride;for(let i=0;i<stride;i++){const a=i>=bpp?line[i-bpp]:0,up=prev[i],c=i>=bpp?prev[i-bpp]:0;let v=line[i];if(ft===1)v+=a;else if(ft===2)v+=up;else if(ft===3)v+=(a+up)>>1;else if(ft===4){const pp=a+up-c,pa=Math.abs(pp-a),pb=Math.abs(pp-up),pc=Math.abs(pp-c);v+=(pa<=pb&&pa<=pc)?a:(pb<=pc?up:c)}line[i]=v&255}
for(let x=0;x<w;x++){out[(y*w+x)*4]=line[x*bpp];out[(y*w+x)*4+1]=line[x*bpp+1];out[(y*w+x)*4+2]=line[x*bpp+2];out[(y*w+x)*4+3]=bpp===4?line[x*bpp+3]:255}prev=line}
return {w,h,data:out}}
function crc(buf){let c,crc=0xffffffff;for(let n=0;n<buf.length;n++){c=(crc^buf[n])&255;for(let k=0;k<8;k++)c=c&1?0xedb88320^(c>>>1):c>>>1;crc=(crc>>>8)^c}return (crc^0xffffffff)>>>0}
function encode(f,w,h,data){const raw=Buffer.alloc((w*4+1)*h);for(let y=0;y<h;y++){raw[y*(w*4+1)]=0;data.copy(raw,y*(w*4+1)+1,y*w*4,(y+1)*w*4)}
const chunk=(t,d)=>{const l=Buffer.alloc(4);l.writeUInt32BE(d.length);const td=Buffer.concat([Buffer.from(t),d]);const c=Buffer.alloc(4);c.writeUInt32BE(crc(td));return Buffer.concat([l,td,c])};
const ih=Buffer.alloc(13);ih.writeUInt32BE(w,0);ih.writeUInt32BE(h,4);ih[8]=8;ih[9]=6;
fs.writeFileSync(f,Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk('IHDR',ih),chunk('IDAT',zlib.deflateSync(raw)),chunk('IEND',Buffer.alloc(0))]))}
module.exports={decode,encode};
