// Day 861: Probability a knight stays on an 8x8 board after k random moves.
// Approach: DP over moves; dp[r][c] = prob of being on this cell, spread 1/8 each move.
// Time: O(k * N^2), Space: O(N^2).
'use strict';

const MOVES = [[-2, -1], [-2, 1], [-1, -2], [-1, 2],
  [1, -2], [1, 2], [2, -1], [2, 1]];

function knightProbability(n, k, sr, sc) {
  let dp = Array.from({ length: n }, () => new Array(n).fill(0));
  dp[sr][sc] = 1;
  for (let step = 0; step < k; step++) {
    const nx = Array.from({ length: n }, () => new Array(n).fill(0));
    for (let r = 0; r < n; r++)
      for (let c = 0; c < n; c++)
        if (dp[r][c] > 0)
          for (const [dr, dc] of MOVES) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < n && nc >= 0 && nc < n) nx[nr][nc] += dp[r][c] / 8;
          }
    dp = nx;
  }
  let total = 0;
  for (const row of dp) for (const v of row) total += v;
  return total;
}

console.log(knightProbability(8, 1, 0, 0)); // 0.25
console.log(knightProbability(8, 2, 0, 0));
