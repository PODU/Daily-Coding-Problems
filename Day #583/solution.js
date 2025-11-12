// Grid DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]). Time O(R*C), Space O(R*C).
const assert = require("assert");

function maxCoins(grid) {
  const R = grid.length, C = grid[0].length;
  const dp = Array.from({ length: R }, () => new Array(C).fill(0));
  for (let i = 0; i < R; i++) {
    for (let j = 0; j < C; j++) {
      let best = 0;
      if (i > 0) best = Math.max(best, dp[i - 1][j]);
      if (j > 0) best = Math.max(best, dp[i][j - 1]);
      dp[i][j] = grid[i][j] + (i === 0 && j === 0 ? 0 : best);
    }
  }
  return dp[R - 1][C - 1];
}

const grid = [[0, 3, 1, 1], [2, 0, 0, 4], [1, 5, 3, 1]];
const result = maxCoins(grid);
assert.strictEqual(result, 12);
console.log("The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = " + result + " coins.");
