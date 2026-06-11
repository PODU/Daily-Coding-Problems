// Regex full-match with '.' and '*' via DP: dp[i][j] = s[0..i) matches p[0..j).
// '*' uses zero-copy (dp[i][j-2]) or one-more (prev char match). Time/Space O(m*n).

function isMatch(s, p) {
  const m = s.length, n = p.length;
  const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(false));
  dp[0][0] = true;
  for (let j = 1; j <= n; j++) {
    if (p[j - 1] === '*' && j >= 2) dp[0][j] = dp[0][j - 2];
  }
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (p[j - 1] === '*') {
        dp[i][j] = j >= 2 && dp[i][j - 2];
        const prev = j >= 2 ? p[j - 2] : '';
        if (j >= 2 && (prev === '.' || prev === s[i - 1])) {
          dp[i][j] = dp[i][j] || dp[i - 1][j];
        }
      } else if (p[j - 1] === '.' || p[j - 1] === s[i - 1]) {
        dp[i][j] = dp[i - 1][j - 1];
      }
    }
  }
  return dp[m][n];
}

console.log(isMatch("ray", "ra."));
console.log(isMatch("raymond", "ra."));
console.log(isMatch("chat", ".*at"));
console.log(isMatch("chats", ".*at"));
