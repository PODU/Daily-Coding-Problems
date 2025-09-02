// Day 201: Maximum weight path in a triangle.
// Bottom-up DP: each cell becomes its value + max of the two children below.
// Time: O(n^2), Space: O(n).
function maxPath(t) {
  const dp = [...t[t.length - 1]];
  for (let r = t.length - 2; r >= 0; r--)
    for (let c = 0; c <= r; c++)
      dp[c] = t[r][c] + Math.max(dp[c], dp[c + 1]);
  return dp[0];
}

console.log(maxPath([[1], [2, 3], [1, 5, 1]])); // 9
