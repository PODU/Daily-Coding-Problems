// Three entries summing to k: sort + fix one + two-pointer.
// O(n^2) time, O(1) extra space.
'use strict';

function threeSum(arr, k) {
  const a = arr.slice().sort((x, y) => x - y);
  const n = a.length;
  for (let i = 0; i < n - 2; ++i) {
    let lo = i + 1, hi = n - 1;
    const target = k - a[i];
    while (lo < hi) {
      const s = a[lo] + a[hi];
      if (s === target) return true;
      if (s < target) ++lo; else --hi;
    }
  }
  return false;
}

console.log(threeSum([20, 303, 3, 4, 25], 49)); // true
