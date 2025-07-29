// Day 46: Longest palindromic substring via Manacher's algorithm.
// Time: O(n), Space: O(n).
function longestPalindrome(s) {
  if (!s) return "";
  const t = "^#" + s.split("").join("#") + "#$";
  const n = t.length;
  const p = new Array(n).fill(0);
  let c = 0, r = 0;
  for (let i = 1; i < n - 1; i++) {
    if (i < r) p[i] = Math.min(r - i, p[2 * c - i]);
    while (t[i + 1 + p[i]] === t[i - 1 - p[i]]) p[i]++;
    if (i + p[i] > r) { c = i; r = i + p[i]; }
  }
  let maxLen = 0, center = 0;
  for (let i = 1; i < n - 1; i++) if (p[i] > maxLen) { maxLen = p[i]; center = i; }
  const start = (center - maxLen) / 2;
  return s.substring(start, start + maxLen);
}

console.log('"' + longestPalindrome("aabcdcb") + '"');
console.log('"' + longestPalindrome("bananas") + '"');
