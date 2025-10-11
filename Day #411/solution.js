// Day 411: Can we make s a palindrome by deleting at most k chars?
// Answer: n - LPS(s) <= k, where LPS = longest palindromic subsequence via DP. O(n^2) time, O(n^2) space.
function lps(s) {
  const n = s.length;
  const dp = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let i = n - 1; i >= 0; i--) {
    dp[i][i] = 1;
    for (let j = i + 1; j < n; j++) {
      if (s[i] === s[j]) dp[i][j] = dp[i + 1][j - 1] + 2;
      else dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
    }
  }
  return dp[0][n - 1];
}

function canMakePalindrome(s, k) {
  return s.length - lps(s) <= k;
}

const s = "waterrfetawx";
const k = 2;
console.log(canMakePalindrome(s, k) ? "True" : "False");
