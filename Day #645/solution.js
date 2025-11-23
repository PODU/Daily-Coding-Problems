// Day 645: Find a word in a grid going left-to-right or top-to-bottom.
// Approach: scan every row and every column for the target as a substring start.
// Time: O(R*C*L), Space: O(1).
function findWord(grid, word) {
  const R = grid.length, C = grid[0].length, L = word.length;
  for (let r = 0; r < R; r++)
    for (let c = 0; c + L <= C; c++) {
      let ok = true;
      for (let k = 0; k < L; k++) if (grid[r][c + k] !== word[k]) { ok = false; break; }
      if (ok) return true;
    }
  for (let c = 0; c < C; c++)
    for (let r = 0; r + L <= R; r++) {
      let ok = true;
      for (let k = 0; k < L; k++) if (grid[r + k][c] !== word[k]) { ok = false; break; }
      if (ok) return true;
    }
  return false;
}

const grid = [
  ['F', 'A', 'C', 'I'],
  ['O', 'B', 'Q', 'P'],
  ['A', 'N', 'O', 'B'],
  ['M', 'A', 'S', 'S'],
];
console.log(findWord(grid, "FOAM")); // true
console.log(findWord(grid, "MASS")); // true
