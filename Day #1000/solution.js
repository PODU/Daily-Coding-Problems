// Day 1000: Minimum of a rotated sorted array (no duplicates).
// Binary search comparing mid with the right end. O(log N) time, O(1) space.

function findMin(nums) {
  let lo = 0,
    hi = nums.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (nums[mid] > nums[hi]) lo = mid + 1;
    else hi = mid;
  }
  return nums[lo];
}

console.log(findMin([5, 7, 10, 3, 4])); // 3
