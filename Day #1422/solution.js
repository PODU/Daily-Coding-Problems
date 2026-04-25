// Day 1422: largest sum of non-adjacent numbers (values may be 0/negative).
// Approach: rolling include/exclude DP; empty subset allowed (floor 0). O(n) time, O(1) space.

function maxNonAdjacent(nums) {
  let incl = 0, excl = 0;
  for (const n of nums) {
    const newIncl = excl + n;
    const newExcl = Math.max(incl, excl);
    incl = newIncl;
    excl = newExcl;
  }
  return Math.max(incl, excl);
}

console.log(maxNonAdjacent([2, 4, 6, 2, 5])); // 13
console.log(maxNonAdjacent([5, 1, 1, 5]));    // 10
