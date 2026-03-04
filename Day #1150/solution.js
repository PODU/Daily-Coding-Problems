// Day 1150: Skyline - sweep line over building edges with a max-heap (lazy deletion).
// Track current max height; emit point when it changes. Time O(n log n), Space O(n).
class MaxHeap {
  constructor() { this.h = []; }
  push(v) {
    this.h.push(v);
    let i = this.h.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.h[p] >= this.h[i]) break;
      [this.h[p], this.h[i]] = [this.h[i], this.h[p]];
      i = p;
    }
  }
  pop() {
    const top = this.h[0], last = this.h.pop();
    if (this.h.length) {
      this.h[0] = last;
      let i = 0, n = this.h.length;
      while (true) {
        let b = i, l = 2 * i + 1, r = 2 * i + 2;
        if (l < n && this.h[l] > this.h[b]) b = l;
        if (r < n && this.h[r] > this.h[b]) b = r;
        if (b === i) break;
        [this.h[b], this.h[i]] = [this.h[i], this.h[b]];
        i = b;
      }
    }
    return top;
  }
  peek() { return this.h[0]; }
}

function skyline(buildings) {
  const events = [];
  for (const [l, r, h] of buildings) {
    events.push([l, -h]);
    events.push([r, h]);
  }
  events.sort((a, b) => (a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]));
  const heap = new MaxHeap();
  heap.push(0);
  const removed = new Map();
  let prev = 0;
  const res = [];
  for (const [x, h] of events) {
    if (h < 0) heap.push(-h);
    else removed.set(h, (removed.get(h) || 0) + 1);
    while (heap.h.length && removed.get(heap.peek()) > 0) {
      removed.set(heap.peek(), removed.get(heap.peek()) - 1);
      heap.pop();
    }
    const cur = heap.peek();
    if (cur !== prev) { res.push([x, cur]); prev = cur; }
  }
  return res;
}

const bld = [[0, 15, 3], [4, 11, 5], [19, 23, 4]];
const sk = skyline(bld);
console.log("[" + sk.map(([x, h]) => `(${x}, ${h})`).join(", ") + "]");
// [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
