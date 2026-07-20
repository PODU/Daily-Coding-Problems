// Day 1848: Longest common subsequence of three strings via 3D dynamic programming.
// Time O(L1*L2*L3), Space O(L1*L2*L3).

function lcs3(a, b, c) {
  const la = a.length,
    lb = b.length,
    lc = c.length;
  const dp = Array.from({ length: la + 1 }, () =>
    Array.from({ length: lb + 1 }, () => new Array(lc + 1).fill(0))
  );
  for (let i = 1; i <= la; i++)
    for (let j = 1; j <= lb; j++)
      for (let k = 1; k <= lc; k++) {
        if (a[i - 1] === b[j - 1] && b[j - 1] === c[k - 1])
          dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
        else dp[i][j][k] = Math.max(dp[i - 1][j][k], dp[i][j - 1][k], dp[i][j][k - 1]);
      }
  return dp[la][lb][lc];
}

function main() {
  console.log(lcs3("epidemiologist", "refrigeration", "supercalifragilisticexpialodocious")); // 5
}

main();
