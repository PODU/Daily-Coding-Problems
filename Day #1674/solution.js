// Day 1674: Low-bandwidth file sync (rsync algorithm).
// Receiver sends per-block weak(rolling)+strong checksums; sender emits COPY/LITERAL
// tokens using a rolling hash to find matches. Time O(n) amortized, transfers only diffs.
const M = 1 << 16;

function weakInit(d, i, L) {
  let a = 0, b = 0;
  for (let k = i; k < i + L; k++) {
    a = (a + d[k]) % M;
    b = (b + (L - (k - i)) * d[k]) % M;
  }
  return [a, b];
}
function weakRoll(a, b, d, i, L) {
  a = ((a - d[i] + d[i + L]) % M + M) % M;
  b = ((b - L * d[i] + a) % M + M) % M;
  return [a, b];
}
function strong(block) {
  let h = 1469598103934665603n;
  const mask = (1n << 64n) - 1n;
  for (const byte of block) h = ((h ^ BigInt(byte)) * 1099511628211n) & mask;
  return h.toString();
}
function diff(old, neu, L) {
  const table = new Map();
  const nblocks = Math.floor(old.length / L);
  for (let idx = 0; idx < nblocks; idx++) {
    const [a, b] = weakInit(old, idx * L, L);
    const w = (b << 16) | a;
    if (!table.has(w)) table.set(w, new Map());
    table.get(w).set(strong(old.slice(idx * L, idx * L + L)), idx);
  }
  const tokens = [];
  let lit = [], i = 0, a = 0, b = 0, have = false;
  const n = neu.length;
  while (i < n) {
    if (i + L <= n) {
      if (!have) { [a, b] = weakInit(neu, i, L); have = true; }
      const w = (b << 16) | a;
      const s = strong(neu.slice(i, i + L));
      if (table.has(w) && table.get(w).has(s)) {
        if (lit.length) { tokens.push(["LIT", lit]); lit = []; }
        tokens.push(["COPY", table.get(w).get(s)]);
        i += L; have = false; continue;
      }
      lit.push(neu[i]);
      if (i + L <= n - 1) [a, b] = weakRoll(a, b, neu, i, L);
      else have = false;
      i++;
    } else { lit.push(neu[i]); i++; }
  }
  if (lit.length) tokens.push(["LIT", lit]);
  return tokens;
}
function patch(old, tokens, L) {
  const out = [];
  for (const [kind, val] of tokens) {
    if (kind === "COPY") for (let k = val * L; k < val * L + L; k++) out.push(old[k]);
    else for (const x of val) out.push(x);
  }
  return out;
}

const L = 5;
const enc = (s) => Array.from(Buffer.from(s));
const old = enc("the quick brown fox jumps over");
const neu = enc("the quick brown cat jumps over");
const tokens = diff(old, neu, L);
const rebuilt = patch(old, tokens, L);
const litBytes = tokens.filter(t => t[0] === "LIT").reduce((s, t) => s + t[1].length, 0);
console.log(JSON.stringify(rebuilt) === JSON.stringify(neu)); // true
console.log(litBytes, "of", neu.length);
