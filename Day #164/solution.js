// Day 164: Find duplicate via Floyd's cycle detection (values as next-pointers).
// O(n) time, O(1) extra space (beats the O(n)-space requirement).
'use strict';

function findDuplicate(nums) {
  let slow = nums[0], fast = nums[0];
  do {
    slow = nums[slow];
    fast = nums[nums[fast]];
  } while (slow !== fast);
  slow = nums[0];
  while (slow !== fast) {
    slow = nums[slow];
    fast = nums[fast];
  }
  return slow;
}

const nums = [1, 2, 3, 4, 2]; // n = 4, length n+1
console.log(findDuplicate(nums)); // 2
