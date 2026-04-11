// Day 1336: Count distinct N-Queens arrangements.
// Backtracking with column/diagonal bitmasks. Time: O(N!) worst, Space: O(N).
function totalNQueens(n) {
  function count(row, cols, d1, d2) {
    if (row === n) return 1;
    let total = 0;
    let avail = ((1 << n) - 1) & ~(cols | d1 | d2);
    while (avail) {
      const bit = avail & -avail;
      avail -= bit;
      total += count(row + 1, cols | bit, (d1 | bit) << 1, (d2 | bit) >> 1);
    }
    return total;
  }
  return count(0, 0, 0, 0);
}

console.log("N=1 ->", totalNQueens(1));
console.log("N=4 ->", totalNQueens(4));
console.log("N=8 ->", totalNQueens(8));
