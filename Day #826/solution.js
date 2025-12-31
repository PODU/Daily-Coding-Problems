// Day 826: rsync-style file sync over a low-bandwidth link.
// Receiver signs old file's fixed-size blocks (weak rolling Adler checksum +
// strong FNV-1a hash); sender rolls a window over the new file to find matching
// blocks, emitting COPY/LITERAL tokens; receiver rebuilds.
// Time O(n) average (rolling hash), Space O(n).
const MOD = 1 << 16;
const L = 4;

function strongHash(bytes, off, len) {
  let h = 1469598103934665603n;
  const M = (1n << 64n) - 1n;
  for (let k = 0; k < len; k++) {
    h ^= BigInt(bytes[off + k]);
    h = (h * 1099511628211n) & M;
  }
  return h;
}

function weakBlock(bytes, off, len) {
  let a = 0, s = 0;
  for (let k = 0; k < len; k++) {
    a += bytes[off + k];
    s += (len - k) * bytes[off + k];
  }
  a %= MOD; s %= MOD;
  return [a, s, a + MOD * s];
}

function main() {
  const oldF = Buffer.from("the quick brown fox jumps");
  const newF = Buffer.from("the quick red fox leaps over");

  // signature
  const blocks = [];
  for (let i = 0; i < oldF.length; i += L) blocks.push(oldF.subarray(i, Math.min(i + L, oldF.length)));
  const table = new Map(); // weak -> [{strong, idx}]
  blocks.forEach((blk, idx) => {
    if (blk.length === L) {
      const w = weakBlock(blk, 0, L)[2];
      if (!table.has(w)) table.set(w, []);
      table.get(w).push({ strong: strongHash(blk, 0, L), idx });
    }
  });

  const eq = (arr, off, blk) => {
    if (off + blk.length > arr.length) return false;
    for (let k = 0; k < blk.length; k++) if (arr[off + k] !== blk[k]) return false;
    return true;
  };

  // diff
  const delta = [];
  let lit = [];
  const n = newF.length;
  let i = 0, a = 0, s = 0, cs = 0, have = false;
  while (i < n) {
    if (i + L <= n) {
      if (!have) { [a, s, cs] = weakBlock(newF, i, L); have = true; }
      let matched = false;
      const bucket = table.get(cs);
      if (bucket) {
        const hh = strongHash(newF, i, L);
        for (const pr of bucket) {
          if (pr.strong === hh && eq(newF, i, blocks[pr.idx])) {
            if (lit.length) { delta.push({ copy: false, lit: Buffer.from(lit) }); lit = []; }
            delta.push({ copy: true, idx: pr.idx });
            i += L; have = false; matched = true; break;
          }
        }
      }
      if (matched) continue;
      lit.push(newF[i]);
      if (i + L < n) {
        const out = newF[i], inB = newF[i + L];
        a = ((a - out + inB) % MOD + MOD) % MOD;
        s = ((s - L * out + a) % MOD + MOD) % MOD;
        cs = a + MOD * s;
      }
      i++;
    } else {
      lit.push(newF[i]); i++;
    }
  }
  if (lit.length) delta.push({ copy: false, lit: Buffer.from(lit) });

  // reconstruct
  const parts = [];
  let copies = 0;
  for (const t of delta) {
    if (t.copy) { parts.push(blocks[t.idx]); copies++; }
    else parts.push(t.lit);
  }
  const rebuilt = Buffer.concat(parts);
  console.log(rebuilt.toString());
  console.log("reconstruction OK: " + (rebuilt.equals(newF) ? "True" : "False"));
  console.log("blocks reused (copy tokens): " + copies);
}

main();
