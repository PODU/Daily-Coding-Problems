// Kadane's max subarray sum; empty subarray (0) allowed so answer is never negative. O(N) time, O(1) space.
'use strict';

function maxSubarraySum(a) {
  let best = 0, cur = 0;
  for (const x of a) {
    cur = Math.max(0, cur + x);
    best = Math.max(best, cur);
  }
  return best;
}

console.log(maxSubarraySum([34, -50, 42, 14, -5, 86])); // 137
console.log(maxSubarraySum([-5, -1, -8, -9]));          // 0
