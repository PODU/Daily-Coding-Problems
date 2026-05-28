// Day 1579: Maximum circular subarray sum (empty allowed -> 0).
// ans = max(Kadane-with-empty, total - minSubarray). Time: O(n); Space: O(1).
"use strict";

function maxCircular(a) {
  let total = 0, maxEnd = 0, maxSum = 0;
  let minEnd = 0, minSum = Infinity;
  for (const x of a) {
    total += x;
    maxEnd = Math.max(x, maxEnd + x);
    maxSum = Math.max(maxSum, maxEnd);
    minEnd = Math.min(x, minEnd + x);
    minSum = Math.min(minSum, minEnd);
  }
  return Math.max(maxSum, total - minSum);
}

console.log(maxCircular([8, -1, 3, 4])); // 15
console.log(maxCircular([-4, 5, 1, 0])); // 6
