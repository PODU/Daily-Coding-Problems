// Day 1431: Majority element (appears > floor(n/2)).
// Approach: Boyer-Moore voting. Time: O(n), Space: O(1).

function majorityElement(nums) {
  let count = 0;
  let candidate = null;
  for (const x of nums) {
    if (count === 0) candidate = x;
    count += x === candidate ? 1 : -1;
  }
  return candidate;
}

console.log(majorityElement([1, 2, 1, 1, 3, 4, 0])); // 1
