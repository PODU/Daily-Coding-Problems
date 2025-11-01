// Day 528: Huffman coding. Build a prefix tree by repeatedly merging the two
// lowest-frequency nodes (min-heap), then read each char's code from its
// root-to-leaf path (left=0, right=1). Time O(n log n), space O(n).
// Note: the README's example tree is illustrative, not a strict Huffman tree;
// this produces a correct, deterministic optimal-prefix Huffman mapping.

// Simple array-backed min-heap keyed by (freq, order) for deterministic ties.
class MinHeap {
  constructor() { this.a = []; }
  _less(i, j) {
    const x = this.a[i], y = this.a[j];
    return x.freq !== y.freq ? x.freq < y.freq : x.order < y.order;
  }
  push(v) {
    const a = this.a; a.push(v);
    let i = a.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (this._less(i, p)) { [a[i], a[p]] = [a[p], a[i]]; i = p; } else break; }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) { a[0] = last; let i = 0; const n = a.length;
      for (;;) { let l = 2 * i + 1, r = l + 1, s = i;
        if (l < n && this._less(l, s)) s = l;
        if (r < n && this._less(r, s)) s = r;
        if (s === i) break; [a[i], a[s]] = [a[s], a[i]]; i = s; } }
    return top;
  }
  get size() { return this.a.length; }
}

function buildHuffman(freq) {
  const heap = new MinHeap();
  let order = 0;
  for (const [ch, f] of Object.entries(freq)) heap.push({ freq: f, order: order++, ch, l: null, r: null });
  while (heap.size > 1) {
    const a = heap.pop(), b = heap.pop();
    heap.push({ freq: a.freq + b.freq, order: order++, ch: null, l: a, r: b });
  }
  const root = heap.pop();
  const codes = {};
  (function walk(n, path) {
    if (!n.l && !n.r) { codes[n.ch] = path || "0"; return; }
    walk(n.l, path + "0");
    walk(n.r, path + "1");
  })(root, "");
  return codes;
}

const freq = { c: 1, a: 1, t: 1, s: 1 };
const codes = buildHuffman(freq);
for (const ch of Object.keys(codes).sort()) console.log(`${ch} -> ${codes[ch]}`);

const word = "cats";
const encoded = [...word].map((c) => codes[c]).join("");
console.log(`${word} -> ${encoded}`);
