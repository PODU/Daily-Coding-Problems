// Day 54: Sudoku solver via backtracking with bitmask row/col/box constraints.
// Worst case exponential; bitmasks make pruning fast. Space: O(1).
function solveSudoku(g) {
  const rows = new Array(9).fill(0);
  const cols = new Array(9).fill(0);
  const boxes = new Array(9).fill(0);
  for (let r = 0; r < 9; r++)
    for (let c = 0; c < 9; c++)
      if (g[r][c]) {
        const bit = 1 << g[r][c];
        rows[r] |= bit; cols[c] |= bit; boxes[((r / 3) | 0) * 3 + ((c / 3) | 0)] |= bit;
      }

  function solve(cell) {
    if (cell === 81) return true;
    const r = (cell / 9) | 0, c = cell % 9, b = ((r / 3) | 0) * 3 + ((c / 3) | 0);
    if (g[r][c]) return solve(cell + 1);
    for (let d = 1; d <= 9; d++) {
      const bit = 1 << d;
      if ((rows[r] | cols[c] | boxes[b]) & bit) continue;
      g[r][c] = d;
      rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
      if (solve(cell + 1)) return true;
      g[r][c] = 0;
      rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
    }
    return false;
  }

  solve(0);
  return g;
}

const grid = [
  [5, 3, 0, 0, 7, 0, 0, 0, 0],
  [6, 0, 0, 1, 9, 5, 0, 0, 0],
  [0, 9, 8, 0, 0, 0, 0, 6, 0],
  [8, 0, 0, 0, 6, 0, 0, 0, 3],
  [4, 0, 0, 8, 0, 3, 0, 0, 1],
  [7, 0, 0, 0, 2, 0, 0, 0, 6],
  [0, 6, 0, 0, 0, 0, 2, 8, 0],
  [0, 0, 0, 4, 1, 9, 0, 0, 5],
  [0, 0, 0, 0, 8, 0, 0, 7, 9],
];
for (const row of solveSudoku(grid)) console.log(row.join(""));
