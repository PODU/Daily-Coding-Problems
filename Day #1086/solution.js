// Sudoku solver via backtracking with bitmask constraints (rows/cols/boxes).
// Worst-case exponential, fast in practice; Space O(1).
function solveSudoku(grid) {
  const rows = new Array(9).fill(0);
  const cols = new Array(9).fill(0);
  const boxes = new Array(9).fill(0);
  const boxId = (r, c) => Math.floor(r / 3) * 3 + Math.floor(c / 3);

  for (let r = 0; r < 9; r++)
    for (let c = 0; c < 9; c++) {
      const d = grid[r][c];
      if (d) { const bit = 1 << d; rows[r] |= bit; cols[c] |= bit; boxes[boxId(r, c)] |= bit; }
    }

  function solve(pos) {
    if (pos === 81) return true;
    const r = Math.floor(pos / 9), c = pos % 9;
    if (grid[r][c] !== 0) return solve(pos + 1);
    const b = boxId(r, c);
    for (let d = 1; d <= 9; d++) {
      const bit = 1 << d;
      if (!((rows[r] | cols[c] | boxes[b]) & bit)) {
        grid[r][c] = d; rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
        if (solve(pos + 1)) return true;
        grid[r][c] = 0; rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
      }
    }
    return false;
  }
  solve(0);
  return grid;
}

const puzzle = [
  [5, 3, 0, 0, 7, 0, 0, 0, 0],
  [6, 0, 0, 1, 9, 5, 0, 0, 0],
  [0, 9, 8, 0, 0, 0, 0, 6, 0],
  [8, 0, 0, 0, 6, 0, 0, 0, 3],
  [4, 0, 0, 8, 0, 3, 0, 0, 1],
  [7, 0, 0, 0, 2, 0, 0, 0, 6],
  [0, 6, 0, 0, 0, 0, 2, 8, 0],
  [0, 0, 0, 4, 1, 9, 0, 0, 5],
  [0, 0, 0, 0, 8, 0, 0, 7, 9]];
console.log(solveSudoku(puzzle).map(row => row.join('')).join('\n'));
