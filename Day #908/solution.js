// Count columns to delete so each remaining column is non-decreasing top->bottom.
// Scan each column for any adjacent out-of-order pair. Time O(N*M), Space O(1).

function minDeletions(grid) {
  if (grid.length === 0) return 0;
  const n = grid.length, m = grid[0].length;
  let count = 0;
  for (let c = 0; c < m; c++) {
    for (let i = 0; i + 1 < n; i++) {
      if (grid[i][c] > grid[i + 1][c]) { count++; break; }
    }
  }
  return count;
}

const grid = ["cba", "daf", "ghi"];
console.log(minDeletions(grid));
