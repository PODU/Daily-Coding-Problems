// K nearest points: max-heap of size k keyed by squared distance. Time O(n log k), Space O(k).

class MaxHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  push(v) {
    const a = this.a; a.push(v); let i = a.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (a[p][0] >= a[i][0]) break; [a[p], a[i]] = [a[i], a[p]]; i = p; }
  }
  pop() {
    const a = this.a; const top = a[0]; const last = a.pop();
    if (a.length) { a[0] = last; let i = 0; const n = a.length;
      while (true) { let l = 2 * i + 1, r = 2 * i + 2, m = i;
        if (l < n && a[l][0] > a[m][0]) m = l;
        if (r < n && a[r][0] > a[m][0]) m = r;
        if (m === i) break; [a[m], a[i]] = [a[i], a[m]]; i = m; } }
    return top;
  }
}

function kNearest(pts, c, k) {
  const heap = new MaxHeap();
  for (let i = 0; i < pts.length; i++) {
    const dx = pts[i][0] - c[0], dy = pts[i][1] - c[1];
    heap.push([dx * dx + dy * dy, i]);
    if (heap.size() > k) heap.pop();
  }
  const idx = [];
  while (heap.size()) idx.push(heap.pop()[1]);
  idx.sort((a, b) => a - b); // keep original order for stable output
  return idx.map((i) => pts[i]);
}

const pts = [[0, 0], [5, 4], [3, 1]];
const c = [1, 2];
const k = 2;
const res = kNearest(pts, c, k);
console.log("[" + res.map(([x, y]) => `(${x}, ${y})`).join(", ") + "]");
