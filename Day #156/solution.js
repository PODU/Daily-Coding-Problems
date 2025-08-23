// Day 156: Min perfect squares summing to n via DP. dp[i] = min over squares
// j*j<=i of dp[i-j*j]+1. Time O(n*sqrt(n)), Space O(n).
'use strict';

function numSquares(n) {
  const dp = new Array(n + 1).fill(Infinity);
  dp[0] = 0;
  for (let i = 1; i <= n; i++)
    for (let j = 1; j * j <= i; j++)
      dp[i] = Math.min(dp[i], dp[i - j * j] + 1);
  return dp[n];
}

console.log(numSquares(13)); // 2
console.log(numSquares(27)); // 3
