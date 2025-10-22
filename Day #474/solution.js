// Min coins via DP over amounts (optimal for arbitrary denominations).
// Greedy also works for canonical US coins {1,5,10,25}. Time: O(n*k), Space: O(n).
"use strict";

function minCoins(n, coins) {
    const INF = Infinity;
    const dp = new Array(n + 1).fill(INF);
    dp[0] = 0;
    for (let a = 1; a <= n; a++) {
        for (const c of coins) {
            if (c <= a && dp[a - c] !== INF) {
                dp[a] = Math.min(dp[a], dp[a - c] + 1);
            }
        }
    }
    return dp[n];
}

function main() {
    const coins = [1, 5, 10, 25];
    const n = 16;
    console.log(minCoins(n, coins));
}

main();
