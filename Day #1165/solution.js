// Knight on board probability via DP over moves. prob(m,r,c)=avg of 8 neighbors.
// Time: O(k*64*8), Space: O(64).
function knightProbability(k, startR, startC) {
  const moves = [[-2,-1],[-2,1],[-1,-2],[-1,2],[1,-2],[1,2],[2,-1],[2,1]];
  let prob = Array.from({ length: 8 }, () => new Array(8).fill(1.0));
  for (let m = 0; m < k; m++) {
    const next = Array.from({ length: 8 }, () => new Array(8).fill(0.0));
    for (let r = 0; r < 8; r++)
      for (let c = 0; c < 8; c++) {
        let s = 0.0;
        for (const [dr, dc] of moves) {
          const nr = r + dr, nc = c + dc;
          if (nr >= 0 && nr < 8 && nc >= 0 && nc < 8) s += prob[nr][nc];
        }
        next[r][c] = s / 8.0;
      }
    prob = next;
  }
  return prob[startR][startC];
}

const ans = knightProbability(1, 0, 0);
console.log(String(parseFloat(ans.toFixed(2))));
