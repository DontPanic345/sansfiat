// A PNG codec in plain Node (node:zlib and node:fs only), for the atlas build. Images are
// `{ width, height, data }` with `data` RGBA8, unpremultiplied.
import { readFileSync, writeFileSync } from 'node:fs';
import { crc32, deflateSync, inflateSync } from 'node:zlib';

const SIGNATURE = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);

const paeth = (a, b, c) => {
  const p = a + b - c, pa = Math.abs(p - a), pb = Math.abs(p - b), pc = Math.abs(p - c);
  return pa <= pb && pa <= pc ? a : pb <= pc ? b : c;
};

/** Decodes a PNG (8-bit RGB or RGBA, not interlaced, any filter) into RGBA8. Anything else throws. */
export function decode(buffer) {
  if (!buffer.subarray(0, 8).equals(SIGNATURE)) throw new Error('not a PNG');
  let header = null;
  const idat = [];
  for (let p = 8; p + 12 <= buffer.length; ) {
    const len = buffer.readUInt32BE(p), type = buffer.toString('ascii', p + 4, p + 8);
    const body = buffer.subarray(p + 8, p + 8 + len);
    if (type === 'IHDR') header = body;
    else if (type === 'IDAT') idat.push(body);
    else if (type === 'IEND') break;
    p += 12 + len;
  }
  if (!header) throw new Error('PNG has no IHDR');
  const width = header.readUInt32BE(0), height = header.readUInt32BE(4);
  const depth = header[8], colour = header[9], interlace = header[12];
  if (depth !== 8 || (colour !== 2 && colour !== 6)) throw new Error(`PNG colour type ${colour}, bit depth ${depth}: only 8-bit RGB (2) and RGBA (6)`);
  if (interlace !== 0) throw new Error('PNG is interlaced: not supported');
  const bpp = colour === 6 ? 4 : 3, stride = width * bpp;
  const raw = inflateSync(Buffer.concat(idat));
  if (raw.length !== (stride + 1) * height) throw new Error(`PNG data is ${raw.length} bytes, expected ${(stride + 1) * height}`);
  // Unfilter each row in place, in a buffer of the image's own rows.
  const rows = Buffer.alloc(stride * height);
  for (let y = 0; y < height; y++) {
    const type = raw[y * (stride + 1)], src = y * (stride + 1) + 1, row = y * stride, prior = row - stride;
    for (let x = 0; x < stride; x++) {
      const a = x >= bpp ? rows[row + x - bpp] : 0;
      const b = y > 0 ? rows[prior + x] : 0;
      const c = x >= bpp && y > 0 ? rows[prior + x - bpp] : 0;
      const v = raw[src + x];
      let r;
      switch (type) {
        case 0: r = v; break;
        case 1: r = v + a; break;
        case 2: r = v + b; break;
        case 3: r = v + ((a + b) >> 1); break;
        case 4: r = v + paeth(a, b, c); break;
        default: throw new Error(`PNG row ${y} has filter ${type}`);
      }
      rows[row + x] = r & 255;
    }
  }
  const data = Buffer.alloc(width * height * 4);
  for (let i = 0; i < width * height; i++) {
    data[i * 4] = rows[i * bpp];
    data[i * 4 + 1] = rows[i * bpp + 1];
    data[i * 4 + 2] = rows[i * bpp + 2];
    data[i * 4 + 3] = bpp === 4 ? rows[i * bpp + 3] : 255;
  }
  return { width, height, data };
}

/** Encodes RGBA8 as a PNG: filter 0 on every row, one IDAT, zlib level 9. */
export function encode({ width, height, data }) {
  const raw = Buffer.alloc((width * 4 + 1) * height);
  for (let y = 0; y < height; y++) data.copy(raw, y * (width * 4 + 1) + 1, y * width * 4, (y + 1) * width * 4);
  const chunk = (type, body) => {
    const head = Buffer.alloc(8), tail = Buffer.alloc(4);
    head.writeUInt32BE(body.length, 0);
    head.write(type, 4, 'ascii');
    const td = Buffer.concat([head.subarray(4), body]);
    tail.writeUInt32BE(crc32(td) >>> 0, 0);
    return Buffer.concat([head, body, tail]);
  };
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8; // depth
  ihdr[9] = 6; // RGBA
  return Buffer.concat([
    SIGNATURE,
    chunk('IHDR', ihdr),
    chunk('IDAT', deflateSync(raw, { level: 9 })),
    chunk('IEND', Buffer.alloc(0)),
  ]);
}

export const readPng = (path) => decode(readFileSync(path));
export const writePng = (path, image) => writeFileSync(path, encode(image));
