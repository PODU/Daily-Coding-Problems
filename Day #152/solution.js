// Day 152: Weighted random sampling. Build cumulative distribution, draw u in
// [0,1) and binary-search the bucket. Preprocess O(n), per-sample O(log n).
'use strict';

class WeightedSampler {
  constructor(nums, probs) {
    this.nums = nums;
    this.cdf = [];
    let acc = 0;
    for (const p of probs) {
      acc += p;
      this.cdf.push(acc);
    }
  }
  sample() {
    const u = Math.random();
    let lo = 0, hi = this.cdf.length - 1;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.cdf[mid] < u) lo = mid + 1;
      else hi = mid;
    }
    return this.nums[lo];
  }
}

const nums = [1, 2, 3, 4];
const probs = [0.1, 0.5, 0.2, 0.2];
const s = new WeightedSampler(nums, probs);
const N = 1000000;
const counts = new Map(nums.map((n) => [n, 0]));
for (let i = 0; i < N; i++) {
  const v = s.sample();
  counts.set(v, counts.get(v) + 1);
}
for (const n of nums) {
  console.log(`${n}: ${((100.0 * counts.get(n)) / N).toFixed(2)}%`);
}
