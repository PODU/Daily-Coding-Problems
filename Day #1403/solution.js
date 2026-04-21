// Monte-Carlo simulation of two stop conditions plus exact theory.
// E[rolls until 5->6] = 36 (distinct faces); E[rolls until 5->5] = 42 (same face).
// Time O(trials * rolls), Space O(1).

// Seeded PRNG (mulberry32) for reproducibility.
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const rand = mulberry32(42);
const roll = () => Math.floor(rand() * 6) + 1;

function play(second) {
  let rolls = 0, prev = 0;
  for (;;) {
    const cur = roll();
    rolls++;
    if (prev === 5 && cur === second) return rolls;
    prev = cur;
  }
}

const trials = 200000;
let s56 = 0, s55 = 0;
for (let i = 0; i < trials; i++) s56 += play(6);
for (let i = 0; i < trials; i++) s55 += play(5);
console.log(`Game 1 (five then six): simulated avg = ${(s56 / trials).toFixed(2)}, theoretical = 36`);
console.log(`Game 2 (five then five): simulated avg = ${(s55 / trials).toFixed(2)}, theoretical = 42`);
console.log("Alice should play Game 1 (five-then-six): fewer expected rolls, less pay.");
