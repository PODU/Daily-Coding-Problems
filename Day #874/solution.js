// Longest palindromic substring via Manacher's algorithm. Time O(n), Space O(n).

function longestPalindrome(s) {
  if (s.length === 0) return "";
  const t = "^#" + s.split("").join("#") + "#$";
  const n = t.length;
  const p = new Array(n).fill(0);
  let center = 0, right = 0;
  for (let i = 1; i < n - 1; i++) {
    if (i < right) p[i] = Math.min(right - i, p[2 * center - i]);
    while (t[i + p[i] + 1] === t[i - p[i] - 1]) p[i]++;
    if (i + p[i] > right) { center = i; right = i + p[i]; }
  }
  let maxLen = 0, centerIndex = 0;
  for (let i = 1; i < n - 1; i++) {
    if (p[i] > maxLen) { maxLen = p[i]; centerIndex = i; }
  }
  const start = (centerIndex - maxLen) / 2;
  return s.substring(start, start + maxLen);
}

console.log('"' + longestPalindrome("aabcdcb") + '"');
console.log('"' + longestPalindrome("bananas") + '"');
