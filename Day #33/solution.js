// Running median with two binary heaps (max-heap lower half, min-heap upper half). O(log n) per insert.
class Heap {
  constructor(cmp) { this.a = []; this.cmp = cmp; }
  size() { return this.a.length; }
  peek() { return this.a[0]; }
  push(v) {
    const a = this.a; a.push(v); let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.cmp(a[i], a[p]) < 0) { [a[i], a[p]] = [a[p], a[i]]; i = p; } else break;
    }
  }
  pop() {
    const a = this.a; const top = a[0]; const last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0; const n = a.length;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < n && this.cmp(a[l], a[s]) < 0) s = l;
        if (r < n && this.cmp(a[r], a[s]) < 0) s = r;
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
    const m = (lo.size() === hi.size()) ? (lo.peek() + hi.peek()) / 2 : lo.peek();
    out.push(m);
  }
  return out;
}

for (const m of runningMedian([2, 1, 5, 7, 2, 0, 5])) console.log(m);
