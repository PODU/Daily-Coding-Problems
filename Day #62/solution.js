// Count lattice paths in N x M grid via combinatorics C(n+m-2, n-1).
// Time O(min(n,m)) multiplicative, Space O(1).
function paths(n, m) {
  const down = n - 1, right = m - 1;
  const k = Math.min(down, right), total = down + right;
  let res = 1;
  for (let i = 1; i <= k; ++i) {
    res = (res * (total - k + i)) / i;
  }
  return res;
}

console.log(`2 by 2 matrix -> ${paths(2, 2)}`);
console.log(`5 by 5 matrix -> ${paths(5, 5)}`);
