// Day 860: Regex matching with '.' and '*'.
// Approach: bottom-up DP, dp[i][j] = does s[i:] match p[j:].
// Time: O(n*m), Space: O(n*m).
'use strict';

function isMatch(s, p) {
  const n = s.length, m = p.length;
  const dp = Array.from({ length: n + 1 }, () => new Array(m + 1).fill(false));
  dp[n][m] = true;
  for (let i = n; i >= 0; i--)
    for (let j = m - 1; j >= 0; j--) {
      const first = i < n && (p[j] === s[i] || p[j] === '.');
      if (j + 1 < m && p[j + 1] === '*')
        dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
      else
        dp[i][j] = first && dp[i + 1][j + 1];
    }
  return dp[0][0];
}

console.log(isMatch('ray', 'ra.'));      // true
console.log(isMatch('raymond', 'ra.'));  // false
console.log(isMatch('chat', '.*at'));    // true
console.log(isMatch('chats', '.*at'));   // false
