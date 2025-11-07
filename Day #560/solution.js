// Two-sum existence: one pass with a hash set of seen numbers.
// Time: O(N), Space: O(N).
'use strict';

function hasPairWithSum(nums, k) {
  const seen = new Set();
  for (const x of nums) {
    if (seen.has(k - x)) return true;
    seen.add(x);
  }
  return false;
}

function main() {
  const nums = [10, 15, 3, 7];
  const k = 17;
  console.log(hasPairWithSum(nums, k));
}

main();
