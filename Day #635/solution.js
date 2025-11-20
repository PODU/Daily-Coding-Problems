// Day 635: Regular expression matching with '.' and '*' (full-string match).
// Approach: bottom-up DP; dp[i][j] = does s[i:] match p[j:].
// Time: O(m*n), Space: O(m*n).
function isMatch(s, p) {
  const m = s.length, n = p.length;
  const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(false));
  dp[m][n] = true;
  for (let i = m; i >= 0; i--) {
    for (let j = n - 1; j >= 0; j--) {
      const first = i < m && (p[j] === s[i] || p[j] === '.');
      if (j + 1 < n && p[j + 1] === '*') {
        dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
      } else {
        dp[i][j] = first && dp[i + 1][j + 1];
      }
    }
  }
  return dp[0][0];
}

console.log(isMatch("ray", "ra."));     // true
console.log(isMatch("raymond", "ra.")); // false
console.log(isMatch("chat", ".*at"));   // true
console.log(isMatch("chats", ".*at"));  // false
