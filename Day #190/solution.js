// Day 190: Maximum circular subarray sum, empty subarray (sum 0) allowed.
// ans = max(0, maxKadane, total - minKadane). Time O(n), Space O(1).
function maxCircularSum(a) {
  let total = 0, maxK = -Infinity, curMax = 0, minK = Infinity, curMin = 0;
  for (const x of a) {
    total += x;
    curMax = Math.max(x, curMax + x); maxK = Math.max(maxK, curMax);
    curMin = Math.min(x, curMin + x); minK = Math.min(minK, curMin);
  }
  return Math.max(0, maxK, total - minK);
}

console.log(maxCircularSum([8, -1, 3, 4]));
console.log(maxCircularSum([-4, 5, 1, 0]));
