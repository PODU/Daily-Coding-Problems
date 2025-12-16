// Longest palindromic substring via Manacher's algorithm.
// Transform with '#' separators, expand radii using mirror symmetry.
// Time: O(n), Space: O(n).

function longestPalindrome(s) {
  if (s.length === 0) return "";
  const t = '#' + s.split('').join('#') + '#';
  const n = t.length;
  const p = new Array(n).fill(0);
  let c = 0, r = 0;
  for (let i = 0; i < n; i++) {
    if (i < r) p[i] = Math.min(r - i, p[2 * c - i]);
    while (i - p[i] - 1 >= 0 && i + p[i] + 1 < n && t[i - p[i] - 1] === t[i + p[i] + 1]) p[i]++;
    if (i + p[i] > r) { c = i; r = i + p[i]; }
  }
  let maxLen = 0, center = 0;
  for (let i = 0; i < n; i++) if (p[i] > maxLen) { maxLen = p[i]; center = i; }
  const start = (center - maxLen) / 2;
  return s.substring(start, start + maxLen);
}

console.log(longestPalindrome("aabcdcb")); // bcdcb
console.log(longestPalindrome("bananas")); // anana
