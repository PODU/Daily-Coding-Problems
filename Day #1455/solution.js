// Day 1455: Maximum top-to-bottom path sum in a triangle. Bottom-up DP folding
// each row into the one above. Time O(n^2), Space O(n).
function maxPathSum(triangle) {
  if (triangle.length === 0) return 0;
  const dp = triangle[triangle.length - 1].slice();
  for (let r = triangle.length - 2; r >= 0; r--)
    for (let i = 0; i <= r; i++)
      dp[i] = triangle[r][i] + Math.max(dp[i], dp[i + 1]);
  return dp[0];
}

const triangle = [[1], [2, 3], [1, 5, 1]];
console.log(maxPathSum(triangle)); // 9  (1 -> 3 -> 5)
