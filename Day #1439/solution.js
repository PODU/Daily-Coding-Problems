// Day 1439: Find a word in a char grid reading left-to-right or top-to-bottom.
// Approach: build each row and column string, check if target is a substring.
// Time: O(R*C*L) substring scan, Space: O(R+C).

function findWord(grid, target) {
  if (grid.length === 0) return false;
  const rows = grid.length;
  const cols = grid[0].length;
  for (const row of grid) {
    if (row.join("").includes(target)) return true;
  }
  for (let c = 0; c < cols; c++) {
    let col = "";
    for (let r = 0; r < rows; r++) col += grid[r][c];
    if (col.includes(target)) return true;
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
