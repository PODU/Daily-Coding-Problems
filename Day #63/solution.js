// Search word in matrix rows (L->R) and columns (top->bottom) via substring check.
// Time O(N*M*L), Space O(max(N,M)).
function findWord(grid, word) {
  const n = grid.length, m = grid[0].length;
  for (let r = 0; r < n; ++r) {
    if (grid[r].join("").includes(word)) return true;
  }
  for (let c = 0; c < m; ++c) {
    let col = "";
    for (let r = 0; r < n; ++r) col += grid[r][c];
    if (col.includes(word)) return true;
  }
  return false;
}

const grid = [
  ['F', 'A', 'C', 'I'],
  ['O', 'B', 'Q', 'P'],
  ['A', 'N', 'O', 'B'],
  ['M', 'A', 'S', 'S'],
];
console.log(`'FOAM' -> ${findWord(grid, "FOAM")}`);
console.log(`'MASS' -> ${findWord(grid, "MASS")}`);
