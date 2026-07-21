// Day 1852: Longest Increasing Subsequence (strict).
// Patience sorting: maintain tails[]; binary-search insertion point. O(n log n) time, O(n) space.

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
