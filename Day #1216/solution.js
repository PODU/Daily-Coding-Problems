// Day 1216: Min columns to delete so each column is non-decreasing top->bottom.
// Approach: scan each column once, count unsorted columns. Time O(N*M), Space O(1).
function minDeletions(grid) {
  if (grid.length === 0) return 0;
  const rows = grid.length, cols = grid[0].length;
  let count = 0;
  for (let c = 0; c < cols; c++) {
    for (let r = 1; r < rows; r++) {
      if (grid[r][c] < grid[r - 1][c]) { count++; break; }
    }
  }
  return count;
}

const grid = ["cba", "daf", "ghi"];
console.log(minDeletions(grid)); // 1
