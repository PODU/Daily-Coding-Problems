// Day 852: maximum circular subarray sum (empty allowed -> 0).
// answer = max(maxKadane clamped at 0, total - minKadane). O(n) time, O(1) space.
function maxCircular(a) {
  let total = 0, maxK = -Infinity, minK = Infinity, curMax = 0, curMin = 0;
  for (const x of a) {
    total += x;
    curMax = Math.max(x, curMax + x); maxK = Math.max(maxK, curMax);
    curMin = Math.min(x, curMin + x); minK = Math.min(minK, curMin);
  }
  const nonWrap = Math.max(0, maxK);
  const wrap = total - minK;
  return Math.max(nonWrap, wrap);
}

console.log(maxCircular([8, -1, 3, 4])); // 15
console.log(maxCircular([-4, 5, 1, 0])); // 6
