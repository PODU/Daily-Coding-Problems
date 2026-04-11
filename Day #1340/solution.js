// Count grid paths (right/down only) via combinatorics C(N+M-2, M-1).
// Overflow-safe multiplicative loop with BigInt. Time O(N+M), Space O(1).

function countPaths(n, m) {
  const total = n + m - 2;
  const k = Math.min(n - 1, m - 1);
  let res = 1n;
  for (let i = 1; i <= k; i++) {
    res = (res * BigInt(total - k + i)) / BigInt(i);
  }
  return res;
}

console.log(`2 by 2 -> ${countPaths(2, 2)}`);
console.log(`5 by 5 -> ${countPaths(5, 5)}`);
