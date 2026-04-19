// Von Neumann fair coin from a biased toss: toss twice, (0,1)->0, (1,0)->1, else retry. O(1) expected per fair toss.
'use strict';

// Seeded RNG (mulberry32) for reproducibility.
function mulberry32(seed) {
  let a = seed >>> 0;
  return function () {
    a |= 0; a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const rng = mulberry32(12345);

function tossBiased() { return rng() < 0.3 ? 1 : 0; }

function fairToss() {
  while (true) {
    const a = tossBiased(), b = tossBiased();
    if (a === 0 && b === 1) return 0;
    if (a === 1 && b === 0) return 1;
  }
}

function main() {
  const n = 100000;
  let heads = 0;
  for (let i = 0; i < n; i++) heads += fairToss();
  console.log(`heads fraction: ${(heads / n).toFixed(2)}`);
}

main();
