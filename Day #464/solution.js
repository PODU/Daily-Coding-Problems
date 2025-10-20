// Day 464: Largest divisible subset.
// Approach: sort, then LIS-style DP where dp[i]=longest chain ending at i;
// j divides i means a[i] % a[j] === 0. Reconstruct via parent pointers.
// Time: O(n^2), Space: O(n).
"use strict";

function largestDivisibleSubset(nums) {
  if (nums.length === 0) return [];
  nums = nums.slice().sort((a, b) => a - b);
  const n = nums.length;
  const dp = new Array(n).fill(1);
  const parent = new Array(n).fill(-1);
  let best = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++) {
      if (nums[i] % nums[j] === 0 && dp[j] + 1 > dp[i]) {
        dp[i] = dp[j] + 1;
        parent[i] = j;
      }
    }
    if (dp[i] > dp[best]) best = i;
  }
  const res = [];
  for (let i = best; i >= 0; i = parent[i]) res.push(nums[i]);
  return res.reverse();
}

console.log(JSON.stringify(largestDivisibleSubset([3, 5, 10, 20, 21])).replace(/,/g, ", "));
console.log(JSON.stringify(largestDivisibleSubset([1, 3, 6, 24])).replace(/,/g, ", "));
