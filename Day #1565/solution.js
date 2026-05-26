// Subset-sum DP: partition into equal halves iff target=sum/2 reachable. Time O(n*sum), Space O(sum).
function canPartition(nums) {
  const total = nums.reduce((a, b) => a + b, 0);
  if (total % 2 !== 0) return false;
  const target = total / 2;
  const dp = new Array(target + 1).fill(false);
  dp[0] = true;
  for (const x of nums) {
    for (let s = target; s >= x; s--) {
      if (dp[s - x]) dp[s] = true;
    }
  }
  return dp[target];
}

function main() {
  const nums = [15, 5, 20, 10, 35, 15, 10];
  console.log(canPartition(nums) ? "true" : "false");
}

main();
