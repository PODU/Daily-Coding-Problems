// Day 598: Expected number of rounds flipping n coins (remove tails) until one remains.
// DP: f(n) = (1 + 2^-n * sum_{k<n} C(n,k) f(k)) / (1 - 2^-n). Time O(n^2), Space O(n^2).
'use strict';

function expectedRounds(n) {
    if (n <= 1) return 0.0;
    const C = Array.from({ length: n + 1 }, () => new Array(n + 1).fill(0));
    for (let i = 0; i <= n; i++) {
        C[i][0] = 1;
        for (let j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
    }
    const f = new Array(n + 1).fill(0);
    for (let m = 2; m <= n; m++) {
        const half = Math.pow(0.5, m);
        let s = 0;
        for (let k = 0; k < m; k++) s += C[m][k] * f[k];
        f[m] = (1 + half * s) / (1 - half);
    }
    return f[n];
}

function main() {
    const n = 4;
    console.log(expectedRounds(n).toFixed(4)); // ~2.0571
}

main();
