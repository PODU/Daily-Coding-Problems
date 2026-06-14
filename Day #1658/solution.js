// Max weight path top->bottom in triangle, bottom-up DP collapsing rows. O(n) space.
function maxPath(t) {
  const dp = [...t[t.length - 1]];
  for (let i = t.length - 2; i >= 0; i--)
    for (let j = 0; j <= i; j++)
      dp[j] = t[i][j] + Math.max(dp[j], dp[j + 1]);
  return dp[0];
}

console.log(maxPath([[1], [2, 3], [1, 5, 1]]));
