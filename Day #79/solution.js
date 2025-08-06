// Day 79: Can array become non-decreasing with at most one modification?
// Greedy: on each violation, prefer lowering nums[i]; allow only one fix. Time O(n), Space O(1).
function checkPossibility(input) {
  const nums = input.slice();
  let count = 0;
  for (let i = 1; i < nums.length; i++) {
    if (nums[i - 1] > nums[i]) {
      if (++count > 1) return false;
      if (i < 2 || nums[i - 2] <= nums[i]) nums[i - 1] = nums[i];
      else nums[i] = nums[i - 1];
    }
  }
  return true;
}

console.log(checkPossibility([10, 5, 7])); // true
console.log(checkPossibility([10, 5, 1])); // false
