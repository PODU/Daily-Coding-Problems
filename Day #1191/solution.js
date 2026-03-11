// Approximate median: take a constant-size random sample and return its median.
// Sublinear: O(k log k) for constant k samples, independent of N. Space: O(k).
"use strict";

// Simple seeded PRNG (mulberry32) for reproducibility.
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function approxMedian(a) {
  const N = a.length;
  const k = Math.min(N, 31);
  const rng = mulberry32(42); // fixed seed
  const sample = [];
  for (let i = 0; i < k; i++) sample.push(a[Math.floor(rng() * N)]);
  sample.sort((x, y) => x - y);
  const med = sample[Math.floor(sample.length / 2)];
  let rank = 0;
  for (const x of a) if (x <= med) rank++;
  return { med, rank };
}

const a = Array.from({ length: 100 }, (_, i) => i + 1);
const N = a.length;
const { med, rank } = approxMedian(a);
console.log(`Approximate median: ${med} (rank ${rank} within [${Math.floor(N / 4)}, ${Math.floor((3 * N) / 4)}])`);
