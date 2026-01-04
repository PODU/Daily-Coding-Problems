// Day 855: count N-Queens solutions via backtracking with bitmasks for column/diagonals.
// O(N!) worst case, O(N) space. Bitmask makes conflict checks O(1).
function countNQueens(n) {
  const full = (1 << n) - 1;
  let count = 0;
  function solve(cols, diag1, diag2) {
    if (cols === full) { count++; return; }
    let avail = full & ~(cols | diag1 | diag2);
    while (avail) {
      const p = avail & (-avail);
      avail -= p;
      solve(cols | p, (diag1 | p) << 1, (diag2 | p) >> 1);
    }
  }
  solve(0, 0, 0);
  return count;
}

for (let n = 1; n <= 8; n++) console.log(`N=${n}: ${countNQueens(n)}`); // N=8: 92
