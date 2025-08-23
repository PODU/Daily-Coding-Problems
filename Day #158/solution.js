// Day 158: Count paths (right/down only) avoiding walls. DP: dp[j] = ways into a
// free cell from top+left; walls contribute 0. Time O(N*M), Space O(M).
'use strict';

function countPaths(grid) {
  const n = grid.length, m = grid[0].length;
  const dp = new Array(m).fill(0);
  dp[0] = grid[0][0] === 0 ? 1 : 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
      if (grid[i][j] === 1) dp[j] = 0;
      else if (j > 0) dp[j] += dp[j - 1];
    }
  }
  return dp[m - 1];
}

const grid = [
  [0, 0, 1],
  [0, 0, 1],
  [1, 0, 0],
];
console.log(countPaths(grid)); // 2
