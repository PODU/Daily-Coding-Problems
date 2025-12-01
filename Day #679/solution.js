// Count inversions via modified merge sort. Time O(N log N), Space O(N).
"use strict";

function countInversions(arr) {
  function sortCount(a) {
    if (a.length <= 1) return [a, 0];
    const mid = a.length >> 1;
    const [left, il] = sortCount(a.slice(0, mid));
    const [right, ir] = sortCount(a.slice(mid));
    const merged = [];
    let i = 0, j = 0, inv = 0;
    while (i < left.length && j < right.length) {
      if (left[i] <= right[j]) merged.push(left[i++]);
      else { merged.push(right[j++]); inv += left.length - i; }
    }
    while (i < left.length) merged.push(left[i++]);
    while (j < right.length) merged.push(right[j++]);
    return [merged, il + ir + inv];
  }
  return sortCount(arr)[1];
}

console.log(countInversions([2, 4, 1, 3, 5]));
console.log(countInversions([5, 4, 3, 2, 1]));
