// Approach: sort segments by p, then count inversions in the q-order via merge sort.
// Two segments cross iff their p-order and q-order disagree => an inversion. Time O(n log n), Space O(n).
'use strict';

function mergeCount(a) {
  if (a.length <= 1) return [a, 0];
  const m = a.length >> 1;
  const [left, il] = mergeCount(a.slice(0, m));
  const [right, ir] = mergeCount(a.slice(m));
  const merged = [];
  let inv = il + ir, i = 0, j = 0;
  while (i < left.length && j < right.length) {
    if (left[i] <= right[j]) merged.push(left[i++]);
    else { merged.push(right[j++]); inv += left.length - i; }
  }
  while (i < left.length) merged.push(left[i++]);
  while (j < right.length) merged.push(right[j++]);
  return [merged, inv];
}

function countIntersections(p, q) {
  const order = [...p.keys()].sort((x, y) => p[x] - p[y]);
  const qs = order.map(k => q[k]);
  return mergeCount(qs)[1];
}

const p = [1, 2, 3, 4];
const q = [4, 3, 2, 1];
console.log(countIntersections(p, q));
