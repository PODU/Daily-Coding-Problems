// Day 1452: Sort a k-sorted array (each element <= k from its place) using a
// min-heap of size k+1. Time O(N log k), Space O(k).
class MinHeap {
  constructor() { this.a = []; }
  size() { return this.a.length; }
  push(x) {
    const a = this.a; a.push(x); let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p] <= a[i]) break;
      [a[p], a[i]] = [a[i], a[p]]; i = p;
    }
  }
  pop() {
    const a = this.a, top = a[0], last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0;
      for (;;) {
        const l = 2 * i + 1, r = 2 * i + 2; let m = i;
        if (l < a.length && a[l] < a[m]) m = l;
        if (r < a.length && a[r] < a[m]) m = r;
        if (m === i) break;
        [a[m], a[i]] = [a[i], a[m]]; i = m;
      }
    }
    return top;
  }
}

function sortKSorted(a, k) {
  const heap = new MinHeap(), out = [];
  for (const x of a) {
    heap.push(x);
    if (heap.size() > k) out.push(heap.pop());
  }
  while (heap.size()) out.push(heap.pop());
  return out;
}

const a = [2, 6, 3, 12, 56, 8];
console.log(sortKSorted(a, 3)); // [ 2, 3, 6, 8, 12, 56 ]
