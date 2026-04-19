// Max coins on a grid moving right/down via DP: dp[i][j]=m[i][j]+max(up,left). O(R*C) time/space.
'use strict';

function maxCoins(m) {
    const R = m.length, C = m[0].length;
    const dp = Array.from({ length: R }, () => new Array(C).fill(0));
    for (let i = 0; i < R; i++)
        for (let j = 0; j < C; j++) {
            let best = 0;
            if (i > 0) best = Math.max(best, dp[i-1][j]);
            if (j > 0) best = Math.max(best, dp[i][j-1]);
            dp[i][j] = m[i][j] + ((i === 0 && j === 0) ? 0 : best);
        }
    return dp[R-1][C-1];
}

function main() {
    const m = [
        [0, 3, 1, 1],
        [2, 0, 0, 4],
        [1, 5, 3, 1],
    ];
    console.log(maxCoins(m)); // 12
}

main();
