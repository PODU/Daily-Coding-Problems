// Day 867: Maximum weight path from top to bottom of a triangle.
// Approach: bottom-up DP, fold each row into the one above using max of adjacent.
// Time: O(n^2), Space: O(n).
'use strict';

function maxPath(triangle) {
  const dp = [...triangle[triangle.length - 1]];
  for (let i = triangle.length - 2; i >= 0; i--)
    for (let j = 0; j <= i; j++)
      dp[j] = triangle[i][j] + Math.max(dp[j], dp[j + 1]);
  return dp[0];
}

console.log(maxPath([[1], [2, 3], [1, 5, 1]])); // 9
