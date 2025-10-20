// Day 461: Number of right/down paths in an N x M grid = C(N+M-2, N-1).
// Multiplicative binomial. Time O(min(N,M)), Space O(1).
function countPaths(n, m) {
  const a = (n - 1) + (m - 1), b = Math.min(n - 1, m - 1);
  let res = 1n;
  for (let i = 1; i <= b; i++) {
    res = (res * BigInt(a - b + i)) / BigInt(i);
  }
  return res;
}

console.log(countPaths(2, 2).toString()); // 2
console.log(countPaths(5, 5).toString()); // 70
