// Day 858: Running median of a stream.
// Approach: two binary heaps (max-heap lower half, min-heap upper half), balanced.
// Time: O(n log n) total, Space: O(n).
'use strict';

class Heap {
  constructor(cmp) { this.a = []; this.cmp = cmp; }
  size() { return this.a.length; }
  peek() { return this.a[0]; }
  push(v) {
    const a = this.a; a.push(v);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.cmp(a[i], a[p]) < 0) { [a[i], a[p]] = [a[p], a[i]]; i = p; }
      else break;
    }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) {
      a[0] = last;
      let i = 0;
      for (;;) {
        let l = 2 * i + 1, r = l + 1, s = i;
        if (l < a.length && this.cmp(a[l], a[s]) < 0) s = l;
        if (r < a.length && this.cmp(a[r], a[s]) < 0) s = r;
        if (s === i) break;
        [a[i], a[s]] = [a[s], a[i]]; i = s;
      }
    }
    return top;
  }
}

function runningMedian(stream) {
  const lo = new Heap((x, y) => y - x); // max-heap
  const hi = new Heap((x, y) => x - y); // min-heap
  const out = [];
  for (const x of stream) {
    if (lo.size() === 0 || x <= lo.peek()) lo.push(x); else hi.push(x);
    if (lo.size() > hi.size() + 1) hi.push(lo.pop());
    else if (hi.size() > lo.size()) lo.push(hi.pop());
    out.push(lo.size() > hi.size() ? lo.peek() : (lo.peek() + hi.peek()) / 2);
  }
  return out;
}

for (const m of runningMedian([2, 1, 5, 7, 2, 0, 5])) {
  console.log(Number.isInteger(m) ? String(m) : String(m));
}
