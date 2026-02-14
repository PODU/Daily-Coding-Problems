// Coin game DP: dp[i][j] = max guaranteed for current player on coins[i..j]. O(n^2) time/space.

function coinGame(v) {
    const n = v.length;
    if (n === 0) return 0;
    const dp = Array.from({length: n}, () => new Array(n).fill(0));
    for (let i = 0; i < n; i++) dp[i][i] = v[i];
    for (let len = 2; len <= n; len++) {
        for (let i = 0; i <= n - len; i++) {
            const j = i + len - 1;
            const takeI = v[i] + Math.min(
                (i+2 <= j ? dp[i+2][j] : 0),
                (i+1 <= j-1 ? dp[i+1][j-1] : 0)
            );
            const takeJ = v[j] + Math.min(
                (i+1 <= j-1 ? dp[i+1][j-1] : 0),
                (i <= j-2 ? dp[i][j-2] : 0)
            );
            dp[i][j] = Math.max(takeI, takeJ);
        }
    }
    return dp[0][n-1];
}

const a = [8, 15, 3, 7];
console.log("Max guaranteed: " + coinGame(a));
const b = [2, 2, 2, 2];
console.log("Max guaranteed: " + coinGame(b));
