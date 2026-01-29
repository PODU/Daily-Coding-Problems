// Longest Increasing Subsequence via patience sorting: maintain a "tails" array and
// binary-search the insertion point for each element. Time O(n log n), Space O(n).
function lengthOfLIS(nums) {
  const tails = []; // tails[i] = smallest tail of an increasing subseq of length i+1
  for (const x of nums) {
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

const nums = [10, 9, 2, 5, 3, 7, 101, 18];
console.log("Input:", JSON.stringify(nums));
console.log("LIS length:", lengthOfLIS(nums)); // 4
