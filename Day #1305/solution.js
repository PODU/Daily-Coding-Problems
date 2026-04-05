// Day 1305: Longest common subsequence of three strings.
// 3D DP over prefixes. O(|a||b||c|) time, O(|a||b||c|) space.
function lcs3(a, b, c) {
  const n = a.length, m = b.length, p = c.length;
  const dp = Array.from({ length: n + 1 }, () =>
    Array.from({ length: m + 1 }, () => new Array(p + 1).fill(0))
  );
  for (let i = 1; i <= n; i++)
    for (let j = 1; j <= m; j++)
      for (let k = 1; k <= p; k++) {
        if (a[i - 1] === b[j - 1] && b[j - 1] === c[k - 1])
          dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
        else
          dp[i][j][k] = Math.max(dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1]);
      }
  return dp[n][m][p];
}

console.log(
  lcs3("epidemiologist", "refrigeration", "supercalifragilisticexpialodocious")
); // 5
