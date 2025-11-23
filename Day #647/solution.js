// Partition equal subset sum: if total odd -> false, else subset-sum DP for sum/2.
// Time O(n*sum), Space O(sum).
function canPartition(nums) {
  const total = nums.reduce((a, b) => a + b, 0);
  if (total % 2 !== 0) return false;
  const target = total / 2;
  const dp = new Array(target + 1).fill(false);
  dp[0] = true;
  for (const x of nums)
    for (let j = target; j >= x; --j)
      if (dp[j - x]) dp[j] = true;
  return dp[target];
}

const a = [15, 5, 20, 10, 35, 15, 10];
const b = [15, 5, 20, 10, 35];
console.log(canPartition(a) ? "true" : "false");
console.log(canPartition(b) ? "true" : "false");
