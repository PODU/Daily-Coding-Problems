// Day 1672: Determine if black king is in check on an 8x8 board.
// Scan knight jumps + 8 rays from king (first blocker decides). Time O(64), Space O(1).
function inCheck(b) {
  let kr = -1, kc = -1;
  for (let r = 0; r < 8; r++)
    for (let c = 0; c < 8; c++)
      if (b[r][c] === 'K') { kr = r; kc = c; }

  const km = [[-2,-1],[-2,1],[-1,-2],[-1,2],[1,-2],[1,2],[2,-1],[2,1]];
  for (const [dr, dc] of km) {
    const r = kr + dr, c = kc + dc;
    if (r >= 0 && r < 8 && c >= 0 && c < 8 && b[r][c] === 'N') return true;
  }
  const dirs = [[1,0],[-1,0],[0,1],[0,-1],[1,1],[1,-1],[-1,1],[-1,-1]];
  for (let d = 0; d < 8; d++) {
    const diag = d >= 4;
    for (let step = 1; step < 8; step++) {
      const r = kr + dirs[d][0]*step, c = kc + dirs[d][1]*step;
      if (r < 0 || r >= 8 || c < 0 || c >= 8) break;
      const p = b[r][c];
      if (p === '.') continue;
      if (diag) {
        if (p === 'B' || p === 'Q') return true;
        if (step === 1 && p === 'K') return true;
        if (step === 1 && p === 'P' && dirs[d][0] === 1) return true;
      } else {
        if (p === 'R' || p === 'Q') return true;
        if (step === 1 && p === 'K') return true;
      }
      break;
    }
  }
  return false;
}

const board = ["...K....","........",".B......","......P.",
               ".......R","..N.....","........",".....Q.."];
console.log(inCheck(board) ? "True" : "False"); // True
