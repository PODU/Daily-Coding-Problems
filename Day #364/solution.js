// Day 364: Longest strictly increasing subsequence length.
// Patience sorting: keep tails[], binary-search first tail >= x and replace.
// Time O(n log n), Space O(n).
'use strict';

function lis(a) {
  const tails = [];
  for (const x of a) {
    let lo = 0, hi = tails.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (tails[mid] < x) lo = mid + 1;
      else hi = mid;
    }
    tails[lo] = x;
  }
  return tails.length;
}

console.log(lis([10, 9, 2, 5, 3, 7, 101, 18])); // 4
