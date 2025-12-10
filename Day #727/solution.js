// Day 727: Running median over a stream.
// Approach: Two binary heaps (max-heap lower half, min-heap upper half), balanced.
// Time: O(log n) per element, Space: O(n).

class Heap {
  constructor(cmp) { this.a = []; this.cmp = cmp; }
  size() { return this.a.length; }
  peek() { return this.a[0]; }
  push(v) {
    const a = this.a; a.push(v); let i = a.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (this.cmp(a[i], a[p]) < 0) { [a[i], a[p]] = [a[p], a[i]]; i = p; } else break; }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) { a[0] = last; let i = 0;
      for (;;) { let l = 2 * i + 1, r = l + 1, s = i;
        if (l < a.length && this.cmp(a[l], a[s]) < 0) s = l;
        if (r < a.length && this.cmp(a[r], a[s]) < 0) s = r;
        if (s === i) break; [a[i], a[s]] = [a[s], a[i]]; i = s; } }
    return top;
  }
}

function printMedian(m) {
  console.log(Number.isInteger(m) ? String(m) : String(m));
}

function runningMedian(stream) {
  const lo = new Heap((a, b) => b - a); // max-heap
  const hi = new Heap((a, b) => a - b); // min-heap
  for (const x of stream) {
    if (!lo.size() || x <= lo.peek()) lo.push(x); else hi.push(x);
    if (lo.size() > hi.size() + 1) hi.push(lo.pop());
    else if (hi.size() > lo.size()) lo.push(hi.pop());
    if (lo.size() === hi.size()) printMedian((lo.peek() + hi.peek()) / 2);
    else printMedian(lo.peek());
  }
}

runningMedian([2, 1, 5, 7, 2, 0, 5]);
