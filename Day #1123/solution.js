// Day 1123 - Largest divisible subset
// Sort, LIS-style DP where j extends i if a[i] % a[j] == 0; reconstruct via
// parent pointers. Time: O(n^2), Space: O(n).

function largestDivisibleSubset(nums) {
  if (nums.length === 0) return [];
  nums = [...nums].sort((a, b) => a - b);
  const n = nums.length;
  const dp = new Array(n).fill(1);
  const parent = new Array(n).fill(-1);
  let best = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < i; j++)
      if (nums[i] % nums[j] === 0 && dp[j] + 1 > dp[i]) {
        dp[i] = dp[j] + 1;
        parent[i] = j;
      }
    if (dp[i] > dp[best]) best = i;
  }
  const res = [];
  for (let k = best; k !== -1; k = parent[k]) res.push(nums[k]);
  return res.reverse();
}

console.log(largestDivisibleSubset([3, 5, 10, 20, 21])); // [5, 10, 20]
console.log(largestDivisibleSubset([1, 3, 6, 24]));      // [1, 3, 6, 24]
