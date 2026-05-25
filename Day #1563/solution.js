// Paths in grid = C(N+M-2, N-1), computed multiplicatively to avoid overflow. Time O(min(N,M)), Space O(1).
function countPaths(n, m) {
  const total = n + m - 2;
  const k = Math.min(n - 1, m - 1);
  let res = 1;
  for (let i = 1; i <= k; i++) {
    res = (res * (total - k + i)) / i;
  }
  return Math.round(res);
}

function main() {
  console.log(countPaths(2, 2));
}

main();
