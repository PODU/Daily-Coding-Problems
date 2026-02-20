// Day 1105: Partition multiset into two equal-sum subsets (subset-sum DP).
// If total odd -> false; else can we reach total/2. Time: O(N*sum). Space: O(sum).
function canPartition(nums) {
  const total = nums.reduce((a, b) => a + b, 0);
  if (total % 2 === 1) return false;
  const target = total / 2;
  const dp = new Array(target + 1).fill(false);
  dp[0] = true;
  for (const x of nums)
    for (let s = target; s >= x; s--)
      if (dp[s - x]) dp[s] = true;
  return dp[target];
}

console.log(canPartition([15, 5, 20, 10, 35, 15, 10])); // true
console.log(canPartition([15, 5, 20, 10, 35]));         // false
