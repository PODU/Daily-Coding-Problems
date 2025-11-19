// Split array into k contiguous parts minimizing the max partition sum.
// Binary search on answer in [max, total]; greedy feasibility check. O(n log(sum)).
'use strict';

function feasible(nums, k, cap) {
  let parts = 1, cur = 0;
  for (const x of nums) {
    if (cur + x > cap) { parts++; cur = x; }
    else cur += x;
  }
  return parts <= k;
}

function splitArray(nums, k) {
  let lo = Math.max(...nums);
  let hi = nums.reduce((a, b) => a + b, 0);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (feasible(nums, k, mid)) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}

console.log(splitArray([5, 1, 2, 7, 3, 4], 3)); // expected 8
