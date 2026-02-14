// Day 1064: Implement rand7() from rand5() via rejection sampling.
// (rand5()-1)*5 + rand5() -> uniform 1..25; reject >21; ((v-1)%7)+1. Expected O(1) calls, O(1) space.

// Deterministic seeded PRNG (mulberry32) so the demo is reproducible.
function mulberry32(seed) {
  return function () {
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const rand = mulberry32(42);

function rand5() {
  return Math.floor(rand() * 5) + 1;
}

function rand7() {
  while (true) {
    const v = (rand5() - 1) * 5 + rand5(); // uniform 1..25
    if (v <= 21) return ((v - 1) % 7) + 1;
  }
}

const counts = new Array(8).fill(0);
for (let i = 0; i < 70000; i++) counts[rand7()]++;
for (let i = 1; i <= 7; i++) console.log(`${i}: ${counts[i]}`);
