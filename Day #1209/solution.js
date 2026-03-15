// Day 1209: Largest divisible subset.
// Sort, dp[i]=longest chain ending at i with parent pointers. Time O(n^2), Space O(n).
function largestDivisibleSubset(a) {
  a = a.slice().sort((x, y) => x - y);
  const n = a.length;
  if (n === 0) return [];
  const dp = new Array(n).fill(1), par = new Array(n).fill(-1);
  let best = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++)
      if (a[i] % a[j] === 0 && dp[j] + 1 > dp[i]) { dp[i] = dp[j] + 1; par[i] = j; }
    if (dp[i] > dp[best]) best = i;
  }
  const res = [];
  for (let i = best; i !== -1; i = par[i]) res.push(a[i]);
  return res.reverse();
}

console.log(largestDivisibleSubset([3, 5, 10, 20, 21])); // [ 5, 10, 20 ]
