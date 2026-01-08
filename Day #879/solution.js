// Two numbers summing to k via one-pass hash set. Time O(n), Space O(n).

function twoSum(nums, k) {
  const seen = new Set();
  for (const x of nums) {
    if (seen.has(k - x)) return true;
    seen.add(x);
  }
  return false;
}

console.log(twoSum([10, 15, 3, 7], 17));
