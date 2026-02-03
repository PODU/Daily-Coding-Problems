// Huffman coding: min-heap repeatedly merges two smallest nodes, then DFS assigns codes (left='0', right='1').
// Tie-break by insertion order for determinism. O(k log k) time, O(k) space.
class MinHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  cmp(x, y) { return x.freq !== y.freq ? x.freq - y.freq : x.order - y.order; }
  push(v) {
    const a = this.a; a.push(v); let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.cmp(a[i], a[p]) < 0) { [a[i], a[p]] = [a[p], a[i]]; i = p; } else break;
    }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0; const n = a.length;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < n && this.cmp(a[l], a[s]) < 0) s = l;
        if (r < n && this.cmp(a[r], a[s]) < 0) s = r;
        if (s !== i) { [a[i], a[s]] = [a[s], a[i]]; i = s; } else break;
      }
    }
    return top;
  }
}

function huffman(freqs) {
  const heap = new MinHeap();
  let order = 0;
  for (const ch of Object.keys(freqs).sort())
    heap.push({ freq: freqs[ch], order: order++, ch, l: null, r: null });
  while (heap.size() > 1) {
    const a = heap.pop(), b = heap.pop();
    heap.push({ freq: a.freq + b.freq, order: order++, ch: null, l: a, r: b });
  }
  const codes = {};
  const root = heap.pop();
  function dfs(n, code) {
    if (!n.l && !n.r) { codes[n.ch] = code || "0"; return; }
    dfs(n.l, code + "0");
    dfs(n.r, code + "1");
  }
  dfs(root, "");
  return codes;
}

const freqs = { a: 5, b: 9, c: 12, d: 13, e: 16, f: 45 };
const codes = huffman(freqs);
for (const ch of Object.keys(codes).sort()) console.log(`${ch}: ${codes[ch]}`);
