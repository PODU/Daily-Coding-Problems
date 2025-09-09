// Split array into k parts minimizing max partition sum.
// Binary search on answer in [max, sum], greedy feasibility check. O(n log(sum)).
function canSplit(nums, k, cap) {
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
    if (canSplit(nums, k, mid)) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}

const nums = [5, 1, 2, 7, 3, 4];
console.log(splitArray(nums, 3));
