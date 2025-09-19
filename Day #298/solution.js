// Longest contiguous subarray with at most 2 distinct values via sliding window + count map.
// Time: O(n), Space: O(1) (at most 3 keys in map).
function longestAtMost2(a) {
  const cnt = new Map();
  let left = 0, best = 0;
  for (let right = 0; right < a.length; right++) {
    cnt.set(a[right], (cnt.get(a[right]) || 0) + 1);
    while (cnt.size > 2) {
      const v = a[left];
      cnt.set(v, cnt.get(v) - 1);
      if (cnt.get(v) === 0) cnt.delete(v);
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}

console.log(longestAtMost2([2, 1, 2, 3, 3, 1, 3, 5]));
