// rsync-style delta sync: index fixed blocks of the old file by a rolling
// (Adler-like) weak hash + exact-block strong check; slide a rolling window over
// the new file emitting COPY(block) or literal bytes. Time O(N) avg, Space O(old/B).
const B = 4;

function weak(buf, off, len) {
  let a = 0, b = 0;
  for (let i = 0; i < len; i++) {
    const c = buf[off + i];
    a = (a + c) & 0xffff;
    b = (b + (len - i) * c) & 0xffff;
  }
  return [a, b];
}

function blockStr(buf, off) {
  return Buffer.from(buf.slice(off, off + B)).toString("latin1");
}

function makeDelta(old, nw) {
  const table = new Map(); // h -> [{idx, str}]
  for (let idx = 0; idx + B <= old.length; idx += B) {
    const [a, b] = weak(old, idx, B);
    const h = (b << 16) | a;
    if (!table.has(h)) table.set(h, []);
    table.get(h).push({ idx, str: blockStr(old, idx) });
  }
  const delta = [];
  const n = nw.length;
  let i = 0, a = 0, b = 0;
  if (n >= B) { [a, b] = weak(nw, 0, B); }
  while (i < n) {
    if (i + B <= n) {
      const h = (b << 16) | a;
      let match = -1;
      const cand = table.get(h);
      if (cand) {
        const win = blockStr(nw, i);
        for (const c of cand) if (c.str === win) { match = c.idx; break; }
      }
      if (match >= 0) {
        delta.push({ copy: true, idx: match });
        i += B;
        if (i + B <= n) { [a, b] = weak(nw, i, B); }
        continue;
      }
    }
    delta.push({ copy: false, lit: nw[i] });
    if (i + B < n) {
      const out = nw[i], inb = nw[i + B];
      a = (a - out + inb) & 0xffff;
      b = (b - B * out + a) & 0xffff;
    }
    i++;
  }
  return delta;
}

function rebuild(old, delta) {
  const out = [];
  for (const t of delta) {
    if (t.copy) for (let k = 0; k < B; k++) out.push(old[t.idx + k]);
    else out.push(t.lit);
  }
  return Buffer.from(out).toString("latin1");
}

const old = Buffer.from("the quick brown fox jumps over the lazy dog", "latin1");
const nw = Buffer.from("the quick brown cat jumps over the lazy dog", "latin1");
const delta = makeDelta(old, nw);
let copies = 0, lits = 0;
for (const t of delta) t.copy ? copies++ : lits++;
const rb = rebuild(old, delta);
console.log("Reconstructed:", rb);
console.log("Match:", rb === nw.toString("latin1"));
console.log("copied blocks:", copies, "literal bytes:", lits);
