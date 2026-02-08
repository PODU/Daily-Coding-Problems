// Shortest palindrome by inserting chars; lexicographically earliest on ties.
// DP dp[i][j]=min insertions; memoized build(i,j) reconstructs string. Time O(n^2), Space O(n^2).
'use strict';

function solve(s) {
    const n = s.length;
    if (n === 0) return "";
    const dp = Array.from({ length: n }, () => new Array(n).fill(0));
    for (let len = 2; len <= n; len++) {
        for (let i = 0; i + len - 1 < n; i++) {
            const j = i + len - 1;
            if (s[i] === s[j]) dp[i][j] = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : 0;
            else dp[i][j] = 1 + Math.min(dp[i + 1][j], dp[i][j - 1]);
        }
    }
    const memo = new Map();
    function build(i, j) {
        if (i > j) return "";
        if (i === j) return s[i];
        const key = i * n + j;
        if (memo.has(key)) return memo.get(key);
        let res;
        if (s[i] === s[j]) {
            res = s[i] + build(i + 1, j - 1) + s[i];
        } else {
            const a = s[i] + build(i + 1, j) + s[i];
            const b = s[j] + build(i, j - 1) + s[j];
            if (dp[i + 1][j] < dp[i][j - 1]) res = a;
            else if (dp[i][j - 1] < dp[i + 1][j]) res = b;
            else res = a <= b ? a : b;
        }
        memo.set(key, res);
        return res;
    }
    return build(0, n - 1);
}

console.log("race ->", solve("race"));
console.log("google ->", solve("google"));
