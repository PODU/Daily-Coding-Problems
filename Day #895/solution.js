// Palindrome by deleting at most k chars: min deletions = n - LPS(s).
// LPS via interval DP (space-optimized to O(n)). Time O(n^2), Space O(n).
function canMakePalindrome(s, k) {
  const n = s.length;
  if (n === 0) return 0 <= k;
  let prev = new Array(n).fill(0);
  let cur = new Array(n).fill(0);
  for (let i = n - 1; i >= 0; --i) {
    cur = new Array(n).fill(0);
    cur[i] = 1;
    for (let j = i + 1; j < n; ++j) {
      if (s[i] === s[j]) cur[j] = prev[j - 1] + 2;
      else cur[j] = Math.max(prev[j], cur[j - 1]);
    }
    prev = cur;
  }
  const lps = cur[n - 1];
  return (n - lps) <= k;
}

console.log(canMakePalindrome("waterrfetawx", 2) ? "True" : "False");
