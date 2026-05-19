// Two numbers summing to k via a single-pass hash set.
// Time O(n), Space O(n).
function hasPairSum(nums, k) {
  const seen = new Set();
  for (const x of nums) {
    if (seen.has(k - x)) return true;
    seen.add(x);
  }
  return false;
}

console.log(hasPairSum([10, 15, 3, 7], 17));
