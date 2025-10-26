// Day 495: Reservoir sampling (size 1) from a stream of unknown length.
// For the i-th element (1-indexed) keep it with probability 1/i. Uses O(1) memory.
// Time: O(n) per pass, Space: O(1).

// Processes a stream (any iterable) without storing it; rng() returns a uniform double in [0,1).
function reservoirSample(stream, rng) {
  let chosen = null;
  let count = 0;
  for (const x of stream) {
    count++;
    if (rng() < 1 / count) chosen = x;
  }
  return chosen;
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

function* range(a, b) {
  for (let i = a; i <= b; i++) yield i;
}

const rng = mulberry32(42);
const TRIALS = 100000;
const counts = new Map();
for (let t = 0; t < TRIALS; t++) {
  const v = reservoirSample(range(1, 10), rng);
  counts.set(v, (counts.get(v) || 0) + 1);
}

console.log("Empirical selection frequency per element (~0.100 each):");
for (let v = 1; v <= 10; v++) {
  console.log(`${v}: ${((counts.get(v) || 0) / TRIALS).toFixed(3)}`);
}
console.log("One sampled value: " + reservoirSample(range(1, 10), rng));
