// Largest consecutive range via hash set: from each start (n-1 absent) extend up. O(n) time, O(n) space.
function largestRange(nums) {
  const s = new Set(nums);
  let bestLo = nums[0], bestHi = nums[0], bestLen = 0;
  for (const n of s) {
    if (s.has(n - 1)) continue;
    let hi = n;
    while (s.has(hi + 1)) hi++;
    if (hi - n + 1 > bestLen) { bestLen = hi - n + 1; bestLo = n; bestHi = hi; }
  }
  return [bestLo, bestHi];
}

const nums = [9, 6, 1, 3, 8, 10, 12, 11];
const [lo, hi] = largestRange(nums);
console.log(`(${lo}, ${hi})`);
