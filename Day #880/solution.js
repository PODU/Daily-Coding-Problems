// Min palindrome partition via DP with palindrome table + reconstruction.
// Time O(n^2), Space O(n^2).

function minPalindromePartition(s) {
  const n = s.length;
  if (n === 0) return [];
  const pal = Array.from({ length: n }, () => new Array(n).fill(false));
  for (let i = n - 1; i >= 0; i--)
    for (let j = i; j < n; j++)
      if (s[i] === s[j] && (j - i < 2 || pal[i + 1][j - 1])) pal[i][j] = true;

  const INF = Infinity;
  const dp = new Array(n + 1).fill(INF);
  const cut = new Array(n + 1).fill(-1);
  dp[0] = 0;
  for (let i = 1; i <= n; i++)
    for (let j = 0; j < i; j++)
      if (pal[j][i - 1] && dp[j] + 1 < dp[i]) { dp[i] = dp[j] + 1; cut[i] = j; }

  const res = [];
  for (let i = n; i > 0; i = cut[i]) res.push(s.substring(cut[i], i));
  return res.reverse();
}

console.log(minPalindromePartition("racecarannakayak"));
console.log(minPalindromePartition("abc"));
