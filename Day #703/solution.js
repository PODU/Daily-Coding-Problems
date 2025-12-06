// Day 703: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over board cells; each move spreads prob/8 to valid targets.
// Time O(k * N^2 * 8), Space O(N^2).
const MOVES = [[1, 2], [1, -2], [-1, 2], [-1, -2], [2, 1], [2, -1], [-2, 1], [-2, -1]];

function knightProbability(n, k, r, c) {
  let dp = Array.from({ length: n }, () => new Array(n).fill(0));
  dp[r][c] = 1.0;
  for (let step = 0; step < k; step++) {
    const nd = Array.from({ length: n }, () => new Array(n).fill(0));
    for (let i = 0; i < n; i++)
      for (let j = 0; j < n; j++)
        if (dp[i][j])
          for (const [dr, dc] of MOVES) {
            const ni = i + dr, nj = j + dc;
            if (ni >= 0 && ni < n && nj >= 0 && nj < n) nd[ni][nj] += dp[i][j] / 8.0;
          }
    dp = nd;
  }
  return dp.flat().reduce((a, b) => a + b, 0);
}

console.log(knightProbability(8, 2, 0, 0)); // 0.1875
