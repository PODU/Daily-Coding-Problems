// Day 414: Count N-Queens arrangements via bitmask backtracking.
// Track used columns/diagonals as bitmasks. Time O(N!)-ish, Space O(N).
function countNQueens(n) {
  function solve(row, cols, diag1, diag2) {
    if (row === n) return 1;
    let count = 0;
    let avail = ((1 << n) - 1) & ~(cols | diag1 | diag2);
    while (avail) {
      const bit = avail & -avail;
      avail -= bit;
      count += solve(row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1);
    }
    return count;
  }
  return solve(0, 0, 0, 0);
}

for (let n = 1; n <= 8; n++) console.log(`N=${n}: ${countNQueens(n)}`);
console.log(countNQueens(8)); // 92
