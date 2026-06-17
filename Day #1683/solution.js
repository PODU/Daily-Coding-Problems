// Day 1683: Find word reading left-to-right (a row) or top-to-bottom (a column).
// Build row/column strings, substring search. Time O(N*M), Space O(N+M).
function findWord(grid, word) {
  const rows = grid.length, cols = grid[0].length;
  for (let r = 0; r < rows; r++)
    if (grid[r].join("").includes(word)) return true;
  for (let c = 0; c < cols; c++) {
    let col = "";
    for (let r = 0; r < rows; r++) col += grid[r][c];
    if (col.includes(word)) return true;
  }
  return false;
}

const grid = [['F', 'A', 'C', 'I'],
              ['O', 'B', 'Q', 'P'],
              ['A', 'N', 'O', 'B'],
              ['M', 'A', 'S', 'S']];
console.log("'FOAM' ->", findWord(grid, "FOAM")); // true
console.log("'MASS' ->", findWord(grid, "MASS")); // true
