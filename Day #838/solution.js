// Day 838: Max coins moving only right/down through a grid.
// DP: dp[i][j] = grid[i][j] + max(dp[i-1][j], dp[i][j-1]).  Time O(R*C), Space O(R*C).
'use strict';

function maxCoins(grid) {
    if (!grid.length || !grid[0].length) return 0;
    const rows = grid.length, cols = grid[0].length;
    const dp = Array.from({ length: rows }, () => new Array(cols).fill(0));
    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            let best = 0;
            if (i > 0) best = Math.max(best, dp[i - 1][j]);
            if (j > 0) best = Math.max(best, dp[i][j - 1]);
            dp[i][j] = grid[i][j] + best;
        }
    }
    return dp[rows - 1][cols - 1];
}

const matrix = [
    [0, 3, 1, 1],
    [2, 0, 0, 4],
    [1, 5, 3, 1],
];
console.log(maxCoins(matrix));
