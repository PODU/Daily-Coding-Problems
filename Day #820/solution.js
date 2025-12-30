// First missing positive via cyclic sort: place nums[i] at index nums[i]-1, then scan. O(n) time, O(1) space.

function firstMissingPositive(nums) {
  const n = nums.length;
  for (let i = 0; i < n; i++) {
    while (nums[i] > 0 && nums[i] <= n && nums[nums[i] - 1] !== nums[i]) {
      const j = nums[i] - 1;
      [nums[i], nums[j]] = [nums[j], nums[i]];
    }
  }
  for (let i = 0; i < n; i++) {
    if (nums[i] !== i + 1) return i + 1;
  }
  return n + 1;
}

console.log(firstMissingPositive([3, 4, -1, 1]));
console.log(firstMissingPositive([1, 2, 0]));
