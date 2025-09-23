// Approximate median: median of k random samples (seeded RNG) -> rank in [N/4, 3N/4] whp.
// Time: O(k log k), o(N) for k<N; Space: O(k).

// Simple seeded PRNG (mulberry32) for deterministic demo.
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function approxMedian(a, k, seed) {
  const rng = mulberry32(seed);
  const sample = [];
  for (let i = 0; i < k; i++) sample.push(a[Math.floor(rng() * a.length)]);
  sample.sort((x, y) => x - y);
  return sample[Math.floor(k / 2)];
}

const a = [];
for (let i = 0; i <= 100; i++) a.push(i); // N = 101, values 0..100
const N = a.length;
const val = approxMedian(a, 15, 42);
const rank = val; // rank in sorted 0..100 equals value
const ok = Math.floor(N / 4) <= rank && rank <= Math.floor((3 * N) / 4);
console.log(`Approximate median is within [N/4, 3N/4]: ${ok ? "true" : "false"}`);
