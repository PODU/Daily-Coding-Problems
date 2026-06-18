// Day 1684: Levenshtein edit distance via 1D rolling DP.
// Time O(n*m), Space O(min(n,m)).
function editDistance(a, b) {
  const n = a.length, m = b.length;
  const dp = Array.from({ length: m + 1 }, (_, j) => j);
  for (let i = 1; i <= n; i++) {
    let prev = dp[0];
    dp[0] = i;
    for (let j = 1; j <= m; j++) {
      const tmp = dp[j];
      dp[j] = a[i - 1] === b[j - 1] ? prev
            : 1 + Math.min(prev, dp[j], dp[j - 1]);
      prev = tmp;
    }
  }
  return dp[m];
}

console.log(editDistance("kitten", "sitting")); // 3
