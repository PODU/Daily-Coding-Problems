// Day 304: Probability knight stays on board after k random moves. DP over board.
// Time O(k*N^2*8), space O(N^2).
function knightProb(N, k, sr, sc) {
  let dp = Array.from({ length: N }, () => new Array(N).fill(0));
  dp[sr][sc] = 1.0;
  const moves = [[1,2],[1,-2],[-1,2],[-1,-2],[2,1],[2,-1],[-2,1],[-2,-1]];
  for (let step = 0; step < k; step++) {
    const nd = Array.from({ length: N }, () => new Array(N).fill(0));
    for (let r = 0; r < N; r++) for (let c = 0; c < N; c++) if (dp[r][c] > 0)
      for (const [dr, dc] of moves) {
        const nr = r + dr, nc = c + dc;
        if (nr >= 0 && nr < N && nc >= 0 && nc < N) nd[nr][nc] += dp[r][c] / 8.0;
      }
    dp = nd;
  }
  let tot = 0;
  for (const row of dp) for (const v of row) tot += v;
  return tot;
}
console.log(knightProb(8, 1, 0, 0)); // 0.25
