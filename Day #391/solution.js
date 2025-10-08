// Day 391: Longest common contiguous subsequence (substring) of two histories.
// DP on suffix-run lengths. Time O(n*m), Space O(n*m).
function longestCommon(a, b) {
  const n = a.length, m = b.length;
  const dp = Array.from({ length: n + 1 }, () => new Array(m + 1).fill(0));
  let best = 0, endI = 0;
  for (let i = 1; i <= n; i++)
    for (let j = 1; j <= m; j++)
      if (a[i - 1] === b[j - 1]) {
        dp[i][j] = dp[i - 1][j - 1] + 1;
        if (dp[i][j] > best) { best = dp[i][j]; endI = i; }
      }
  return a.slice(endI - best, endI);
}

const user1 = ['/home', '/register', '/login', '/user', '/one', '/two'];
const user2 = ['/home', '/red', '/login', '/user', '/one', '/pink'];
console.log(longestCommon(user1, user2));
