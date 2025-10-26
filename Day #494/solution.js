// Day 494: Maximum circular subarray sum (empty allowed => 0).
// max(maxKadane, total - minKadane); if all negative answer is 0. Time O(n), Space O(1).
function maxCircularSubarray(a) {
  let total = 0;
  let maxK = -Infinity, curMax = 0;
  let minK = Infinity, curMin = 0;
  for (const x of a) {
    total += x;
    curMax = Math.max(x, curMax + x);
    maxK = Math.max(maxK, curMax);
    curMin = Math.min(x, curMin + x);
    minK = Math.min(minK, curMin);
  }
  if (maxK < 0) return 0; // all negative -> empty subarray
  return Math.max(maxK, total - minK);
}

console.log(maxCircularSubarray([8, -1, 3, 4])); // 15
console.log(maxCircularSubarray([-4, 5, 1, 0])); // 6
