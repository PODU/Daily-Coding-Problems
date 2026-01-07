// Approximate median: take a small random sample (size s) and return its median.
// Rank lands in [N/4, 3N/4] w.h.p. Time O(s log s) = sub-linear, Space O(s).

// Seeded PRNG (mulberry32) for reproducible demo output.
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function approxMedian(a, sampleSize, rand) {
  const n = a.length;
  const s = Math.min(sampleSize, n);
  const sample = [];
  for (let i = 0; i < s; i++) sample.push(a[Math.floor(rand() * n)]);
  sample.sort((x, y) => x - y);
  return sample[Math.floor(s / 2)];
}

const rand = mulberry32(42);
const n = 1000;
const a = Array.from({ length: n }, (_, i) => i);
for (let i = n - 1; i > 0; i--) {
  const j = Math.floor(rand() * (i + 1));
  [a[i], a[j]] = [a[j], a[i]];
}
const m = approxMedian(a, 51, rand);
const rank = a.filter((x) => x < m).length;
console.log("approx median =", m);
const ok = rank >= n / 4 && rank <= (3 * n) / 4;
console.log(`rank ${rank} in [${n / 4}, ${(3 * n) / 4}]: ${ok}`);
