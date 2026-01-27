// Day 967: Find the duplicate in array of n+1 ints from {1..n}.
// Approach: Floyd's tortoise & hare on value->index cycle. Time O(n), Space O(1).

function findDuplicate(nums) {
  let slow = nums[0], fast = nums[0];
  do {
    slow = nums[slow];
    fast = nums[nums[fast]];
  } while (slow !== fast);
  slow = nums[0];
  while (slow !== fast) { slow = nums[slow]; fast = nums[fast]; }
  return slow;
}

console.log(findDuplicate([1, 3, 4, 2, 2])); // 2
