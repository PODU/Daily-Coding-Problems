// Day 306: Sort a k-sorted array with a min-heap of size k+1. O(N log k).
class MinHeap {
  constructor() { this.h = []; }
  size() { return this.h.length; }
  push(v) { const h = this.h; h.push(v); let i = h.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (h[p] <= h[i]) break; [h[p], h[i]] = [h[i], h[p]]; i = p; } }
  pop() { const h = this.h; const top = h[0]; const last = h.pop();
    if (h.length) { h[0] = last; let i = 0; const n = h.length;
      while (true) { let l = 2*i+1, r = 2*i+2, s = i;
        if (l < n && h[l] < h[s]) s = l; if (r < n && h[r] < h[s]) s = r;
        if (s === i) break; [h[s], h[i]] = [h[i], h[s]]; i = s; } }
    return top; }
}
function sortK(arr, k) {
  const heap = new MinHeap();
  const res = [];
  let i = 0;
  for (; i < arr.length && i <= k; i++) heap.push(arr[i]);
  for (; i < arr.length; i++) { res.push(heap.pop()); heap.push(arr[i]); }
  while (heap.size()) res.push(heap.pop());
  return res;
}
console.log(sortK([6, 5, 3, 2, 8, 10, 9], 3).join(" ")); // 2 3 5 6 8 9 10
