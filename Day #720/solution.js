// Day 720: Sudoku solver via backtracking with bitmasks for rows/cols/boxes,
// always filling the next empty cell. Time exponential worst-case but fast in practice.
function solveSudoku(grid) {
  const rows = new Array(9).fill(0);
  const cols = new Array(9).fill(0);
  const boxes = new Array(9).fill(0);
  const bidx = (r, c) => ((r / 3) | 0) * 3 + ((c / 3) | 0);

  for (let r = 0; r < 9; r++)
    for (let c = 0; c < 9; c++)
      if (grid[r][c]) {
        const bit = 1 << grid[r][c];
        rows[r] |= bit; cols[c] |= bit; boxes[bidx(r, c)] |= bit;
      }

  function solve(pos) {
    if (pos === 81) return true;
    const r = (pos / 9) | 0, c = pos % 9;
    if (grid[r][c]) return solve(pos + 1);
    const b = bidx(r, c);
    for (let d = 1; d <= 9; d++) {
      const bit = 1 << d;
      if ((rows[r] | cols[c] | boxes[b]) & bit) continue;
      grid[r][c] = d; rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
      if (solve(pos + 1)) return true;
      grid[r][c] = 0; rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
    }
    return false;
  }
  solve(0);
  return grid;
}

const puzzle = [
  "530070000", "600195000", "098000060",
  "800060003", "400803001", "700020006",
  "060000280", "000419005", "000080079"];
const grid = puzzle.map(row => row.split("").map(Number));
solveSudoku(grid);
for (const row of grid) console.log(row.join(""));
