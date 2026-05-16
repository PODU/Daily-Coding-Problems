// Count cells equal to X in an N x N multiplication table (cell(i,j)=i*j).
// For each row i, X is at column X/i iff i divides X and 1<=X/i<=N. O(N) time, O(1) space.
function countCells(n, x) {
  let count = 0;
  for (let i = 1; i <= n; i++) {
    if (x % i === 0) {
      const j = x / i;
      if (j >= 1 && j <= n) count++;
    }
  }
  return count;
}

console.log(countCells(6, 12)); // expected 4
