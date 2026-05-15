// Huffman coding: build tree via min-heap, merge two smallest nodes, assign 0/1 edges.
// Time: O(n log n) for n symbols. Space: O(n).
'use strict';

class MinHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  push(x) {
    const a = this.a; a.push(x); let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.less(a[i], a[p])) { [a[i], a[p]] = [a[p], a[i]]; i = p; } else break;
    }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) { a[0] = last; let i = 0;
      for (;;) {
        let l = 2*i+1, r = 2*i+2, s = i;
        if (l < a.length && this.less(a[l], a[s])) s = l;
        if (r < a.length && this.less(a[r], a[s])) s = r;
        if (s === i) break; [a[i], a[s]] = [a[s], a[i]]; i = s;
      }
    }
    return top;
  }
  less(x, y) { return x.freq !== y.freq ? x.freq < y.freq : x.order < y.order; }
}

function build(node, code, out) {
  if (!node) return;
  if (!node.l && !node.r) { out[node.ch] = code || "0"; return; }
  build(node.l, code + "0", out);
  build(node.r, code + "1", out);
}

function main() {
  const freqs = { a: 5, b: 9, c: 12, d: 13, e: 16, f: 45 };
  const heap = new MinHeap();
  let counter = 0;
  for (const ch of Object.keys(freqs)) heap.push({ ch, freq: freqs[ch], order: counter++, l: null, r: null });
  while (heap.size() > 1) {
    const a = heap.pop(), b = heap.pop();
    heap.push({ ch: null, freq: a.freq + b.freq, order: counter++, l: a, r: b });
  }
  const root = heap.pop();
  const codes = {};
  build(root, "", codes);
  let totalBits = 0;
  for (const ch of Object.keys(codes).sort()) {
    console.log(`${ch}: ${codes[ch]}`);
    totalBits += codes[ch].length * freqs[ch];
  }
  console.log(`Total encoded bits: ${totalBits}`);
}

main();
