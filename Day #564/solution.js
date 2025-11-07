// Day 564: Largest sum of non-adjacent numbers.
// DP tracking incl/excl running maxes. Time O(n), Space O(1).
function largestNonAdjacent(nums) {
  let incl = 0, excl = 0; // best sums including / excluding previous element
  for (const x of nums) {
    const newIncl = excl + x;
    const newExcl = Math.max(incl, excl);
    incl = newIncl;
    excl = newExcl;
  }
  return Math.max(incl, excl);
}

console.log(largestNonAdjacent([2, 4, 6, 2, 5]));
console.log(largestNonAdjacent([5, 1, 1, 5]));
