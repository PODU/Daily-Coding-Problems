// Day 1328: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i]=min pieces for prefix i, with parent pointers to rebuild. O(n^2) time/space.

function minPalindromePartition(s) {
  const n = s.length;
  if (n === 0) return [];
  const pal = Array.from({ length: n }, () => new Array(n).fill(false));
  for (let i = n - 1; i >= 0; i--)
    for (let j = i; j < n; j++)
      pal[i][j] = s[i] === s[j] && (j - i < 2 || pal[i + 1][j - 1]);

  const dp = new Array(n + 1).fill(Infinity);
  const prev = new Array(n + 1).fill(-1);
  dp[0] = 0;
  for (let i = 1; i <= n; i++)
    for (let j = 0; j < i; j++)
      if (pal[j][i - 1] && dp[j] + 1 < dp[i]) { dp[i] = dp[j] + 1; prev[i] = j; }

  const res = [];
  for (let i = n; i > 0; i = prev[i]) res.push(s.substring(prev[i], i));
  return res.reverse();
}

console.log(minPalindromePartition("racecarannakayak")); // [ 'racecar', 'anna', 'kayak' ]
console.log(minPalindromePartition("abc"));               // [ 'a', 'b', 'c' ]
