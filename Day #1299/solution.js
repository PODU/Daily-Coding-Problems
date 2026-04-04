// Day 1299: Count ordered ways to climb N stairs using step sizes from set X.
// DP: dp[i] = sum(dp[i-x]) for x in X. O(N*|X|) time, O(N) space.
function climbWays(n, steps = [1, 2]) {
  const dp = new Array(n + 1).fill(0);
  dp[0] = 1;
  for (let i = 1; i <= n; i++)
    for (const x of steps) if (i - x >= 0) dp[i] += dp[i - x];
  return dp[n];
}

console.log(climbWays(4, [1, 2]));     // 5
console.log(climbWays(10, [1, 3, 5])); // generalized
