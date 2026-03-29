// Day 1278: Sudoku solver via backtracking with row/col/box bitmasks.
// Time: exponential worst-case but fast with constraint pruning. Space O(1).
function solveSudoku(g) {
  const rowm = new Array(9).fill(0);
  const colm = new Array(9).fill(0);
  const boxm = new Array(9).fill(0);
  const bidx = (r, c) => ((r / 3) | 0) * 3 + ((c / 3) | 0);

  for (let r = 0; r < 9; r++)
    for (let c = 0; c < 9; c++) {
      const v = g[r][c];
      if (v) { const bit = 1 << v; rowm[r] |= bit; colm[c] |= bit; boxm[bidx(r, c)] |= bit; }
    }

  function solve(pos) {
    if (pos === 81) return true;
    const r = (pos / 9) | 0, c = pos % 9;
    if (g[r][c]) return solve(pos + 1);
    const b = bidx(r, c);
    const used = rowm[r] | colm[c] | boxm[b];
    for (let d = 1; d <= 9; d++) {
      const bit = 1 << d;
      if (used & bit) continue;
      g[r][c] = d; rowm[r] |= bit; colm[c] |= bit; boxm[b] |= bit;
      if (solve(pos + 1)) return true;
      g[r][c] = 0; rowm[r] &= ~bit; colm[c] &= ~bit; boxm[b] &= ~bit;
    }
    return false;
  }

  solve(0);
  return g;
}

const puzzle = [
  "53..7....", "6..195...", ".98....6.",
  "8...6...3", "4..8.3..1", "7...2...6",
  ".6....28.", "...419..5", "....8..79"];
const grid = puzzle.map((row) => [...row].map((ch) => (ch === '.' ? 0 : +ch)));
for (const row of solveSudoku(grid)) console.log(row.join(""));
