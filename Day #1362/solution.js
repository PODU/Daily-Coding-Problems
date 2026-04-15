// Longest contiguous subarray with at most two distinct values.
// Sliding window + hashmap of counts, shrink when distinct > 2. Time O(n), Space O(1).
'use strict';

function longestTwoDistinct(a) {
  const cnt = new Map();
  let left = 0;
  let best = 0;
  for (let right = 0; right < a.length; right++) {
    cnt.set(a[right], (cnt.get(a[right]) || 0) + 1);
    while (cnt.size > 2) {
      const c = cnt.get(a[left]) - 1;
      if (c === 0) cnt.delete(a[left]);
      else cnt.set(a[left], c);
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}

const a = [2, 1, 2, 3, 3, 1, 3, 5];
console.log(longestTwoDistinct(a));
