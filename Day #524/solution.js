// Kadane's algorithm: max contiguous subarray sum, empty subarray allowed (>=0).
// Time O(n), Space O(1).
function maxSubarraySum(a) {
  let best = 0, cur = 0; // empty subarray allowed -> min 0
  for (const x of a) {
    cur = Math.max(0, cur + x);
    best = Math.max(best, cur);
  }
  return best;
}

console.log(maxSubarraySum([34, -50, 42, 14, -5, 86]));
console.log(maxSubarraySum([-5, -1, -8, -9]));
