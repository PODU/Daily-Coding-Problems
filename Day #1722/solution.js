// Day 1722: Approximate median via random sampling.
// Sample a sublinear number of elements (~constant), return their exact median.
// With high probability its rank lies in [N/4, 3N/4]. Time: O(s log s) << O(N), Space: O(s).
'use strict';

// Deterministic mulberry32 PRNG for reproducible output.
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function approxMedian(a, rand) {
  const n = a.length;
  const s = Math.min(n, 99); // sublinear sample size
  const sample = [];
  for (let i = 0; i < s; i++) sample.push(a[Math.floor(rand() * n)]);
  sample.sort((x, y) => x - y);
  return sample[s >> 1];
}

function main() {
  // Demo: values 0..99 shuffled deterministically.
  const N = 100;
  const a = Array.from({ length: N }, (_, i) => i);
  const rand = mulberry32(42);
  for (let i = N - 1; i > 0; i--) {
    const j = Math.floor(rand() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }

  const m = approxMedian(a, rand);
  const rank = a.filter((x) => x < m).length;
  console.log(`Approximate median: ${m} (rank ${rank} within [N/4, 3N/4] = `
    + `[${Math.floor(N / 4)}, ${Math.floor(3 * N / 4)}])`);
}

main();
