// Day 1033: Minimum subset-sum difference (partition into two subsets).
// Boolean subset-sum DP over reachable sums up to total/2; answer total-2*best. O(n*sum) time, O(sum) space.
function minDiff(a) {
  const total = a.reduce((x, y) => x + y, 0);
  const half = Math.floor(total / 2);
  const dp = new Array(half + 1).fill(false);
  dp[0] = true;
  for (const x of a)
    for (let s = half; s >= x; s--) if (dp[s - x]) dp[s] = true;
  for (let s = half; s >= 0; s--) if (dp[s]) return total - 2 * s;
  return total;
}

console.log(minDiff([5, 10, 15, 20, 25]));
