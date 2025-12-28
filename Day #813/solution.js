// Non-decreasing with at most 1 modification: single pass counting violations,
// greedily lower nums[i-1] or raise nums[i]. Time O(n), Space O(1).

function canBeNonDecreasing(input) {
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

function main() {
  console.log(canBeNonDecreasing([10, 5, 7]));
  console.log(canBeNonDecreasing([10, 5, 1]));
}

main();
