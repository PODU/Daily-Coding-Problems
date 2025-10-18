// Day 454: Longest Increasing Subsequence length.
// Patience sorting with binary search. Time O(n log n), Space O(n).
function lis(a) {
  const tails = [];
  for (const x of a) {
    let lo = 0, hi = tails.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (tails[mid] < x) lo = mid + 1; else hi = mid;
    }
    tails[lo] = x;
  }
  return tails.length;
}

console.log(lis([10, 9, 2, 5, 3, 7, 101, 18])); // 4
