// Count right/down paths in a grid with walls. DP: dp[i][j] = paths to cell (0 if wall).
// Time: O(N*M); Space: O(N*M).
'use strict';

function countPaths(grid) {
  const n = grid.length, m = grid[0].length;
  if (grid[0][0] === 1 || grid[n - 1][m - 1] === 1) return 0;
  const dp = Array.from({ length: n }, () => new Array(m).fill(0));
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
      if (grid[i][j] === 1) { dp[i][j] = 0; continue; }
      if (i === 0 && j === 0) { dp[i][j] = 1; continue; }
      const up = i > 0 ? dp[i - 1][j] : 0;
      const left = j > 0 ? dp[i][j - 1] : 0;
      dp[i][j] = up + left;
    }
  }
  return dp[n - 1][m - 1];
}

function main() {
  const grid = [[0, 0, 1], [0, 0, 1], [1, 0, 0]];
  console.log(countPaths(grid));
}

main();
