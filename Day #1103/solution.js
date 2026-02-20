// Day 1103: Count right/down paths through a grid avoiding walls (1).
// DP: dp[i][j] = dp[i-1][j] + dp[i][j-1], 0 at walls. Time: O(N*M). Space: O(M).
function countPaths(g) {
  const n = g.length, m = g[0].length;
  const dp = new Array(m).fill(0);
  for (let i = 0; i < n; i++)
    for (let j = 0; j < m; j++) {
      if (g[i][j] === 1) dp[j] = 0;
      else if (i === 0 && j === 0) dp[j] = 1;
      else dp[j] = (j > 0 ? dp[j - 1] : 0) + dp[j];
    }
  return dp[m - 1];
}

const g = [[0, 0, 1], [0, 0, 1], [1, 0, 0]];
console.log(countPaths(g)); // 2
