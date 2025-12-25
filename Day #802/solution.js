// Day 802: Sample a number per given probability distribution.
// Build CDF prefix sums once O(n); each sample draws u~U(0,1) + binary search O(log n).

// Seeded PRNG (mulberry32) for a deterministic demo.
function mulberry32(a) {
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

class WeightedSampler {
  constructor(nums, probs, seed) {
    this.nums = nums;
    this.cdf = [];
    let acc = 0;
    for (const p of probs) { acc += p; this.cdf.push(acc); }
    this.rng = mulberry32(seed);
  }
  sample() {
    const u = this.rng();
    let lo = 0, hi = this.cdf.length - 1;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.cdf[mid] < u) lo = mid + 1; else hi = mid;
    }
    return this.nums[lo];
  }
}

const numbers = [1, 2, 3, 4];
const probs = [0.1, 0.5, 0.2, 0.2];
const s = new WeightedSampler(numbers, probs, 42);
const trials = 100000;
const count = new Map(numbers.map((n) => [n, 0]));
for (let i = 0; i < trials; i++) {
  const v = s.sample();
  count.set(v, count.get(v) + 1);
}
for (const n of numbers) console.log(`${n}: ${(count.get(n) / trials).toFixed(2)}`);
// ~ 1:0.10  2:0.50  3:0.20  4:0.20
