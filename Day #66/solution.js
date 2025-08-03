// Von Neumann extractor: toss biased coin twice; (0,1)->0, (1,0)->1, else retry => 50/50. Time O(1) expected.

// Deterministic seeded PRNG (mulberry32) so the demo is stable.
let seed = 12345;
function rand() {
  seed |= 0;
  seed = (seed + 0x6D2B79F5) | 0;
  let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
  t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
}

function tossBiased() { // simulated bias p = 0.3 for 1
  return rand() < 0.3 ? 1 : 0;
}

function tossFair() {
  while (true) {
    const a = tossBiased();
    const b = tossBiased();
    if (a === 0 && b === 1) return 0;
    if (a === 1 && b === 0) return 1;
  }
}

const trials = 100000;
let ones = 0;
for (let i = 0; i < trials; i++) ones += tossFair();
const frac = ones / trials;
if (!(frac > 0.48 && frac < 0.52)) throw new Error("not fair");
console.log("Fair coin ~0.5");
