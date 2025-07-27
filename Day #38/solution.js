// N-Queens count via bitmask backtracking (columns + two diagonals). O(N!) worst, O(N) space.
function countNQueens(n) {
  const full = (1 << n) - 1;
  function solve(cols, diag1, diag2) {
    if (cols === full) return 1;
    let count = 0;
    let avail = ~(cols | diag1 | diag2) & full;
    while (avail) {
      const bit = avail & (-avail);
      avail -= bit;
      count += solve(cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1);
    }
    return count;
  }
  return solve(0, 0, 0);
}

console.log(countNQueens(8));
