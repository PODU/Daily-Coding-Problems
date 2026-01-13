// Lattice paths in N x M grid = C(N+M-2, N-1), computed iteratively.
// Time: O(min(N,M)), Space: O(1). BigInt avoids overflow.
function paths(n, m) {
  const total = BigInt(n + m - 2);
  const k = BigInt(Math.min(n - 1, m - 1));
  let res = 1n;
  for (let i = 1n; i <= k; i++) {
    res = (res * (total - k + i)) / i;
  }
  return res;
}

console.log(paths(2, 2).toString());
console.log(paths(5, 5).toString());
