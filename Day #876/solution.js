// Sort a k-sorted array with a min-heap of size k+1. Time O(N log k), Space O(k).

class MinHeap {
  constructor() { this.h = []; }
  size() { return this.h.length; }
  push(v) {
    const h = this.h;
    h.push(v);
    let i = h.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (h[p] <= h[i]) break;
      [h[p], h[i]] = [h[i], h[p]];
      i = p;
    }
  }
  pop() {
    const h = this.h;
    const top = h[0];
    const last = h.pop();
    if (h.length) {
      h[0] = last;
      let i = 0;
      const n = h.length;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < n && h[l] < h[s]) s = l;
        if (r < n && h[r] < h[s]) s = r;
        if (s === i) break;
        [h[s], h[i]] = [h[i], h[s]];
        i = s;
      }
    }
    return top;
  }
}

function sortKSorted(a, k) {
  const heap = new MinHeap();
  const res = [];
  for (const x of a) {
    heap.push(x);
    if (heap.size() > k) res.push(heap.pop());
  }
  while (heap.size()) res.push(heap.pop());
  return res;
}

const a = [6, 5, 3, 2, 8, 10, 9];
console.log(sortKSorted(a, 3));
