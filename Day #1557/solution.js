// Grid DP: max coins from top-left to bottom-right moving right/down only.
// dp[j] = grid + max(top, left). Time O(m*n), Space O(n).
"use strict";

function main() {
  const grid = [
    [0, 3, 1, 1],
    [2, 0, 0, 4],
    [1, 5, 3, 1],
  ];
  const m = grid.length, n = grid[0].length;
  const dp = new Array(n).fill(0);
  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      let best;
      if (i === 0 && j === 0) best = 0;
      else if (i === 0) best = dp[j - 1];
      else if (j === 0) best = dp[j];
      else best = Math.max(dp[j], dp[j - 1]);
      dp[j] = grid[i][j] + best;
    }
  }
  console.log(dp[n - 1]);
}

main();
