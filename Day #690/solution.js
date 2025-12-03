// Longest strictly Increasing Subsequence length via patience sorting + binary search.
// Time O(N log N), Space O(N). Also reconstructs one valid LIS.
'use strict';

function lisLength(nums) {
  const tails = [];      // value of smallest tail per length
  const tailsIdx = [];   // index in nums of that tail
  const prev = new Array(nums.length).fill(-1);
  for (let i = 0; i < nums.length; i++) {
    const x = nums[i];
    // lower_bound
    let lo = 0, hi = tails.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (tails[mid] < x) lo = mid + 1;
      else hi = mid;
    }
    const pos = lo;
    tails[pos] = x;
    tailsIdx[pos] = i;
    prev[i] = pos > 0 ? tailsIdx[pos - 1] : -1;
  }
  const seq = [];
  let k = tailsIdx.length ? tailsIdx[tailsIdx.length - 1] : -1;
  while (k !== -1) {
    seq.push(nums[k]);
    k = prev[k];
  }
  seq.reverse();
  return [tails.length, seq];
}

function main() {
  const nums = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
  const [len, seq] = lisLength(nums);
  console.log(len);
  console.log('[' + seq.join(', ') + ']');
}

main();
