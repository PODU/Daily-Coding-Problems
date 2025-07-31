// Day 59: File sync over low-bandwidth link, rsync-style.
// Receiver sends per-block (weak rolling + strong) checksums of its OLD file;
// sender scans NEW file with a rolling checksum, emitting COPY(block) tokens for
// matches and LITERAL bytes otherwise -> only differing bytes cross the wire.
// Time: O(n) expected, bandwidth ~ size of the changes.
const M = 1 << 16;
const B = 4;

function fnv1a(d, s, e) {
  let h = 0xcbf29ce484222325n;
  for (let i = s; i < e; i++) {
    h ^= BigInt(d[i]);
    h = (h * 0x100000001b3n) & 0xFFFFFFFFFFFFFFFFn;
  }
  return h;
}

function weakFull(d, s, e) {
  let a = 0, b = 0;
  for (let i = s; i < e; i++) {
    a = (a + d[i]) % M;
    b = (b + (e - i) * d[i]) % M;
  }
  return [a, b];
}

// Receiver side: weak -> Map(strong -> index).
function signatures(old) {
  const sigs = new Map();
  const n = Math.floor(old.length / B);
  for (let i = 0; i < n; i++) {
    const [a, b] = weakFull(old, i * B, i * B + B);
    const weak = (b << 16) | a;
    if (!sigs.has(weak)) sigs.set(weak, new Map());
    sigs.get(weak).set(fnv1a(old, i * B, i * B + B), i);
  }
  return sigs;
}

function diff(nw, sigs) {
  const tokens = [];
  let literal = [];
  let pos = 0, a = 0, b = 0, haveWindow = false;
  const n = nw.length;
  while (pos + B <= n) {
    if (!haveWindow) { [a, b] = weakFull(nw, pos, pos + B); haveWindow = true; }
    const weak = (b << 16) | a;
    let idx;
    if (sigs.has(weak)) idx = sigs.get(weak).get(fnv1a(nw, pos, pos + B));
    if (idx !== undefined) {
      if (literal.length) { tokens.push({ copy: false, lit: literal }); literal = []; }
      tokens.push({ copy: true, idx });
      pos += B; haveWindow = false;
    } else {
      const first = nw[pos];
      literal.push(nw[pos]);
      a = ((a - first + nw[pos + B]) % M + M) % M;
      b = ((b - B * first + a) % M + M) % M;
      pos++;
    }
  }
  for (let i = pos; i < n; i++) literal.push(nw[i]);
  if (literal.length) tokens.push({ copy: false, lit: literal });
  return tokens;
}

function applyDelta(old, tokens) {
  const out = [];
  for (const t of tokens) {
    if (t.copy) for (let i = 0; i < B; i++) out.push(old[t.idx * B + i]);
    else out.push(...t.lit);
  }
  return out;
}

const enc = (s) => Array.from(Buffer.from(s));
const old = enc("the quick brown fox jumps over the lazy dog");
const nw = enc("the quick brown cat jumps over the lazy dog");
const sigs = signatures(old);
const tokens = diff(nw, sigs);
const rebuilt = applyDelta(old, tokens);
const literalBytes = tokens.filter(t => !t.copy).reduce((s, t) => s + t.lit.length, 0);
const copied = tokens.filter(t => t.copy).length;
console.log("synced:", rebuilt.length === nw.length && rebuilt.every((v, i) => v === nw[i]));
console.log("literal bytes sent:", literalBytes, "of", nw.length);
console.log("blocks reused:", copied);
