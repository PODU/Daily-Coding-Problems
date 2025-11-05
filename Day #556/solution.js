// Non-decreasing with at most one modification: single pass, greedy fix. O(n) time, O(1) space.
function checkPossibility(nums) {
  nums = nums.slice();
  let cnt = 0;
  for (let i = 1; i < nums.length; i++) {
    if (nums[i] < nums[i - 1]) {
      if (++cnt > 1) return false;
      if (i < 2 || nums[i - 2] <= nums[i]) nums[i - 1] = nums[i];
      else nums[i] = nums[i - 1];
    }
  }
  return true;
}

console.log(checkPossibility([10, 5, 7]));
console.log(checkPossibility([10, 5, 1]));
