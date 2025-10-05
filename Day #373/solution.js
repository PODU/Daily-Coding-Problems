// Day 373: Longest run of consecutive integers formable from the list.
// Hash set; start counting only at run-starts (x with no x-1 present). O(n) time.
'use strict';

function longestConsecutive(nums) {
  const s = new Set(nums);
  let best = 0;
  for (const x of s) {
    if (!s.has(x - 1)) {
      let len = 1, cur = x;
      while (s.has(cur + 1)) { cur++; len++; }
      best = Math.max(best, len);
    }
  }
  return best;
}

console.log(longestConsecutive([5, 2, 99, 3, 4, 1, 100])); // 5
