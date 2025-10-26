// Day 493: Sample from a discrete distribution.
// Precompute cumulative probabilities; binary-search a uniform r in [0,1).
// Time: O(n) preprocessing, O(log n) per sample. Space: O(n).

class DiscreteSampler {
  constructor(numbers, probs) {
    this.numbers = numbers;
    this.cdf = [];
    let acc = 0;
    for (const p of probs) {
      acc += p;
      this.cdf.push(acc);
    }
  }

  sample(r) {
    // first index whose cdf > r
    let lo = 0, hi = this.cdf.length - 1;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (this.cdf[mid] > r) hi = mid;
      else lo = mid + 1;
    }
    return this.numbers[lo];
  }
}

// Deterministic PRNG (mulberry32) for a reproducible demo.
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const numbers = [1, 2, 3, 4];
const probs = [0.1, 0.5, 0.2, 0.2];
const sampler = new DiscreteSampler(numbers, probs);

const rng = mulberry32(42);
const N = 100000;
const counts = new Map();
for (let i = 0; i < N; i++) {
  const v = sampler.sample(rng());
  counts.set(v, (counts.get(v) || 0) + 1);
}
for (const n of numbers) {
  console.log(`${n}: ${((counts.get(n) || 0) / N).toFixed(3)}`);
}
