// Two dice games via Monte Carlo simulation. Expected rolls: "5 then 6"=36, "5 then 5"=42.
// Time: O(trials * rolls_per_trial). Space: O(1).
'use strict';

// Seeded PRNG (mulberry32) for reproducibility.
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function simulate(first, second, trials, rand) {
  let total = 0;
  for (let t = 0; t < trials; t++) {
    let prev = 0, rolls = 0;
    for (;;) {
      const r = Math.floor(rand() * 6) + 1;
      rolls++;
      if (prev === first && r === second) break;
      prev = r;
    }
    total += rolls;
  }
  return total / trials;
}

function main() {
  const rand = mulberry32(12345);
  const trials = 500000;
  const e1 = simulate(5, 6, trials, rand);
  const e2 = simulate(5, 5, trials, rand);
  console.log(`Game 1 (five then six) expected rolls: ${e1.toFixed(2)}`);
  console.log(`Game 2 (five then five) expected rolls: ${e2.toFixed(2)}`);
  console.log("Alice should play: Game 1 (five then six)");
}

main();
