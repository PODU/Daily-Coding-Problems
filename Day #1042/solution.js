// LCS of three strings via 3D DP dp[i][j][k]. Time O(n1*n2*n3), Space O(n1*n2*n3).
function lcs3(a, b, c) {
    const n1 = a.length, n2 = b.length, n3 = c.length;
    const dp = Array.from({ length: n1 + 1 }, () =>
        Array.from({ length: n2 + 1 }, () => new Array(n3 + 1).fill(0)));
    for (let i = 1; i <= n1; i++)
        for (let j = 1; j <= n2; j++)
            for (let k = 1; k <= n3; k++) {
                if (a[i - 1] === b[j - 1] && b[j - 1] === c[k - 1])
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                else
                    dp[i][j][k] = Math.max(dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1]);
            }
    return dp[n1][n2][n3];
}

console.log(lcs3("epidemiologist", "refrigeration",
    "supercalifragilisticexpialodocious"));
