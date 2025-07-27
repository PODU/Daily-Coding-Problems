// Min-insertion palindrome, lexicographically earliest. DP over substrings; O(n^2) states, O(n^3) overall.
function solve(s) {
  const n = s.length;
  if (n === 0) return "";
  const dp = Array.from({ length: n }, () => new Array(n).fill(""));
  for (let i = 0; i < n; i++) dp[i][i] = s[i];
  for (let len = 2; len <= n; len++) {
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      if (s[i] === s[j]) {
        const inner = (i + 1 <= j - 1) ? dp[i + 1][j - 1] : "";
        dp[i][j] = s[i] + inner + s[j];
      } else {
        const c1 = s[i] + dp[i + 1][j] + s[i];
        const c2 = s[j] + dp[i][j - 1] + s[j];
        if (c1.length < c2.length) dp[i][j] = c1;
        else if (c2.length < c1.length) dp[i][j] = c2;
        else dp[i][j] = c1 < c2 ? c1 : c2;
      }
    }
  }
  return dp[0][n - 1];
}

console.log('"' + solve("race") + '"');
