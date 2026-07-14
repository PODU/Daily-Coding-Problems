// Knight-on-board probability after k random moves: DP over board states.
// dp[r][c] = prob of being on (r,c); each move spreads 1/8 to each target.
// Time: O(k*64*8). Space: O(64).
const MOVES = [[-2,-1],[-2,1],[-1,-2],[-1,2],[1,-2],[1,2],[2,-1],[2,1]];

function knightProbability(n, k, r0, c0) {
  let dp = Array.from({ length: n }, () => new Array(n).fill(0));
  dp[r0][c0] = 1.0;
  for (let m = 0; m < k; m++) {
    const nxt = Array.from({ length: n }, () => new Array(n).fill(0));
    for (let r = 0; r < n; r++)
      for (let c = 0; c < n; c++) {
        if (dp[r][c] === 0) continue;
        for (const [dr, dc] of MOVES) {
          const nr = r + dr, nc = c + dc;
          if (nr >= 0 && nr < n && nc >= 0 && nc < n) nxt[nr][nc] += dp[r][c] / 8;
        }
      }
    dp = nxt;
  }
  let total = 0;
  for (const row of dp) for (const v of row) total += v;
  return total;
}

// corner (0,0), k=1 -> 2/8 = 0.25
console.log(knightProbability(8, 1, 0, 0)); // 0.25
