// Day 198: Largest divisible subset.
// Sort, then DP: dp[i] = longest chain ending at i (a[j] | a[i]); track parent for reconstruction.
// Time: O(n^2), Space: O(n).
function largestDivisibleSubset(arr) {
  const a = [...arr].sort((x, y) => x - y);
  const n = a.length;
  if (n === 0) return [];
  const dp = new Array(n).fill(1);
  const parent = new Array(n).fill(-1);
  let best = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++)
      if (a[i] % a[j] === 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; parent[i] = j; }
    if (dp[i] > dp[best]) best = i;
  }
  const res = [];
  for (let i = best; i !== -1; i = parent[i]) res.push(a[i]);
  return res.reverse();
}

console.log(largestDivisibleSubset([3, 5, 10, 20, 21])); // [5, 10, 20]
console.log(largestDivisibleSubset([1, 3, 6, 24]));      // [1, 3, 6, 24]
