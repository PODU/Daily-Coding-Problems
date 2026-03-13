// Recover coin denominations from change-ways array A (unbounded coin change).
// dp starts [1,0,...]; if A[i] != dp[i], i is a coin -> unbounded-knapsack update. O(N^2) time, O(N) space.

function recoverCoins(A) {
  const n = A.length;
  const dp = new Array(n).fill(0);
  dp[0] = 1;
  const coins = [];
  for (let i = 1; i < n; i++) {
    if (A[i] !== dp[i]) {
      coins.push(i);
      for (let v = i; v < n; v++) dp[v] += dp[v - i];
    }
  }
  return coins;
}

function formatList(xs) {
  const n = xs.length;
  return xs
    .map((x, i) => (i === n - 1 && n > 1 ? "and " + x : String(x)))
    .join(", ");
}

const A = [1, 0, 1, 1, 2];
console.log(formatList(recoverCoins(A))); // 2, 3, and 4
