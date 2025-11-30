// Day 674: Longest contiguous subarray with at most 2 distinct values. Sliding window.
// Time O(n), Space O(1) (at most 3 keys in the map).
function longestTwoTypes(a) {
  const cnt = new Map();
  let best = 0, l = 0;
  for (let r = 0; r < a.length; r++) {
    cnt.set(a[r], (cnt.get(a[r]) || 0) + 1);
    while (cnt.size > 2) {
      cnt.set(a[l], cnt.get(a[l]) - 1);
      if (cnt.get(a[l]) === 0) cnt.delete(a[l]);
      l++;
    }
    best = Math.max(best, r - l + 1);
  }
  return best;
}

console.log(longestTwoTypes([2, 1, 2, 3, 3, 1, 3, 5])); // 4
