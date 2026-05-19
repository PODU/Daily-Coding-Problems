// Maximum contiguous subarray sum (Kadane), empty subarray allowed (min 0).
// Time O(n), Space O(1).
function maxSubarray(a) {
  let best = 0, cur = 0;
  for (const x of a) {
    cur = Math.max(0, cur + x);
    best = Math.max(best, cur);
  }
  return best;
}

console.log(maxSubarray([34, -50, 42, 14, -5, 86]));
