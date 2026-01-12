// Nearest k points: max-heap of size k by squared distance. Time O(n log k), Space O(k).
class MaxHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  push(v) { this.a.push(v); this._up(this.a.length - 1); }
  pop() {
    const top = this.a[0], last = this.a.pop();
    if (this.a.length) { this.a[0] = last; this._down(0); }
    return top;
  }
  _up(i) {
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.a[p][0] >= this.a[i][0]) break;
      [this.a[p], this.a[i]] = [this.a[i], this.a[p]]; i = p;
    }
  }
  _down(i) {
    const n = this.a.length;
    while (true) {
      let l = 2 * i + 1, r = 2 * i + 2, m = i;
      if (l < n && this.a[l][0] > this.a[m][0]) m = l;
      if (r < n && this.a[r][0] > this.a[m][0]) m = r;
      if (m === i) break;
      [this.a[m], this.a[i]] = [this.a[i], this.a[m]]; i = m;
    }
  }
}

function nearestK(points, central, k) {
  const heap = new MaxHeap();
  for (const [x, y] of points) {
    const d2 = (x - central[0]) ** 2 + (y - central[1]) ** 2;
    heap.push([d2, x, y]);
    if (heap.size() > k) heap.pop();
  }
  const res = heap.a.map(([, x, y]) => [x, y]);
  res.sort((a, b) => (a[0] - b[0]) || (a[1] - b[1]));
  return res;
}

const points = [[0, 0], [5, 4], [3, 1]];
const res = nearestK(points, [1, 2], 2);
console.log("[" + res.map(([x, y]) => `(${x}, ${y})`).join(", ") + "]");
