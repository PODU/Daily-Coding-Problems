// Coins-in-a-row: dp[i][j] = max value first-to-move guarantees from coins[i..j],
// dp[i][j]=max(v[i]+min(dp[i+2][j],dp[i+1][j-1]), v[j]+min(dp[i+1][j-1],dp[i][j-2])). Time/Space O(n^2).
'use strict';

function maxGuaranteed(v) {
    const n = v.length;
    if (n === 0) return 0;
    const dp = Array.from({ length: n }, () => new Array(n).fill(0));
    for (let len = 1; len <= n; len++) {
        for (let i = 0; i + len - 1 < n; i++) {
            const j = i + len - 1;
            const a = (i + 2 <= j) ? dp[i + 2][j] : 0;
            const b = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;
            const c = (i <= j - 2) ? dp[i][j - 2] : 0;
            const takeFirst = v[i] + Math.min(a, b);
            const takeLast = v[j] + Math.min(b, c);
            dp[i][j] = Math.max(takeFirst, takeLast);
        }
    }
    return dp[0][n - 1];
}

const coins = [3, 9, 1, 2];
console.log(maxGuaranteed(coins));
