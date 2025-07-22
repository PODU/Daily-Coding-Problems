// Staircase ways with step set X: dp[i] = sum dp[i-x] for x in X.
// Time: O(N*|X|), Space: O(N).
function staircase(n, X = [1, 2]) {
  const dp = new Array(n + 1).fill(0);
  dp[0] = 1;
  for (let i = 1; i <= n; i++)
    for (const x of X)
      if (i - x >= 0) dp[i] += dp[i - x];
  return dp[n];
}

console.log(staircase(4, [1, 2])); // 5
