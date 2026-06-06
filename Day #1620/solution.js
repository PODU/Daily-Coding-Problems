// Sudoku solver: backtracking with row/col/box bitmasks; pick first empty cell.
// Time: exponential worst case, fast in practice. Space: O(1) extra (fixed 9x9).
'use strict';

const rows = new Array(9).fill(0);
const cols = new Array(9).fill(0);
const boxes = new Array(9).fill(0);
const grid = Array.from({ length: 9 }, () => new Array(9).fill('.'));

function boxIndex(r, c) {
  return Math.floor(r / 3) * 3 + Math.floor(c / 3);
}

function solve(pos) {
  if (pos === 81) return true;
  const r = Math.floor(pos / 9);
  const c = pos % 9;
  if (grid[r][c] !== '0' && grid[r][c] !== '.') return solve(pos + 1);
  const b = boxIndex(r, c);
  for (let d = 1; d <= 9; d++) {
    const bit = 1 << d;
    if ((rows[r] & bit) || (cols[c] & bit) || (boxes[b] & bit)) continue;
    rows[r] |= bit; cols[c] |= bit; boxes[b] |= bit;
    grid[r][c] = String(d);
    if (solve(pos + 1)) return true;
    rows[r] &= ~bit; cols[c] &= ~bit; boxes[b] &= ~bit;
    grid[r][c] = '.';
  }
  return false;
}

function main() {
  const puzzle = [
    '53..7....',
    '6..195...',
    '.98....6.',
    '8...6...3',
    '4..8.3..1',
    '7...2...6',
    '.6....28.',
    '...419..5',
    '....8..79',
  ];
  for (let r = 0; r < 9; r++) {
    for (let c = 0; c < 9; c++) {
      const ch = puzzle[r][c];
      grid[r][c] = ch;
      if (ch !== '.' && ch !== '0') {
        const bit = 1 << (ch.charCodeAt(0) - '0'.charCodeAt(0));
        rows[r] |= bit; cols[c] |= bit; boxes[boxIndex(r, c)] |= bit;
      }
    }
  }
  solve(0);
  for (let r = 0; r < 9; r++) console.log(grid[r].join(''));
}

main();
