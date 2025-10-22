// rsync-style sync: split destination into fixed blocks indexed by a weak rolling Adler-32 checksum
// + strong FNV hash; slide a rolling window over the source to emit COPY/LITERAL tokens (only diffs
// sent), then reconstruct new = old blocks + literals. Time O(n) (constant block size).
const M = 65521, BLOCK = 4;

function bytes(s) { return Array.from(Buffer.from(s)); }

function fnv(d, from, len) {
  let h = 0xcbf29ce484222325n;
  for (let k = from; k < from + len; k++) {
    h ^= BigInt(d[k]);
    h = (h * 0x100000001b3n) & 0xFFFFFFFFFFFFFFFFn;
  }
  return h;
}

function run() {
  const oldS = "the quick brown fox jumps over the lazy dog";
  const newS = "the quick BROWN fox jumps over the lazy cat";
  const oldB = bytes(oldS), newB = bytes(newS);

  const idx = new Map(); // weak -> [[blockIndex, strong], ...]
  const nb = Math.floor(oldB.length / BLOCK);
  for (let bi = 0; bi < nb; bi++) {
    const s = bi * BLOCK;
    let a = 0, b = 0;
    for (let k = s; k < s + BLOCK; k++) { a = (a + oldB[k]) % M; b = (b + a) % M; }
    const weak = a + b * 65536;
    if (!idx.has(weak)) idx.set(weak, []);
    idx.get(weak).push([bi, fnv(oldB, s, BLOCK)]);
  }

  const tokens = [];
  let lit = [], n = newB.length, pos = 0, a = 0, b = 0, have = false;
  while (pos < n) {
    if (pos + BLOCK <= n) {
      if (!have) {
        a = 0; b = 0;
        for (let k = pos; k < pos + BLOCK; k++) { a = (a + newB[k]) % M; b = (b + a) % M; }
        have = true;
      }
      const weak = a + b * 65536;
      let matched = -1;
      if (idx.has(weak)) {
        const strong = fnv(newB, pos, BLOCK);
        for (const c of idx.get(weak)) if (c[1] === strong) { matched = c[0]; break; }
      }
      if (matched >= 0) {
        if (lit.length > 0) { tokens.push({ lit }); lit = []; }
        tokens.push({ copy: matched });
        pos += BLOCK; have = false; continue;
      }
      lit.push(newB[pos]);
      if (pos + BLOCK < n) {                                    // roll forward one byte
        const out = newB[pos], inb = newB[pos + BLOCK];
        a = ((a - out + inb) % M + M) % M;
        b = ((b - BLOCK * out + a) % M + M) % M;
      } else have = false;
      pos++;
    } else {
      lit.push(newB[pos]); pos++;
    }
  }
  if (lit.length > 0) tokens.push({ lit });

  let outB = [], mb = 0, lb = 0;
  for (const t of tokens) {
    if (t.copy !== undefined) { const s = t.copy * BLOCK; for (let k = s; k < s + BLOCK; k++) outB.push(oldB[k]); mb++; }
    else { for (const x of t.lit) outB.push(x); lb += t.lit.length; }
  }
  const recon = Buffer.from(outB).toString();
  console.log("Matched blocks: " + mb + " (" + (mb * BLOCK) + " bytes), Literal bytes: " + lb);
  console.log("Reconstructed: " + recon);
  console.log("Equals new file: " + (recon === newS));
}

run();
