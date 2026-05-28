// Day 1580: Largest divisible subset (every pair mutually divisible).
// Sort, then LIS-style DP: dp[i] = longest chain ending at i where a[i] % a[j] == 0.
// Time: O(n^2); Space: O(n).
"use strict";

function largestDivisible(a) {
  a = [...a].sort((x, y) => x - y);
  const n = a.length;
  if (n === 0) return [];
  const dp = new Array(n).fill(1);
  const prev = new Array(n).fill(-1);
  let best = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++) {
      if (a[i] % a[j] === 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; prev[i] = j; }
    }
    if (dp[i] > dp[best]) best = i;
  }
  const res = [];
  for (let i = best; i !== -1; i = prev[i]) res.push(a[i]);
  return res.reverse();
}

console.log(largestDivisible([3, 5, 10, 20, 21])); // [5, 10, 20]
