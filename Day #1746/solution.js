// Day 1746: Weighted random sampler.
// Approach: prefix-sum (CDF) of probabilities + binary search on uniform U[0,1).
// Build O(n), sample O(log n) time, O(n) space.
'use strict';

function buildCDF(probs) {
  const cdf = [];
  let acc = 0;
  for (const p of probs) { acc += p; cdf.push(acc); }
  return cdf;
}

function sample(nums, cdf) {
  const r = Math.random();
  let lo = 0, hi = cdf.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (cdf[mid] < r) lo = mid + 1; else hi = mid;
  }
  return nums[lo];
}

function main() {
  const numbers = [1, 2, 3, 4];
  const probs = [0.1, 0.5, 0.2, 0.2];
  const cdf = buildCDF(probs);

  const N = 1000000;
  const cnt = new Map();
  for (let i = 0; i < N; i++) {
    const v = sample(numbers, cdf);
    cnt.set(v, (cnt.get(v) || 0) + 1);
  }
  console.log(`Observed frequencies over ${N} samples:`);
  for (const x of numbers)
    console.log(`${x}: ${(100.0 * (cnt.get(x) || 0) / N).toFixed(1)}%`);
  console.log('Expected: 1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time');
}

main();
