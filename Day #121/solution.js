// Day 121: Can form palindrome by deleting at most k chars.
// Min deletions = n - LongestPalindromicSubsequence. DP O(n^2) time, O(n^2) space.
function lps(s) {
  const n = s.length;
  if (n === 0) return 0;
  const dp = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let i = 0; i < n; i++) dp[i][i] = 1;
  for (let len = 2; len <= n; len++)
    for (let i = 0; i + len - 1 < n; i++) {
      const j = i + len - 1;
      if (s[i] === s[j]) dp[i][j] = 2 + (len === 2 ? 0 : dp[i + 1][j - 1]);
      else dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
    }
  return dp[0][n - 1];
}

function canMakePalindrome(s, k) {
  return s.length - lps(s) <= k;
}

const s = "waterrfetawx";
const k = 2;
console.log(
  canMakePalindrome(s, k)
    ? "You could delete f and x to get 'waterretaw'."
    : "Not possible"
);
