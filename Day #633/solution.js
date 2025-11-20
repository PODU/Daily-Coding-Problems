// Day 633: Sort a k-sorted (nearly sorted) array.
// Approach: min-heap of size k+1; pop smallest as we slide the window.
// Time: O(N log k), Space: O(k).
class MinHeap {
  constructor() { this.h = []; }
  size() { return this.h.length; }
  push(v) {
    const h = this.h; h.push(v);
    let i = h.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (h[p] <= h[i]) break;
      [h[p], h[i]] = [h[i], h[p]]; i = p;
    }
  }
  pop() {
    const h = this.h, top = h[0], last = h.pop();
    if (h.length) {
      h[0] = last; let i = 0;
      while (true) {
        let s = i, l = 2 * i + 1, r = 2 * i + 2;
        if (l < h.length && h[l] < h[s]) s = l;
        if (r < h.length && h[r] < h[s]) s = r;
        if (s === i) break;
        [h[s], h[i]] = [h[i], h[s]]; i = s;
      }
    }
    return top;
  }
}

function sortKSorted(arr, k) {
  const pq = new MinHeap();
  const res = [];
  for (const x of arr) {
    pq.push(x);
    if (pq.size() > k) res.push(pq.pop());
  }
  while (pq.size()) res.push(pq.pop());
  return res;
}

const arr = [2, 1, 4, 3, 5]; // k = 1
console.log(sortKSorted(arr, 1));
