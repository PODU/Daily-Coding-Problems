// Day 1636: Can a string be made a palindrome by deleting at most k chars.
// min deletions = n - LongestPalindromicSubsequence; DP O(n^2) time/space.
function canMakePalindrome(s, k) {
  const n = s.length;
  if (n === 0) return true;
  const dp = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let i = 0; i < n; i++) dp[i][i] = 1;
  for (let len = 2; len <= n; len++)
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      if (s[i] === s[j]) dp[i][j] = 2 + (len > 2 ? dp[i + 1][j - 1] : 0);
      else dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
    }
  const lps = dp[0][n - 1];
  return n - lps <= k;
}

console.log(canMakePalindrome("waterrfetawx", 2) ? "True" : "False");
