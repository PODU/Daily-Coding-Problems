// Day 1473: Count occurrences of X in an N x N multiplication table.
// For each row i, X appears iff i divides X and X/i is within [1, N].
// Time O(N), Space O(1).

function countX(n, x) {
  let count = 0;
  for (let i = 1; i <= n; ++i) {
    if (x % i === 0 && x / i >= 1 && x / i <= n) ++count;
  }
  return count;
}

console.log(countX(6, 12)); // 4
