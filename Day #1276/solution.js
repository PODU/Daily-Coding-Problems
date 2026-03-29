// Day 1276: Partition a multiset into two subsets of equal sum.
// Subset-sum DP (can we reach totalSum/2?). Time O(n*S), Space O(S).
function canPartition(nums) {
  const total = nums.reduce((a, b) => a + b, 0);
  if (total % 2) return false;
  const target = total / 2;
  const dp = new Uint8Array(target + 1);
  dp[0] = 1;
  for (const x of nums)
    for (let s = target; s >= x; --s)
      if (dp[s - x]) dp[s] = 1;
  return dp[target] === 1;
}

console.log(canPartition([15, 5, 20, 10, 35, 15, 10]));
console.log(canPartition([15, 5, 20, 10, 35]));
