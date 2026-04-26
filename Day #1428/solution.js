// Day 1428: Partition array into two subsets minimizing sum difference.
// Approach: subset-sum DP over half the total; reconstruct one subset.
// Time: O(n * sum), Space: O(n * sum).

function minSubsetDiff(a) {
  const n = a.length;
  const total = a.reduce((x, y) => x + y, 0);
  const half = Math.floor(total / 2);
  const dp = Array.from({ length: n + 1 }, () => new Array(half + 1).fill(false));
  for (let i = 0; i <= n; i++) dp[i][0] = true;
  for (let i = 1; i <= n; i++)
    for (let s = 0; s <= half; s++) {
      dp[i][s] = dp[i - 1][s];
      if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) dp[i][s] = true;
    }

  let best = 0;
  for (let s = half; s >= 0; s--)
    if (dp[n][s]) { best = s; break; }

  const subset = [];
  let s = best;
  for (let i = n; i >= 1; i--) {
    if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) {
      subset.push(a[i - 1]);
      s -= a[i - 1];
    }
  }

  return { diff: total - 2 * best, subset };
}

const { diff, subset } = minSubsetDiff([5, 10, 15, 20, 25]);
console.log("Difference:", diff); // Difference: 5
console.log("Subset:", subset);
