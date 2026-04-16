// Can form palindrome by deleting <= k chars: min deletions = n - LPS(s).
// LPS via DP. Time O(n^2), Space O(n^2).
function lps(s) {
  const n = s.length;
  const dp = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let i = 0; i < n; i++) dp[i][i] = 1;
  for (let len = 2; len <= n; len++)
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      if (s[i] === s[j]) dp[i][j] = len === 2 ? 2 : dp[i + 1][j - 1] + 2;
      else dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
    }
  return dp[0][n - 1];
}

function canMakePalindrome(s, k) {
  return s.length - lps(s) <= k;
}

console.log(canMakePalindrome("waterrfetawx", 2) ? "True" : "False");
