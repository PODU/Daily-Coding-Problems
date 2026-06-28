// Day 1728: Simulate a fair coin from a biased one (Von Neumann trick).
// Toss biased coin twice; (0,1)->0, (1,0)->1, else retry. Expected P(heads) ~= 0.5.
// Time: O(1) expected tosses per fair() call. Space: O(1).

// Seeded RNG (mulberry32) for reproducible output.
function makeRng(seed) {
  let a = seed >>> 0;
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const rng = makeRng(12345);

// Biased coin: returns 1 with probability p (= 0.3), else 0.
function tossBiased() {
  return rng() < 0.3 ? 1 : 0;
}

// Von Neumann: extract a fair bit from the biased coin.
function fair() {
  while (true) {
    const a = tossBiased();
    const b = tossBiased();
    if (a === 0 && b === 1) return 0;
    if (a === 1 && b === 0) return 1;
  }
}

function main() {
  const N = 100000;
  let heads = 0;
  for (let i = 0; i < N; i++) heads += fair();
  const ratio = heads / N;
  console.log(`Fair coin over ${N} tosses, P(heads) ~= ${ratio.toFixed(2)}`);
}

main();
