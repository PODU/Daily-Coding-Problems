// Sort a k-sorted array (each element <= k from its sorted position).
// Min-heap of size k+1: pop the min as the next sorted element. O(N log k) time, O(k) space.

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
        let s = i, l = 2 * i + 1, r = 2 * i + 2;
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

function sortKSorted(arr, k) {
  const heap = new MinHeap();
  const result = [];
  let i = 0;
  for (; i <= k && i < arr.length; i++) heap.push(arr[i]);
  for (; i < arr.length; i++) {
    result.push(heap.pop());
    heap.push(arr[i]);
  }
  while (heap.size()) result.push(heap.pop());
  return result;
}

const arr = [2, 1, 4, 3, 6, 5]; // k-sorted with k = 2
console.log(sortKSorted(arr, 2).join(" "));
