// Day 957: skyline problem via sweep line with a max-heap (lazy deletion).
// Time O(n log n), Space O(n).

class MaxHeap {
  constructor() { this.a = [0]; } // baseline 0
  size() { return this.a.length; }
  peek() { return this.a[0]; }
  push(v) {
    const a = this.a; a.push(v);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p] >= a[i]) break;
      [a[p], a[i]] = [a[i], a[p]]; i = p;
    }
  }
  pop() {
    const a = this.a; const top = a[0]; const last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < a.length && a[l] > a[s]) s = l;
        if (r < a.length && a[r] > a[s]) s = r;
        if (s === i) break;
        [a[s], a[i]] = [a[i], a[s]]; i = s;
      }
    }
    return top;
  }
}

function skyline(buildings) {
  const events = [];
  for (const [l, r, h] of buildings) {
    events.push([l, -h]); // start
    events.push([r, h]);  // end
  }
  events.sort((p, q) => (p[0] !== q[0] ? p[0] - q[0] : p[1] - q[1]));
  const heap = new MaxHeap();
  const removed = new Map();
  let prev = 0;
  const res = [];
  for (const [x, h] of events) {
    if (h < 0) heap.push(-h);
    else removed.set(h, (removed.get(h) || 0) + 1);
    while (heap.size() && removed.get(heap.peek()) > 0) {
      removed.set(heap.peek(), removed.get(heap.peek()) - 1);
      heap.pop();
    }
    const cur = heap.peek();
    if (cur !== prev) { res.push([x, cur]); prev = cur; }
  }
  return res;
}

const res = skyline([[0, 15, 3], [4, 11, 5], [19, 23, 4]]);
console.log("[" + res.map(([x, h]) => `(${x}, ${h})`).join(", ") + "]");
// [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
