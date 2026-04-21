// Huffman coding: merge the two lowest-frequency nodes (binary-heap min) to build
// an optimal prefix tree, then DFS to assign codes (left=0, right=1).
// Time O(C log C) for C distinct chars, Space O(C).

class MinHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  push(x) {
    const a = this.a; a.push(x); let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this._less(a[i], a[p])) { [a[i], a[p]] = [a[p], a[i]]; i = p; }
      else break;
    }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) { a[0] = last; let i = 0;
      for (;;) {
        let l = 2*i+1, r = 2*i+2, s = i;
        if (l < a.length && this._less(a[l], a[s])) s = l;
        if (r < a.length && this._less(a[r], a[s])) s = r;
        if (s === i) break;
        [a[i], a[s]] = [a[s], a[i]]; i = s;
      }
    }
    return top;
  }
  _less(x, y) { return x.freq !== y.freq ? x.freq < y.freq : x.order < y.order; }
}

function buildCodes(freq) {
  const heap = new MinHeap();
  let cnt = 0;
  for (const [ch, f] of freq) heap.push({ freq: f, ch, order: cnt++, l: null, r: null });
  while (heap.size() > 1) {
    const a = heap.pop(), b = heap.pop();
    heap.push({ freq: a.freq + b.freq, ch: null, order: cnt++, l: a, r: b });
  }
  const root = heap.pop();
  const codes = {};
  (function dfs(n, path) {
    if (!n.l && !n.r) { codes[n.ch] = path || "0"; return; }
    dfs(n.l, path + "0"); dfs(n.r, path + "1");
  })(root, "");
  return codes;
}

const freq = [['c', 1], ['a', 4], ['t', 3], ['s', 2]];
const codes = buildCodes(freq);
for (const ch of Object.keys(codes).sort()) console.log(`${ch}: ${codes[ch]}`);
console.log("encode(cats):", [..."cats"].map(c => codes[c]).join(""));
