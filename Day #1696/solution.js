// N-Queens count via backtracking with bitmasks (columns + two diagonals).
// Time O(N!) worst case (heavily pruned), Space O(N) recursion.

function totalNQueens(n) {
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

function main() {
  console.log(totalNQueens(8)); // 92
}

main();
