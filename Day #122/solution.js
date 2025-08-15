// Day 122: Max coins from top-left to bottom-right moving right/down.
// DP O(R*C) time and space, with path reconstruction (prefer left on ties).
function solve(g) {
  const R = g.length, C = g[0].length;
  const dp = Array.from({ length: R }, () => new Array(C).fill(0));
  for (let i = 0; i < R; i++)
    for (let j = 0; j < C; j++) {
      let best = 0;
      if (i === 0 && j === 0) best = 0;
      else if (i === 0) best = dp[i][j - 1];
      else if (j === 0) best = dp[i - 1][j];
      else best = Math.max(dp[i - 1][j], dp[i][j - 1]);
      dp[i][j] = g[i][j] + best;
    }
  const path = [];
  let i = R - 1, j = C - 1;
  while (i > 0 || j > 0) {
    path.push(g[i][j]);
    if (i === 0) j--;
    else if (j === 0) i--;
    else if (dp[i - 1][j] > dp[i][j - 1]) i--;
    else j--;
  }
  path.push(g[0][0]);
  path.reverse();
  return [dp[R - 1][C - 1], path];
}

const g = [[0, 3, 1, 1], [2, 0, 0, 4], [1, 5, 3, 1]];
const [total, path] = solve(g);
console.log("The most we can collect is " + path.join(" + ") + " = " + total + " coins.");
