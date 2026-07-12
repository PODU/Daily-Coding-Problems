// Longest contiguous subarray with at most 2 distinct values via sliding window + hashmap. O(n) time, O(1) space.
function longestTwoDistinct(a) {
  const cnt = new Map();
  let left = 0, best = 0;
  for (let right = 0; right < a.length; right++) {
    cnt.set(a[right], (cnt.get(a[right]) || 0) + 1);
    while (cnt.size > 2) {
      cnt.set(a[left], cnt.get(a[left]) - 1);
      if (cnt.get(a[left]) === 0) cnt.delete(a[left]);
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}

console.log(longestTwoDistinct([2, 1, 2, 3, 3, 1, 3, 5])); // 4
