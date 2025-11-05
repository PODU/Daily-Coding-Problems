// Count occurrences of X in an N x N multiplication table.
// For each row i (1..N), X appears iff i divides X and X/i is in [1,N]. O(N) time, O(1) space.
function countX(n, x) {
  let cnt = 0;
  for (let i = 1; i <= n; i++) {
    if (x % i === 0) {
      const q = x / i;
      if (q >= 1 && q <= n) cnt++;
    }
  }
  return cnt;
}

console.log(countX(6, 12)); // 4
