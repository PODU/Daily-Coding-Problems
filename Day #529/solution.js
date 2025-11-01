// Day 529: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i] = min cuts for prefix i with parent pointers
// to reconstruct one optimal partition. Time O(n^2), space O(n^2).

function minPalindromePartition(s) {
  const n = s.length;
  const pal = Array.from({ length: n }, () => new Array(n).fill(false));
  for (let i = n - 1; i >= 0; i--)
    for (let j = i; j < n; j++)
      pal[i][j] = s[i] === s[j] && (j - i < 2 || pal[i + 1][j - 1]);

  const dp = new Array(n + 1).fill(Infinity);
  const prev = new Array(n + 1).fill(-1);
  dp[0] = 0;
  for (let j = 1; j <= n; j++)
    for (let i = 0; i < j; i++)
      if (pal[i][j - 1] && dp[i] + 1 < dp[j]) {
        dp[j] = dp[i] + 1;
        prev[j] = i;
      }

  const parts = [];
  for (let j = n; j > 0; j = prev[j]) parts.push(s.slice(prev[j], j));
  parts.reverse();
  return parts;
}

const s = "racecarannakayak";
console.log(minPalindromePartition(s)); // expected: [ 'racecar', 'anna', 'kayak' ]
