// Day 754: Optimal coin game (interval DP / minimax).
// dp[i][j] = max value first player guarantees from coins[i..j].
// Time: O(n^2), Space: O(n^2).
"use strict";

function maxCoins(v) {
    const n = v.length;
    if (n === 0) return 0;
    const pre = new Array(n + 1).fill(0);
    for (let i = 0; i < n; i++) pre[i + 1] = pre[i] + v[i];

    const dp = Array.from({ length: n }, () => new Array(n).fill(0));
    for (let i = 0; i < n; i++) dp[i][i] = v[i];
    for (let len = 2; len <= n; len++)
        for (let i = 0; i + len - 1 < n; i++) {
            const j = i + len - 1;
            const takeLeft  = v[i] + (pre[j + 1] - pre[i + 1]) - dp[i + 1][j];
            const takeRight = v[j] + (pre[j] - pre[i]) - dp[i][j - 1];
            dp[i][j] = Math.max(takeLeft, takeRight);
        }
    return dp[0][n - 1];
}

console.log(maxCoins([8, 15, 3, 7]));  // 22
