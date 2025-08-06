// Day 76: Minimum columns to remove so every column is sorted top-to-bottom.
// Greedy scan: count columns that are not non-decreasing. Time O(N*M), Space O(1).
function minColumnsToRemove(grid) {
  if (grid.length === 0) return 0;
  const rows = grid.length, cols = grid[0].length;
  let remove = 0;
  for (let c = 0; c < cols; c++) {
    for (let r = 1; r < rows; r++) {
      if (grid[r][c] < grid[r - 1][c]) { remove++; break; }
    }
  }
  return remove;
}

const grid = ["cba", "daf", "ghi"];
console.log(minColumnsToRemove(grid)); // 1
