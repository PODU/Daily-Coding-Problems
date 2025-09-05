// Day 220: Optimal coin-picking game (first player guaranteed max).
// Approach: interval DP. dp[i][j] = max you can collect from coins i..j vs optimal opponent.
// Time O(n^2), Space O(n^2).
function maxCoins(v) {
  const n = v.length;
  if (n === 0) return 0;
  const dp = Array.from({ length: n }, () => Array(n).fill(0));
  for (let i = 0; i < n; i++) dp[i][i] = v[i];
  for (let len = 2; len <= n; len++) {
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      const innerLeft = i + 2 <= j ? dp[i + 2][j] : 0;
      const innerMid = i + 1 <= j - 1 ? dp[i + 1][j - 1] : 0;
      const innerRight = i <= j - 2 ? dp[i][j - 2] : 0;
      const takeI = v[i] + Math.min(innerLeft, innerMid);
      const takeJ = v[j] + Math.min(innerMid, innerRight);
      dp[i][j] = Math.max(takeI, takeJ);
    }
  }
  return dp[0][n - 1];
}

console.log(maxCoins([8, 15, 3, 7])); // 22
