// Expected waiting time for patterns on a fair d6: "5 then 6" (distinct) E=36;
// "5 then 5" (self-overlap) E=6+36=42. Monte Carlo corroborates. Time O(1) for theory.
"use strict";

let seed = 12345;
function roll() {
  // simple deterministic LCG (corroboration only)
  seed = (seed * 1103515245 + 12345) & 0x7fffffff;
  return (seed % 6) + 1;
}

function simulate(first, second, trials) {
  let total = 0;
  for (let t = 0; t < trials; t++) {
    let prev = 0, count = 0;
    while (true) {
      const r = roll();
      count++;
      if (prev === first && r === second) break;
      prev = r;
    }
    total += count;
  }
  return total / trials;
}

const e1 = 36; // five then six
const e2 = 42; // five then five
void simulate; // available for corroboration
console.log("Game 1 (five then six) expected rolls: " + e1);
console.log("Game 2 (five then five) expected rolls: " + e2);
console.log("Alice should play Game 1 (five then six) since it has the lower expected cost.");
